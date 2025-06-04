use pyo3::{pyclass, pymethods, PyResult};
use pyo3::prelude::*;
use pyo3::exceptions::PyValueError;
use feagi_core_data_structures_and_processing::neuron_data::neuron_mappings::*;
use crate::cortical_data::{PyCorticalID};
use super::neuron_arrays::{PyNeuronXYZPArrays};

#[pyclass]
#[derive(Clone)]
#[pyo3(name = "CorticalMappedXYZPNeuronData")]
pub struct PyCorticalMappedXYZPNeuronData { // HashMap<CorticalID, NeuronYXCPArrays>
    pub inner: CorticalMappedXYZPNeuronData
}

#[pymethods]
impl PyCorticalMappedXYZPNeuronData {
    #[new]
    pub fn new() -> PyCorticalMappedXYZPNeuronData {
        PyCorticalMappedXYZPNeuronData {
            inner: CorticalMappedXYZPNeuronData::new()
        }
    }

    pub fn insert(&mut self, cortical_id: PyCorticalID, data: PyNeuronXYZPArrays) -> PyResult<()> {
        if self.inner.contains(cortical_id.inner.clone()) { // TODO fix clone
            return Err(PyValueError::new_err(format!("Cortical ID of {} already exists in this CorticalMappedNeuronData object!", cortical_id.as_str())));
        }
        self.inner.insert(cortical_id.inner, data.inner);
        Ok(())
    }

    pub fn contains(&self, cortical_id: PyCorticalID) -> PyResult<bool> {
        Ok(self.inner.contains(cortical_id.inner))
    }

}