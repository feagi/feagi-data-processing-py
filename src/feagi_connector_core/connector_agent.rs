use crate::create_pyclass_no_clone_unsendable;
use crate::feagi_connector_core::data_pipeline::pipeline_stage_properties::PyPipelineStageProperties;
use crate::feagi_connector_core::data_types::descriptors::PyMiscDataDimensions;
use crate::feagi_connector_core::data_types::descriptors::*;
use crate::feagi_connector_core::data_types::*;
use crate::feagi_connector_core::wrapped_io_data::py_any_to_wrapped_io_data;
use crate::feagi_data_structures::genomic::cortical_area::*;
use crate::py_error::PyFeagiError;
use feagi_data_structures::genomic::cortical_area::descriptors::*;
use feagi_data_structures::genomic::cortical_area::io_cortical_area_configuration_flag::FrameChangeHandling;
use feagi_data_structures::genomic::cortical_area::io_cortical_area_configuration_flag::PercentageNeuronPositioning;
use feagi_data_structures::genomic::MotorCorticalUnit;
use feagi_data_structures::{motor_cortical_units, sensor_cortical_units, FeagiDataError};
use feagi_sensorimotor::caching::{MotorDeviceCache, SensorDeviceCache};
use feagi_sensorimotor::data_pipeline::PipelineStagePropertyIndex;
use feagi_sensorimotor::data_types::descriptors::*;
use feagi_sensorimotor::data_types::*;
use feagi_sensorimotor::wrapped_io_data::WrappedIOData;
use feagi_sensorimotor::ConnectorCache;
use pyo3::exceptions::PyTypeError;
use pyo3::prelude::*;
use pyo3::pymethods;
use pyo3::types::{PyByteArray, PyBytes, PyTuple};
use pyo3::PyResult;
use std::sync::MutexGuard;
use std::time::Instant;

type Pybool = bool; // ALL HAIL THE LOAD BEARING BOOLEAN

macro_rules! sensor_unit_functions {
    (
        SensoryCorticalUnit {
            $(
                $(#[doc = $doc:expr])?
                $cortical_type_key_name:ident => {
                    friendly_name: $friendly_name:expr,
                    accepted_wrapped_io_data_type: $accepted_wrapped_io_data_type:ident,
                    cortical_id_unit_reference: $cortical_id_unit_reference:expr,
                    number_cortical_areas: $number_cortical_areas:expr,
                    $(default_firing_threshold: $default_firing_threshold:expr,)?
                    $(default_firing_threshold_increment: [$default_firing_threshold_increment_x:expr, $default_firing_threshold_increment_y:expr, $default_firing_threshold_increment_z:expr],)?
                    $(default_mp_charge_accumulation: $default_mp_charge_accumulation:expr,)?
                    cortical_type_parameters: {
                        $($param_name:ident: $param_type:ty),* $(,)?
                    },
                    $(allowed_frame_change_handling: [$($allowed_frame:ident),* $(,)?],)?
                    cortical_area_properties: {
                        $($area_index:tt => ($cortical_area_type_expr:expr, relative_position: [$rel_x:expr, $rel_y:expr, $rel_z:expr], channel_dimensions_default: [$dim_default_x:expr, $dim_default_y:expr, $dim_default_z:expr], channel_dimensions_min: [$dim_min_x:expr, $dim_min_y:expr, $dim_min_z:expr], channel_dimensions_max: [$dim_max_x:expr, $dim_max_y:expr, $dim_max_z:expr])),* $(,)?
                    }
                }
            ),* $(,)?
        }
    ) =>
    {
        $(
            sensor_unit_functions!(@generate_functions
            $cortical_type_key_name,
            $accepted_wrapped_io_data_type
            );
        )*
    };


    //region Similar Functions
    // Helper macro to generate stage and other similar functions
    // "it's time for me to live up to my family name and face full life consequences"
    (@generate_similar_functions
        $cortical_type_key_name:ident,

        $wrapped_data_type:ident
    ) => {
        ::paste::paste! {

            #[pymethods]
            impl PyConnectorAgent {

                pub fn [<sensor_ $cortical_type_key_name:snake _write>](
                    &mut self,
                    py: Python<'_>,
                    group: u8,
                    channel_index: u32,
                    data: &Bound<'_, PyAny>,
                ) -> PyResult<()> {


                    let group: CorticalUnitIndex = group.into();
                    let channel_index: CorticalChannelIndex = channel_index.into();
                    let data: WrappedIOData = py_any_to_wrapped_io_data(py, data).map_err(PyFeagiError::from)?;

                    self.get_sensor_cache().[<$cortical_type_key_name:snake _write>](group, channel_index, data).map_err(PyFeagiError::from)?;
                    Ok(())
                }

                pub fn [<sensor_ $cortical_type_key_name:snake _read_postprocessed_cache_value>](
                    &mut self,
                    _py: Python<'_>,
                    group: u8,
                    channel_index: u32,
                ) -> PyResult<[<Py $wrapped_data_type>]> {

                    let group: CorticalUnitIndex = group.into();
                    let channel_index: CorticalChannelIndex = channel_index.into();

                    let expected_data = self.get_sensor_cache().[<$cortical_type_key_name:snake _read_postprocessed_cache_value>](group, channel_index).map_err(PyFeagiError::from)?;
                    Ok(expected_data.into())
                }

                pub fn [<sensor_ $cortical_type_key_name:snake _get_single_stage_properties>](
                    &mut self,
                    py: Python<'_>,
                    group: u8,
                    channel_index: u32,
                    pipeline_stage_property_index: u32
                ) -> PyResult<Py<PyPipelineStageProperties>>
                {
                    let group: CorticalUnitIndex = group.into();
                    let channel_index: CorticalChannelIndex = channel_index.into();
                    let pipeline_stage_property_index: PipelineStagePropertyIndex = pipeline_stage_property_index.into();

                    let boxed_stage = self.get_sensor_cache().[<$cortical_type_key_name:snake _get_single_stage_properties>](group, channel_index, pipeline_stage_property_index).map_err(PyFeagiError::from)?;
                    let py_stage = PyPipelineStageProperties::from_box_to_parent_typed(py, boxed_stage)?;
                    Ok(py_stage)
                }

                pub fn [<sensor_ $cortical_type_key_name:snake _get_all_stage_properties>](
                    &mut self,
                    py: Python<'_>,
                    group: u8,
                    channel_index: u32,
                ) -> PyResult<Vec<pyo3::Py<PyPipelineStageProperties>>>
                {
                    let group: CorticalUnitIndex = group.into();
                    let channel_index: CorticalChannelIndex = channel_index.into();

                    let boxed_stages = self.get_sensor_cache().[<$cortical_type_key_name:snake _get_all_stage_properties>](group, channel_index).map_err(PyFeagiError::from)?;
                    PyPipelineStageProperties::from_vec_box_to_vec_parent_typed(py, boxed_stages)
                }


                pub fn [<sensor_ $cortical_type_key_name:snake _update_single_stage_properties>](
                    &mut self,
                    py: Python<'_>,
                    group: u8,
                    channel_index: u32,
                    pipeline_stage_property_index: u32,
                    updating_property: Py<PyPipelineStageProperties> // TODO move to bound
                ) -> PyResult<()>
                {
                    let group: CorticalUnitIndex = group.into();
                    let channel_index: CorticalChannelIndex = channel_index.into();
                    let pipeline_stage_property_index: PipelineStagePropertyIndex = pipeline_stage_property_index.into();
                    let updating_property = PyPipelineStageProperties::from_py_to_box(py, &updating_property)?;

                    self.get_sensor_cache().[<$cortical_type_key_name:snake _update_single_stage_properties>](group, channel_index, pipeline_stage_property_index, updating_property).map_err(PyFeagiError::from)?;
                    Ok(())
                }

                pub fn [<sensor_ $cortical_type_key_name:snake _update_all_stage_properties>](
                    &mut self,
                    _py: Python<'_>,
                    group: u8,
                    channel_index: u32,
                    updated_pipeline_stage_properties: Vec<pyo3::Py<PyPipelineStageProperties>>
                ) -> PyResult<()>
                {
                    let group: CorticalUnitIndex = group.into();
                    let channel_index: CorticalChannelIndex = channel_index.into();
                    let updated_pipeline_stage_properties = PyPipelineStageProperties::from_vec_py_to_vec(updated_pipeline_stage_properties)?;

                    self.get_sensor_cache().[<$cortical_type_key_name:snake _update_all_stage_properties>](group, channel_index, updated_pipeline_stage_properties).map_err(PyFeagiError::from)?;

                    Ok(())
                }


                pub fn [<sensor_ $cortical_type_key_name:snake _replace_single_stage>](
                    &mut self,
                    py: Python<'_>,
                    group: u8,
                    channel_index: u32,
                    pipeline_stage_property_index: u32,
                    updating_property: Py<PyPipelineStageProperties> // TODO move to bound
                ) -> PyResult<()>
                {
                    let group: CorticalUnitIndex = group.into();
                    let channel_index: CorticalChannelIndex = channel_index.into();
                    let pipeline_stage_property_index: PipelineStagePropertyIndex = pipeline_stage_property_index.into();
                    let updating_property = PyPipelineStageProperties::from_py_to_box(py, &updating_property)?;

                    self.get_sensor_cache().[<$cortical_type_key_name:snake _replace_single_stage>](group, channel_index, pipeline_stage_property_index, updating_property).map_err(PyFeagiError::from)?;
                    Ok(())
                }


                pub fn [<sensor_ $cortical_type_key_name:snake _replace_all_stages>](
                    &mut self,
                    _py: Python<'_>,
                    group: u8,
                    channel_index: u32,
                    updated_pipeline_stage_properties: Vec<pyo3::Py<PyPipelineStageProperties>>
                ) -> PyResult<()>
                {
                    let group: CorticalUnitIndex = group.into();
                    let channel_index: CorticalChannelIndex = channel_index.into();
                    let updated_pipeline_stage_properties = PyPipelineStageProperties::from_vec_py_to_vec(updated_pipeline_stage_properties)?;

                    self.get_sensor_cache().[<$cortical_type_key_name:snake _replace_all_stages>](group, channel_index, updated_pipeline_stage_properties).map_err(PyFeagiError::from)?;
                    Ok(())
                }


                pub fn [<sensor_ $cortical_type_key_name:snake _removing_all_stages>](
                    &mut self,
                    _py: Python<'_>,
                    group: u8,
                    channel_index: u32
                ) -> PyResult<()>
                {
                    let group: CorticalUnitIndex = group.into();
                    let channel_index: CorticalChannelIndex = channel_index.into();
                    self.get_sensor_cache().[<$cortical_type_key_name:snake _removing_all_stages>](group, channel_index).map_err(PyFeagiError::from)?;
                    Ok(())
                }

             }
        }
    };
    //endregion


    // Arm for WrappedIOType::Boolean
    (@generate_functions
        $sensory_unit:ident,
        Boolean
    ) => {


        ::paste::paste! {

            #[pymethods]
            impl PyConnectorAgent {
                pub fn [<sensor_ $sensory_unit _register>](
                    &mut self,
                    _py: Python<'_>,
                    group: u8,
                    number_channels: u32,
                    ) -> PyResult<()>
                {
                    let group: CorticalUnitIndex = group.into();
                    let number_channels: CorticalChannelCount = number_channels.try_into().map_err(PyFeagiError::from)?;

                    self.get_sensor_cache().[<$sensory_unit:snake _register>](group, number_channels).map_err(PyFeagiError::from)?;
                    Ok(())
                }
            }

        }
        // NOTE: Used the type Pybool at the to work. Fucking Cursed.
        sensor_unit_functions!(@generate_similar_functions $sensory_unit, bool);
    };

    // Arm for WrappedIOType::Percentage
    (@generate_functions
        $sensory_unit:ident,

        Percentage
    ) => {
        ::paste::paste! {
            #[pymethods]
            impl PyConnectorAgent {
                pub fn [<sensor_ $sensory_unit _register>](
                    &mut self,
                    _py: Python<'_>,
                    group: u8,
                    number_channels: u32,
                    frame_change_handling: PyFrameChangeHandling,
                    z_neuron_resolution: u32,
                    percentage_neuron_positioning: PyPercentageNeuronPositioning
                    ) -> PyResult<()>
                {
                    let group: CorticalUnitIndex = group.into();
                    let number_channels: CorticalChannelCount = number_channels.try_into().map_err(PyFeagiError::from)?;
                    let frame_change_handling: FrameChangeHandling = frame_change_handling.into();
                    let z_neuron_resolution: NeuronDepth = z_neuron_resolution.try_into().map_err(PyFeagiError::from)?;
                    let percentage_neuron_positioning: PercentageNeuronPositioning = percentage_neuron_positioning.into();

                    self.get_sensor_cache().[<$sensory_unit:snake _register>](group, number_channels, frame_change_handling, z_neuron_resolution, percentage_neuron_positioning).map_err(PyFeagiError::from)?;
                    Ok(())
                }
             }

        }

        sensor_unit_functions!(@generate_similar_functions $sensory_unit, Percentage);
    };

    // Arm for WrappedIOType::Percentage_3D
    (@generate_functions
        $sensory_unit:ident,

        Percentage_3D
    ) => {
        ::paste::paste! {
            #[pymethods]
            impl PyConnectorAgent {
                pub fn [<sensor_ $sensory_unit _register>](
                    &mut self,
                    _py: Python<'_>,
                    group: u8,
                    number_channels: u32,
                    frame_change_handling: PyFrameChangeHandling,
                    z_neuron_resolution: u32,
                    percentage_neuron_positioning: PyPercentageNeuronPositioning
                    ) -> PyResult<()>
                {
                    let group: CorticalUnitIndex = group.into();
                    let number_channels: CorticalChannelCount = number_channels.try_into().map_err(PyFeagiError::from)?;
                    let frame_change_handling: FrameChangeHandling = frame_change_handling.into();
                    let z_neuron_resolution: NeuronDepth = z_neuron_resolution.try_into().map_err(PyFeagiError::from)?;
                    let percentage_neuron_positioning: PercentageNeuronPositioning = percentage_neuron_positioning.into();

                    self.get_sensor_cache().[<$sensory_unit:snake _register>](group, number_channels, frame_change_handling, z_neuron_resolution, percentage_neuron_positioning).map_err(PyFeagiError::from)?;
                    Ok(())
                }
             }

        }

        sensor_unit_functions!(@generate_similar_functions $sensory_unit, Percentage3D);
    };

    // Arm for WrappedIOType::SignedPercentage_4D
    (@generate_functions
        $sensory_unit:ident,
        SignedPercentage_4D
    ) => {
        ::paste::paste! {
            #[pymethods]
            impl PyConnectorAgent {
                pub fn [<sensor_ $sensory_unit _register>](
                    &mut self,
                    _py: Python<'_>,
                    group: u8,
                    number_channels: u32,
                    frame_change_handling: PyFrameChangeHandling,
                    z_neuron_resolution: u32,
                    percentage_neuron_positioning: PyPercentageNeuronPositioning
                    ) -> PyResult<()>
                {
                    let group: CorticalUnitIndex = group.into();
                    let number_channels: CorticalChannelCount = number_channels.try_into().map_err(PyFeagiError::from)?;
                    let frame_change_handling: FrameChangeHandling = frame_change_handling.into();
                    let z_neuron_resolution: NeuronDepth = z_neuron_resolution.try_into().map_err(PyFeagiError::from)?;
                    let percentage_neuron_positioning: PercentageNeuronPositioning = percentage_neuron_positioning.into();

                    self.get_sensor_cache().[<$sensory_unit:snake _register>](group, number_channels, frame_change_handling, z_neuron_resolution, percentage_neuron_positioning).map_err(PyFeagiError::from)?;
                    Ok(())
                }
             }

        }

        sensor_unit_functions!(@generate_similar_functions $sensory_unit, SignedPercentage4D);
    };

    // Arm for WrappedIOType::RawIMU
    //
    // Raw IMU is a composite, multi-sub-area sensor unit. Registration parameters
    // mirror the 3-D / 4-D percentage arms (frame_change_handling, z resolution,
    // percentage neuron positioning) because each of the three sub-areas
    // internally encodes a `SignedPercentage3D`. The cache itself stores ONE
    // composite value per (group, channel) which the Rust encoder then spreads
    // across the three sub-cortical-areas.
    //
    // Two write surfaces are exposed:
    //   * the whole-composite `_write` generated by `@generate_similar_functions`
    //     (accepts a full `PyRawIMU`); used when the controller can supply all
    //     three axes every tick.
    //   * three partial-axis writers (`_write_accelerometer`, `_write_gyroscope`,
    //     `_write_magnetometer`) that perform a read-modify-write on the cache
    //     slot. These exist specifically so a controller exposing only a
    //     subset of axes (e.g. accel + gyro, no magnetometer - a common MuJoCo
    //     case) can update the present axes without forcing the missing ones
    //     to zero each tick. Missing axes therefore retain whatever was last
    //     written for them (the registered initial zero until/unless the
    //     controller writes them). This is the FFI counterpart of the
    //     read-modify-write helpers added in feagi-core's
    //     `sensor_device_cache.rs`.
    (@generate_functions
        $sensory_unit:ident,
        RawIMU
    ) => {
        ::paste::paste! {
            #[pymethods]
            impl PyConnectorAgent {
                pub fn [<sensor_ $sensory_unit _register>](
                    &mut self,
                    _py: Python<'_>,
                    group: u8,
                    number_channels: u32,
                    frame_change_handling: PyFrameChangeHandling,
                    z_neuron_resolution: u32,
                    percentage_neuron_positioning: PyPercentageNeuronPositioning
                    ) -> PyResult<()>
                {
                    let group: CorticalUnitIndex = group.into();
                    let number_channels: CorticalChannelCount = number_channels.try_into().map_err(PyFeagiError::from)?;
                    let frame_change_handling: FrameChangeHandling = frame_change_handling.into();
                    let z_neuron_resolution: NeuronDepth = z_neuron_resolution.try_into().map_err(PyFeagiError::from)?;
                    let percentage_neuron_positioning: PercentageNeuronPositioning = percentage_neuron_positioning.into();

                    self.get_sensor_cache().[<$sensory_unit:snake _register>](group, number_channels, frame_change_handling, z_neuron_resolution, percentage_neuron_positioning).map_err(PyFeagiError::from)?;
                    Ok(())
                }

                pub fn [<sensor_ $sensory_unit:snake _write_accelerometer>](
                    &mut self,
                    _py: Python<'_>,
                    group: u8,
                    channel_index: u32,
                    accelerometer: PySignedPercentage3D,
                ) -> PyResult<()> {
                    let group: CorticalUnitIndex = group.into();
                    let channel_index: CorticalChannelIndex = channel_index.into();
                    let accelerometer: feagi_sensorimotor::data_types::SignedPercentage3D =
                        accelerometer.into();
                    self.get_sensor_cache()
                        .[<$sensory_unit:snake _write_accelerometer>](group, channel_index, accelerometer)
                        .map_err(PyFeagiError::from)?;
                    Ok(())
                }

                pub fn [<sensor_ $sensory_unit:snake _write_gyroscope>](
                    &mut self,
                    _py: Python<'_>,
                    group: u8,
                    channel_index: u32,
                    gyroscope: PySignedPercentage3D,
                ) -> PyResult<()> {
                    let group: CorticalUnitIndex = group.into();
                    let channel_index: CorticalChannelIndex = channel_index.into();
                    let gyroscope: feagi_sensorimotor::data_types::SignedPercentage3D =
                        gyroscope.into();
                    self.get_sensor_cache()
                        .[<$sensory_unit:snake _write_gyroscope>](group, channel_index, gyroscope)
                        .map_err(PyFeagiError::from)?;
                    Ok(())
                }

                pub fn [<sensor_ $sensory_unit:snake _write_magnetometer>](
                    &mut self,
                    _py: Python<'_>,
                    group: u8,
                    channel_index: u32,
                    magnetometer: PySignedPercentage3D,
                ) -> PyResult<()> {
                    let group: CorticalUnitIndex = group.into();
                    let channel_index: CorticalChannelIndex = channel_index.into();
                    let magnetometer: feagi_sensorimotor::data_types::SignedPercentage3D =
                        magnetometer.into();
                    self.get_sensor_cache()
                        .[<$sensory_unit:snake _write_magnetometer>](group, channel_index, magnetometer)
                        .map_err(PyFeagiError::from)?;
                    Ok(())
                }
             }

        }

        sensor_unit_functions!(@generate_similar_functions $sensory_unit, RawIMU);
    };

    // Arm for WrappedIOType::SegmentedImageFrame
    (@generate_functions
        $sensory_unit:ident,
        SegmentedImageFrame
    ) => {
        ::paste::paste! {
            #[pymethods]
            impl PyConnectorAgent {
                pub fn [<sensor_ $sensory_unit _register>](
                    &mut self,
                    _py: Python<'_>,
                    group: u8,
                    number_channels: u32,
                    frame_change_handling: &pyo3::Bound<PyFrameChangeHandling>,
                    input_image_properties: &pyo3::Bound<PyImageFrameProperties>,
                    segmented_image_properties: &pyo3::Bound<PySegmentedImageFrameProperties>,
                    initial_gaze: &pyo3::Bound<PyGazeProperties>,
                    ) -> PyResult<()>
                {
                    let group: CorticalUnitIndex = group.into();
                    let number_channels: CorticalChannelCount = number_channels.try_into().map_err(PyFeagiError::from)?;
                    let frame_change_handling: FrameChangeHandling = PyFrameChangeHandling::from_bound(frame_change_handling);
                    let input_image_properties: ImageFrameProperties = PyImageFrameProperties::copy_out_from_bound(input_image_properties);
                    let segmented_image_properties: SegmentedImageFrameProperties = PySegmentedImageFrameProperties::copy_out_from_bound(segmented_image_properties);
                    let initial_gaze: GazeProperties = PyGazeProperties::copy_out_from_bound(initial_gaze);

                    self.get_sensor_cache().[<$sensory_unit:snake _register>](group, number_channels, frame_change_handling, input_image_properties, segmented_image_properties, initial_gaze).map_err(PyFeagiError::from)?;
                    Ok(())
                }
            }
        }


        sensor_unit_functions!(@generate_similar_functions $sensory_unit, SegmentedImageFrame);
    };

    // Arm for WrappedIOType::MiscData
    (@generate_functions
        $sensory_unit:ident,

        MiscData
    ) => {
        ::paste::paste! {
            #[pymethods]
            impl PyConnectorAgent {
                pub fn [<sensor_ $sensory_unit _register>](
                    &mut self,
                    _py: Python<'_>,
                    group: u8,
                    number_channels: u32,
                    frame_change_handling: PyFrameChangeHandling,
                    misc_data_dimensions: PyMiscDataDimensions,
                    ) -> PyResult<()>
                {
                    let group: CorticalUnitIndex = group.into();
                    let number_channels: CorticalChannelCount = number_channels.try_into().map_err(PyFeagiError::from)?;
                    let frame_change_handling: FrameChangeHandling = frame_change_handling.into();
                    let misc_data_dimensions: MiscDataDimensions = misc_data_dimensions.into();

                    self.get_sensor_cache().[<$sensory_unit:snake _register>](group, number_channels, frame_change_handling, misc_data_dimensions).map_err(PyFeagiError::from)?;
                    Ok(())
                }
             }

        }

        sensor_unit_functions!(@generate_similar_functions $sensory_unit, MiscData);
    };


    // Arm for WrappedIOType::ImageFrame
    (@generate_functions
        $sensory_unit:ident,

        ImageFrame
    ) => {
        ::paste::paste! {
            #[pymethods]
            impl PyConnectorAgent {
                pub fn [<sensor_ $sensory_unit _register>](
                    &mut self,
                    _py: Python<'_>,
                    group: u8,
                    number_channels: u32,
                    frame_change_handling: PyFrameChangeHandling,
                    image_properties: PyImageFrameProperties
                    ) -> PyResult<()>
                {
                    let group: CorticalUnitIndex = group.into();
                    let number_channels: CorticalChannelCount = number_channels.try_into().map_err(PyFeagiError::from)?;
                    let frame_change_handling: FrameChangeHandling = frame_change_handling.into();
                    let image_properties: ImageFrameProperties = image_properties.into();

                    self.get_sensor_cache().[<$sensory_unit:snake _register>](group, number_channels, frame_change_handling, image_properties).map_err(PyFeagiError::from)?;
                    Ok(())
                }
             }

        }

        sensor_unit_functions!(@generate_similar_functions $sensory_unit, ImageFrame);
    };
}

macro_rules! motor_unit_functions {
    (
        MotorCorticalUnit {
            $(
                $(#[doc = $doc:expr])?
                $cortical_type_key_name:ident => {
                    friendly_name: $friendly_name:expr,
                    accepted_wrapped_io_data_type: $accepted_wrapped_io_data_type:ident,
                    cortical_id_unit_reference: $cortical_id_unit_reference:expr,
                    number_cortical_areas: $number_cortical_areas:expr,
                    cortical_type_parameters: {
                        $($param_name:ident: $param_type:ty),* $(,)?
                    },
                    $(allowed_frame_change_handling: [$($allowed_frame:ident),* $(,)?],)?
                    cortical_area_properties: {
                        $($area_index:tt => ($cortical_area_type_expr:expr, relative_position: [$rel_x:expr, $rel_y:expr, $rel_z:expr], channel_dimensions_default: [$dim_default_x:expr, $dim_default_y:expr, $dim_default_z:expr], channel_dimensions_min: [$dim_min_x:expr, $dim_min_y:expr, $dim_min_z:expr], channel_dimensions_max: [$dim_max_x:expr, $dim_max_y:expr, $dim_max_z:expr])),* $(,)?
                    }
                }
            ),* $(,)?
        }
    ) =>
    {
        $(
            motor_unit_functions!(@generate_functions
            $cortical_type_key_name,
            $accepted_wrapped_io_data_type
            );
        )*
    };

    //region Similar Functions
    // Helper macro to generate stage and other similar functions
    (@generate_similar_functions
        $cortical_type_key_name:ident,

        $wrapped_data_type:ident
    ) => {
        ::paste::paste! {

            #[pymethods]
            impl PyConnectorAgent {

                pub fn [<motor_ $cortical_type_key_name:snake _read_preprocessed_cache_value>](
                    &mut self,
                    _py: Python<'_>,
                    group: u8,
                    channel_index: u32,
                ) -> PyResult<[<Py $wrapped_data_type>]> {

                    let group: CorticalUnitIndex = group.into();
                    let channel_index: CorticalChannelIndex = channel_index.into();

                    let expected_data = self.get_motor_cache().[<$cortical_type_key_name:snake _read_preprocessed_cache_value>](group, channel_index).map_err(PyFeagiError::from)?;
                    Ok(expected_data.into())
                }

                pub fn [<motor_ $cortical_type_key_name:snake _read_postprocessed_cache_value>](
                    &mut self,
                    _py: Python<'_>,
                    group: u8,
                    channel_index: u32,
                ) -> PyResult<[<Py $wrapped_data_type>]> {

                    let group: CorticalUnitIndex = group.into();
                    let channel_index: CorticalChannelIndex = channel_index.into();

                    let expected_data = self.get_motor_cache().[<$cortical_type_key_name:snake _read_postprocessed_cache_value>](group, channel_index).map_err(PyFeagiError::from)?;
                    Ok(expected_data.into())
                }

                pub fn [<motor_ $cortical_type_key_name:snake _get_single_stage_properties>](
                    &mut self,
                    py: Python<'_>,
                    group: u8,
                    channel_index: u32,
                    pipeline_stage_property_index: u32
                ) -> PyResult<Py<PyPipelineStageProperties>>
                {
                    let group: CorticalUnitIndex = group.into();
                    let channel_index: CorticalChannelIndex = channel_index.into();
                    let pipeline_stage_property_index: PipelineStagePropertyIndex = pipeline_stage_property_index.into();

                    let boxed_stage = self.get_motor_cache().[<$cortical_type_key_name:snake _get_single_stage_properties>](group, channel_index, pipeline_stage_property_index).map_err(PyFeagiError::from)?;
                    let py_stage = PyPipelineStageProperties::from_box_to_parent_typed(py, boxed_stage)?;
                    Ok(py_stage)
                }

                pub fn [<motor_ $cortical_type_key_name:snake _get_all_stage_properties>](
                    &mut self,
                    py: Python<'_>,
                    group: u8,
                    channel_index: u32,
                ) -> PyResult<Vec<pyo3::Py<PyPipelineStageProperties>>>
                {
                    let group: CorticalUnitIndex = group.into();
                    let channel_index: CorticalChannelIndex = channel_index.into();

                    let boxed_stages = self.get_motor_cache().[<$cortical_type_key_name:snake _get_all_stage_properties>](group, channel_index).map_err(PyFeagiError::from)?;
                    PyPipelineStageProperties::from_vec_box_to_vec_parent_typed(py, boxed_stages)
                }

                pub fn [<motor_ $cortical_type_key_name:snake _update_single_stage_properties>](
                    &mut self,
                    py: Python<'_>,
                    group: u8,
                    channel_index: u32,
                    pipeline_stage_property_index: u32,
                    updating_property: Py<PyPipelineStageProperties>
                ) -> PyResult<()>
                {
                    let group: CorticalUnitIndex = group.into();
                    let channel_index: CorticalChannelIndex = channel_index.into();
                    let pipeline_stage_property_index: PipelineStagePropertyIndex = pipeline_stage_property_index.into();
                    let updating_property = PyPipelineStageProperties::from_py_to_box(py, &updating_property)?;

                    self.get_motor_cache().[<$cortical_type_key_name:snake _update_single_stage_properties>](group, channel_index, pipeline_stage_property_index, updating_property).map_err(PyFeagiError::from)?;
                    Ok(())
                }

                pub fn [<motor_ $cortical_type_key_name:snake _update_all_stage_properties>](
                    &mut self,
                    _py: Python<'_>,
                    group: u8,
                    channel_index: u32,
                    updated_pipeline_stage_properties: Vec<pyo3::Py<PyPipelineStageProperties>>
                ) -> PyResult<()>
                {
                    let group: CorticalUnitIndex = group.into();
                    let channel_index: CorticalChannelIndex = channel_index.into();
                    let updated_pipeline_stage_properties = PyPipelineStageProperties::from_vec_py_to_vec(updated_pipeline_stage_properties)?;

                    self.get_motor_cache().[<$cortical_type_key_name:snake _update_all_stage_properties>](group, channel_index, updated_pipeline_stage_properties).map_err(PyFeagiError::from)?;

                    Ok(())
                }

                pub fn [<motor_ $cortical_type_key_name:snake _replace_single_stage>](
                    &mut self,
                    py: Python<'_>,
                    group: u8,
                    channel_index: u32,
                    pipeline_stage_property_index: u32,
                    updating_property: Py<PyPipelineStageProperties>
                ) -> PyResult<()>
                {
                    let group: CorticalUnitIndex = group.into();
                    let channel_index: CorticalChannelIndex = channel_index.into();
                    let pipeline_stage_property_index: PipelineStagePropertyIndex = pipeline_stage_property_index.into();
                    let updating_property = PyPipelineStageProperties::from_py_to_box(py, &updating_property)?;

                    self.get_motor_cache().[<$cortical_type_key_name:snake _replace_single_stage>](group, channel_index, pipeline_stage_property_index, updating_property).map_err(PyFeagiError::from)?;
                    Ok(())
                }

                pub fn [<motor_ $cortical_type_key_name:snake _replace_all_stages>](
                    &mut self,
                    _py: Python<'_>,
                    group: u8,
                    channel_index: u32,
                    updated_pipeline_stage_properties: Vec<pyo3::Py<PyPipelineStageProperties>>
                ) -> PyResult<()>
                {
                    let group: CorticalUnitIndex = group.into();
                    let channel_index: CorticalChannelIndex = channel_index.into();
                    let updated_pipeline_stage_properties = PyPipelineStageProperties::from_vec_py_to_vec(updated_pipeline_stage_properties)?;

                    self.get_motor_cache().[<$cortical_type_key_name:snake _replace_all_stages>](group, channel_index, updated_pipeline_stage_properties).map_err(PyFeagiError::from)?;
                    Ok(())
                }

                pub fn [<motor_ $cortical_type_key_name:snake _removing_all_stages>](
                    &mut self,
                    _py: Python<'_>,
                    group: u8,
                    channel_index: u32
                ) -> PyResult<()>
                {
                    let group: CorticalUnitIndex = group.into();
                    let channel_index: CorticalChannelIndex = channel_index.into();
                    self.get_motor_cache().[<$cortical_type_key_name:snake _removing_all_stages>](group, channel_index).map_err(PyFeagiError::from)?;
                    Ok(())
                }

            }
        }
    };
    //endregion

    // Arm for WrappedIOType::GazeProperties
    (@generate_functions
        $motor_unit:ident,

        GazeProperties
    ) => {
        ::paste::paste! {
            #[pymethods]
            impl PyConnectorAgent {
                pub fn [<motor_ $motor_unit:snake _register>](
                    &mut self,
                    _py: Python<'_>,
                    group: u8,
                    number_channels: u32,
                    frame_change_handling: PyFrameChangeHandling,
                    eccentricity_z_neuron_resolution: u32,
                    modulation_z_neuron_resolution: u32,
                    percentage_neuron_positioning: PyPercentageNeuronPositioning
                ) -> PyResult<()>
                {
                    let group: CorticalUnitIndex = group.into();
                    let number_channels: CorticalChannelCount = number_channels.try_into().map_err(PyFeagiError::from)?;
                    let frame_change_handling: FrameChangeHandling = frame_change_handling.into();
                    let eccentricity_z_neuron_resolution: NeuronDepth = eccentricity_z_neuron_resolution.try_into().map_err(PyFeagiError::from)?;
                    let modulation_z_neuron_resolution: NeuronDepth = modulation_z_neuron_resolution.try_into().map_err(PyFeagiError::from)?;
                    let percentage_neuron_positioning: PercentageNeuronPositioning = percentage_neuron_positioning.into();

                    self.get_motor_cache().[<$motor_unit:snake _register>](group, number_channels, frame_change_handling, eccentricity_z_neuron_resolution, modulation_z_neuron_resolution, percentage_neuron_positioning).map_err(PyFeagiError::from)?;
                    Ok(())
                }
            }
        }

        motor_unit_functions!(@generate_similar_functions $motor_unit, GazeProperties);

    };

    // Arm for WrappedIOType::ImageFilteringSettings
    (@generate_functions
        $motor_unit:ident,

        ImageFilteringSettings
    ) => {
        ::paste::paste! {
            #[pymethods]
            impl PyConnectorAgent {
                pub fn [<motor_ $motor_unit:snake _register>](
                    &mut self,
                    _py: Python<'_>,
                    group: u8,
                    number_channels: u32,
                    frame_change_handling: PyFrameChangeHandling,
                    z_neuron_resolution: u32,
                    percentage_neuron_positioning: PyPercentageNeuronPositioning
                ) -> PyResult<()>
                {
                    let group: CorticalUnitIndex = group.into();
                    let number_channels: CorticalChannelCount =
                        number_channels.try_into().map_err(PyFeagiError::from)?;
                    let frame_change_handling: FrameChangeHandling = frame_change_handling.into();
                    let z_neuron_resolution: NeuronDepth =
                        z_neuron_resolution.try_into().map_err(PyFeagiError::from)?;
                    let percentage_neuron_positioning: PercentageNeuronPositioning =
                        percentage_neuron_positioning.into();

                    self.get_motor_cache()
                        .[<$motor_unit:snake _register>](
                            group,
                            number_channels,
                            frame_change_handling,
                            z_neuron_resolution,
                            percentage_neuron_positioning,
                        )
                        .map_err(PyFeagiError::from)?;
                    Ok(())
                }
            }
        }

        motor_unit_functions!(@generate_similar_functions $motor_unit, ImageFilteringSettings);
    };

    // Arm for WrappedIOType::Percentage
    (@generate_functions
        $motor_unit:ident,

        Percentage
    ) => {
        ::paste::paste! {
            #[pymethods]
            impl PyConnectorAgent {
                pub fn [<motor_ $motor_unit:snake _register>](
                    &mut self,
                    _py: Python<'_>,
                    group: u8,
                    number_channels: u32,
                    frame_change_handling: PyFrameChangeHandling,
                    z_neuron_resolution: u32,
                    percentage_neuron_positioning: PyPercentageNeuronPositioning
                ) -> PyResult<()>
                {
                    let group: CorticalUnitIndex = group.into();
                    let number_channels: CorticalChannelCount = number_channels.try_into().map_err(PyFeagiError::from)?;
                    let frame_change_handling: FrameChangeHandling = frame_change_handling.into();
                    let z_neuron_resolution: NeuronDepth = z_neuron_resolution.try_into().map_err(PyFeagiError::from)?;
                    let percentage_neuron_positioning: PercentageNeuronPositioning = percentage_neuron_positioning.into();

                    self.get_motor_cache().[<$motor_unit:snake _register>](group, number_channels, frame_change_handling, z_neuron_resolution, percentage_neuron_positioning).map_err(PyFeagiError::from)?;
                    Ok(())
                }
            }
        }

        motor_unit_functions!(@generate_similar_functions $motor_unit, Percentage);

    };

    // Arm for WrappedIOType::Percentage3D
    (@generate_functions
        $motor_unit:ident,

        Percentage_3D
    ) => {
        ::paste::paste! {
            #[pymethods]
            impl PyConnectorAgent {
                pub fn [<motor_ $motor_unit:snake _register>](
                    &mut self,
                    _py: Python<'_>,
                    group: u8,
                    number_channels: u32,
                    frame_change_handling: PyFrameChangeHandling,
                    z_neuron_resolution: u32,
                    percentage_neuron_positioning: PyPercentageNeuronPositioning
                ) -> PyResult<()>
                {
                    let group: CorticalUnitIndex = group.into();
                    let number_channels: CorticalChannelCount = number_channels.try_into().map_err(PyFeagiError::from)?;
                    let frame_change_handling: FrameChangeHandling = frame_change_handling.into();
                    let z_neuron_resolution: NeuronDepth = z_neuron_resolution.try_into().map_err(PyFeagiError::from)?;
                    let percentage_neuron_positioning: PercentageNeuronPositioning = percentage_neuron_positioning.into();

                    self.get_motor_cache().[<$motor_unit:snake _register>](group, number_channels, frame_change_handling, z_neuron_resolution, percentage_neuron_positioning).map_err(PyFeagiError::from)?;
                    Ok(())
                }
            }
        }

        motor_unit_functions!(@generate_similar_functions $motor_unit, Percentage3D);

    };

    // Arm for WrappedIOType::SignedPercentage
    (@generate_functions
        $motor_unit:ident,

        SignedPercentage
    ) => {
        ::paste::paste! {
            #[pymethods]
            impl PyConnectorAgent {
                pub fn [<motor_ $motor_unit:snake _register>](
                    &mut self,
                    _py: Python<'_>,
                    group: u8,
                    number_channels: u32,
                    frame_change_handling: PyFrameChangeHandling,
                    z_neuron_resolution: u32,
                    percentage_neuron_positioning: PyPercentageNeuronPositioning
                ) -> PyResult<()>
                {
                    let group: CorticalUnitIndex = group.into();
                    let number_channels: CorticalChannelCount = number_channels.try_into().map_err(PyFeagiError::from)?;
                    let frame_change_handling: FrameChangeHandling = frame_change_handling.into();
                    let z_neuron_resolution: NeuronDepth = z_neuron_resolution.try_into().map_err(PyFeagiError::from)?;
                    let percentage_neuron_positioning: PercentageNeuronPositioning = percentage_neuron_positioning.into();

                    self.get_motor_cache().[<$motor_unit:snake _register>](group, number_channels, frame_change_handling, z_neuron_resolution, percentage_neuron_positioning).map_err(PyFeagiError::from)?;
                    Ok(())
                }
            }
        }

        motor_unit_functions!(@generate_similar_functions $motor_unit, SignedPercentage);
    };

    // Arm for WrappedIOType::MiscData
    (@generate_functions
        $motor_unit:ident,

        MiscData
    ) => {
        ::paste::paste! {
            #[pymethods]
            impl PyConnectorAgent {
                pub fn [<motor_ $motor_unit:snake _register>](
                    &mut self,
                    _py: Python<'_>,
                    group: u8,
                    number_channels: u32,
                    frame_change_handling: &pyo3::Bound<PyFrameChangeHandling>,
                    misc_data_dimensions: &pyo3::Bound<PyMiscDataDimensions>,
                    ) -> PyResult<()>
                {

                    let group: CorticalUnitIndex = group.into();
                    let number_channels: CorticalChannelCount = number_channels.try_into().map_err(PyFeagiError::from)?;
                    let frame_change_handling: FrameChangeHandling = PyFrameChangeHandling::from_bound(frame_change_handling);
                    let misc_data_dimensions: MiscDataDimensions = PyMiscDataDimensions::copy_out_from_bound(misc_data_dimensions);

                    self.get_motor_cache().[<$motor_unit:snake _register>](group, number_channels, frame_change_handling, misc_data_dimensions).map_err(PyFeagiError::from)?;
                    Ok(())
                }
            }
        }
        motor_unit_functions!(@generate_similar_functions $motor_unit, MiscData);
    };

    // Arm for WrappedIOType::ImageFrame
    (@generate_functions
        $motor_unit:ident,

        ImageFrame
    ) => {
        // ImageFrame for motor output (oimg) - typically doesn't need Python connector registration
        // The motor device cache handles it internally via the Rust decoder.
        // Stub to satisfy macro - no-op.
    };

    // Arm for WrappedIOType::SpatialPointer3D
    (@generate_functions
        $motor_unit:ident,
        SpatialPointer3D
    ) => {
        ::paste::paste! {
            #[pymethods]
            impl PyConnectorAgent {
                /// Registers a SpatialPointer motor area.
                ///
                /// `frame_change_handling` selects the decode mechanism and the output type:
                /// - `Absolute` decodes an unsigned position (`Percentage3D`, axes in [0, 1]).
                ///   `window_ms`/`max_axis_velocity` are ignored.
                /// - `Incremental` decodes a signed motion vector (`SignedPercentage3D`, axes
                ///   in [-1, 1], 0 = no motion) and REQUIRES `window_ms` and
                ///   `max_axis_velocity` (the rolling-window length and the per-axis velocity
                ///   mapped to full scale).
                #[pyo3(signature = (
                    group,
                    number_channels,
                    frame_change_handling,
                    percentage_neuron_positioning,
                    width,
                    height,
                    depth,
                    window_ms=None,
                    max_axis_velocity=None,
                ))]
                #[allow(clippy::too_many_arguments)]
                pub fn [<motor_ $motor_unit:snake _register>](
                    &mut self,
                    _py: Python<'_>,
                    group: u8,
                    number_channels: u32,
                    frame_change_handling: PyFrameChangeHandling,
                    percentage_neuron_positioning: PyPercentageNeuronPositioning,
                    width: u32,
                    height: u32,
                    depth: u32,
                    window_ms: Option<u32>,
                    max_axis_velocity: Option<f32>,
                ) -> PyResult<()>
                {
                    let group: CorticalUnitIndex = group.into();
                    let number_channels: CorticalChannelCount =
                        number_channels.try_into().map_err(PyFeagiError::from)?;
                    let frame_change_handling: FrameChangeHandling = frame_change_handling.into();
                    let percentage_neuron_positioning: PercentageNeuronPositioning =
                        percentage_neuron_positioning.into();

                    let pointer_properties = match frame_change_handling {
                        FrameChangeHandling::Absolute => {
                            SpatialPointerProperties::new_absolute(width, height, depth)
                                .map_err(PyFeagiError::from)?
                        }
                        FrameChangeHandling::Incremental => {
                            let window_ms = window_ms.ok_or_else(|| {
                                PyFeagiError::from(FeagiDataError::BadParameters(
                                    "Incremental SpatialPointer requires window_ms".into(),
                                ))
                            })?;
                            let max_axis_velocity = max_axis_velocity.ok_or_else(|| {
                                PyFeagiError::from(FeagiDataError::BadParameters(
                                    "Incremental SpatialPointer requires max_axis_velocity".into(),
                                ))
                            })?;
                            SpatialPointerProperties::new_incremental(
                                width,
                                height,
                                depth,
                                window_ms,
                                max_axis_velocity,
                            )
                            .map_err(PyFeagiError::from)?
                        }
                    };

                    self.get_motor_cache()
                        .[<$motor_unit:snake _register>](
                            group,
                            number_channels,
                            frame_change_handling,
                            percentage_neuron_positioning,
                            pointer_properties,
                        )
                        .map_err(PyFeagiError::from)?;
                    Ok(())
                }

                /// Reads the preprocessed Incremental motion vector (`SignedPercentage3D`).
                ///
                /// Use in Incremental mode; Absolute mode uses
                /// `motor_*_read_preprocessed_cache_value` (unsigned position).
                pub fn [<motor_ $motor_unit:snake _read_signed_preprocessed_cache_value>](
                    &mut self,
                    _py: Python<'_>,
                    group: u8,
                    channel_index: u32,
                ) -> PyResult<PySignedPercentage3D> {
                    let group: CorticalUnitIndex = group.into();
                    let channel_index: CorticalChannelIndex = channel_index.into();
                    let expected_data = self
                        .get_motor_cache()
                        .[<$motor_unit:snake _read_signed_preprocessed_cache_value>](group, channel_index)
                        .map_err(PyFeagiError::from)?;
                    Ok(expected_data.into())
                }

                /// Reads the postprocessed Incremental motion vector (`SignedPercentage3D`).
                pub fn [<motor_ $motor_unit:snake _read_signed_postprocessed_cache_value>](
                    &mut self,
                    _py: Python<'_>,
                    group: u8,
                    channel_index: u32,
                ) -> PyResult<PySignedPercentage3D> {
                    let group: CorticalUnitIndex = group.into();
                    let channel_index: CorticalChannelIndex = channel_index.into();
                    let expected_data = self
                        .get_motor_cache()
                        .[<$motor_unit:snake _read_signed_postprocessed_cache_value>](group, channel_index)
                        .map_err(PyFeagiError::from)?;
                    Ok(expected_data.into())
                }
            }
        }

        motor_unit_functions!(@generate_similar_functions $motor_unit, Percentage3D);
    };

    // Arm for WrappedIOType::PoseEstimationData
    (@generate_functions
        $motor_unit:ident,
        PoseEstimationData
    ) => {
        // FEAGI core now exposes PoseEstimationData motor unit metadata through the template.
        // Python bindings for full pose schema objects are not implemented in this crate yet.
        // Provide a deterministic runtime error instead of failing compilation.
        ::paste::paste! {
            #[pymethods]
            impl PyConnectorAgent {
                pub fn [<motor_ $motor_unit:snake _register>](
                    &mut self,
                    _py: Python<'_>,
                    _group: u8,
                    _number_channels: u32,
                ) -> PyResult<()>
                {
                    Err(pyo3::exceptions::PyNotImplementedError::new_err(
                        "PoseEstimationData registration is not yet exposed in feagi_rust_py_libs Python bindings.",
                    ))
                }
            }
        }
    };

}

/// Dispatches `MotorCorticalUnit` to the matching `MotorDeviceCache::*_try_register_motor_callback`.
macro_rules! motor_python_callback_dispatcher {
    (
        MotorCorticalUnit {
            $(
                $(#[doc = $doc:expr])?
                $motor_variant:ident => {
                    $($inner:tt)*
                }
            ),* $(,)?
        }
    ) => {
        ::paste::paste! {
            fn connector_agent_dispatch_motor_python_callback<F>(
                motor_cache: &mut MotorDeviceCache,
                motor_unit: MotorCorticalUnit,
                group: CorticalUnitIndex,
                channel_index: CorticalChannelIndex,
                callback: F,
            ) -> Result<feagi_data_structures::FeagiSignalIndex, FeagiDataError>
            where
                F: Fn(&WrappedIOData) + Send + Sync + 'static,
            {
                match motor_unit {
                    $(
                        MotorCorticalUnit::$motor_variant => motor_cache
                            .[<$motor_variant:snake _try_register_motor_callback>](
                                group,
                                channel_index,
                                callback,
                            ),
                    )*
                }
            }
        }
    };
}

create_pyclass_no_clone_unsendable!(PyConnectorAgent, ConnectorCache, "ConnectorAgent");

impl PyConnectorAgent {
    fn get_sensor_cache(&self) -> MutexGuard<'_, SensorDeviceCache> {
        self.inner.get_sensor_cache()
    }

    fn get_motor_cache(&self) -> MutexGuard<'_, MotorDeviceCache> {
        self.inner.get_motor_cache()
    }
}

/// Initialize Rust tracing logging (call once from Python)
#[pyfunction]
pub fn init_rust_logging() {
    use std::sync::OnceLock;
    static INIT: OnceLock<()> = OnceLock::new();

    INIT.get_or_init(|| {
        use tracing_subscriber::{fmt, EnvFilter};

        // Default to INFO level if RUST_LOG not set
        let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));

        fmt()
            .with_env_filter(filter)
            .with_target(false)
            .with_thread_ids(false)
            .with_file(false)
            .try_init()
            // Avoid panicking if another module already installed a global subscriber.
            // This can happen if both connector_core and agent_sdk call init_rust_logging()
            // within the same Python process.
            .ok();
    });
}

#[pymethods]
impl PyConnectorAgent {
    #[new]
    #[pyo3(signature = (agent_descriptor_b64=None))]
    pub fn new(agent_descriptor_b64: Option<String>) -> PyResult<Self> {
        let _ = agent_descriptor_b64;
        Ok(PyConnectorAgent {
            inner: ConnectorCache::new(),
        })
    }

    /// Export all registered device capabilities as JSON string in new format
    ///
    /// Returns a JSON string containing all registered sensors and motors with their
    /// configurations including pipeline stages and friendly names.
    ///
    /// # Returns
    /// JSON string in format: {"capabilities": {"input": {...}, "output": {...}}}
    pub fn export_capabilities_json(&self, _py: Python<'_>) -> PyResult<String> {
        let json_value = self
            .inner
            .export_device_registrations_as_config_json()
            .map_err(PyFeagiError::from)?;
        serde_json::to_string_pretty(&json_value)
            .map_err(|e| PyFeagiError::from(FeagiDataError::SerializationError(e.to_string())))
            .map_err(Into::into)
    }

    /// Derive motor cortical IDs from the current device registrations.
    ///
    /// Returns the same IDs that FEAGI derives when auto-creating cortical areas.
    /// Use for verification so controller expectations match FEAGI exactly.
    pub fn get_motor_cortical_ids_for_verification(&self, py: Python<'_>) -> PyResult<Vec<String>> {
        let json_value = self
            .inner
            .export_device_registrations_as_config_json()
            .map_err(PyFeagiError::from)?;
        let ids = py.detach(|| {
            crate::feagi_connector_core::device_registration_derive::derive_motor_cortical_ids_from_device_registrations(&json_value)
        })
        .map_err(|e| PyFeagiError::from(FeagiDataError::BadParameters(e.into())))?;
        Ok(ids.into_iter().collect())
    }

    /// Derive sensory cortical IDs from the current device registrations.
    ///
    /// Returns the same IDs that FEAGI derives when auto-creating cortical areas.
    /// Use for verification so controller expectations match FEAGI exactly.
    pub fn get_sensory_cortical_ids_for_verification(
        &self,
        py: Python<'_>,
    ) -> PyResult<Vec<String>> {
        let json_value = self
            .inner
            .export_device_registrations_as_config_json()
            .map_err(PyFeagiError::from)?;
        let ids = py.detach(|| {
            crate::feagi_connector_core::device_registration_derive::derive_sensory_cortical_ids_from_device_registrations(&json_value)
        })
        .map_err(|e| PyFeagiError::from(FeagiDataError::BadParameters(e.into())))?;
        Ok(ids.into_iter().collect())
    }

    /// Import capabilities from JSON string (devices must be registered first!)
    ///
    /// Parses JSON and updates pipeline stages and friendly names for already-registered devices.
    /// Devices must be registered first using the appropriate register functions (e.g., sensor_simple_vision_register).
    ///
    /// # Arguments
    /// * `json_str` - JSON string in new capabilities format
    ///
    /// # Raises
    /// * `FeagiError` - If JSON is malformed or references unregistered devices
    pub fn import_capabilities_json(&mut self, json_str: &str, py: Python<'_>) -> PyResult<()> {
        py.detach(|| {
            let json_value: serde_json::Value = serde_json::from_str(json_str).map_err(|e| {
                PyFeagiError::from(feagi_data_structures::FeagiDataError::DeserializationError(
                    e.to_string(),
                ))
            })?;
            self.inner
                .import_device_registrations_as_config_json(json_value)
                .map_err(PyFeagiError::from)?;
            Ok(())
        })
    }

    /// Encode all cached sensor data to bytes
    ///
    /// Encodes all sensor data that has been written to cache into neuron voxel format
    /// and then serializes to FeagiByteContainer. This should be called after writing
    /// sensor data and before sending to FEAGI.
    pub fn sensors_encode_cached_sensor_data_to_bytes(&mut self) -> PyResult<()> {
        use std::time::Instant;

        let mut sensor_cache = self.get_sensor_cache();

        // Get current time for burst
        let time_of_burst = Instant::now();

        // Encode all sensors to neurons
        sensor_cache
            .encode_all_sensors_to_neurons(time_of_burst)
            .map_err(PyFeagiError::from)?;

        // Encode neurons to bytes
        sensor_cache
            .encode_neurons_to_bytes()
            .map_err(PyFeagiError::from)?;

        Ok(())
    }

    pub fn sensors_read_bytes(&mut self) -> PyResult<Vec<u8>> {
        let sensor_cache = self.get_sensor_cache();
        let byte_container = sensor_cache.get_feagi_byte_container();
        let bytes = byte_container.get_byte_ref().to_vec();
        Ok(bytes)
    }

    /// Can take in a BytesArray (faster) or Bytes. Loads into rust memory and ensures the structure is sound.
    pub fn motors_load_in_bytes_and_verify(
        &mut self,
        _py: Python<'_>,
        obj: &Bound<PyAny>,
    ) -> PyResult<()> {
        if let Ok(bytes) = Bound::cast::<PyByteArray>(obj) {
            let byte_data = bytes.to_vec();
            let mut motor_cache = self.get_motor_cache();
            let byte_container = motor_cache.get_feagi_byte_container_mut();
            byte_container
                .try_write_data_by_ownership_to_container_and_verify(byte_data)
                .map_err(PyFeagiError::from)?;
            return Ok(());
        } else if let Ok(bytes) = Bound::cast::<PyBytes>(obj) {
            let byte_data = bytes.extract::<&[u8]>()?;
            let mut motor_cache = self.get_motor_cache();
            let byte_container = motor_cache.get_feagi_byte_container_mut();
            byte_container
                .try_write_data_by_copy_and_verify(byte_data)
                .map_err(PyFeagiError::from)?;
            return Ok(());
        }
        Err(PyFeagiError::from(FeagiDataError::BadParameters(
            "Expected preferably a ByteArray or Bytes!".into(),
        ))
        .into())
    }

    pub fn motors_decode_cached_byte_data_to_motor(&mut self) -> PyResult<()> {
        let mut motor_cache = self.get_motor_cache();
        motor_cache
            .try_decode_bytes_to_neural_data()
            .map_err(PyFeagiError::from)?;
        motor_cache
            .try_decode_neural_data_into_cache(Instant::now())
            .map_err(PyFeagiError::from)?;
        Ok(())
    }

    /// Reads every registered motor unit's latest decoded value as a flat snapshot.
    ///
    /// Returns a list of `(group, channel, mode, value)` tuples where `mode` is
    /// `"absolute"` or `"incremental"` and `value` is a scalar (`[0, 1]` for unsigned
    /// types, `[-1, 1]` for signed types). Multi-axis units (e.g. SpatialPointer 3D)
    /// are flattened to one tuple per axis. This is the generic accessor used by the
    /// Python SDK to build its motor command map without per-unit typed reads.
    ///
    /// When ``updated_only`` is true, only channels updated since the last full
    /// snapshot read are included (see
    /// ``MotorDeviceCache::read_decoded_motor_snapshot_updated_only``).
    #[pyo3(signature = (updated_only=false))]
    pub fn motors_read_decoded_snapshot(
        &self,
        _py: Python<'_>,
        updated_only: bool,
    ) -> PyResult<Vec<(u32, u32, String, f64)>> {
        let motor_cache = self.get_motor_cache();
        let snapshot = if updated_only {
            motor_cache.read_decoded_motor_snapshot_updated_only()
        } else {
            motor_cache.read_decoded_motor_snapshot()
        };
        Ok(snapshot
            .into_iter()
            .map(|entry| {
                (
                    entry.group,
                    entry.channel,
                    entry.mode.to_string(),
                    entry.value,
                )
            })
            .collect())
    }

    /// Python SDK parity: route motor cache updates into a callable `(value, command_mode=None, ...)`.
    ///
    /// `motor_unit` is a [`crate::feagi_data_structures::genomic::PyMotorCorticalUnit`] or a length-1 tuple
    /// containing one (matching existing SDK call sites).
    #[pyo3(signature = (*, motor_unit, group, channel, callback, command_mode=None))]
    pub fn register_callback(
        &mut self,
        py: Python<'_>,
        motor_unit: Bound<'_, PyAny>,
        group: u8,
        channel: u32,
        callback: Py<PyAny>,
        command_mode: Option<String>,
    ) -> PyResult<()> {
        use crate::feagi_connector_core::wrapped_io_data::wrapped_io_data_to_py_object;
        use crate::feagi_data_structures::genomic::PyMotorCorticalUnit;

        fn extract_py_motor_unit(any: &Bound<'_, PyAny>) -> PyResult<PyMotorCorticalUnit> {
            if let Ok(u) = any.extract::<PyMotorCorticalUnit>() {
                return Ok(u);
            }
            if let Ok(tuple) = Bound::cast::<PyTuple>(any) {
                if tuple.len() == 1 {
                    if let Ok(first) = tuple.get_item(0) {
                        if let Ok(u) = first.extract::<PyMotorCorticalUnit>() {
                            return Ok(u);
                        }
                    }
                }
            }
            Err(PyTypeError::new_err(
                "motor_unit must be MotorCorticalUnit (feagi_rust_py_libs.data_structures.genomic) \
                 or a 1-tuple containing it",
            ))
        }

        let py_unit = extract_py_motor_unit(&motor_unit)?;
        let rust_unit: MotorCorticalUnit = py_unit.into();
        let group: CorticalUnitIndex = group.into();
        let channel: CorticalChannelIndex = channel.into();

        let py_cb = callback.clone_ref(py);
        let command_mode = command_mode;

        let bridge = move |wired: &WrappedIOData| {
            #[allow(deprecated)]
            // FIXME: migrate to Python::attach when FEAGI's MSRV/Python policy settles
            Python::with_gil(|py| {
                let py_val: Py<PyAny> = match wrapped_io_data_to_py_object(py, wired) {
                    Ok(v) => v,
                    Err(e) => {
                        tracing::warn!(
                            target: "feagi_connector_py",
                            "register_callback: failed to convert motor value to Python: {}",
                            e
                        );
                        return;
                    }
                };
                let mode_arg: Py<PyAny> = match &command_mode {
                    Some(mode) => pyo3::types::PyString::new(py, mode).into_any().unbind(),
                    None => py.None(),
                };
                if let Err(e) = py_cb.call1(py, (py_val, mode_arg)) {
                    tracing::warn!(
                        target: "feagi_connector_py",
                        "register_callback: Python motor handler raised: {}",
                        e
                    );
                }
            });
        };

        let mut motor_cache = self.get_motor_cache();
        connector_agent_dispatch_motor_python_callback(
            &mut motor_cache,
            rust_unit,
            group,
            channel,
            bridge,
        )
        .map_err(PyFeagiError::from)?;
        Ok(())
    }

    /// Seed PositionalServo preprocessed cache value (`Percentage` in `[0, 1]`).
    ///
    /// Controllers use this to align incremental decoder state to live hardware
    /// angle before consuming FEAGI motor bytes, preventing first-command snaps.
    pub fn motor_positional_servo_write_preprocessed_cache_value(
        &mut self,
        _py: Python<'_>,
        group: u8,
        channel_index: u32,
        value_0_1: f64,
    ) -> PyResult<()> {
        let group: CorticalUnitIndex = group.into();
        let channel_index: CorticalChannelIndex = channel_index.into();
        let value = (value_0_1 as f32).clamp(0.0, 1.0);
        let value = Percentage::new_from_0_1(value).map_err(PyFeagiError::from)?;
        self.get_motor_cache()
            .motor_positional_servo_write_preprocessed_cache_value(group, channel_index, value)
            .map_err(PyFeagiError::from)?;
        Ok(())
    }

    // While technically possible, we are going to discourage grabbing the FeagiByteContainer directly and
    // instead push to use the above methods to access the byte data, as they make use of
    // internal optimizations
    /*
    /// Get the encoded sensor byte container
    ///
    /// Returns the FeagiByteContainer after encoding. Call sensors_encode_cached_data_to_bytes()
    /// first to encode the data.
    pub fn sensor_get_byte_container(&self) -> PyResult<PyFeagiByteContainer> {
        use crate::feagi_serialization::PyFeagiByteContainer;

        let sensor_cache = self.get_sensor_cache();
        let byte_container = sensor_cache.get_feagi_byte_container();

        // Convert to PyFeagiByteContainer (clone the inner FeagiByteContainer)
        // PyFeagiByteContainer has pub(crate) inner field, so we can create it directly
        Ok(PyFeagiByteContainer {
            inner: byte_container.clone()
        })

            /// Get the encoded motor byte container
    ///
    /// Returns the FeagiByteContainer after encoding. Call motors_encode_cached_data_to_bytes()
    /// first to encode the data.
    pub fn motor_get_byte_container(&self) -> PyResult<PyFeagiByteContainer> {
        use crate::feagi_serialization::PyFeagiByteContainer;

        let motor_cache = self.get_motor_cache();
        let byte_container = motor_cache.get_feagi_byte_container();

        // Convert to PyFeagiByteContainer
        Ok(PyFeagiByteContainer {
            inner: byte_container.clone()
        })
    }
    }

     */
}

sensor_cortical_units!(sensor_unit_functions);

motor_cortical_units!(motor_unit_functions);
motor_cortical_units!(motor_python_callback_dispatcher);
