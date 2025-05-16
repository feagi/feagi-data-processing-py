use feagi_core_data_structures_and_processing::brain_input::vision::single_frame::{ImageFrame};
use feagi_core_data_structures_and_processing::brain_input::vision::single_frame_processing::*;
use numpy::{PyArray3, PyReadonlyArray3};
use ndarray::{Array3};
use pyo3::prelude::*;
use pyo3::exceptions::PyValueError;
use pyo3::types::{PyBytes};

#[pyclass(eq, eq_int)]
#[derive(PartialEq, Clone)]
#[pyo3(name = "ChannelFormat")]
pub enum PyChannelFormat {
    GrayScale,
    RG,
    RGB,
    RGBA
}

fn py_from_channel_format(channel_format: PyChannelFormat) -> ChannelFormat {
    match channel_format {
        PyChannelFormat::GrayScale => ChannelFormat::GrayScale,
        PyChannelFormat::RG => ChannelFormat::RG,
        PyChannelFormat::RGB => ChannelFormat::RGB,
        PyChannelFormat::RGBA => ChannelFormat::RGBA,
    }
}
#[pyclass]
#[pyo3(name = "ImageFrame")]
#[derive(Clone)]
pub struct PyImageFrame {
    pub inner: ImageFrame,
}

#[pymethods]
impl PyImageFrame {
    #[new]
    pub fn new(channel_format: PyChannelFormat, xy_resolution: (usize, usize)) -> PyImageFrame {
        let channel = py_from_channel_format(channel_format);
        PyImageFrame {
            inner: ImageFrame::new(&channel, &ColorSpace::Gamma, &xy_resolution)
        }
    }

    #[staticmethod]
    pub fn from_array(input: PyReadonlyArray3<f32>) -> PyResult<Self> {
        let array: Array3<f32>  = input.as_array().to_owned();
        let result = ImageFrame::from_array(array, ColorSpace::Gamma, MemoryOrderLayout::HeightsWidthsChannels);
        match result {
            Ok(image_frame) => Ok(PyImageFrame{inner: image_frame}),
            Err(err) => Err(PyValueError::new_err(err.to_string()))
        }
    }
    
    /*
    #[staticmethod]
    pub fn from_source_frame_crop_and_resize(source_frame: PyImageFrame, corner_points: PyCornerPoints, new_resolution: (usize, usize)) -> PyResult<Self> {
        let result = ImageFrame::from_source_frame_crop_and_resize(&source_frame.inner, &corner_points.inner, &new_resolution);
        match result {
            Ok(image_frame) => Ok(PyImageFrame{inner: image_frame}),
            Err(err) => Err(PyValueError::new_err(err.to_string()))
        }
    }

    pub fn in_place_crop_and_nearest_neighbor_resize_to_self(&mut self, source_cropping_points: PyCornerPoints, source: PyImageFrame) -> PyResult<()> {
        let result = self.inner.in_place_crop_and_nearest_neighbor_resize_to_self(&source_cropping_points.inner, &source.inner);
        match result {
            Ok(_) => Ok(()),
            Err(err) => Err(PyValueError::new_err(err.to_string()))
        }
    }

    pub fn get_number_of_bytes_needed_to_hold_xyzp_uncompressed(& self) -> usize {
        return self.inner.get_number_of_bytes_needed_to_hold_xyzp_uncompressed();
    }
    
     */

    pub fn to_bytes<'py>(&self, py: Python<'py>) -> Bound<'py, PyBytes> {
        let result = self.inner.to_bytes();
        PyBytes::new(py, &result)
    }
    
    pub fn copy_to_numpy_array(&self, py: Python) -> PyResult<Py<PyArray3<f32>>> {
        Ok(Py::from(PyArray3::from_array(py, &self.inner.get_pixels_view())))
    }


}
