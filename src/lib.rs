mod feagi_data_vision;
mod feagi_data_brain;
mod feagi_byte_structures;

use numpy::ndarray::AssignElem;
use pyo3::{prelude::*};
use pyo3::{wrap_pyfunction, wrap_pymodule};
use pyo3::types::IntoPyDict;



#[pymodule]
fn feagi_data_processing(_py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    register_data_vision(m)?;
    Ok(())
}

fn register_data_vision(parent_module: &Bound<'_, PyModule>) -> PyResult<()> {
    let child_module = PyModule::new(parent_module.py(), "data_vision")?;
    child_module.add_function(wrap_pyfunction!(test_function, &child_module)?)?;
    parent_module.add_submodule(&child_module)
}

#[pyfunction]
fn test_function() -> String {
    "Subfunction".to_string()
}