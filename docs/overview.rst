Overview of pyarc2
==================

Basic functionality
-------------------

Most of the functionality of the library is exposed via the
:class:`pyarc2.Instrument` class. You would typically initialise a new
Instrument instance and interact with ArC TWO through its methods. For example
this is how you can read the current between two ArC TWO crosspoints.

.. code-block:: pycon

   >>> from pyarc2 import Instrument, find_ids
   >>> ids = find_ids()
   >>> if len(ids) == 0:
   >>>     # no devices found
   >>>     return
   >>> # fw.bin is the firmware to load on ArC TWO
   >>> arc = Instrument(ids[0], 'fw.bin')
   >>> # perform a current read between channels 0 and 22
   >>> # at 200 mV
   >>> current = arc.read_one(0, 22, 0.2)
   6.35432e-6

Instead of reading a single crosspoint one could read a whole word- or bit-line.
For example this would return all currents along a single column.

.. code-block:: pycon

   >>> from pyarc2 import Instrument, find_ids
   >>> ids = find_ids()
   >>> if len(ids) == 0:
   >>>     return
   >>> arc = Instrument(ids[0], 'fw.bin')
   >>> data = arc.read_slice(22, 0.2)
   >>> print(type(data))
   numpy.ndarray
   >>> print(data.shape)
   (32, )

In this particular case channel 22 is selected as the sink channel. Internally
this is considered as a "wordline" so method
:meth:`~pyarc2.Instrument.read_slice` will measure the current along all
bitlines with channel 22 as the corresponding low-voltage (typically grounded)
channel. As a general guidance channels 0 to 15 and 32 to 47 are considered
"bitline channels" (aka rows) whereas channels 16 to 31 and 48 to 64 considered
"wordline channels" (aka columns). Currently this is hardcoded within
``libarc2`` but might get fully configurable in the future. Presently this
corresponds to a typical usecase of having channels arranged in 32×32 crossbar
array fashion.

Result layout
-------------

With the exception of functions that operate on a single crosspoint most
methods will report a block of data from the FPGA's memory. Since almost always
this value is a current value the raw ADC output from ArC TWO will be decoded
into a meaningful value. As a general rule memory on the ArC TWO FPGA is
subdivided in blocks of 256 bytes. This is essentially one value (4 bytes) per
each one of the 64 available channels.  Typically functions that operate on
more than one device will return all 64 values or, where applicable, the 32
values that correspond to the selected word- or bitline. Functions that operate
on the whole array will return a numpy ndarray with 32 rows and 32 columns
(shape ``(2, 2)``) to closely match the layout of a typical crossbar array.

Operation lifecycle and command buffer
--------------------------------------

In addition to read/pulse operations ``libarc2`` also exposes a set of complex
functions. These correspond to typical scenarios in a testing flow and although
they do add complexity they are useful components in a testing toolkit. As
these are typically sequential or time-dependent operations ``libarc2`` will
spawn a background thread to offload instructions to ArC TWO and gather results
into an internal buffer. These will be made available as soon as they come. This
is an example of generating a voltage ramp consisting of 2 pulses per step ranging
from 0.0 V to 1.0 V with 0.1 V step and apply it on crosspoint (0, 22). Pulse
width is 1 μs and interpulse 10 μs.

.. code-block:: python

   from pyarc2 import Instrument, find_ids, ReadAt, ReadAfter, \
       IdleMode, DataMode
   import numpy as np
   ids = find_ids()
   if len(ids) == 0:
       return
   arc = Instrument(ids[0], 'fw.bin')
   # ensure all channels are detached from GND first
   arc.connect_to_gnd(np.array([], dtype=np.uint64))
   # generate the ramp instruction, do 2 programming pulses
   # at each step, then read after each set of 2 pulses (a block)
   # at arbitrary voltage (200 mV)
   arc.generate_ramp(22, 0, 0.0, 0.1, 1.0, 1000, 10000, 2, \
       ReadAt.Arb(0.2), ReadAfter.Block)
   # then switch all channels back to GND
   arc.finalise_operation(IdleMode.SoftGND)
   # and submit it for execution
   arc.execute()
   # the ramp is now being applied...
   # start picking the data, we will read the wordline values as
   # channel 22 is a word channel. `get_iter` will return an
   # iterator on the internal output buffer which will block until
   # either a new result is in or the operation has finished (and
   # in that case the loop will break)
   for datum in arc.get_iter(DataMode.Words):
       # datum now holds all the wordline currents. However
       # since only channel 22 is selected all other values
       # are NaN
       print(datum.shape)
       # (32, )
       # ...

There is quite a lot of information to unpack here. This is our first
interaction with the **command buffer**. Internally ArC TWO has a command
buffer that schedules instructions for execution. With the exception of
methods that return a single value (essentially read or pulse read operations
on one device/slice/array which return either a single value or numpy ndarray)
all other commands are initially submitted to the command buffer for execution.
This does not happen until :meth:`~pyarc2.Instrument.execute` is called. At this
point ArC TWO will go over each one of its command on the buffer and execute them
sequentially. You can check if ArC TWO is executing instructions by using the
:meth:`~pyarc2.Instrument.busy` method. You can also block until all instructions
have been executed by using the :meth:`pyarc2.Instrument.wait` method.

The lifecyle of an operation typically consists of (a) releasing the channels
from GND; (b) calling the necessary method; (c) grounding or re-floating the
channels by selecting an idle mode and (d) calling
:meth:`~pyarc2.Instrument.execute`. On the example above step (a) is the call
to :meth:`~pyarc2.Instrument.connect_to_gnd`. This will connect all selected
channels to GND, however rather unintuitively in this case no channels have
been selected because the argument is an empty numpy ndarray. As this is
internally a bitmask the empty array clears that bitmask, effectively releasing
all channels from GND. The operation (step b) is the call to
:meth:`~pyarc2.Instrument.generate_ramp`. This is a complex ramp generator that
optionally allows for reading devices after each pulse or block of pulses (as
in this case). The final two arguments of
:meth:`~pyarc2.Instrument.generate_ramp` are essentially pythonic versions of
Rust enums that are used as flags that control the operation of the ramp (see
:class:`~pyarc2.ReadAt` and :class:`~pyarc2.ReadAfter`). As a general remark
Rust enums are exposed in Python as classes with static fields
(``ReadAfter.Block`` is such an example) or static functions
(``ReadAt.Arb(0.2)`` in this case). Step (c) is reflected in the call of
:meth:`~pyarc2.Instrument.finalise_operation`. This method essentially
sets the idle state of the channels. In this case we are setting all channels
to 0.0 V ("soft" ground). See :class:`~pyarc2.Instrument.IdleMode` for more.
Note that up to this point **no command has been executed**. It's only when
:meth:`~pyarc2.Instrument.execute` is called that ArC TWO starts to apply
the queued instructions.

In the example above :meth:`~pyarc2.Instrument.generate_ramp` generates a
complex set of instructions which generates quite a lot of results. These are
stored in the FPGA memory (what we call the *internal output buffer*).  They
can be retrieved independently of the state of the command buffer (ie.
regardless if ArC TWO is busy executing or not) by using the
:meth:`~pyarc2.Instrument.get_iter` method. It will iterate over all the
available values freeing memory slots from the FPGA memory as it goes. If a
result is not yet available it will block until it is. The iterator will
terminate if an operation has finished executing and all data is retrieved.

A note about types
------------------

The API of ``pyarc2`` closely matches that of libarc2_ and tries to wrap it as
faithfully as possibly. To that extent ``pyarc2`` is not a extremely high-level
fully pythonic layer although certain provisions have been made to adapt to the
specifics of the Python lax type system. At least on the functions exposed by
:class:`~pyarc2.Instrument` the only valid arguments are floats, integers and
numpy arrays. No automatic conversion is done between lists and numpy arrays.

Expanding functionality
-----------------------

``pyarc2`` can be used to implement new Python-facing APIs but using the
low-level Rust codebase instead. This is especially relevant if you application
has performance-critical parts that are bottlenecked by Python or if you want
tighter control of the ArC TWO instruction pipeline. You can access the
underlying Rust object that ``pyarc2`` wraps via ``Instrument::inner()`` or
``Instrument::inner_mut()`` functions. However this does require some degree of
familiarity with Rust, the Python C API and libarc2_ itself.


.. _libarc2: https://github.com/arc-instruments/libarc2
.. _beastlink: https://www.cesys.com/en/our-products/software-ip/beastlink.html
.. _`CESYS download page`: https://www.cesys.com/en/service-support/download-center/fpga.html
.. _`our repository`: https://github.com/arc-instruments/beastlink-rs/tree/master/contrib
.. _website: http://arc-instruments.co.uk/
.. _arc2control: https://github.com/arc-instruments/arc2control
