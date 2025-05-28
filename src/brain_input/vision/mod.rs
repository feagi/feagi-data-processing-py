use pyo3::prelude::*;
use pyo3::{Bound, PyResult};

mod segmented_vision_frame;
mod image_frame;
mod single_frame_processing;
mod descriptors;

pub fn register_vision(parent_module: &Bound<'_, PyModule>) -> PyResult<()> {
    let child_module = PyModule::new(parent_module.py(), "vision")?;

    register_single_frame_processing(&child_module)?;
    register_peripheral_segmentation(&child_module)?;
    register_single_frame(&child_module)?;
    
    parent_module.add_submodule(&child_module)
}

pub fn register_single_frame_processing(parent_module: &Bound<'_, PyModule>) -> PyResult<()> {
    let child_module = PyModule::new(parent_module.py(), "single_frame_processing")?;
    
    child_module.add_class::<single_frame_processing::PyCornerPoints>()?;
    
    parent_module.add_submodule(&child_module)
}

pub fn register_peripheral_segmentation(parent_module: &Bound<'_, PyModule>) -> PyResult<()> {
    let child_module = PyModule::new(parent_module.py(), "peripheral_segmentation")?;

    child_module.add_class::<segmented_vision_frame::PySegmentedVisionCenterProperties>()?;
    child_module.add_class::<segmented_vision_frame::PySegmentedVisionTargetResolutions>()?;
    child_module.add_class::<segmented_vision_frame::PySegmentedVisionFrame>()?;

    parent_module.add_submodule(&child_module)
}

pub fn register_single_frame(parent_module: &Bound<'_, PyModule>) -> PyResult<()> {
    let child_module = PyModule::new(parent_module.py(), "single_frame")?;

    child_module.add_class::<image_frame::PyImageFrame>()?;

    parent_module.add_submodule(&child_module)
}