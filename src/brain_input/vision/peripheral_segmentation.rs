use pyo3::prelude::*;
use pyo3::exceptions::PyValueError;
use super::cropping_utils::*;
use feagi_core_data_structures_and_processing::brain_input::vision::peripheral_segmentation::{SegmentedVisionCenterProperties, SegmentedVisionFrame, SegmentedVisionTargetResolutions};
use pyo3::types::{PyBytes};

use crate::brain_input::vision::single_frame::PyImageFrame;

#[pyclass]
#[derive(Clone)]
#[pyo3(name = "SegmentedVisionCenterProperties")]
pub struct PySegmentedVisionCenterProperties{
    pub inner: SegmentedVisionCenterProperties,
}

#[pymethods]
impl PySegmentedVisionCenterProperties{
    #[new]
    pub fn new(center_coordinates_normalized: (f32, f32), center_size_normalized: (f32, f32)) -> PyResult<Self> {
        let result = SegmentedVisionCenterProperties::new(center_coordinates_normalized, center_size_normalized);
        match result {
            Ok(inner) => Ok(PySegmentedVisionCenterProperties{inner}),
            Err(msg) => Err(PyErr::new::<PyValueError, _>(msg))
        }
    }

    pub fn calculate_pixel_coordinates_of_center_corners(&self, source_frame_resolution: (usize, usize)) -> PyResult<PyCornerPoints>{
        let result = self.inner.calculate_pixel_coordinates_of_center_corners(source_frame_resolution);
        match result {
            Ok(corner_points) => Ok(PyCornerPoints{inner: corner_points}),
            Err(msg) => Err(PyErr::new::<PyValueError, _>(msg))
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
impl PySegmentedVisionTargetResolutions{
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
            Err(msg) => Err(PyErr::new::<PyValueError, _>(msg))
        }
    }
}

#[pyclass]
#[pyo3(name = "SegmentedVisionFrame")]
pub struct PySegmentedVisionFrame{
    inner: SegmentedVisionFrame,
}

#[pymethods]
impl PySegmentedVisionFrame {
    #[new]
    pub fn new(source_frame: PyImageFrame, center_properties: PySegmentedVisionCenterProperties, segment_resolutions: PySegmentedVisionTargetResolutions) -> PyResult<Self> {
        let result = SegmentedVisionFrame::new(&source_frame.inner, &center_properties.inner, segment_resolutions.inner);
        match result {
            Ok(inner) => Ok(PySegmentedVisionFrame {inner}),
            Err(msg) => Err(PyErr::new::<PyValueError, _>(msg))
        }
    }
    
    pub fn update_in_place(&mut self, source_frame: PyImageFrame) -> PyResult<()> {
        let result = self.inner.update_in_place(&source_frame.inner);
        match result {
            Ok(()) => Ok(()),
            Err(msg) => Err(PyErr::new::<PyValueError, _>(msg))
        }
    }
    
    pub fn direct_export_as_byte_neuron_potential_categorical_xyz<'py>(&self, py: Python<'py>, camera_index: u8) -> PyResult<Bound<'py, PyBytes>> {
        let result = self.inner.direct_export_as_byte_neuron_potential_categorical_xyz(camera_index);
        match result {
            Ok(byte_array) => Ok(PyBytes::new(py, &byte_array)),
            Err(msg) => Err(PyErr::new::<PyValueError, _>(msg))
        }
    }
}