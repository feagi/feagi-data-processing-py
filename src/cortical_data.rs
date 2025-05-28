use pyo3::{pyclass, pymethods, PyResult};
use pyo3::exceptions::PyValueError;
use feagi_core_data_structures_and_processing::cortical_data::CorticalID;

#[pyclass(eq)]
#[derive(PartialEq, Clone)]
#[pyo3(name = "CorticalID")]
pub struct PyCorticalID {
    pub inner: CorticalID,
}

#[pymethods]
impl PyCorticalID {
    #[new]
    pub fn new(id: &str) -> PyResult<Self> {
        let result = CorticalID::from_str(id);
        match result {
            Ok(cortical_id) => Ok(PyCorticalID { inner: cortical_id}),
            Err(err) => Err(PyValueError::new_err(err.to_string()))
        }
    }

    pub fn as_str(&self) -> &str {
        self.inner.as_str()
    }
}