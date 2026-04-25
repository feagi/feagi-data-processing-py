use crate::feagi_connector_core::data_types::{PyImageFrame, PySegmentedImageFrame};
use crate::py_error::PyFeagiError;
use crate::{__base_py_class_shared, create_pyclass};
use feagi_sensorimotor::data_types::descriptors::*;
use pyo3::prelude::*;
use pyo3::{pyclass, pymethods, PyResult};

//region Images

//region Image XY

create_pyclass!(PyImageXYPoint, ImageXYPoint, "ImageXYPoint");

#[pymethods]
impl PyImageXYPoint {
    #[new]
    pub fn new(x: u32, y: u32) -> PyResult<Self> {
        Ok(PyImageXYPoint {
            inner: ImageXYPoint::new(x, y),
        })
    }

    #[getter]
    pub fn x(&self) -> u32 {
        self.inner.x
    }

    #[getter]
    pub fn y(&self) -> u32 {
        self.inner.y
    }
}

impl TryFrom<(u32, u32)> for PyImageXYPoint {
    type Error = PyErr;
    fn try_from(value: (u32, u32)) -> Result<Self, Self::Error> {
        PyImageXYPoint::new(value.0, value.1)
    }
}

impl From<PyImageXYPoint> for (u32, u32) {
    fn from(value: PyImageXYPoint) -> Self {
        (value.inner.x, value.inner.y)
    }
}

create_pyclass!(PyImageXYResolution, ImageXYResolution, "ImageXYResolution");

#[pymethods]
impl PyImageXYResolution {
    #[new]
    pub fn new(width: u32, height: u32) -> PyResult<Self> {
        Ok(PyImageXYResolution {
            inner: ImageXYResolution::new(width, height).map_err(PyFeagiError::from)?,
        })
    }

    #[getter]
    pub fn width(&self) -> u32 {
        self.inner.width
    }

    #[getter]
    pub fn height(&self) -> u32 {
        self.inner.height
    }
}

impl TryFrom<(u32, u32)> for PyImageXYResolution {
    type Error = PyErr;
    fn try_from(value: (u32, u32)) -> Result<Self, Self::Error> {
        PyImageXYResolution::new(value.0, value.1)
    }
}

impl From<PyImageXYResolution> for (u32, u32) {
    fn from(value: PyImageXYResolution) -> Self {
        (value.inner.width, value.inner.height)
    }
}

//endregion

//region Image XYZ

create_pyclass!(
    PyImageXYZDimensions,
    ImageXYZDimensions,
    "ImageXYZDimensions"
);

#[pymethods]
impl PyImageXYZDimensions {
    #[new]
    pub fn new(x: u32, y: u32, z: u32) -> PyResult<Self> {
        Ok(PyImageXYZDimensions {
            inner: ImageXYZDimensions::new(x, y, z).map_err(PyFeagiError::from)?,
        })
    }

    #[getter]
    pub fn width(&self) -> u32 {
        self.inner.width
    }

    #[getter]
    pub fn height(&self) -> u32 {
        self.inner.height
    }

    #[getter]
    pub fn depth(&self) -> u32 {
        self.inner.depth
    }
}

impl TryFrom<(u32, u32, u32)> for PyImageXYZDimensions {
    type Error = PyErr;
    fn try_from(value: (u32, u32, u32)) -> Result<Self, Self::Error> {
        PyImageXYZDimensions::new(value.0, value.1, value.2)
    }
}

impl From<PyImageXYZDimensions> for (u32, u32, u32) {
    fn from(value: PyImageXYZDimensions) -> Self {
        (value.inner.width, value.inner.height, value.inner.depth)
    }
}

//endregion

//region Segmented Image XY Resolutions

create_pyclass!(
    PySegmentedXYImageResolutions,
    SegmentedXYImageResolutions,
    "SegmentedXYImageResolutions"
);

#[pymethods]
impl PySegmentedXYImageResolutions {
    #[new]
    pub fn new(
        lower_left: PyImageXYResolution,
        lower_middle: PyImageXYResolution,
        lower_right: PyImageXYResolution,
        middle_left: PyImageXYResolution,
        center: PyImageXYResolution,
        middle_right: PyImageXYResolution,
        upper_left: PyImageXYResolution,
        upper_middle: PyImageXYResolution,
        upper_right: PyImageXYResolution,
    ) -> Self {
        let inner = SegmentedXYImageResolutions::new(
            lower_left.into(),
            lower_middle.into(),
            lower_right.into(),
            middle_left.into(),
            center.into(),
            middle_right.into(),
            upper_left.into(),
            upper_middle.into(),
            upper_right.into(),
        );
        PySegmentedXYImageResolutions { inner }
    }

    #[staticmethod]
    pub fn create_with_same_sized_peripheral(
        center_resolution: PyImageXYResolution,
        peripheral_resolutions: PyImageXYResolution,
    ) -> PySegmentedXYImageResolutions {
        SegmentedXYImageResolutions::create_with_same_sized_peripheral(
            center_resolution.into(),
            peripheral_resolutions.into(),
        )
        .into()
    }

    pub fn as_ordered_array(&self) -> Vec<PyImageXYResolution> {
        let refs = self.inner.as_ordered_array();
        refs.iter().map(|&res| res.clone().into()).collect()
    }

    #[getter]
    pub fn lower_left(&self) -> PyImageXYResolution {
        self.inner.lower_left.into()
    }

    #[getter]
    pub fn lower_middle(&self) -> PyImageXYResolution {
        self.inner.lower_middle.into()
    }

    #[getter]
    pub fn lower_right(&self) -> PyImageXYResolution {
        self.inner.lower_right.into()
    }

    #[getter]
    pub fn middle_left(&self) -> PyImageXYResolution {
        self.inner.middle_left.into()
    }

    #[getter]
    pub fn center(&self) -> PyImageXYResolution {
        self.inner.center.into()
    }

    #[getter]
    pub fn middle_right(&self) -> PyImageXYResolution {
        self.inner.middle_right.into()
    }

    #[getter]
    pub fn upper_left(&self) -> PyImageXYResolution {
        self.inner.upper_left.into()
    }

    #[getter]
    pub fn upper_middle(&self) -> PyImageXYResolution {
        self.inner.upper_middle.into()
    }

    #[getter]
    pub fn upper_right(&self) -> PyImageXYResolution {
        self.inner.upper_right.into()
    }
}

//endregion

//region Enums

//region ColorSpace

// Add ColorSpace enum for Python
#[pyclass(eq, eq_int)]
#[derive(PartialEq, Clone)]
#[pyo3(name = "ColorSpace")]
pub enum PyColorSpace {
    Linear,
    Gamma,
}

impl From<PyColorSpace> for ColorSpace {
    fn from(py_color_space: PyColorSpace) -> Self {
        match py_color_space {
            PyColorSpace::Linear => ColorSpace::Linear,
            PyColorSpace::Gamma => ColorSpace::Gamma,
        }
    }
}

impl From<ColorSpace> for PyColorSpace {
    fn from(color_space: ColorSpace) -> Self {
        match color_space {
            ColorSpace::Linear => PyColorSpace::Linear,
            ColorSpace::Gamma => PyColorSpace::Gamma,
        }
    }
}

//endregion

//region ColorChannelLayout

#[pyclass(eq, eq_int)]
#[derive(PartialEq, Clone)]
#[pyo3(name = "ColorChannelLayout")]
pub enum PyColorChannelLayout {
    GrayScale,
    RG,
    RGB,
    RGBA,
}

impl From<PyColorChannelLayout> for ColorChannelLayout {
    fn from(py_channel_format: PyColorChannelLayout) -> Self {
        match py_channel_format {
            PyColorChannelLayout::GrayScale => ColorChannelLayout::GrayScale,
            PyColorChannelLayout::RG => ColorChannelLayout::RG,
            PyColorChannelLayout::RGB => ColorChannelLayout::RGB,
            PyColorChannelLayout::RGBA => ColorChannelLayout::RGBA,
        }
    }
}

impl From<ColorChannelLayout> for PyColorChannelLayout {
    fn from(channel_layout: ColorChannelLayout) -> Self {
        match channel_layout {
            ColorChannelLayout::GrayScale => PyColorChannelLayout::GrayScale,
            ColorChannelLayout::RG => PyColorChannelLayout::RG,
            ColorChannelLayout::RGB => PyColorChannelLayout::RGB,
            ColorChannelLayout::RGBA => PyColorChannelLayout::RGBA,
        }
    }
}

impl From<&ColorChannelLayout> for PyColorChannelLayout {
    fn from(channel_layout: &ColorChannelLayout) -> Self {
        channel_layout.clone().into()
    }
}

//endregion

//region MemoryOrderLayout

#[pyclass(eq, eq_int)]
#[derive(PartialEq, Clone)]
#[pyo3(name = "MemoryOrderLayout")]
pub enum PyMemoryOrderLayout {
    HeightsWidthsChannels, // default, also called row major
    ChannelsHeightsWidths, // common in machine learning
    WidthsHeightsChannels, // cartesian, the best one
    HeightsChannelsWidths,
    ChannelsWidthsHeights,
    WidthsChannelsHeights,
}

impl From<PyMemoryOrderLayout> for MemoryOrderLayout {
    fn from(py_memory_layout: PyMemoryOrderLayout) -> Self {
        match py_memory_layout {
            PyMemoryOrderLayout::HeightsWidthsChannels => MemoryOrderLayout::HeightsWidthsChannels,
            PyMemoryOrderLayout::ChannelsHeightsWidths => MemoryOrderLayout::ChannelsHeightsWidths,
            PyMemoryOrderLayout::WidthsHeightsChannels => MemoryOrderLayout::WidthsHeightsChannels,
            PyMemoryOrderLayout::HeightsChannelsWidths => MemoryOrderLayout::HeightsChannelsWidths,
            PyMemoryOrderLayout::ChannelsWidthsHeights => MemoryOrderLayout::ChannelsWidthsHeights,
            PyMemoryOrderLayout::WidthsChannelsHeights => MemoryOrderLayout::WidthsChannelsHeights,
        }
    }
}

impl From<MemoryOrderLayout> for PyMemoryOrderLayout {
    fn from(memory_order_layout: MemoryOrderLayout) -> Self {
        match memory_order_layout {
            MemoryOrderLayout::HeightsWidthsChannels => PyMemoryOrderLayout::HeightsWidthsChannels,
            MemoryOrderLayout::HeightsChannelsWidths => PyMemoryOrderLayout::HeightsChannelsWidths,
            MemoryOrderLayout::ChannelsHeightsWidths => PyMemoryOrderLayout::ChannelsHeightsWidths,
            MemoryOrderLayout::ChannelsWidthsHeights => PyMemoryOrderLayout::ChannelsWidthsHeights,
            MemoryOrderLayout::WidthsChannelsHeights => PyMemoryOrderLayout::WidthsChannelsHeights,
            MemoryOrderLayout::WidthsHeightsChannels => PyMemoryOrderLayout::WidthsChannelsHeights,
        }
    }
}

//endregion

//endregion

//region Image Frame Properties

create_pyclass!(
    PyImageFrameProperties,
    ImageFrameProperties,
    "ImageFrameProperties"
);

#[pymethods]
impl PyImageFrameProperties {
    #[new]
    pub fn new(
        xy_resolution: PyImageXYResolution,
        color_space: PyColorSpace,
        color_channel_layout: PyColorChannelLayout,
    ) -> PyResult<Self> {
        let color_space: ColorSpace = color_space.into();
        let color_channel_layout: ColorChannelLayout = color_channel_layout.into();
        let inner =
            ImageFrameProperties::new(xy_resolution.into(), color_space, color_channel_layout)
                .map_err(PyFeagiError::from)?;
        Ok(PyImageFrameProperties { inner })
    }

    #[getter]
    pub fn xy_resolution(&self) -> PyResult<PyImageXYResolution> {
        Ok(self.inner.get_image_resolution().into())
    }

    #[getter]
    pub fn color_space(&self) -> PyResult<PyColorSpace> {
        Ok(self.inner.get_color_space().into())
    }

    #[getter]
    pub fn channel_layout(&self) -> PyResult<PyColorChannelLayout> {
        Ok(self.inner.get_color_channel_layout().into())
    }

    pub fn get_number_of_channels(&self) -> usize {
        self.inner.get_number_of_channels()
    }

    pub fn get_number_of_samples(&self) -> usize {
        self.inner.get_number_of_samples()
    }

    pub fn verify_image_frame_matches_properties(
        &self,
        image_frame: &PyImageFrame,
    ) -> PyResult<()> {
        self.inner
            .verify_image_frame_matches_properties(&image_frame.inner)
            .map_err(PyFeagiError::from)?;
        Ok(())
    }
}

//endregion

//region Segmented Image Frame Properties

create_pyclass!(
    PySegmentedImageFrameProperties,
    SegmentedImageFrameProperties,
    "SegmentedImageFrameProperties"
);

#[pymethods]
impl PySegmentedImageFrameProperties {
    #[new]
    pub fn new(
        segment_xy_resolutions: PySegmentedXYImageResolutions,
        center_color_channels: PyColorChannelLayout,
        peripheral_color_channels: PyColorChannelLayout,
        color_space: PyColorSpace,
    ) -> PyResult<Self> {
        Ok(SegmentedImageFrameProperties::new(
            segment_xy_resolutions.into(),
            center_color_channels.into(),
            peripheral_color_channels.into(),
            color_space.into(),
        )
        .into())
    }

    #[getter]
    pub fn resolutions(&self) -> PySegmentedXYImageResolutions {
        self.inner.get_resolutions().clone().into()
    }

    #[getter]
    pub fn center_color_channel(&self) -> PyColorChannelLayout {
        self.inner.get_center_color_channel().into()
    }

    #[getter]
    pub fn peripheral_color_channels(&self) -> PyColorChannelLayout {
        self.inner.get_peripheral_color_channels().into()
    }

    #[getter]
    pub fn color_space(&self) -> PyColorSpace {
        self.inner.get_color_space().clone().into()
    }

    pub fn verify_segmented_image_frame_matches_properties(
        &self,
        segmented_image_frame: &PySegmentedImageFrame,
    ) -> PyResult<()> {
        self.inner
            .verify_segmented_image_frame_matches_properties(&segmented_image_frame.inner)
            .map_err(PyFeagiError::from)?;
        Ok(())
    }
}

//endregion

//region Corner Points

create_pyclass!(PyCornerPoints, CornerPoints, "CornerPoints");

#[pymethods]
impl PyCornerPoints {
    #[new]
    pub fn new(upper_left: PyImageXYPoint, lower_right: PyImageXYPoint) -> PyResult<Self> {
        let inner =
            CornerPoints::new(upper_left.into(), lower_right.into()).map_err(PyFeagiError::from)?;
        Ok(PyCornerPoints { inner })
    }

    #[getter]
    pub fn upper_left(&self) -> PyImageXYPoint {
        self.inner.upper_left.into()
    }

    #[getter]
    pub fn lower_right(&self) -> PyImageXYPoint {
        self.inner.lower_right.into()
    }

    pub fn get_upper_right(&self) -> PyImageXYPoint {
        self.inner.get_upper_right().into()
    }

    pub fn get_lower_left(&self) -> PyImageXYPoint {
        self.inner.get_lower_left().into()
    }

    pub fn get_width(&self) -> u32 {
        self.inner.get_width()
    }

    pub fn get_height(&self) -> u32 {
        self.inner.get_height()
    }

    pub fn enclosed_area_width_height(&self) -> PyImageXYResolution {
        self.inner.enclosed_area_width_height().into()
    }

    pub fn verify_fits_in_resolution(&self, resolution: PyImageXYResolution) -> PyResult<()> {
        self.inner
            .verify_fits_in_resolution(resolution.into())
            .map_err(PyFeagiError::from)?;
        Ok(())
    }
}

//endregion

//endregion

//region Misc Data Dimensions

create_pyclass!(
    PyMiscDataDimensions,
    MiscDataDimensions,
    "MiscDataDimensions"
);

#[pymethods]
impl PyMiscDataDimensions {
    #[new]
    pub fn new(x: u32, y: u32, z: u32) -> PyResult<Self> {
        Ok(PyMiscDataDimensions {
            inner: MiscDataDimensions::new(x, y, z).map_err(PyFeagiError::from)?,
        })
    }

    #[getter]
    pub fn width(&self) -> u32 {
        self.inner.width
    }

    #[getter]
    pub fn height(&self) -> u32 {
        self.inner.height
    }

    #[getter]
    pub fn depth(&self) -> u32 {
        self.inner.depth
    }
}

//endregion
