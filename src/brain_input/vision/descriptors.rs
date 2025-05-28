use pyo3::{pyclass, pymethods, PyResult};
use pyo3::prelude::*;
use pyo3::exceptions::PyValueError;
use feagi_core_data_structures_and_processing::brain_input::vision::descriptors::*;


#[pyclass]
#[pyo3(name = "FrameProcessingParameters")]
#[derive(Clone)]
pub struct PyFrameProcessingParameters {
    pub inner: FrameProcessingParameters,
}

#[pymethods]
impl PyFrameProcessingParameters {
    #[new]
    fn new() -> PyResult<Self> {
        Ok(PyFrameProcessingParameters {
            inner: FrameProcessingParameters::new(),
        })
    }
    
    fn clear_all_settings(&mut self) {
        self.inner.clear_all_settings();
    }
    
    fn set_cropping_from(&mut self, cropping_from: &PyCornerPoints) -> PyResult<()> {
        self.inner.set_cropping_from(cropping_from.inner);
        Ok(())
    }
    
    fn set_resizing_to(&mut self, resizing_to: (usize, usize)) -> PyResult<()> {
        self.inner.set_resizing_to(resizing_to);
        Ok(())
    }
    
    fn set_multiply_brightness_by(&mut self, multiply_brightness_by: f32) -> PyResult<()> {
        match self.inner.set_multiply_brightness_by(multiply_brightness_by) {
            Ok(_) => Ok(()),
            Err(err) => Err(PyErr::new::<PyValueError, _>(err.to_string())),
        }
    }
    
    fn set_change_contrast_by(&mut self, change_contrast_by: f32) -> PyResult<()> {
        match self.inner.set_change_contrast_by(change_contrast_by) {
            Ok(_) => Ok(()),
            Err(err) => Err(PyErr::new::<PyValueError, _>(err.to_string())),
        }
    }
    
    fn set_source_array_ordering(&mut self, new_source_array_ordering: PyMemoryOrderLayout) -> PyResult<()> {
        match self.inner.set_source_array_ordering(new_source_array_ordering.into()) {
            Ok(_) => Ok(()),
            Err(err) => Err(PyErr::new::<PyValueError, _>(err.to_string())),
        }
    }
    
    fn enable_convert_to_grayscale(&mut self) -> PyResult<()> {
        self.inner.enable_convert_to_grayscale();
        Ok(())
    }
    
    fn enable_convert_to_color_space_to(&mut self, color_space: PyColorSpace) -> PyResult<()> {
        self.inner.enable_convert_to_color_space_to(color_space.into());
        Ok(())
    }
    
    fn process_steps_required_to_run(&self) -> (bool, bool, bool, bool, bool, bool) {
        self.inner.process_steps_required_to_run()
    }
    
    fn get_final_width_height(&self) -> PyResult<(usize, usize)> {
        match self.inner.get_final_width_height() {
            Ok(dimensions) => Ok(dimensions),
            Err(err) => Err(PyErr::new::<PyValueError, _>(err.to_string())),
        }
    }
}

#[pyclass]
#[pyo3(name = "CornerPoints")]
#[derive(Clone)]
pub struct PyCornerPoints {
    pub inner: CornerPoints,
}

#[pymethods]
impl PyCornerPoints {
    #[new]
    fn new_from_row_major_where_origin_top_left(lower_left: (usize, usize), upper_right: (usize, usize)) -> PyResult<Self> {
        let result = CornerPoints::new_from_row_major(lower_left, upper_right);
        match result {
            Ok(inner) => Ok(PyCornerPoints { inner }),
            Err(msg) => Err(PyErr::new::<PyValueError, _>(msg.to_string()))
        }
    }

    #[staticmethod]
    fn new_from_cartesian_where_origin_bottom_left(lower_left: (usize, usize), upper_right: (usize, usize), total_resolution_width_height: (usize, usize)) -> PyResult<Self> {
        let result = CornerPoints::new_from_cartesian(lower_left, upper_right, total_resolution_width_height);
        match result {
            Ok(inner) => Ok(PyCornerPoints { inner }),
            Err(msg) => Err(PyErr::new::<PyValueError, _>(msg.to_string()))
        }
    }

    fn does_fit_in_frame_of_width_height(&self, source_total_resolution: (usize, usize)) -> bool {
        return self.inner.does_fit_in_frame_of_width_height(source_total_resolution);
    }

    fn enclosed_area_width_height(&self) -> (usize, usize) {
        return self.inner.enclosed_area_width_height();
    }

    #[getter]
    fn lower_right_row_major(&self) -> (usize, usize) {
        return self.inner.lower_right_row_major();
    }

    #[getter]
    fn upper_left_row_major(&self) -> (usize, usize) {
        return self.inner.upper_left_row_major();
    }

    #[getter]
    fn lower_left_row_major(&self) -> (usize, usize) {
        return self.inner.lower_left_row_major();
    }

    #[getter]
    fn upper_right_row_major(&self) -> (usize, usize) {
        return self.inner.upper_right_row_major();
    }
}

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

#[pyclass(eq, eq_int)]
#[derive(PartialEq, Clone)]
#[pyo3(name = "ChannelFormat")]
pub enum PyChannelFormat {
    GrayScale,
    RG,
    RGB,
    RGBA
}

impl From<PyChannelFormat> for ChannelFormat {
    fn from(py_channel_format: PyChannelFormat) -> Self {
        match py_channel_format {
            PyChannelFormat::GrayScale => ChannelFormat::GrayScale,
            PyChannelFormat::RG => ChannelFormat::RG,
            PyChannelFormat::RGB => ChannelFormat::RGB,
            PyChannelFormat::RGBA => ChannelFormat::RGBA,
        }
    }
}

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


#[pyclass]
#[derive(Clone)]
#[pyo3(name = "SegmentedVisionFrameSourceCroppingPointGrouping")]
pub struct PySegmentedVisionFrameSourceCroppingPointGrouping {
    pub inner: SegmentedVisionFrameSourceCroppingPointGrouping,
}

#[pymethods]
impl PySegmentedVisionFrameSourceCroppingPointGrouping {
    #[getter]
    fn lower_left(&self) -> PyCornerPoints {
        PyCornerPoints { inner: self.inner.lower_left }
    }

    #[getter]
    fn middle_left(&self) -> PyCornerPoints {
        PyCornerPoints { inner: self.inner.middle_left }
    }

    #[getter]
    fn upper_left(&self) -> PyCornerPoints {
        PyCornerPoints { inner: self.inner.upper_left }
    }

    #[getter]
    fn upper_middle(&self) -> PyCornerPoints {
        PyCornerPoints { inner: self.inner.upper_middle }
    }

    #[getter]
    fn upper_right(&self) -> PyCornerPoints {
        PyCornerPoints { inner: self.inner.upper_right }
    }

    #[getter]
    fn middle_right(&self) -> PyCornerPoints {
        PyCornerPoints { inner: self.inner.middle_right }
    }

    #[getter]
    fn lower_right(&self) -> PyCornerPoints {
        PyCornerPoints { inner: self.inner.lower_right }
    }

    #[getter]
    fn lower_middle(&self) -> PyCornerPoints {
        PyCornerPoints { inner: self.inner.lower_middle }
    }

    #[getter]
    fn center(&self) -> PyCornerPoints {
        PyCornerPoints { inner: self.inner.center }
    }
}

#[pyclass]
#[derive(Clone)]
#[pyo3(name = "SegmentedVisionCenterProperties")]
pub struct PySegmentedVisionCenterProperties{
    pub inner: SegmentedVisionCenterProperties,
}

#[pymethods]
impl PySegmentedVisionCenterProperties {
    #[new]
    fn new_row_major_where_origin_top_left(center_coordinates_normalized_yx: (f32, f32), center_size_normalized_yx: (f32, f32)) -> PyResult<Self> {
        let result = SegmentedVisionCenterProperties::new_row_major_where_origin_top_left(center_coordinates_normalized_yx, center_size_normalized_yx);
        match result {
            Ok(inner) => Ok(PySegmentedVisionCenterProperties { inner }),
            Err(msg) => Err(PyErr::new::<PyValueError, _>(msg.to_string())),
        }
    }

    #[staticmethod]
    fn cartesian_where_origin_bottom_left(center_coordinates_normalized_cartesian_xy: (f32, f32), center_size_normalized_xy: (f32, f32)) -> PyResult<Self> {
        let result = SegmentedVisionCenterProperties::cartesian_where_origin_bottom_left(center_coordinates_normalized_cartesian_xy, center_size_normalized_xy);
        match result {
            Ok(inner) => Ok(PySegmentedVisionCenterProperties { inner }),
            Err(msg) => Err(PyErr::new::<PyValueError, _>(msg.to_string())),
        }
    }

    #[staticmethod]
    fn create_default_centered() -> Self {
        PySegmentedVisionCenterProperties {
            inner: SegmentedVisionCenterProperties::create_default_centered(),
        }
    }

    fn calculate_source_corner_points_for_segemented_video_frame(&self, source_frame_width_height: (usize, usize)) -> PyResult<PySegmentedVisionFrameSourceCroppingPointGrouping> {
        match self.inner.calculate_source_corner_points_for_segemented_video_frame(source_frame_width_height) {
            Ok(grouping) => Ok(PySegmentedVisionFrameSourceCroppingPointGrouping { inner: grouping }),
            Err(err) => Err(PyErr::new::<PyValueError, _>(err.to_string())),
        }
    }
}

#[pyclass]
#[derive(Clone)]
#[pyo3(name = "SegmentedVisionTargetResolutions")]
pub struct PySegmentedVisionTargetResolutions{
    pub inner: SegmentedVisionTargetResolutions,
}

#[pymethods]
impl PySegmentedVisionTargetResolutions {
    #[new]
    pub fn new(        lower_left: (usize, usize),
                       middle_left: (usize, usize),
                       upper_left: (usize, usize),
                       upper_middle: (usize, usize),
                       upper_right: (usize, usize),
                       middle_right: (usize, usize),
                       lower_right: (usize, usize),
                       lower_middle: (usize, usize),
                       center: (usize, usize)
    ) -> PyResult<Self> {
        let result = SegmentedVisionTargetResolutions::new(lower_left, middle_left, upper_left, upper_middle, upper_right, middle_right, lower_right, lower_middle, center);
        match result {
            Ok(inner) => Ok(PySegmentedVisionTargetResolutions { inner }),
            Err(msg) => Err(PyErr::new::<PyValueError, _>(msg.to_string()))
        }
    }

    #[staticmethod]
    fn create_with_same_sized_peripheral(center_width_height: (usize, usize), peripheral_width_height: (usize, usize)) -> PyResult<Self> {
        let result = SegmentedVisionTargetResolutions::create_with_same_sized_peripheral(center_width_height, peripheral_width_height);
        match result {
            Ok(inner) => Ok(PySegmentedVisionTargetResolutions { inner }),
            Err(msg) => Err(PyErr::new::<PyValueError, _>(msg.to_string()))
        }
    }

    #[getter]
    fn lower_left(&self) -> (usize, usize) {
        self.inner.lower_left
    }

    #[getter]
    fn middle_left(&self) -> (usize, usize) {
        self.inner.middle_left
    }

    #[getter]
    fn upper_left(&self) -> (usize, usize) {
        self.inner.upper_left
    }

    #[getter]
    fn upper_middle(&self) -> (usize, usize) {
        self.inner.upper_middle
    }

    #[getter]
    fn upper_right(&self) -> (usize, usize) {
        self.inner.upper_right
    }

    #[getter]
    fn middle_right(&self) -> (usize, usize) {
        self.inner.middle_right
    }

    #[getter]
    fn lower_right(&self) -> (usize, usize) {
        self.inner.lower_right
    }

    #[getter]
    fn lower_middle(&self) -> (usize, usize) {
        self.inner.lower_middle
    }

    #[getter]
    fn center(&self) -> (usize, usize) {
        self.inner.center
    }
}