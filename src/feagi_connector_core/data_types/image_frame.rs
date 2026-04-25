use crate::feagi_connector_core::data_types::descriptors::*;
use crate::{__base_py_class_shared, create_pyclass};
use feagi_sensorimotor::data_types::ImageFrame;
use numpy::{PyArray3, PyReadonlyArray3};
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::types::PyBytes;

create_pyclass!(PyImageFrame, ImageFrame, "ImageFrame");

#[pymethods]
impl PyImageFrame {
    #[staticmethod]
    #[allow(non_snake_case)]
    pub fn INTERNAL_MEMORY_LAYOUT() -> PyMemoryOrderLayout {
        ImageFrame::INTERNAL_MEMORY_LAYOUT.into()
    }

    //region Common Constructors
    #[new]
    pub fn new(
        channel_format: PyColorChannelLayout,
        color_space: PyColorSpace,
        xy_resolution: PyImageXYResolution,
    ) -> PyResult<Self> {
        let result = ImageFrame::new(
            &channel_format.into(),
            &color_space.into(),
            &xy_resolution.into(),
        );
        match result {
            Ok(image_frame) => Ok(PyImageFrame { inner: image_frame }),
            Err(err) => Err(PyErr::new::<PyValueError, _>(format!("{}", err))),
        }
    }

    #[staticmethod]
    pub fn new_from_image_frame_properties(
        image_frame_properties: PyImageFrameProperties,
    ) -> PyResult<PyImageFrame> {
        let result = ImageFrame::new_from_image_frame_properties(&image_frame_properties.into());
        match result {
            Ok(inner) => Ok(PyImageFrame { inner }),
            Err(err) => Err(PyErr::new::<PyValueError, _>(err.to_string())),
        }
    }

    #[staticmethod]
    pub fn new_from_array(
        input: PyReadonlyArray3<u8>,
        color_space: PyColorSpace,
        source_memory_order: PyMemoryOrderLayout,
        _py: Python,
    ) -> PyResult<PyImageFrame> {
        let array = input.as_array().to_owned();
        match ImageFrame::from_array(array, &color_space.into(), &source_memory_order.into()) {
            Ok(inner) => Ok(PyImageFrame { inner }),
            Err(err) => Err(PyErr::new::<PyValueError, _>(err.to_string())),
        }
    }

    #[staticmethod]
    pub fn new_from_png_bytes<'py>(
        _py: Python<'py>,
        bytes: Bound<'py, PyBytes>,
        color_space: PyColorSpace,
    ) -> PyResult<Self> {
        let bytes_vec = bytes.as_bytes().to_vec();
        let result = ImageFrame::new_from_png_bytes(&bytes_vec, &color_space.into());
        match result {
            Ok(image_frame) => Ok(PyImageFrame { inner: image_frame }),
            Err(err) => Err(PyErr::new::<PyValueError, _>(err.to_string())),
        }
    }

    #[staticmethod]
    pub fn new_from_bmp_bytes<'py>(
        _py: Python<'py>,
        bytes: Bound<'py, PyBytes>,
        color_space: PyColorSpace,
    ) -> PyResult<Self> {
        let bytes_vec = bytes.as_bytes().to_vec();
        let result = ImageFrame::new_from_bmp_bytes(&bytes_vec, &color_space.into());
        match result {
            Ok(image_frame) => Ok(PyImageFrame { inner: image_frame }),
            Err(err) => Err(PyErr::new::<PyValueError, _>(err.to_string())),
        }
    }

    #[staticmethod]
    pub fn new_from_jpeg_bytes<'py>(
        _py: Python<'py>,
        bytes: Bound<'py, PyBytes>,
        color_space: PyColorSpace,
    ) -> PyResult<Self> {
        let bytes_vec = bytes.as_bytes().to_vec();
        let result = ImageFrame::new_from_jpeg_bytes(&bytes_vec, &color_space.into());
        match result {
            Ok(image_frame) => Ok(PyImageFrame { inner: image_frame }),
            Err(err) => Err(PyErr::new::<PyValueError, _>(err.to_string())),
        }
    }

    #[staticmethod]
    pub fn new_from_tiff_bytes<'py>(
        _py: Python<'py>,
        bytes: Bound<'py, PyBytes>,
        color_space: PyColorSpace,
    ) -> PyResult<Self> {
        let bytes_vec = bytes.as_bytes().to_vec();
        let result = ImageFrame::new_from_tiff_bytes(&bytes_vec, &color_space.into());
        match result {
            Ok(image_frame) => Ok(PyImageFrame { inner: image_frame }),
            Err(err) => Err(PyErr::new::<PyValueError, _>(err.to_string())),
        }
    }

    //endregion

    //region Properties

    pub fn get_image_frame_properties(&self) -> PyImageFrameProperties {
        self.inner.get_image_frame_properties().into()
    }

    #[getter]
    pub fn channel_layout(&self) -> PyColorChannelLayout {
        self.inner.get_channel_layout().clone().into()
    }

    #[getter]
    pub fn color_space(&self) -> PyColorSpace {
        self.inner.get_color_space().clone().into()
    }

    #[getter]
    pub fn color_channel_count(&self) -> usize {
        self.inner.get_color_channel_count()
    }

    // NOTE: get_pixels_view, get_pixels_view_mut skipped, equivalent is copy_to_numpy_array

    #[getter]
    pub fn get_xy_resolution(&self) -> PyImageXYResolution {
        self.inner.get_xy_resolution().into()
    }

    #[getter]
    pub fn get_number_elements(&self) -> usize {
        self.inner.get_number_elements()
    }

    #[getter]
    pub fn get_dimensions(&self) -> PyImageXYZDimensions {
        self.inner.get_dimensions().into()
    }

    pub fn copy_to_numpy_array<'py>(&self, py: Python) -> PyResult<Py<PyArray3<u8>>> {
        Ok(Py::from(PyArray3::from_array(
            py,
            &self.inner.get_pixels_view(),
        )))
    }

    #[getter]
    pub fn skip_encoding(&self) -> bool {
        // Since we cannot expose the inner public property, we do this
        self.inner.skip_encoding
    }

    //endregion

    //region Export as Image

    pub fn export_as_png_bytes<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyBytes>> {
        match self.inner.export_as_png_bytes() {
            Ok(bytes) => Ok(PyBytes::new(py, &bytes)),
            Err(err) => Err(PyErr::new::<PyValueError, _>(err.to_string())),
        }
    }

    pub fn export_as_bmp_bytes<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyBytes>> {
        match self.inner.export_as_bmp_bytes() {
            Ok(bytes) => Ok(PyBytes::new(py, &bytes)),
            Err(err) => Err(PyErr::new::<PyValueError, _>(err.to_string())),
        }
    }

    pub fn export_as_jpeg_bytes<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyBytes>> {
        match self.inner.export_as_jpeg_bytes() {
            Ok(bytes) => Ok(PyBytes::new(py, &bytes)),
            Err(err) => Err(PyErr::new::<PyValueError, _>(err.to_string())),
        }
    }

    pub fn export_as_tiff_bytes<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyBytes>> {
        match self.inner.export_as_tiff_bytes() {
            Ok(bytes) => Ok(PyBytes::new(py, &bytes)),
            Err(err) => Err(PyErr::new::<PyValueError, _>(err.to_string())),
        }
    }

    //endregion

    //region Image Processing

    //region In-Place

    pub fn change_brightness(&mut self, brightness_factor: i32) -> PyResult<()> {
        self.inner.change_brightness(brightness_factor);
        Ok(())
    }

    pub fn change_contrast(&mut self, contrast_factor: f32) -> PyResult<()> {
        self.inner.change_contrast(contrast_factor);
        Ok(())
    }

    pub fn blink_image(&mut self) -> PyResult<()> {
        self.inner.blink_image();
        Ok(())
    }

    //endregion

    //endregion

    //region Neuron Export

    // NOTE: write_thresholded_xyzp_neuron_arrays is not exposed as it makes no sense for python

    //endregion
}
