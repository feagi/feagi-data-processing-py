use pyo3::{pyclass, pymethods, PyErr, PyResult};
use pyo3::exceptions::PyValueError;
use feagi_core_data_structures_and_processing::brain_input::vision::single_frame_processing::*;

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
        let result = CornerPoints::new_from_row_major_where_origin_top_left(lower_left, upper_right);
        match result {
            Ok(inner) => Ok(PyCornerPoints{ inner }),
            Err(msg) => Err(PyErr::new::<PyValueError, _>(msg.to_string()))
        }

    }
    
    #[staticmethod]
    fn new_from_cartesian_where_origin_bottom_left(lower_left: (usize, usize), upper_right: (usize, usize), total_resolution_width_height: (usize, usize)) -> PyResult<Self> {
        let result = CornerPoints::new_from_cartesian_where_origin_bottom_left(lower_left, upper_right, total_resolution_width_height);
        match result {
            Ok(inner) => Ok(PyCornerPoints{ inner }),
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