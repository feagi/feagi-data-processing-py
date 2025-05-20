use std::collections::HashMap;
use feagi_core_data_structures_and_processing::neuron_state::neuron_data::{NeuronYXCPArrays, CorticalMappedNeuronData};
use feagi_core_data_structures_and_processing::cortical_area_state::cortical_data::{CorticalID};
use pyo3::exceptions::PyValueError;
use pyo3::pyclass;
use pyo3::prelude::*;
use pyo3::types::{PyBytes};

#[pyclass]
#[pyo3(name = "NeuronYXCPArrays")]
pub struct PyNeuronYXCPArrays {
    inner: NeuronYXCPArrays,
}

#[pymethods]
impl PyNeuronYXCPArrays {
    #[new]
    pub fn new(maximum_number_of_neurons_possibly_needed: usize) -> PyResult<Self> {
        let result = NeuronYXCPArrays::new(maximum_number_of_neurons_possibly_needed);
        match result {
            Ok(inner) => Ok(PyNeuronYXCPArrays {inner}),
            Err(e) => Err(PyValueError::new_err(e.to_string()))
        }
    }
    
    #[staticmethod]
    pub fn cortical_mapped_neuron_data_to_bytes<'py>(py: Python<'py>, mapped_data: PyCorticalMappedNeuronData) -> PyResult<Bound<'py, PyBytes>> {
        let result = NeuronYXCPArrays::cortical_mapped_neuron_data_to_bytes(mapped_data.inner);
        match result {
            Ok(vector) => Ok(PyBytes::new(py, &vector)),
            Err(e) => Err(PyValueError::new_err(e.to_string()))
        }
    }
}

// encapsulation for hashmap
#[pyclass]
#[derive(Clone)]
#[pyo3(name = "CorticalMappedNeuronData")]
pub struct PyCorticalMappedNeuronData {
    pub inner: HashMap<CorticalID, NeuronYXCPArrays> // un alias of "CorticalMappedNeuronData"
}
