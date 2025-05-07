use pyo3::prelude::*;
use pyo3::{pymodule, Bound, PyResult};

pub mod cropping_utils;

/*
#[pymodule]
pub fn py_module_feagi_data_vision() -> PyResult<()> {
    let this_child_module = PyModule::new(py(), "data_vision")?;
    
    this_child_module.add_submodule( cropping_utils::py_module_cropping_utils())?;

    Ok(this_child_module)
}

 */