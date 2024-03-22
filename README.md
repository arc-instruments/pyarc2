# Python bindings for libarc2

[![](https://img.shields.io/badge/-docs-default?logo=gitbook&logoColor=white)](https://files.arc-instruments.co.uk/documents/pyarc2/latest/)

## Introduction

This library presents a python interface to the low-level `libarc2` library
used to interface with ArC TWO™. Most of the user-facing facilities of
`libarc2` are present in this library. That being said, `pyarc2` itself is
still relatively low-level and a general understanding of the internals of
ArC TWO is required.

## Use

`pyarc2` maps most of the functionality of `libarc2` so the API translates
fairly transparent. Most of the interaction with ArC TWO will be through the
`Instrument` class that encapsulates the implemented functionality of
`libarc2`. The library will keep being updated as new functionality is added.
For instance to read the current between two channels you can write

```python
from pyarc2 import Instrument, find_ids

# low voltage channel (typically grounded)
LOWV = 7
# high voltage channel
HIGHV = 33
# read-out voltage
VREAD = 0.2

# Get the ID of the first available ArC TWO
arc2id = find_ids()[0]

# firmware; shipped with your board
fw = 'arc2fw.bin'

# connect to the board
arc = Instrument(arc2id, fw)

current = arc.read_one(LOWV, HIGHV, VREAD)
print('I = %g A' % current)

```

## Additional functionality

`pyarc2` can also be used to implement new plugins based on `libarc2`.  Access
to the lower level object is done via `Instrument::inner()` and
`Instrument::inner_mut()`. These plugins can fully leverage `libarc2`
functionality but require some familiarity with Rust and the `libarc2` API.
