mod brain_input;
mod brain_output;
mod byte_data_functions;
mod cortical_area_state;
mod neuron_state;

use numpy::ndarray::AssignElem;
use pyo3::prelude::*;
use pyo3::{wrap_pyfunction, wrap_pymodule};
use pyo3::types::IntoPyDict;

use brain_input::register_brain_input;
use cortical_area_state::register_cortical_area_state;

/// Core Module, accessible to users
#[pymodule]
fn feagi_data_processing(_py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    register_brain_input(m)?;
    register_cortical_area_state(m)?;
    Ok(())
}

#[pyfunction]
fn test_function() -> String {
    "Subfunction".to_string()
}