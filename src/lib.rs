//! Rust PYO3 Compiled Python Module
//! All docs pertaining to python exposed modules must
//! be reflected to the 'feagi_data_processing.pyi.template' file!

pub mod feagi_data_serialization;
mod feagi_data_structures;
mod py_error;
mod useful_macros;
pub use feagi_data_serialization as feagi_serialization;
mod feagi_agent_sdk;
mod feagi_connector_core;
mod feagi_evo;

use pyo3::prelude::*;
use pyo3::types::PyDict;

fn check_submodule_exists(parent: &Bound<'_, PyModule>, submodule_name: &str) -> bool {
    match parent.getattr(submodule_name) {
        Ok(attr) => attr.is_instance_of::<PyModule>(),
        Err(_) => false,
    }
}

/// Registers a submodule with sys.modules so Python can properly import it
fn register_submodule_in_sys_modules(
    py: Python<'_>,
    full_module_path: &str,
    module: &Bound<'_, PyModule>,
) -> PyResult<()> {
    let sys_modules: Bound<'_, PyDict> = py.import("sys")?.getattr("modules")?.cast_into()?;
    sys_modules.set_item(full_module_path, module)?;
    Ok(())
}

macro_rules! add_python_class {
    ($python:expr, $root_python_module:expr, $class_path:expr, $class:ty) => {{
        let root_name = $root_python_module.name()?.to_string();
        let path: Vec<String> = $class_path.split('.').map(|s| s.to_string()).collect();
        let mut current_module = $root_python_module.clone();
        let mut full_path = root_name.clone();

        for path_step in path {
            full_path = format!("{}.{}", full_path, path_step);

            if !check_submodule_exists(&current_module, &path_step) {
                // we need to add a submodule
                let child_module = PyModule::new($python, &path_step)?;
                current_module.add_submodule(&child_module)?;
                // Register in sys.modules so Python can find it
                register_submodule_in_sys_modules($python, &full_path, &child_module)?;
                current_module = child_module;
            } else {
                // child module already exists. Switch to it
                let child_module = current_module.getattr(&path_step)?;
                current_module = child_module.cast_into::<PyModule>()?;
            }
        }

        current_module.add_class::<$class>()?;
    }};
}

// TODO the above macros can be consolidated

/// Core Module, accessible to users
#[pymodule]
fn feagi_rust_py_libs(py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    //region Feagi Data Structures

    // Genomic
    add_python_class!(
        py,
        m,
        "data_structures.genomic.cortical_area",
        feagi_data_structures::genomic::cortical_area::PyCorticalID
    );
    add_python_class!(
        py,
        m,
        "data_structures.genomic.cortical_area",
        feagi_data_structures::genomic::cortical_area::PyCustomCorticalType
    );
    add_python_class!(
        py,
        m,
        "data_structures.genomic.cortical_area",
        feagi_data_structures::genomic::cortical_area::PyMemoryCorticalType
    );
    add_python_class!(
        py,
        m,
        "data_structures.genomic.cortical_area",
        feagi_data_structures::genomic::cortical_area::PyCorticalAreaType
    );
    add_python_class!(
        py,
        m,
        "data_structures.genomic.cortical_area",
        feagi_data_structures::genomic::cortical_area::PyCoreCorticalType
    );
    add_python_class!(
        py,
        m,
        "data_structures.genomic.cortical_area",
        feagi_data_structures::genomic::cortical_area::PyFrameChangeHandling
    );
    add_python_class!(
        py,
        m,
        "data_structures.genomic.cortical_area",
        feagi_data_structures::genomic::cortical_area::PyIOCorticalAreaConfigurationFlag
    );
    add_python_class!(
        py,
        m,
        "data_structures.genomic.cortical_area",
        feagi_data_structures::genomic::cortical_area::PyPercentageNeuronPositioning
    );
    add_python_class!(
        py,
        m,
        "data_structures.genomic",
        feagi_data_structures::genomic::PyMotorCorticalUnit
    );
    add_python_class!(
        py,
        m,
        "data_structures.genomic",
        feagi_data_structures::genomic::PySensoryCorticalUnit
    );

    // Neurons Voxels
    add_python_class!(
        py,
        m,
        "data_structures.neurons_voxels.xyzp",
        feagi_data_structures::neurons_voxels::xyzp::PyCorticalMappedXYZPNeuronVoxels
    );
    add_python_class!(
        py,
        m,
        "data_structures.neurons_voxels.xyzp",
        feagi_data_structures::neurons_voxels::xyzp::PyNeuronVoxelXYZPArrays
    );
    add_python_class!(
        py,
        m,
        "data_structures.neurons_voxels.xyzp",
        feagi_data_structures::neurons_voxels::xyzp::PyNeuronVoxelXYZP
    );

    //region Feagi Data Serialization
    add_python_class!(
        py,
        m,
        "data_serialization",
        feagi_serialization::PyFeagiByteStructureType
    );
    add_python_class!(
        py,
        m,
        "data_serialization",
        feagi_serialization::PyFeagiSerializable
    );
    add_python_class!(
        py,
        m,
        "data_serialization",
        feagi_serialization::PyFeagiByteContainer
    );

    //endregion

    //region Feagi Connector Core
    // Data Types
    add_python_class!(
        py,
        m,
        "connector_core.data_types",
        feagi_connector_core::data_types::PyImageFrame
    );
    add_python_class!(
        py,
        m,
        "connector_core.data_types",
        feagi_connector_core::data_types::PySegmentedImageFrame
    );
    add_python_class!(
        py,
        m,
        "connector_core.data_types",
        feagi_connector_core::data_types::PyMiscData
    );
    add_python_class!(
        py,
        m,
        "connector_core.data_types",
        feagi_connector_core::data_types::PyPercentage
    );
    add_python_class!(
        py,
        m,
        "connector_core.data_types",
        feagi_connector_core::data_types::PySignedPercentage
    );
    add_python_class!(
        py,
        m,
        "connector_core.data_types",
        feagi_connector_core::data_types::PyPercentage2D
    );
    add_python_class!(
        py,
        m,
        "connector_core.data_types",
        feagi_connector_core::data_types::PySignedPercentage2D
    );
    add_python_class!(
        py,
        m,
        "connector_core.data_types",
        feagi_connector_core::data_types::PyPercentage3D
    );
    add_python_class!(
        py,
        m,
        "connector_core.data_types",
        feagi_connector_core::data_types::PySignedPercentage3D
    );
    add_python_class!(
        py,
        m,
        "connector_core.data_types",
        feagi_connector_core::data_types::PyPercentage4D
    );
    add_python_class!(
        py,
        m,
        "connector_core.data_types",
        feagi_connector_core::data_types::PySignedPercentage4D
    );
    add_python_class!(
        py,
        m,
        "connector_core.data_types",
        feagi_connector_core::data_types::PyRawIMU
    );
    add_python_class!(
        py,
        m,
        "connector_core.data_types",
        feagi_connector_core::data_types::PyGazeProperties
    );
    add_python_class!(
        py,
        m,
        "connector_core.data_types",
        feagi_connector_core::data_types::PyTextTokenCodec
    );
    add_python_class!(
        py,
        m,
        "connector_core.data_types",
        feagi_connector_core::data_types::PyGpt2Tokenizer
    );

    // Data Descriptors
    add_python_class!(
        py,
        m,
        "connector_core.data_types.descriptors",
        feagi_connector_core::data_types::descriptors::PyImageXYPoint
    );
    add_python_class!(
        py,
        m,
        "connector_core.data_types.descriptors",
        feagi_connector_core::data_types::descriptors::PyImageXYResolution
    );
    add_python_class!(
        py,
        m,
        "connector_core.data_types.descriptors",
        feagi_connector_core::data_types::descriptors::PyImageXYZDimensions
    );
    add_python_class!(
        py,
        m,
        "connector_core.data_types.descriptors",
        feagi_connector_core::data_types::descriptors::PySegmentedXYImageResolutions
    );
    add_python_class!(
        py,
        m,
        "connector_core.data_types.descriptors",
        feagi_connector_core::data_types::descriptors::PyColorSpace
    );
    add_python_class!(
        py,
        m,
        "connector_core.data_types.descriptors",
        feagi_connector_core::data_types::descriptors::PyColorChannelLayout
    );
    add_python_class!(
        py,
        m,
        "connector_core.data_types.descriptors",
        feagi_connector_core::data_types::descriptors::PyMemoryOrderLayout
    );
    add_python_class!(
        py,
        m,
        "connector_core.data_types.descriptors",
        feagi_connector_core::data_types::descriptors::PyImageFrameProperties
    );
    add_python_class!(
        py,
        m,
        "connector_core.data_types.descriptors",
        feagi_connector_core::data_types::descriptors::PySegmentedImageFrameProperties
    );
    add_python_class!(
        py,
        m,
        "connector_core.data_types.descriptors",
        feagi_connector_core::data_types::descriptors::PyCornerPoints
    );
    add_python_class!(
        py,
        m,
        "connector_core.data_types.descriptors",
        feagi_connector_core::data_types::descriptors::PyMiscDataDimensions
    );

    // Data Types Processing
    add_python_class!(
        py,
        m,
        "connector_core.data_types.processing",
        feagi_connector_core::data_types::processing::PyImageFrameProcessor
    );

    //Wrapped IO Data
    add_python_class!(
        py,
        m,
        "connector_core.wrapped_io_data",
        feagi_connector_core::wrapped_io_data::PyWrappedIOType
    );

    // Data Pipeline Stage Properties
    add_python_class!(
        py,
        m,
        "connector_core.data_pipeline.stage_properties",
        feagi_connector_core::data_pipeline::pipeline_stage_properties::PyPipelineStageProperties
    );

    // TODO: Add standalone constructor functions for backward compatibility
    // This requires manually building the Python module hierarchy
    // For now, users can call PipelineStageProperties.new_image_frame_segmentator()

    add_python_class!(
        py,
        m,
        "connector_core",
        feagi_connector_core::PyConnectorAgent
    );

    // Register init_rust_logging function
    m.add_function(pyo3::wrap_pyfunction!(
        feagi_connector_core::init_rust_logging,
        m
    )?)?;

    //endregion

    //region FEAGI Agent SDK
    feagi_agent_sdk::register_module(py, m)?;
    // Set __name__ and register in sys.modules so
    // "from feagi_rust_py_libs.feagi_agent import ..." works.
    let feagi_agent_module = m.getattr("feagi_agent")?.cast_into::<PyModule>()?;
    feagi_agent_module.setattr("__name__", "feagi_rust_py_libs.feagi_agent")?;
    feagi_agent_module.setattr("__package__", "feagi_rust_py_libs")?;
    register_submodule_in_sys_modules(py, "feagi_rust_py_libs.feagi_agent", &feagi_agent_module)?;
    //endregion

    //region FEAGI Evo (Genome Validation)
    feagi_evo::validator::register_module(py, m)?;
    // Set __name__ and register in sys.modules so "from feagi_rust_py_libs.genome import ..." works
    let genome_module = m.getattr("genome")?.cast_into::<PyModule>()?;
    genome_module.setattr("__name__", "feagi_rust_py_libs.genome")?;
    genome_module.setattr("__package__", "feagi_rust_py_libs")?;
    register_submodule_in_sys_modules(py, "feagi_rust_py_libs.genome", &genome_module)?;

    feagi_evo::connectome::register_module(py, m)?;
    let connectome_module = m.getattr("connectome")?.cast_into::<PyModule>()?;
    connectome_module.setattr("__name__", "feagi_rust_py_libs.connectome")?;
    connectome_module.setattr("__package__", "feagi_rust_py_libs")?;
    register_submodule_in_sys_modules(py, "feagi_rust_py_libs.connectome", &connectome_module)?;
    //endregion

    Ok(())
}
