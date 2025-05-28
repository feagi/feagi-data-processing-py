mod brain_input;
mod brain_output;
mod byte_data_functions;
mod cortical_data;
mod neuron_data;

use numpy::ndarray::AssignElem;
use pyo3::prelude::*;
use pyo3::{wrap_pyfunction, wrap_pymodule};
use pyo3::types::IntoPyDict;


macro_rules! add_python_class {
    ($python:expr, $root_python_module:expr, $class_path:expr, $class:ty) => {
        {
            fn check_submodule_exists(py: Python, parent: &Bound<'_, PyModule>, submodule_name: &str) -> bool {
                match parent.getattr(submodule_name) {
                    Ok(attr) => attr.is_instance_of::<PyModule>(),
                    Err(_) => false,
                }
            }
            
            let path: Vec<String> = $class_path.split('.').map(|s| s.to_string()).collect();
            let mut current_module = $root_python_module.clone();
            
            for path_step in path {
                if !check_submodule_exists($python, &current_module, &path_step) {
                    // we need to add a submodule
                    let child_module = PyModule::new_bound($python, &path_step)?;
                    current_module.add_submodule(&child_module)?;
                    current_module = child_module;
                }
                else {
                    // child module already exists. Switch to it
                    let child_module = current_module.getattr(&path_step)?;
                    current_module = child_module.downcast::<PyModule>()?.clone();
                }
            }
            
            current_module.add_class::<$class>()?;
        }
    };
}


/// Core Module, accessible to users
#[pymodule]
fn feagi_data_processing(py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    //register_brain_input(m)?;
    //register_cortical_area_state(m)?;
    //register_neuron_state(m)?;
    
    add_python_class!(py, m, "cortical_data", cortical_data::PyCorticalID);
    add_python_class!(py, m, "neuron_data", neuron_data::PyCorticalMappedNeuronData);
    add_python_class!(py, m, "neuron_data", neuron_data::PyNeuronXYCPArrays);
    Ok(())
}

#[pyfunction]
fn test_function() -> String {
    "Subfunction".to_string()
}