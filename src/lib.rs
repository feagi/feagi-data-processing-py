mod brain_input;
mod brain_output;
mod byte_structures;
mod cortical_data;
mod miscellaneous_types;
mod neuron_data;

use numpy::ndarray::AssignElem;
use pyo3::prelude::*;
use pyo3::{wrap_pyfunction, wrap_pymodule};
use pyo3::types::IntoPyDict;


fn check_submodule_exists(parent: &Bound<'_, PyModule>, submodule_name: &str) -> bool {
    match parent.getattr(submodule_name) {
        Ok(attr) => attr.is_instance_of::<PyModule>(),
        Err(_) => false,
    }
}

macro_rules! add_python_class {
    ($python:expr, $root_python_module:expr, $class_path:expr, $class:ty) => {
        {
            
            let path: Vec<String> = $class_path.split('.').map(|s| s.to_string()).collect();
            let mut current_module = $root_python_module.clone();
            
            for path_step in path {
                if !check_submodule_exists(&current_module, &path_step) {
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

macro_rules! add_python_function {
    ($python:expr, $root_python_module:expr, $class_path:expr, $function:ty) => {
        {

            let path: Vec<String> = $class_path.split('.').map(|s| s.to_string()).collect();
            let mut current_module = $root_python_module.clone();

            for path_step in path {
                if !check_submodule_exists(&current_module, &path_step) {
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

            current_module.add_function::<$function>()?;
        }
    };
}

// TODO the above macros can be consolidated


/// Core Module, accessible to users
#[pymodule]
fn feagi_data_processing(py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {

    
    add_python_class!(py, m, "cortical_data", cortical_data::PyCorticalID);
    
    add_python_class!(py, m, "neuron_data.neuron_arrays", neuron_data::neuron_arrays::PyNeuronXYZPArrays);
    add_python_class!(py, m, "neuron_data.neuron_mappings", neuron_data::neuron_mappings::PyCorticalMappedXYZPNeuronData);
    add_python_class!(py, m, "neuron_data.neuron_mappings", neuron_data::neuron_mappings::PyCorticalMappedXYZPNeuronDataFullIter);
    add_python_class!(py, m, "neuron_data.neuron_mappings", neuron_data::neuron_mappings::PyCorticalMappedXYZPNeuronDataEasyIter);
    add_python_class!(py, m, "neuron_data.neurons", neuron_data::neurons::PyNeuronXYZP);

    add_python_class!(py, m, "brain_input.vision.descriptors", brain_input::vision::descriptors::PyChannelFormat);
    add_python_class!(py, m, "brain_input.vision.descriptors", brain_input::vision::descriptors::PySegmentedVisionTargetResolutions);
    add_python_class!(py, m, "brain_input.vision.descriptors", brain_input::vision::descriptors::PySegmentedVisionCenterProperties);
    add_python_class!(py, m, "brain_input.vision.descriptors", brain_input::vision::descriptors::PyColorSpace);
    add_python_class!(py, m, "brain_input.vision.descriptors", brain_input::vision::descriptors::PyCornerPoints);
    add_python_class!(py, m, "brain_input.vision.descriptors", brain_input::vision::descriptors::PyFrameProcessingParameters);
    add_python_class!(py, m, "brain_input.vision.descriptors", brain_input::vision::descriptors::PyMemoryOrderLayout);
    add_python_class!(py, m, "brain_input.vision.descriptors", brain_input::vision::descriptors::PySegmentedVisionFrameSourceCroppingPointGrouping);
    
    add_python_class!(py, m, "brain_input.vision", brain_input::vision::segmented_vision_frame::PySegmentedVisionFrame);
    add_python_class!(py, m, "brain_input.vision", brain_input::vision::image_frame::PyImageFrame);

    add_python_class!(py, m, "byte_structures", byte_structures::PyFeagiByteStructureType);
    add_python_class!(py, m, "byte_structures", byte_structures::feagi_byte_structure::PyFeagiByteStructure);
    add_python_class!(py, m, "byte_structures", byte_structures::PyFeagiByteStructureCompatible);

    add_python_class!(py, m, "misc", miscellaneous_types::json_structure::PyJsonStructure);

    // add_python_class!(py, m, "brain_input.vision", brain_input::vision::quick_image_diff::PyQuickImageDiff);
    
    Ok(())
}
