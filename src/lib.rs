mod feagi_data_vision;
mod feagi_data_brain;
mod feagi_byte_structures;

use numpy::ndarray::AssignElem;
use pyo3::{prelude::*};
use pyo3::{wrap_pyfunction, wrap_pymodule};
use pyo3::types::IntoPyDict;

#[pyfunction]
fn test_function() -> String {
    "Subfunction".to_string()
}

#[pymodule]
fn feagi_data_processing(_py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(test_function, m)?)?;
    Ok(())
}
