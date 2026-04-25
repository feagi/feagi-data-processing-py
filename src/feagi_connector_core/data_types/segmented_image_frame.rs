use crate::feagi_connector_core::data_types::descriptors::*;
use crate::{__base_py_class_shared, create_pyclass};
use feagi_sensorimotor::data_types::SegmentedImageFrame;
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;

create_pyclass!(
    PySegmentedImageFrame,
    SegmentedImageFrame,
    "SegmentedImageFrame"
);

#[pymethods]
impl PySegmentedImageFrame {
    //region Common Constructors
    #[new]
    pub fn new(
        segment_resolutions: PySegmentedXYImageResolutions,
        segment_color_space: PyColorSpace,
        center_color_channels: PyColorChannelLayout,
        peripheral_color_channels: PyColorChannelLayout,
    ) -> PyResult<Self> {
        match SegmentedImageFrame::new(
            &segment_resolutions.into(),
            &segment_color_space.into(),
            &center_color_channels.into(),
            &peripheral_color_channels.into(),
        ) {
            Ok(inner) => Ok(PySegmentedImageFrame { inner }),
            Err(err) => Err(PyErr::new::<PyValueError, _>(err.to_string())),
        }
    }

    #[staticmethod]
    pub fn from_segmented_image_frame_properties(
        properties: PySegmentedImageFrameProperties,
    ) -> PyResult<Self> {
        let result = SegmentedImageFrame::from_segmented_image_frame_properties(&properties.into());
        match result {
            Ok(segmented) => Ok(PySegmentedImageFrame { inner: segmented }),
            Err(err) => Err(PyErr::new::<PyValueError, _>(err.to_string())),
        }
    }
    //endregion

    //region Get Properties

    pub fn get_segmented_image_frame_properties(&self) -> PySegmentedImageFrameProperties {
        self.inner.get_segmented_image_frame_properties().into()
    }

    #[getter]
    pub fn color_space(&self) -> PyColorSpace {
        self.inner.get_color_space().clone().into()
    }

    #[getter]
    pub fn center_channel_layout(&self) -> PyColorChannelLayout {
        self.inner.get_center_channel_layout().into()
    }

    #[getter]
    pub fn peripheral_channel_layout(&self) -> PyColorChannelLayout {
        self.inner.get_peripheral_channel_layout().into()
    }
    #[getter]
    pub fn segmented_frame_target_resolutions(&self) -> PySegmentedXYImageResolutions {
        self.inner.get_segmented_frame_target_resolutions().into()
    }
    //endregion
}
