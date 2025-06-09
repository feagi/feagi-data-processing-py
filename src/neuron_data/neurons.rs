use feagi_core_data_structures_and_processing::neuron_data::neuron_arrays::NeuronXYZPArrays;
use pyo3::{pyclass, pymethods, PyResult};
use pyo3::prelude::*;
use feagi_core_data_structures_and_processing::neuron_data::neurons::NeuronXYZP;

#[pyclass]
#[derive(Clone)]
#[pyo3(name = "NeuronXYZP")]
pub struct PyNeuronXYZP {
    pub inner: NeuronXYZP,
}

#[pymethods]
impl PyNeuronXYZP {
    #[new]
    pub fn new(x: u32, y: u32, z: u32, p: f32) -> Self {
        PyNeuronXYZP {inner: NeuronXYZP::new(x, y, z, p)}
    }

    pub fn as_tuple(&self) -> (u32, u32, u32, f32) {
        self.inner.as_tuple()
    }
}