use crate::feagi_connector_core::data_types::processing::PyImageFrameProcessor;
use crate::feagi_connector_core::wrapped_io_data::PyWrappedIOType;
use feagi_sensorimotor::data_pipeline::PipelineStageProperties;
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::{pymethods, PyResult};

/// PyO3 wrapper for PipelineStageProperties enum
///
/// Note: PipelineStageProperties changed from trait-based to enum-based in json_config merge.
/// Individual stage property wrappers are temporarily disabled until they can be properly
/// reimplemented to work with the new enum structure.
#[pyclass(name = "PipelineStageProperties")]
#[derive(Clone)]
pub struct PyPipelineStageProperties {
    pub(crate) inner: PipelineStageProperties,
}

#[pymethods]
impl PyPipelineStageProperties {
    pub fn get_input_data_type(&self) -> PyResult<PyWrappedIOType> {
        let result = self.inner.get_input_data_type();
        Ok(result.into())
    }

    pub fn get_output_data_type(&self) -> PyResult<PyWrappedIOType> {
        let result = self.inner.get_output_data_type();
        Ok(result.into())
    }

    pub fn variant_name(&self) -> PyResult<String> {
        Ok(self.inner.variant_name().to_string())
    }

    /// Returns the transformer definition for `ImageFrameProcessor` stages.
    ///
    /// This is used by the Python SDK to update brightness/contrast dynamically by
    /// reading the existing processor config, mutating it, and writing it back.
    pub fn get_transformer_definition(&self) -> PyResult<PyImageFrameProcessor> {
        match &self.inner {
            PipelineStageProperties::ImageFrameProcessor {
                transformer_definition,
            } => Ok(transformer_definition.clone().into()),
            other => Err(PyValueError::new_err(format!(
                "Stage properties are not ImageFrameProcessor (got {})",
                other.variant_name()
            ))),
        }
    }

    // Constructor methods for enum variants
    #[staticmethod]
    pub fn new_image_frame_segmentator(
        input_props: crate::feagi_connector_core::data_types::descriptors::PyImageFrameProperties,
        output_props: crate::feagi_connector_core::data_types::descriptors::PySegmentedImageFrameProperties,
        gaze: crate::feagi_connector_core::data_types::PyGazeProperties,
    ) -> PyResult<Self> {
        Ok(Self {
            inner: PipelineStageProperties::ImageFrameSegmentator {
                input_image_properties: input_props.inner,
                output_image_properties: output_props.inner,
                segmentation_gaze: gaze.inner,
            },
        })
    }

    #[staticmethod]
    pub fn new_image_quick_diff(
        per_pixel_min: u8,
        per_pixel_max: u8,
        activity_min: crate::feagi_connector_core::data_types::PyPercentage,
        activity_max: crate::feagi_connector_core::data_types::PyPercentage,
        input_props: crate::feagi_connector_core::data_types::descriptors::PyImageFrameProperties,
    ) -> PyResult<Self> {
        use std::ops::RangeInclusive;
        Ok(Self {
            inner: PipelineStageProperties::ImageQuickDiff {
                per_pixel_allowed_range: RangeInclusive::new(per_pixel_min, per_pixel_max),
                acceptable_amount_of_activity_in_image: RangeInclusive::new(
                    activity_min.inner,
                    activity_max.inner,
                ),
                image_properties: input_props.inner,
            },
        })
    }

    #[staticmethod]
    pub fn new_image_frame_processor(
        transformer_definition: crate::feagi_connector_core::data_types::processing::PyImageFrameProcessor,
    ) -> PyResult<Self> {
        Ok(Self {
            inner: PipelineStageProperties::ImageFrameProcessor {
                transformer_definition: transformer_definition.into(),
            },
        })
    }
}

impl From<PipelineStageProperties> for PyPipelineStageProperties {
    fn from(inner: PipelineStageProperties) -> Self {
        Self { inner }
    }
}

impl From<PyPipelineStageProperties> for PipelineStageProperties {
    fn from(val: PyPipelineStageProperties) -> Self {
        val.inner
    }
}

impl PyPipelineStageProperties {
    /// Convert a single Python PyPipelineStageProperties to Rust PipelineStageProperties
    pub fn from_py_to_box(
        py: Python<'_>,
        py_stage: &Py<PyPipelineStageProperties>,
    ) -> pyo3::PyResult<PipelineStageProperties> {
        let stage = py_stage.borrow(py);
        Ok(stage.inner.clone())
    }

    /// Convert a vector of Python PyPipelineStageProperties to Rust PipelineStageProperties
    pub fn from_vec_py_to_vec(
        py_stages: Vec<Py<PyPipelineStageProperties>>,
    ) -> pyo3::PyResult<Vec<PipelineStageProperties>> {
        Python::attach(|py| {
            py_stages
                .into_iter()
                .map(|py_stage| {
                    let stage = py_stage.borrow(py);
                    Ok(stage.inner.clone())
                })
                .collect()
        })
    }

    /// Convert Rust PipelineStageProperties enum to Python wrapper (for compatibility with old API)
    pub fn from_box_to_parent_typed(
        py: Python<'_>,
        stage: PipelineStageProperties,
    ) -> PyResult<Py<PyPipelineStageProperties>> {
        Py::new(py, PyPipelineStageProperties { inner: stage })
    }

    /// Convert vector of Rust PipelineStageProperties to vector of Python wrappers (for compatibility with old API)
    pub fn from_vec_box_to_vec_parent_typed(
        py: Python<'_>,
        stages: Vec<PipelineStageProperties>,
    ) -> PyResult<Vec<Py<PyPipelineStageProperties>>> {
        stages
            .into_iter()
            .map(|stage| Py::new(py, PyPipelineStageProperties { inner: stage }))
            .collect()
    }
}
