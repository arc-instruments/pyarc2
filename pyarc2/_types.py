from typing import List, Iterable, Optional, Union, Callable, ClassVar, Any, cast
import numpy as np

IntIterable = Union[Iterable[int], np.ndarray]
NpUint = Union[type[np.uint64], type[np.uint32], type[np.uint16]]
