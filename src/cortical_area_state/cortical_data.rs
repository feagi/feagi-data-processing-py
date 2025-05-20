use feagi_core_data_structures_and_processing::cortical_area_state::cortical_data::CorticalID;
use pyo3::exceptions::PyValueError;
use pyo3::pyclass;
use pyo3::prelude::*;

#[pyclass(eq)]
#[derive(PartialEq, Clone)]
#[pyo3(name = "CorticalID")]
pub struct PyCorticalID {
    inner: CorticalID,
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
