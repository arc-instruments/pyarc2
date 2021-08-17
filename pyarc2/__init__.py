from .pyarc2 import Instrument as InstrumentLL
from .pyarc2 import BiasOrder, ControlMode, DataMode
from .pyarc2 import ReadAt, ReadAfter
from .pyarc2 import find_ids

from functools import partial
from enum import Enum


class IdleMode(Enum):
    Float: int = 0b01
    Gnd: int = 0b10


class Instrument(InstrumentLL):

    def __init__(self, x, y):
        super(Instrument, self).__init__()

    def _array_iter_inner(self, mode):
        data = self.pick_one(mode)
        if data is None:
            return None
        return [data]

    def get_iter(self, mode):
        fn = partial(self._array_iter_inner, mode)
        return iter(fn, None)

    def finalise_operation(self, mode):
        if mode == IdleMode.Float:
            self.ground_all_fast().float_all().execute()
        elif mode == IdleMode.Gnd:
            self.ground_all().execute()
