use pyo3::{Bound, PyResult};
use pyo3::prelude::{PyModule, PyModuleMethods};


pub mod cortical_data;
use cortical_data::{PyCorticalID};

pub fn register_cortical_area_state(parent_module: &Bound<'_, PyModule>) -> PyResult<()> {
    let child_module = PyModule::new(parent_module.py(), "cortical_area_state")?;

    register_cortical_data(&child_module)?;

    parent_module.add_submodule(&child_module)
}

pub fn register_cortical_data(parent_module: &Bound<'_, PyModule>) -> PyResult<()> {
    let child_module = PyModule::new(parent_module.py(), "cortical_data")?;

    child_module.add_class::<PyCorticalID>()?;

    parent_module.add_submodule(&child_module)
}