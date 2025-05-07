mod feagi_data_vision;
mod feagi_data_brain;
mod feagi_byte_structures;

use pyo3::prelude::*;

/// A Python module implemented in Rust.
#[pymodule]
//#[pyo3(name = "feagi_data_processing")]
fn feagi_data_processing(py: Python<'_>, m: &Bound<'_, PyModule>) -> PyResult<()> {
    // Register the vision module
    register_feagi_data_vision(py, m)?;
    
    //m.add_class::<feagi_data_vision::cropping_utils::PyCornerPoints>()?;
    
    
    let mod_data_vision = PyModule::new(py, "data_vision");
    
    py_run!(py, mod_data_vision, "import sys; sys.modules['feagi_data_processing.data_vision'] = data_vision");
    /*
    // Register the brain module
    let brain_module = PyModule::new(py, "brain")?;
    feagi_data_brain::py_module_feagi_data_brain(brain_module)?;
    m.add_submodule(brain_module)?;
    
    // Register the byte structures module
    let byte_module = PyModule::new(py, "byte_structures")?;
    feagi_byte_structures::py_module_feagi_byte_structures(byte_module)?;
    m.add_submodule(byte_module)?;
     */
    
    /*
    Python::with_gil(|py| {
        py.import_bound("sys")?
            .getattr("modules")?
            .set_item("feagi_data_processing.data_vision", m)
    });
    
     */
    Ok(())
}

fn register_feagi_data_vision(py: Python, parent_module: &Bound<'_, PyModule>) -> PyResult<()> {
    let child_module = PyModule::new(parent_module.py(), "data_vision")?;
    
    register_cropping_utils(&child_module)?;
    parent_module.add_submodule(&child_module)
    
    
}

// Need to use the full path to PyCornerPoints
fn register_cropping_utils(parent_module: &Bound<'_, PyModule>) -> PyResult<()> {
    let child_module = PyModule::new(parent_module.py(), "cropping_utils")?;

    //child_module.add_class::<feagi_data_vision::cropping_utils::PyCornerPoints>()?;
    
    parent_module.add_submodule(&child_module)
}



