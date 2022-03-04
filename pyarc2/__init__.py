from .pyarc2 import Instrument as __InstrumentLL
from .pyarc2 import BiasOrder, ControlMode, DataMode, WaitFor
from .pyarc2 import ReadAt, ReadAfter, ArC2Error
from .pyarc2 import find_ids as _find_ids

from dataclasses import dataclass
from functools import partial
from enum import Enum
import numpy as np


find_ids = _find_ids
""" Find all available device ids """


class IdleMode(Enum):
    """
    IdleMode is used with `Instrument.finalise_operation` to
    mark at what state the channels should be left. Selecting
    `Float` will disconnect all channels and leave their state
    unchanged. `SoftGnd` will reset all channels to arbitrary voltage
    operation and set them to 0.0 V. `HardGnd` will disconnect all
    channels from the DACs and connect them to hard ground.
    """

    Float: int = 0b01
    SoftGnd: int = 0b10
    HardGnd: int = 0b11


@dataclass
class ArC2Config:
    """
    Convenience dataclass to group ArC2 configuration options.
    """
    idleMode: IdleMode
    controlMode: ControlMode


class Instrument(__InstrumentLL):

    def __init__(self, port, firmware):
        """
        Initialise a new ArC2 connection. Argument `port` is the EFM id that
        can be found with `pyarc2.find_ids` and `firmware` is the path to
        the FPGA firmware to load when initialising.
        """
        super(Instrument, self).__init__()

    def _array_iter_inner(self, mode):
        data = self.pick_one(mode)
        if data is None:
            return None
        return [data]

    def get_iter(self, mode):
        """
        Return an iteration on the internal data buffer. This allows
        users to iterate through the saved results on ArC2's memory
        in the order they were saved. The available modes of retrieval
        are outlined in `DataMode`.

        >>> from pyarc2 import Instrument, ReadAt, ReadAfter, DataMode, IdleMode
        >>> arc = Instrument(0, '/path/to/firmware')
        >>> arc.generate_ramp(3, 3, 0.0, 0.1, 1.0, 1e-7, 10e-6, 5, ReadAt.Bias, ReadAfter.Pulse)
        >>>    .execute()
        >>>    .finalise_operation(IdleMode.Gnd)
        >>>    .wait()
        >>> data = arc.get_iter(DataMode.Bits)
        >>> for datum in data:
        >>>     print(datum) # 32-element array containing bitline currents
        """
        fn = partial(self._array_iter_inner, mode)
        return iter(fn, None)

    def finalise_operation(self, mode=None, control=None):
        """
        This function is used to safely reset channels and daughterboard
        control at the end of an operation. The available options are outlined
        in `IdleMode` and `ControlMode`.  Please note that floating the
        channels will disconnect them and leave them in the configuration they
        were before. For instance at the end of a fast operation (fast pulses,
        fast ramps, etc) the channels will still be left in a High Speed driver
        mode. However explicitly grounding the devices will switch them to
        arbitrary voltage (incurring the 120 μs penalty to do so). Setting any
        of the two arguments as `None` will retain existing configuration.
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
