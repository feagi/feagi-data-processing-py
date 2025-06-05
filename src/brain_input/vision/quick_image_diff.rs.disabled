use pyo3::prelude::*;
use pyo3::exceptions::PyValueError;
use feagi_core_data_structures_and_processing::brain_input::vision::quick_image_diff::QuickImageDiff;

use super::image_frame::PyImageFrame;
use super::descriptors::*;

#[pyclass]
#[pyo3(name = "QuickImageDiff")]
pub struct PyQuickImageDiff {
    pub inner: QuickImageDiff,
}

#[pymethods]
impl PyQuickImageDiff {
    #[new]
    pub fn new_from_preprocessor(
        preprocessor_parameters: &PyFrameProcessingParameters,
        output_color_channels: PyChannelFormat,
        output_color_space: PyColorSpace,
        output_segment_resolutions: &PySegmentedVisionTargetResolutions,
        segmentation_center_properties: &PySegmentedVisionCenterProperties,
        camera_index: u8
    ) -> PyResult<Self> {
        match QuickImageDiff::new_from_preprocessor(
            preprocessor_parameters.inner,
            &output_color_channels.into(),
            &output_color_space.into(),
            output_segment_resolutions.inner,
            segmentation_center_properties.inner,
            camera_index
        ) {
            Ok(inner) => Ok(PyQuickImageDiff { inner }),
            Err(err) => Err(PyErr::new::<PyValueError, _>(err.to_string())),
        }
    }

    pub fn process_incoming_image_to_proper_diff_image(
        &mut self,
        incoming_image_frame: PyImageFrame,
        pixel_threshold: u8,
        camera_index: u8
    ) -> PyResult<Vec<u8>> {
        match self.inner.process_incoming_image_to_proper_diff_image(
            incoming_image_frame.inner,
            pixel_threshold,
            camera_index
        ) {
            Ok(bytes) => Ok(bytes),
            Err(err) => Err(PyErr::new::<PyValueError, _>(err.to_string())),
        }
    }
}