use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::types::{PyByteArray, PyBytes};
use feagi_core_data_structures_and_processing::byte_structures::serializers::FeagiByteSerializer;

pub mod b001_json;


#[pyclass(subclass)]
#[pyo3(name = "FeagiByteSerializer")]
pub struct PyFeagiByteSerializer {}

#[pymethods]
impl PyFeagiByteSerializer {
    #[new]
    pub fn new() -> PyResult<PyFeagiByteSerializer> {
        Err(PyValueError::new_err("The FeagiByteSerializer object cannot be instantiated directly! Please use a child class!"))
    }
    
    #[getter]
    pub fn id(&self) -> u8 { 0 } // placeholder
    #[getter]
    pub fn version(&self) -> u8 { 0 } // placeholder
    #[getter]
    pub fn max_possible_size_when_serialized(&self) -> usize { 0 } // placeholder

    pub fn serialize_new<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyBytes>> { // placeholder
        Err(PyValueError::new_err("This object cannot be used directly")) 
    } 
    
    // NOTE: Skipping exposing serialize_in_place as that makes little performance sense in python due to copying requirements
    
    // NOTE: Skipping exposing generate_global_header as it doesn't make sense in python
    
}