pub mod b001_json;

use pyo3::prelude::*;
use pyo3::pyclass;
use feagi_core_data_structures_and_processing::byte_structures::deserializers::Deserializer;
use pyo3::types::PyBytes;
use super::PyFeagiByteStructureType;



#[pyclass(subclass)]
#[pyo3(name = "FeagiByteDeserializer")]
pub struct PyFeagiByteDeserializer {}

#[pymethods]
impl PyFeagiByteDeserializer {
    #[getter]
    pub fn id(&self) -> u8 { 0 } // placeholder
    #[getter]
    pub fn version(&self) -> u8 { 0 } // placeholder
}
