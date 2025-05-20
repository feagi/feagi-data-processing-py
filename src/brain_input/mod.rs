use pyo3::{Bound, PyResult};
use pyo3::prelude::*;



pub mod vision;

use vision::register_vision;

pub fn register_brain_input(parent_module: &Bound<'_, PyModule>) -> PyResult<()> {
    let child_module = PyModule::new(parent_module.py(), "brain_input")?;
    
    register_vision(&child_module)?;

    parent_module.add_submodule(&child_module)
}

