use libarc2::{Instrument, BiasOrder, ControlMode, DataMode, ReadAt, ReadAfter, find_ids};
use ndarray::{Ix1, Ix2, Array};
use numpy::{PyArray, PyReadonlyArray};
use std::convert::{From, Into};
use numpy::convert::IntoPyArray;
use pyo3::prelude::{pymodule, pyclass, pymethods};
use pyo3::prelude::{PyModule, PyRefMut, PyResult, Python};
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

#[pyclass(name="ReadAt", module="pyarc2")]
#[derive(Clone)]
struct PyReadAt { _inner: ReadAt }

#[allow(non_snake_case)]
#[pymethods]
impl PyReadAt {

    #[classattr]
    fn Bias() -> PyReadAt {
        PyReadAt { _inner: ReadAt::Bias }
    }

    #[staticmethod]
    fn Arb(voltage: f32) -> PyReadAt {
        PyReadAt { _inner: ReadAt::Arb(voltage) }
    }

    #[classattr]
    fn Never() -> PyReadAt {
        PyReadAt { _inner: ReadAt::Never }
    }

    fn voltage(&self) -> PyResult<f32> {
        match self._inner {
            ReadAt::Arb(v) => Ok(v),
            _ => Err(exceptions::PyException::new_err("No voltage associated"))
        }
    }
}

impl From<ReadAt> for PyReadAt {
    fn from(readat: ReadAt) -> Self {
        PyReadAt { _inner: readat }
    }
}

impl From<PyReadAt> for ReadAt {
    fn from(readat: PyReadAt) -> Self {
        readat._inner
    }
}

#[pyclass(name="ReadAfter", module="pyarc2")]
#[derive(Clone)]
struct PyReadAfter { _inner: ReadAfter }

#[allow(non_snake_case)]
#[pymethods]
impl PyReadAfter {

    #[classattr]
    fn Pulse() -> PyReadAfter {
        PyReadAfter { _inner: ReadAfter::Pulse }
    }

    #[classattr]
    fn Ramp() -> PyReadAfter {
        PyReadAfter { _inner: ReadAfter::Ramp }
    }

    #[classattr]
    fn Block() -> PyReadAfter {
        PyReadAfter { _inner: ReadAfter::Block }
    }

    #[classattr]
    fn Never() -> PyReadAfter {
        PyReadAfter { _inner: ReadAfter::Never }
    }
}

impl From<ReadAfter> for PyReadAfter {
    fn from(readafter: ReadAfter) -> Self {
        PyReadAfter { _inner: readafter }
    }
}

impl From<PyReadAfter> for ReadAfter {
    fn from(readafter: PyReadAfter) -> Self {
        readafter._inner
    }
}

#[pyclass(name="ControlMode", module="pyarc2")]
#[derive(Clone)]
struct PyControlMode{ _inner: ControlMode }

#[allow(non_snake_case)]
#[pymethods]
impl PyControlMode {

    #[classattr]
    fn Header() -> PyControlMode {
        PyControlMode { _inner: ControlMode::Header }
    }

    #[classattr]
    fn Internal() -> PyControlMode {
        PyControlMode { _inner: ControlMode::Internal }
    }
}

impl From<ControlMode> for PyControlMode {
    fn from(order: ControlMode) -> Self {
        PyControlMode { _inner: order }
    }
}

impl From<PyControlMode> for ControlMode {
    fn from(order: PyControlMode) -> Self {
        order._inner
    }
}

#[pyclass(name="DataMode", module="pyarc2")]
#[derive(Clone)]
struct PyDataMode { _inner: DataMode }

#[allow(non_snake_case)]
#[pymethods]
impl PyDataMode {

    #[classattr]
    fn Words() -> PyDataMode {
        PyDataMode { _inner: DataMode::Words }
    }

    #[classattr]
    fn Bits() -> PyDataMode {
        PyDataMode { _inner: DataMode::Bits }
    }

    #[classattr]
    fn All() -> PyDataMode {
        PyDataMode { _inner: DataMode::All }
    }
}

impl From<DataMode> for PyDataMode {
    fn from(mode: DataMode) -> Self {
        PyDataMode { _inner: mode }
    }
}

impl From<PyDataMode> for DataMode {
    fn from(mode: PyDataMode) -> Self {
        mode._inner
    }
}

#[pyclass(name="Instrument", module="pyarc2", subclass)]
pub struct PyInstrument {
    _instrument: Instrument
}

impl PyInstrument {

    /// Returns a reference to the underlying Instrument
    pub fn inner(&self) -> &Instrument {
        &self._instrument
    }

    /// Returns a mutable reference to the underlying Instrument
    pub fn inner_mut(&mut self) -> &mut Instrument {
        &mut self._instrument
    }
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
    /// Ground all channels and revert them to arbitrary voltage operation.
    fn ground_all<'py>(mut slf: PyRefMut<'py, Self>) -> PyResult<PyRefMut<'py, Self>> {
        match slf._instrument.ground_all() {
            Ok(_) => Ok(slf),
            Err(err) => Err(exceptions::PyException::new_err(err))
        }
    }

    /// ground_all_fast(self, /)
    /// --
    ///
    /// Ground all channels maintaing current channel operating mode.
    fn ground_all_fast<'py>(mut slf: PyRefMut<'py, Self>) -> PyResult<PyRefMut<'py, Self>> {
        match slf._instrument.ground_all_fast() {
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

        match slf._instrument.pulse_one(low, high, voltage, nanos, true) {
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

        match slf._instrument.pulse_slice(chan, voltage, nanos, true) {
            Ok(_) => Ok(slf),
            Err(err) => Err(exceptions::PyException::new_err(err))
        }
    }

    /// pulse_slice_masked(self, chan, mask, voltage, nanos, /)
    /// --
    ///
    /// Apply a pulse to a row or column using `chan` as the low channel with specified voltage
    /// and pulse width (in nanoseconds) and also limit the high channels to those specified
    /// by the mask array.
    fn pulse_slice_masked<'py>(mut slf: PyRefMut<'py, Self>, chan: usize, voltage: f32, nanos: u128, mask: PyReadonlyArray<usize, Ix1>)
        -> PyResult<PyRefMut<'py, Self>> {

        let actual_mask = mask.as_slice().unwrap();

        match slf._instrument.pulse_slice_masked(chan, actual_mask, voltage, nanos, true) {
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

        match slf._instrument.pulse_all(voltage, nanos, order.into(), true) {
            Ok(_) => Ok(slf),
            Err(err) => Err(exceptions::PyException::new_err(err))
        }
    }

    /// pulseread_one(self, low, high, vpulse, nanos, vread, /)
    /// --
    ///
    /// Pulse and then read a crosspoint. Same semantics as ``pulse_one`` and
    /// ``read_one`` apply.
    fn pulseread_one(&mut self, low: usize, high: usize, vpulse: f32, nanos: u128, vread: f32) -> f32 {
        self._instrument.pulseread_one(low, high, vpulse, nanos, vread).unwrap()
    }

    /// pulseread_slice(self, chan, vpulse, nanos, vread, /)
    /// --
    ///
    /// Pulse and then read a row/column. Same semantics as ``pulse_slice`` and
    /// ``read_slice`` apply.
    fn pulseread_slice<'py>(&mut self, py: Python<'py>, chan: usize, vpulse: f32,
        nanos: u128, vread: f32) -> &'py PyArray<f32, Ix1> {
        self._instrument.pulseread_slice_as_ndarray(chan, vpulse, nanos, vread)
            .unwrap().into_pyarray(py)
    }

    /// pulseread_slice_masked(self, chan, mask, vpulse, nanos, vread, /)
    /// --
    ///
    /// Pulse and read specified high channels that have `chan` as low potential
    /// channel. Same semantics as ``pulse_slice_masked`` and ``read_slice_masked``
    /// apply.
    fn pulseread_slice_masked<'py>(&mut self, py: Python<'py>, chan: usize,
        mask: PyReadonlyArray<usize, Ix1>, vpulse: f32, nanos: u128,
        vread: f32) -> &'py PyArray<f32, Ix1> {

        let slice = mask.as_slice().unwrap();
        self._instrument.pulseread_slice_masked_as_ndarray(chan, slice, vpulse, nanos, vread)
            .unwrap()
            .into_pyarray(py)
    }

    /// pulseread_all(self, vpulse, nanos, vread, order, /)
    /// --
    ///
    /// Pulse and read all the crosspoints. Same semantics as ``pulse_all`` and ``read_all``
    /// apply here as well
    fn pulseread_all<'py>(&mut self, py: Python<'py>, vpulse: f32, nanos: u128,
        vread: f32, order: PyBiasOrder) -> &'py PyArray<f32, Ix2> {

        self._instrument.pulseread_all_as_ndarray(vpulse, nanos, vread, order.into())
            .unwrap().into_pyarray(py)
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

    /// set_control_mode(self, mode, /)
    /// --
    ///
    /// Set daughterboard control mode either as Internal or Header
    fn set_control_mode<'py>(mut slf: PyRefMut<'py, Self>, mode: PyControlMode) -> PyResult<PyRefMut<'py, Self>> {
        match slf._instrument.set_control_mode(mode.into()) {
            Ok(_) => Ok(slf),
            Err(err) => Err(exceptions::PyException::new_err(err))
        }
    }

    /// currents_from_address(self, addr, channels, /)
    /// --
    ///
    /// Read current values from specific address segment. This will return all
    /// the channel values stored in the segment in ascending channel order
    fn currents_from_address<'py>(&self, py: Python<'py>, addr: u32, chans: PyReadonlyArray<usize, Ix1>) -> PyResult<&'py PyArray<f32, Ix1>> {
        match self._instrument.currents_from_address(addr, chans.as_slice().unwrap()) {
            Ok(result) => Ok(Array::from(result).into_pyarray(py)),
            Err(err) => Err(exceptions::PyException::new_err(err))
        }
    }

    /// word_currents_from_address(self, addr, channels, /)
    /// --
    ///
    /// Read all word current values from specific address segment. This will return all
    /// word-related values stored in the segment in ascending channel order
    fn word_currents_from_address<'py>(&self, py: Python<'py>, addr: u32) -> PyResult<&'py PyArray<f32, Ix1>> {
        match self._instrument.word_currents_from_address(addr) {
            Ok(result) => Ok(Array::from(result).into_pyarray(py)),
            Err(err) => Err(exceptions::PyException::new_err(err))
        }
    }

    /// bit_currents_from_address(self, addr, channels, /)
    /// --
    ///
    /// Read all bit current values from specific address segment. This will return all
    /// bit-related values stored in the segment in ascending channel order
    fn bit_currents_from_address<'py>(&self, py: Python<'py>, addr: u32) -> PyResult<&'py PyArray<f32, Ix1>> {
        match self._instrument.bit_currents_from_address(addr) {
            Ok(result) => Ok(Array::from(result).into_pyarray(py)),
            Err(err) => Err(exceptions::PyException::new_err(err))
        }
    }


    /// generate_ramp(self, low, high, vstart, vstep, vstop, pw, inter, npulse, readat, readafter,
    /// /)
    /// --
    ///
    /// Initiate a ramp operation in ArC2. This will spawn a background process that bias
    /// the selected `low` and `high` channels based on the parameters specified.
    fn generate_ramp<'py>(mut slf: PyRefMut<'py, Self>, low: usize, high: usize,
        vstart: f32, vstep: f32, vstop: f32,
        pw_nanos: u128, inter_nanos: u128, num_pulses: usize,
        read_at: PyReadAt, read_after: PyReadAfter) -> PyResult<PyRefMut<'py, Self>> {

        match slf._instrument.generate_ramp(low, high, vstart, vstep, vstop,
            pw_nanos, inter_nanos, num_pulses, read_at.into(),
            read_after.into()) {
            Ok(_) => Ok(slf),
            Err(err) => Err(exceptions::PyException::new_err(err))
        }

    }

    /// pick_one(self, mode, /)
    /// --
    ///
    /// Read a slab of data from the internal long operation buffer. This clears
    /// the memory area after reading.
    fn pick_one<'py>(&mut self, py: Python<'py>, mode: PyDataMode) ->
        PyResult<Option<&'py PyArray<f32, Ix1>>> {

        let mode: DataMode = mode.into();

        match self._instrument.pick_one(mode) {
            Ok(data_opt) => {
                match data_opt {
                    Some(data) => {
                        let array = Array::from(data).into_pyarray(py);
                        Ok(Some(array))
                    },
                    None => Ok(None)
                }
            },
            Err(err) => Err(exceptions::PyException::new_err(err))
        }

    }

}

#[pymodule]
fn pyarc2(_: Python, m: &PyModule) -> PyResult<()> {

    /// find_ids()
    /// --
    ///
    /// Find all available ArC2 devices. This will return a list
    /// with all discovered ids.
    #[pyfn(m)]
    #[pyo3(name="find_ids")]
    fn py_find_ids(_py: Python) -> PyResult<Vec<i32>> {
        match find_ids() {
            Ok(ids) => { Ok(ids) },
            Err(err) => { Err(exceptions::PyException::new_err(err)) }
        }
    }

    m.add_class::<PyInstrument>()?;
    m.add_class::<PyBiasOrder>()?;
    m.add_class::<PyControlMode>()?;
    m.add_class::<PyDataMode>()?;
    m.add_class::<PyReadAt>()?;
    m.add_class::<PyReadAfter>()?;

    Ok(())
}

