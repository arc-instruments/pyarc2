use libarc2::{Instrument, BiasOrder, find_ids};
use ndarray::{Ix1, Ix2};
use numpy::{PyArray, PyReadonlyArray};
use std::convert::{From, Into};
use numpy::convert::IntoPyArray;
use pyo3::prelude::{pymodule, pyclass, pymethods, PyModule, PyRefMut, PyResult, Python};
use pyo3::exceptions;


#[pyclass(name="BiasOrder", module="pyarc2")]
#[derive(Clone)]
struct PyBiasOrder{ _inner: BiasOrder }

#[allow(non_snake_case)]
#[pymethods]
impl PyBiasOrder {

    #[classattr]
    fn Rows() -> PyBiasOrder {
        PyBiasOrder { _inner: BiasOrder::Rows }
    }

    #[classattr]
    fn Cols() -> PyBiasOrder {
        PyBiasOrder { _inner: BiasOrder::Columns }
    }
}

impl From<BiasOrder> for PyBiasOrder {
    fn from(order: BiasOrder) -> Self {
        PyBiasOrder { _inner: order }
    }
}

impl From<PyBiasOrder> for BiasOrder {
    fn from(order: PyBiasOrder) -> Self {
        order._inner
    }
}


#[pyclass(name="Instrument", module="pyarc2")]
struct PyInstrument {
    _instrument: Instrument
}


#[pymethods]
impl PyInstrument {
    #[new(name="Instrument")]
    fn new(id: i32, fw: &str) -> PyResult<Self> {
        match Instrument::open_with_fw(id, fw, true) {
            Ok(instr) => Ok(PyInstrument { _instrument: instr }),
            Err(err) => Err(exceptions::PyException::new_err(err))
        }
    }

    /// ground_all(self, /)
    /// --
    ///
    /// Ground all channels.
    fn ground_all<'py>(mut slf: PyRefMut<'py, Self>) -> PyResult<PyRefMut<'py, Self>> {
        match slf._instrument.ground_all() {
            Ok(_) => Ok(slf),
            Err(err) => Err(exceptions::PyException::new_err(err))
        }
    }

    /// float_all(self, /)
    /// --
    ///
    /// Disconnect all channels.
    fn float_all<'py>(mut slf: PyRefMut<'py, Self>) -> PyResult<PyRefMut<'py, Self>> {
        match slf._instrument.float_all() {
            Ok(_) => Ok(slf),
            Err(err) => Err(exceptions::PyException::new_err(err))
        }
    }

    /// read_one(self, low, high, vread, /)
    /// --
    ///
    /// Perform a current read between the specified channels. The low
    /// channel will be biased with -vread and the current will be read
    /// from the high channel.
    fn read_one(&mut self, low: usize, high: usize, vread: f32) -> f32 {
        self._instrument.read_one(low, high, vread).unwrap()
    }

    /// read_slice(self, chan, vread, /)
    /// --
    ///
    /// Read all the values which have `chan` as the low channel. If `chan` is
    /// between 0 and 15 or 32 and 47 (inclusive) this will correspond to a
    /// row read at `vread` in a standard 32×32 array. Otherwise it's a column
    /// read.
    fn read_slice<'py>(&mut self, py: Python<'py>, chan: usize, vread: f32) -> &'py PyArray<f32, Ix1> {
        self._instrument.read_slice_as_ndarray(chan, vread).unwrap().into_pyarray(py)
    }

    /// read_slice_masked(self, chan, mask, vread, /)
    /// --
    ///
    /// Read all the masked high channels which have `chan` as the low channel.
    /// If `chan` is between 0 and 15 or 32 and 47 (inclusive) this will
    /// correspond to a row read at `vread` in a standard 32×32 array. Otherwise
    /// it's a column read.
    fn read_slice_masked<'py>(&mut self, py: Python<'py>, chan: usize, mask: PyReadonlyArray<usize, Ix1>, vread: f32)
        -> &'py PyArray<f32, Ix1> {
        let slice = mask.as_slice().unwrap();
        self._instrument.read_slice_masked_as_ndarray(chan, slice, vread).unwrap().into_pyarray(py)
    }

    /// read_all(self, vread, order, /)
    /// --
    ///
    /// Read all the available crosspoints at the specified voltage. This can be
    /// done either by biasing columns (BiasOrder.ROWS) or rows (BiasOrder.COLS).
    fn read_all<'py>(&mut self, py: Python<'py>, vread: f32, order: PyBiasOrder) -> &'py PyArray<f32, Ix2> {
        self._instrument.read_all_as_ndarray(vread, order.into()).unwrap().into_pyarray(py)
    }

    /// pulse_one(self, low, high, voltage, nanos, /)
    /// --
    ///
    /// Apply a pulse between the specified crosspoints with specified voltage and
    /// pulse width (in nanoseconds).
    fn pulse_one<'py>(mut slf: PyRefMut<'py, Self>, low: usize, high: usize, voltage: f32, nanos: u128)
        -> PyResult<PyRefMut<'py, Self>> {

        match slf._instrument.pulse_one(low, high, voltage, nanos) {
            Ok(_) => Ok(slf),
            Err(err) => Err(exceptions::PyException::new_err(err))
        }
    }

    /// pulse_slice(self, chan, voltage, nanos, /)
    /// --
    ///
    /// Apply a pulse to a row or column using `chan` as the low channel with specifid voltage
    /// and pulse width (in nanoseconds).
    fn pulse_slice<'py>(mut slf: PyRefMut<'py, Self>, chan: usize, voltage: f32, nanos: u128)
        -> PyResult<PyRefMut<'py, Self>> {

        match slf._instrument.pulse_slice(chan, voltage, nanos) {
            Ok(_) => Ok(slf),
            Err(err) => Err(exceptions::PyException::new_err(err))
        }
    }

    /// pulse_slice_masked(self, chan, mask, voltage, nanos, /)
    /// --
    ///
    /// Apply a pulse to a row or column using `chan` as the low channel with specifid voltage
    /// and pulse width (in nanoseconds) and also limit the high channels to those specified
    /// by the mask array.
    fn pulse_slice_masked<'py>(mut slf: PyRefMut<'py, Self>, chan: usize, voltage: f32, nanos: u128, mask: PyReadonlyArray<usize, Ix1>)
        -> PyResult<PyRefMut<'py, Self>> {

        let actual_mask = mask.as_slice().unwrap();

        match slf._instrument.pulse_slice_masked(chan, actual_mask, voltage, nanos) {
            Ok(_) => Ok(slf),
            Err(err) => Err(exceptions::PyException::new_err(err))
        }

    }

    /// pulse_all(self, voltage, nanos, order, /)
    /// --
    ///
    /// Pulse all crosspoints in the array, either by high biasing rows (BiasOrder.ROWS) or
    /// columns (BiasOrder.COLS).
    fn pulse_all<'py>(mut slf: PyRefMut<'py, Self>, voltage: f32, nanos: u128, order: PyBiasOrder)
        -> PyResult<PyRefMut<'py, Self>> {

        match slf._instrument.pulse_all(voltage, nanos, order.into()) {
            Ok(_) => Ok(slf),
            Err(err) => Err(exceptions::PyException::new_err(err))
        }
    }

    /// execute(self, /)
    /// --
    ///
    /// Write everything in the command buffer to the instrument. This will cause ArC2
    /// to start executing the instructions provided.
    fn execute<'py>(mut slf: PyRefMut<'py, Self>) -> PyResult<PyRefMut<'py, Self>> {
        match slf._instrument.execute() {
            Ok(_) => Ok(slf),
            Err(err) => Err(exceptions::PyException::new_err(err))
        }
    }

    /// busy(self, /)
    /// --
    ///
    /// Returns `True` if the command buffer has not been consumed.
    fn busy(&self) -> bool {
        self._instrument.busy()
    }

    /// wait(self, /)
    /// --
    ///
    /// Block until the instrument has executed its command buffer.
    fn wait(&self) {
        self._instrument.wait();
    }

}

#[pymodule]
fn pyarc2(_: Python, m: &PyModule) -> PyResult<()> {

    /// find_ids()
    /// --
    ///
    /// Find all available ArC2 devices. This will return a list
    /// with all discovered ids.
    #[pyfn(m, "find_ids")]
    fn py_find_ids(_py: Python) -> PyResult<Vec<i32>> {
        match find_ids() {
            Ok(ids) => { Ok(ids) },
            Err(err) => { Err(exceptions::PyException::new_err(err)) }
        }
    }

    m.add_class::<PyInstrument>()?;
    m.add_class::<PyBiasOrder>()?;

    Ok(())
}

