from .pyarc2 import InstrumentLL as _InstrumentLL
from .pyarc2 import BiasOrder, ControlMode, DataMode, ReadType, WaitFor, AuxDACFn
from .pyarc2 import ReadAt, ReadAfter, ArC2Error
from .pyarc2 import find_ids
try:
    from .pyarc2 import LIBARC2_VERSION
except (AttributeError, ImportError):
    LIBARC2_VERSION = None

from collections.abc import Iterable
from dataclasses import dataclass
from functools import partial
from enum import Enum
import numpy as np
from ._types import *


def _inheritdocs(fromfn: Callable, sep: str="\n"):
    def _decorator(fn):
        srcdoc = fromfn.__doc__
        if fn.__doc__ is None:
            fn.__doc__ = srcdoc
        else:
            fn.__doc__ = sep.join([srcdoc, fn.__doc__])
        return fn
    return _decorator


def _ndarray_check(arg: IntIterable, ndim: int = 1, typ: NpUint=np.uint64) -> Optional[np.ndarray]:

    if arg is None:
        return None

    if isinstance(arg, np.ndarray) and arg.dtype == typ and arg.ndim == ndim:
        return arg
    elif isinstance(arg, Iterable) and not isinstance(arg, (str, bytes)):
        a = np.array(arg, dtype=typ)
        if a.ndim == ndim:
            return a
        else:
            raise TypeError('Invalid argument dimensions, must be: %d, ' \
                'found %d instead' % (ndim, a.ndim))
    else:
        raise TypeError('Invalid argument type, must be an iterable')


class IdleMode(Enum):
    """
    IdleMode is used with :meth:`Instrument.finalise_operation` to
    mark at what state the channels should be left. Selecting
    :attr:`Float` will disconnect all channels and leave their state
    unchanged. `SoftGnd` will reset all channels to arbitrary voltage
    operation and set them to 0.0 V. `HardGnd` will disconnect all
    channels from the DACs and connect them to hard ground.
    """

    Float: int = 0b01
    """ Float Channels """
    SoftGnd: int = 0b10
    """ Tie channels to 0 V """
    HardGnd: int = 0b11
    """ Tie channels to GND """


@dataclass
class ArC2Config:
    """
    Convenience dataclass to group ArC2 configuration options.
    """

    idleMode: IdleMode
    controlMode: ControlMode


class Instrument(_InstrumentLL):
    """
    To do anything with ArC TWO you will need first to instantiate an
    ``Instrument``. The constructor requires a port number and a path to load
    fimrware from. Using :meth:`~pyarc2.find_ids` will return all available
    ArC TWO instrument ports.

    >>> from pyarc2 import Instrument, find_ids
    >>> ids = find_ids()
    >>> if len(ids) == 0:  # no instruments ound
    >>>     return
    >>> # Connect to the first available ArC TWO loading firmware "fw.bin"
    >>> arc = Instrument(ids[0], 'fw.bin')

    :param int port: The EFM id of the ArC TWO to connect to
    :param str firwmare: Path of the firmware to load

    :return: A new instance of ``pyarc2.Instrument``
    """

    def __init__(self, port: int, firmware: str):
        _InstrumentLL.__init__(port, firmware)

    def _array_iter_inner(self, mode: DataMode, rtype: ReadType):
        data = self.pick_one(mode, rtype)
        if data is None:
            return None
        return [data]

    def get_iter(self, mode: DataMode, rtype: Optional[ReadType] = None):
        """
        Return an iteration on the internal data buffer. This allows
        users to iterate through the saved results on ArC2's memory
        in the order they were saved. The available modes of retrieval
        are outlined in :class:`pyarc2.DataMode`.

        >>> from pyarc2 import Instrument, ReadAt, ReadAfter, DataMode, IdleMode
        >>> arc = Instrument(0, '/path/to/firmware')
        >>> arc.generate_ramp(3, 3, 0.0, 0.1, 1.0, 1e-7, 10e-6, 5, ReadAt.Bias, ReadAfter.Pulse)
        >>>    .execute()
        >>>    .finalise_operation(IdleMode.Gnd)
        >>>    .wait()
        >>> data = arc.get_iter(DataMode.Bits)
        >>> for datum in data:
        >>>     print(datum) # 32-element array containing bitline currents

        :param mode: A variant of :class:`pyarc2.DataMode`
        :return: An iterator on the internal data buffer
        """

        if rtype is None:
            rtype = ReadType.Current

        fn = partial(self._array_iter_inner, mode, rtype)
        return iter(fn, None)

    def finalise_operation(self, mode: Optional[IdleMode] = None, control: Optional[ControlMode] = None):
        """
        This function is used to safely reset channels and daughterboard
        control at the end of an operation. The available options are outlined
        in ``IdleMode`` and ``ControlMode``.  Please note that floating the
        channels will disconnect them and leave them in the configuration they
        were before. For instance at the end of a fast operation (fast pulses,
        fast ramps, etc) the channels will still be left in a High Speed driver
        mode. However explicitly grounding the devices will switch them to
        arbitrary voltage (incurring the 120 μs penalty to do so). Setting any
        of the two arguments as ``None`` will retain existing configuration.

        :param mode: A variant of :class:`~pyarc2.IdleMode`
        :param control: A variant of :class:`~pyarc2.ControlMode`
        """

        if mode == IdleMode.Float:
            # clear all hard grounds first
            self.connect_to_gnd(np.arange(0, dtype=np.uint64)) \
                .ground_all_fast() \
                .float_all() \
                .execute()
        elif mode == IdleMode.SoftGnd:
            # clear all hard grounds first
            self.connect_to_gnd(np.arange(0, dtype=np.uint64)) \
                .ground_all() \
                .execute()
        elif mode == IdleMode.HardGnd:
            # reset DACs to 0.0, disconnect channels from
            # the DACs and connect them to GND
            self.ground_all_fast() \
                .float_all() \
                .connect_to_gnd(np.arange(64, dtype=np.uint64)) \
                .execute()
        elif mode is None:
            pass
        else:
            raise ArC2Error("Invalid idle mode")

        if control == ControlMode.Header or control == ControlMode.Internal:
            self.set_control_mode(control)
        elif control is None:
            pass
        else:
            raise ArC2Error("Invalid control mode")

    @_inheritdocs(_InstrumentLL.connect_to_gnd)
    def connect_to_gnd(self, chans: IntIterable) -> 'Instrument':
        i = super().connect_to_gnd(_ndarray_check(chans))
        return cast(Instrument, i)

    @_inheritdocs(_InstrumentLL.read_slice_masked)
    def read_slice_masked(self, chan: int, mask: IntIterable, vread: float) -> np.ndarray:
        return super().read_slice_masked(chan, _ndarray_check(mask), vread)

    @_inheritdocs(_InstrumentLL.read_slice_open)
    def read_slice_open(self, highs: IntIterable, ground_after: bool) -> np.ndarray:
        return super().read_slice_open(_ndarray_check(highs), ground_after)

    @_inheritdocs(_InstrumentLL.pulse_slice_masked)
    def pulse_slice_masked(self, chan: int, voltage: float, nanos: int,
        mask: IntIterable) -> 'Instrument':
        i = super().pulse_slice_masked(chan, voltage, nanos, _ndarray_check(mask))
        return cast(Instrument, i)

    @_inheritdocs(_InstrumentLL.pulseread_slice_masked)
    def pulseread_slice_masked(self, chan: int, mask: IntIterable, vpulse: float,
        nanos: int, vread: float) -> np.ndarray:
        return super().pulseread_slice_masked(chan, _ndarray_check(mask), vpulse,
            nanos, vread)

    @_inheritdocs(_InstrumentLL.currents_from_address)
    def currents_from_address(self, addr: int, chans: IntIterable) -> np.ndarray:
        return super().currents_from_address(addr, _ndarray_check(chans))

    @_inheritdocs(_InstrumentLL.vread_channels)
    def vread_channels(self, chans: IntIterable, averaging: bool) -> List[float]:
        return super().vread_channels(_ndarray_check(chans), averaging)

    @_inheritdocs(_InstrumentLL.generate_read_train)
    def generate_read_train(self, lows: Optional[IntIterable], highs: IntIterable,
        vread: float, nreads: int, inter_nanos: int, ground: bool) -> 'Instrument':
        return super().generate_read_train(_ndarray_check(lows),
            _ndarray_check(highs), vread, nreads, inter_nanos, ground)

    @_inheritdocs(_InstrumentLL.generate_vread_train)
    def generate_vread_train(self, chans: IntIterable, averaging: bool, npulses: int,
        inter_nanos: int) -> 'Instrument':
        return super().generate_vread_train(_ndarray_check(chans), averaging, npulses,
            inter_nanos)
