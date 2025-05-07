mod feagi_data_vision;
mod feagi_data_brain;
mod feagi_byte_structures;

use numpy::ndarray::AssignElem;
use pyo3::{prelude::*};
use pyo3::{wrap_pyfunction, wrap_pymodule};
use pyo3::types::IntoPyDict;

use feagi_data_vision::{register_data_vision};

/// Core Module, accessible to users
#[pymodule]
fn feagi_data_processing(_py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    register_data_vision(m)?;
    Ok(())
}

#[pyfunction]
fn test_function() -> String {
    "Subfunction".to_string()
}