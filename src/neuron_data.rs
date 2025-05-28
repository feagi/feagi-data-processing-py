use std::collections::HashMap;
use pyo3::pyclass;
use pyo3::prelude::*;
use pyo3::types::{PyBytes};
use pyo3::exceptions::PyValueError;
use feagi_core_data_structures_and_processing::neuron_data::{NeuronXYCPArrays, CorticalMappedNeuronData};
use crate::cortical_data::PyCorticalID;

#[pyclass]
#[derive(Clone)]
#[pyo3(name = "CorticalMappedNeuronData")]
pub struct PyCorticalMappedNeuronData { // HashMap<CorticalID, NeuronYXCPArrays>
    pub inner: CorticalMappedNeuronData
}

#[pymethods]
impl PyCorticalMappedNeuronData {
    #[new]
    pub fn new() -> PyCorticalMappedNeuronData {
        PyCorticalMappedNeuronData { 
            inner: CorticalMappedNeuronData::new()
        }
    }
    
    pub fn add(&mut self, cortical_id: PyCorticalID, data: PyNeuronXYCPArrays) -> PyResult<()> {
        if self.inner.contains_key(&cortical_id.inner){
            return Err(PyValueError::new_err(format!("Cortical ID of {} already exists in this CorticalMappedNeuronData object!", cortical_id.as_str())));
        }
        self.inner.insert(cortical_id.inner, data.inner);
        Ok(())
    }
    
    pub fn contains(&self, cortical_id: PyCorticalID) -> PyResult<bool> {
        Ok(self.inner.contains_key(&cortical_id.inner))
    }
    
    pub fn remove(&mut self, cortical_id: PyCorticalID) -> PyResult<()> {
        if self.inner.contains_key(&cortical_id.inner) {
            self.inner.remove(&cortical_id.inner);
            return Ok(())
        };
        Err(PyValueError::new_err(format!("Cortical ID of {} does not exist in this CorticalMappedNeuronData object!", cortical_id.as_str())))
    }
    
}

#[pyclass]
#[derive(Clone)]
#[pyo3(name = "NeuronXYCPArrays")]
pub struct PyNeuronXYCPArrays {
    pub inner: NeuronXYCPArrays,
}

#[pymethods]
impl PyNeuronXYCPArrays {
    #[new]
    pub fn new(maximum_number_of_neurons_possibly_needed: usize) -> PyResult<Self> {
        let result = NeuronXYCPArrays::new(maximum_number_of_neurons_possibly_needed);
        match result {
            Ok(inner) => Ok(PyNeuronXYCPArrays {inner}),
            Err(e) => Err(PyValueError::new_err(e.to_string()))
        }
    }
    
    #[staticmethod]
    pub fn new_from_resolution(resolution: (usize, usize, usize))  -> PyResult<Self> {
        let result = NeuronXYCPArrays::new_from_resolution(resolution);
        match result {
            Ok(inner) => Ok(PyNeuronXYCPArrays {inner}),
            Err(e) => Err(PyValueError::new_err(e.to_string()))
        }
    }

    #[staticmethod]
    pub fn cortical_mapped_neuron_data_to_bytes<'py>(py: Python<'py>, mapped_data: PyCorticalMappedNeuronData)  -> PyResult<Bound<'py, PyBytes>> {
        let result = NeuronXYCPArrays::cortical_mapped_neuron_data_to_bytes(&mapped_data.inner);
        match result {
            Ok(vector) => Ok(PyBytes::new(py, &vector)),
            Err(e) => Err(PyValueError::new_err(e.to_string()))
        }
    }
    
    pub fn get_max_neuron_capacity_without_reallocating(&self) -> PyResult<usize> {
        let result = self.inner.get_max_neuron_capacity_without_reallocating();
        Ok(result)
    }
    
    pub fn get_number_of_neurons_used(&self) -> PyResult<usize> {
        let result = self.inner.get_number_of_neurons_used();
        Ok(result)
    }

    // TODO expose "update_vectors_from_external"
    // TODO expose "expand_to_new_max_count_if_required"
    // TODO expose "reset_indexes"
    
    
}

