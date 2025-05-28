use numpy::{PyArray3, PyReadonlyArray3};
use pyo3::prelude::*;
use pyo3::exceptions::PyValueError;
use ndarray::Array3;
use feagi_core_data_structures_and_processing::brain_input::vision::image_frame::ImageFrame;
use feagi_core_data_structures_and_processing::brain_input::vision::descriptors::*;
use feagi_core_data_structures_and_processing::neuron_data::NeuronXYCPArrays;
use crate::brain_input::vision::descriptors::*;
use crate::neuron_data::PyNeuronXYCPArrays;

#[pyclass]
#[pyo3(name = "ImageFrame")]
#[derive(Clone)]
pub struct PyImageFrame {
    pub inner: ImageFrame,
}

#[pymethods]
impl PyImageFrame {
    #[new]
    pub fn new(channel_format: PyChannelFormat, color_space: PyColorSpace, xy_resolution: (usize, usize)) -> PyImageFrame {
        PyImageFrame {
            inner: ImageFrame::new(&channel_format.into(), &color_space.into(), &xy_resolution)
        }
    }

    #[staticmethod]
    pub fn from_array(input: PyReadonlyArray3<f32>, color_space: PyColorSpace, source_memory_order: PyMemoryOrderLayout, py: Python) -> PyResult<PyImageFrame> {
        let array = input.as_array().to_owned();
        match ImageFrame::from_array(array, color_space.into(), source_memory_order.into()) {
            Ok(inner) => Ok(PyImageFrame { inner }),
            Err(err) => Err(PyErr::new::<PyValueError, _>(err.to_string())),
        }
    }

    #[staticmethod]
    pub fn from_array_with_processing(
        source_color_space: PyColorSpace,
        image_processing: &PyFrameProcessingParameters,
        input: PyReadonlyArray3<f32>,
    ) -> PyResult<PyImageFrame> {
        let array = input.as_array().to_owned();
        match ImageFrame::from_array_with_processing(source_color_space.into(), image_processing.inner, array) {
            Ok(inner) => Ok(PyImageFrame { inner }),
            Err(err) => Err(PyErr::new::<PyValueError, _>(err.to_string())),
        }
    }

    #[staticmethod]
    pub fn do_resolutions_channel_depth_and_color_spaces_match(a: &PyImageFrame, b: &PyImageFrame) -> bool {
        ImageFrame::do_resolutions_channel_depth_and_color_spaces_match(&a.inner, &b.inner)
    }

    #[staticmethod]
    pub fn is_array_valid_for_image_frame(array: PyReadonlyArray3<f32>, py: Python) -> bool {
        let array_view = array.as_array().to_owned();
        ImageFrame::is_array_valid_for_image_frame(&array_view.to_owned())
    }

    #[getter]
    pub fn channel_format(&self) -> PyChannelFormat {
        match self.inner.get_channel_format() {
            ChannelFormat::GrayScale => PyChannelFormat::GrayScale,
            ChannelFormat::RG => PyChannelFormat::RG,
            ChannelFormat::RGB => PyChannelFormat::RGB,
            ChannelFormat::RGBA => PyChannelFormat::RGBA,
        }
    }

    #[getter]
    pub fn color_space(&self) -> PyColorSpace {
        match self.inner.get_color_space() {
            ColorSpace::Linear => PyColorSpace::Linear,
            ColorSpace::Gamma => PyColorSpace::Gamma,
        }
    }

    pub fn get_color_channel_count(&self) -> usize {
        self.inner.get_color_channel_count()
    }

    pub fn get_cartesian_width_height(&self) -> (usize, usize) {
        self.inner.get_cartesian_width_height()
    }

    pub fn get_internal_resolution(&self) -> (usize, usize) {
        self.inner.get_internal_resolution()
    }

    pub fn get_internal_shape(&self) -> (usize, usize, usize) {
        self.inner.get_internal_shape()
    }

    pub fn get_max_capacity_neuron_count(&self) -> usize {
        self.inner.get_max_capacity_neuron_count()
    }

    pub fn change_brightness_multiplicative(&mut self, brightness_factor: f32) -> PyResult<()> {
        match self.inner.change_brightness_multiplicative(brightness_factor) {
            Ok(_) => Ok(()),
            Err(err) => Err(PyErr::new::<PyValueError, _>(err.to_string())),
        }
    }

    pub fn change_contrast(&mut self, contrast_factor: f32) -> PyResult<()> {
        match self.inner.change_contrast(contrast_factor) {
            Ok(_) => Ok(()),
            Err(err) => Err(PyErr::new::<PyValueError, _>(err.to_string())),
        }
    }

    pub fn crop_to(&mut self, corners_crop: &PyCornerPoints) -> PyResult<()> {
        match self.inner.crop_to(&corners_crop.inner) {
            Ok(_) => Ok(()),
            Err(err) => Err(PyErr::new::<PyValueError, _>(err.to_string())),
        }
    }

    pub fn resize_nearest_neighbor(&mut self, target_width_height: (usize, usize)) -> PyResult<()> {
        match self.inner.resize_nearest_neighbor(&target_width_height) {
            Ok(_) => Ok(()),
            Err(err) => Err(PyErr::new::<PyValueError, _>(err.to_string())),
        }
    }

    pub fn in_place_run_processor(&mut self, image_processing: &PyFrameProcessingParameters, source: PyImageFrame) -> PyResult<()> {
        match self.inner.in_place_run_processor(image_processing.inner, source.inner) {
            Ok(_) => Ok(()),
            Err(err) => Err(PyErr::new::<PyValueError, _>(err.to_string())),
        }
    }

    pub fn in_place_load_data_unchanged(&mut self, new_array: PyReadonlyArray3<f32>, source_memory_order: PyMemoryOrderLayout, py: Python) -> PyResult<()> {
        let array = new_array.as_array().to_owned();
        match self.inner.in_place_load_data_unchanged(array, source_memory_order.into()) {
            Ok(_) => Ok(()),
            Err(err) => Err(PyErr::new::<PyValueError, _>(err.to_string())),
        }
    }

    pub fn in_place_crop_image(&mut self, source_cropping_points: &PyCornerPoints, source: &PyImageFrame) -> PyResult<()> {
        match self.inner.in_place_crop_image(&source_cropping_points.inner, &source.inner) {
            Ok(_) => Ok(()),
            Err(err) => Err(PyErr::new::<PyValueError, _>(err.to_string())),
        }
    }

    pub fn in_place_nearest_neighbor_resize(&mut self, source: &PyImageFrame) -> PyResult<()> {
        match self.inner.in_place_nearest_neighbor_resize(&source.inner) {
            Ok(_) => Ok(()),
            Err(err) => Err(PyErr::new::<PyValueError, _>(err.to_string())),
        }
    }

    pub fn in_place_crop_and_nearest_neighbor_resize(&mut self, source_cropping_points: &PyCornerPoints, source: &PyImageFrame) -> PyResult<()> {
        match self.inner.in_place_crop_and_nearest_neighbor_resize(&source_cropping_points.inner, &source.inner) {
            Ok(_) => Ok(()),
            Err(err) => Err(PyErr::new::<PyValueError, _>(err.to_string())),
        }
    }

    pub fn in_place_calculate_difference_thresholded(&mut self, previous_frame: &PyImageFrame, next_frame: &PyImageFrame, threshold: u8) -> PyResult<()> {
        match self.inner.in_place_calculate_difference_thresholded(&previous_frame.inner, &next_frame.inner, threshold) {
            Ok(_) => Ok(()),
            Err(err) => Err(PyErr::new::<PyValueError, _>(err.to_string())),
        }
    }

    pub fn write_thresholded_xyzp_neuron_arrays(&mut self, threshold: f32, write_target: &mut PyNeuronXYCPArrays) -> PyResult<()> {
        match self.inner.write_thresholded_xyzp_neuron_arrays(threshold, &mut write_target.inner) {
            Ok(_) => Ok(()),
            Err(err) => Err(PyErr::new::<PyValueError, _>(err.to_string())),
        }
    }

    #[staticmethod]
    pub fn create_from_source_frame_crop_and_resize(
        source_frame: &PyImageFrame,
        corners_crop: &PyCornerPoints,
        new_width_height: (usize, usize)
    ) -> PyResult<PyImageFrame> {
        match ImageFrame::create_from_source_frame_crop_and_resize(&source_frame.inner, &corners_crop.inner, &new_width_height) {
            Ok(inner) => Ok(PyImageFrame { inner }),
            Err(err) => Err(PyErr::new::<PyValueError, _>(err.to_string())),
        }
    }

    pub fn copy_to_numpy_array(&self, py: Python) -> PyResult<Py<PyArray3<f32>>> {
        Ok(Py::from(PyArray3::from_array(py, &self.inner.get_pixels_view())))
    }
}
