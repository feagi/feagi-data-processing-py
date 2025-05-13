use pyo3::prelude::*;
use pyo3::exceptions::PyValueError;
use super::cropping_utils::*;
use feagi_core_data_structures_and_processing::brain_input::vision::peripheral_segmentation::{SegmentedVisionCenterProperties};

#[pyclass]
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