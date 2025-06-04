use feagi_core_data_structures_and_processing::byte_structures::feagi_byte_structure::{FeagiByteStructure, FeagiByteStructureCompatible};
use pyo3::{pyclass, pymethods, PyResult};
use pyo3::prelude::*;
use pyo3::exceptions::PyValueError;
use feagi_core_data_structures_and_processing::neuron_data::neuron_mappings::*;
use crate::byte_structures::feagi_byte_structure::PyFeagiByteStructure;
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
    
    #[staticmethod]
    pub fn from_feagi_byte_structure(byte_structure: PyFeagiByteStructure) -> PyResult<Self> {
        let result = CorticalMappedXYZPNeuronData::new_from_feagi_byte_structure(byte_structure.inner);
        match result {
            Ok(inner) => Ok(PyCorticalMappedXYZPNeuronData { inner }),
            Err(e) => Err(PyValueError::new_err(e.to_string()))
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
    
    // TODO use inheritance properly
    pub fn as_new_feagi_byte_structure(&self) -> PyResult<PyFeagiByteStructure> {
        let result = self.inner.as_new_feagi_byte_structure();
        match result {
            Ok(result) => Ok(PyFeagiByteStructure { inner: result }),
            Err(error) => Err(PyValueError::new_err(error.to_string())),
        }
    }

}