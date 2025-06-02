use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::types::{PyAny, PyDict, PyList, PyBytes};
use serde_json::{Map, Number, Value};
use feagi_core_data_structures_and_processing::byte_structures::serializers::b001_json::JsonSerializerV1;
use feagi_core_data_structures_and_processing::byte_structures::serializers::FeagiByteSerializer;
use super::PyFeagiByteSerializer;

#[pyclass(extends=PyFeagiByteSerializer)]
#[pyo3(name = "JsonSerializer")]
pub struct PyJsonSerializer {
    inner: JsonSerializerV1,
}

#[pymethods]
impl PyJsonSerializer {
    #[new]
    pub fn from_json(json_str: String) -> PyResult<PyJsonSerializer> {
        let json = serde_json::from_str(&json_str);
        match json {
            Ok(val) => { Ok(PyJsonSerializer { inner: JsonSerializerV1::from_json(val).unwrap() })},
            Err(err) => { Err(PyErr::new::<PyValueError, _>(err.to_string()))}
        }
    }
    #[getter]
    pub fn id(&self) -> u8 {self.inner.get_id()}
    #[getter]
    pub fn version(&self) -> u8 {self.inner.get_version()}
    
    #[getter]
    pub fn max_possible_size_when_serialized(&self) -> usize {self.inner.get_max_possible_size_when_serialized()}
    
    pub fn serialize_new<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyBytes>> {
        match self.inner.serialize_new() {
            Ok(bytes) => Ok(PyBytes::new(py, &bytes)),
            Err(err) => Err(PyErr::new::<PyValueError, _>(err.to_string())),
        }
    }
    
    // NOTE: serialize_in_place is skipped since it makes no sense in python
    
    
    
}