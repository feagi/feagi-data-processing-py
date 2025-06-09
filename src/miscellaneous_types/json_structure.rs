use pyo3::{pyclass, pymethods, PyResult, Py};
use pyo3::prelude::*;
use pyo3::exceptions::PyValueError;
use feagi_core_data_structures_and_processing::miscellaneous_types::json_structure::JsonStructure;
use feagi_core_data_structures_and_processing::byte_structures::{FeagiByteStructureCompatible, FeagiByteStructureType};
use crate::byte_structures::feagi_byte_structure::PyFeagiByteStructure;
use crate::byte_structures::{PyFeagiByteStructureCompatible, PyFeagiByteStructureType};

#[pyclass(extends=PyFeagiByteStructureCompatible)]
#[derive(Clone)]
#[pyo3(name = "JsonStructure")]
pub struct PyJsonStructure {
    pub inner: JsonStructure,
}

#[pymethods]
impl PyJsonStructure {
    
    //region Definitions for base class

    #[getter]
    pub fn struct_type(&self) -> PyFeagiByteStructureType {
        PyFeagiByteStructureType::JSON
    }

    pub fn version(&self) -> u8 { 
        self.inner.get_version() 
    }

    #[staticmethod]
    pub fn new_from_feagi_byte_structure<'py>(py: Python<'py>, byte_structure: PyFeagiByteStructure) -> PyResult<PyObject> where Self: Sized {
        let result = JsonStructure::new_from_feagi_byte_structure(&byte_structure.inner);
        match result {
            Ok(inner) => {
                let child = PyJsonStructure { inner };
                let parent = PyFeagiByteStructureCompatible::new();
                let py_obj = Py::new(py, (child, parent))?;
                Ok(py_obj.into())
            },
            Err(e) => Err(PyValueError::new_err(e.to_string()))
        }
    }

    pub fn as_new_feagi_byte_structure(&self) -> PyResult<PyFeagiByteStructure> {
        let result = self.inner.as_new_feagi_byte_structure();
        match result {
            Ok(result) => Ok(PyFeagiByteStructure { inner: result }),
            Err(error) => Err(PyValueError::new_err(error.to_string())),
        }
    }
    
    //endregion
    
    //region Constructors
    
    #[new]
    pub fn new() -> (PyJsonStructure, PyFeagiByteStructureCompatible) {
        // Create an empty JSON object
        let json_value = serde_json::Value::Object(serde_json::Map::new());
        (
            PyJsonStructure {
                inner: JsonStructure::from_json_value(json_value)
            },
            PyFeagiByteStructureCompatible::new()
        )
    }

    #[staticmethod]
    pub fn from_json_string<'py>(py: Python<'py>, json_string: String) -> PyResult<PyObject> {
        match JsonStructure::from_json_string(json_string) {
            Ok(inner) => {
                let child = PyJsonStructure { inner };
                let parent = PyFeagiByteStructureCompatible::new();
                let py_obj = Py::new(py, (child, parent))?;
                Ok(py_obj.into())
            },
            Err(e) => Err(PyValueError::new_err(e.to_string()))
        }
    }
    
    //endregion
    
    //region Data Access
    
    pub fn copy_as_json_string(&self) -> PyResult<String> {
        match self.inner.copy_as_json_string() {
            Ok(json_string) => Ok(json_string),
            Err(e) => Err(PyValueError::new_err(e.to_string()))
        }
    }
    
    /// Get a Python representation of the JSON data
    pub fn to_python<'py>(&self, py: Python<'py>) -> PyResult<PyObject> {
        let json_string = self.copy_as_json_string()?;
        
        // Import Python's json module and parse the string
        let json_module = py.import("json")?;
        let loads_fn = json_module.getattr("loads")?;
        let py_object = loads_fn.call1((json_string,))?;
        
        Ok(py_object.into())
    }
    
    /// Set JSON data from a Python object
    pub fn from_python(&mut self, py_object: PyObject, py: Python<'_>) -> PyResult<()> {
        // Import Python's json module and convert to string
        let json_module = py.import("json")?;
        let dumps_fn = json_module.getattr("dumps")?;
        let json_string: String = dumps_fn.call1((py_object,))?.extract()?;
        
        // Parse and update the inner JsonStructure
        match JsonStructure::from_json_string(json_string) {
            Ok(new_inner) => {
                self.inner = new_inner;
                Ok(())
            },
            Err(e) => Err(PyValueError::new_err(e.to_string()))
        }
    }
    
    //endregion
    
    //region String representation
    
    pub fn __str__(&self) -> PyResult<String> {
        self.copy_as_json_string()
    }
    
    pub fn __repr__(&self) -> PyResult<String> {
        let json_str = self.copy_as_json_string()?;
        Ok(format!("JsonStructure({})", json_str))
    }
    
    //endregion
} 