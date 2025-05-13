use pyo3::prelude::*;
use pyo3::exceptions::PyValueError;
use feagi_core_data_structures_and_processing::brain_input::vision::cropping_utils::CornerPoints;

#[pyclass]
#[pyo3(name = "CornerPoints")]
pub struct PyCornerPoints {
    pub inner: CornerPoints,
}

#[pymethods]
impl PyCornerPoints {
    #[new]
    fn new(lower_left: (usize, usize), upper_right: (usize, usize)) -> PyResult<Self> {
        let result = CornerPoints::new(lower_left, upper_right);
        match result {
            Ok(inner) => Ok(PyCornerPoints{ inner }),
            Err(msg) => Err(PyErr::new::<PyValueError, _>(msg))
        }

    }

    fn does_fit_in_frame_of_resolution(&self, source_total_resolution: (usize, usize)) -> bool {
        return self.inner.does_fit_in_frame_of_resolution(source_total_resolution);
    }

    fn enclosed_area(&self) -> (usize, usize) {
        return self.inner.enclosed_area();
    }

    fn lower_right(&self) -> (usize, usize) {
        return self.inner.lower_right();
    }

    fn upper_left(&self) -> (usize, usize) {
        return self.inner.upper_left();
    }

    #[getter]
    fn lower_left(&self) -> (usize, usize) {
        return self.inner.lower_left;
    }

    #[getter]
    fn upper_right(&self) -> (usize, usize) {
        return self.inner.upper_right;
    }
}