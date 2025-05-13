use pyo3::prelude::*;
use pyo3::{pymodule, Bound, PyResult};

mod cropping_utils;
mod peripheral_segmentation;

pub fn register_data_vision(parent_module: &Bound<'_, PyModule>) -> PyResult<()> {
    let child_module = PyModule::new(parent_module.py(), "data_vision")?;
    
    register_cropping_utils(&child_module)?;
    register_peripheral_segmentation(&child_module)?;
    
    parent_module.add_submodule(&child_module)
}

pub fn register_cropping_utils(parent_module: &Bound<'_, PyModule>) -> PyResult<()> {
    let child_module = PyModule::new(parent_module.py(), "cropping_utils")?;
    
    child_module.add_class::<cropping_utils::PyCornerPoints>()?;
    
    parent_module.add_submodule(&child_module)
}

pub fn register_peripheral_segmentation(parent_module: &Bound<'_, PyModule>) -> PyResult<()> {
    let child_module = PyModule::new(parent_module.py(), "peripheral_segmentation")?;

    child_module.add_class::<peripheral_segmentation::PySegmentedVisionCenterProperties>()?;

    parent_module.add_submodule(&child_module)
}