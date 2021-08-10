from .pyarc2 import Instrument as InstrumentLL
from .pyarc2 import BiasOrder, ControlMode, DataMode
from .pyarc2 import ReadAt, ReadAfter

from functools import partial

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
