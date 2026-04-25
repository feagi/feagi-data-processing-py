use crate::feagi_connector_core::data_types::descriptors::PyImageFrameProperties;
use feagi_sensorimotor::data_types::processing::ImageFrameProcessor;
use pyo3::exceptions::PyValueError;
use pyo3::{pyclass, pymethods, PyResult};

#[pyclass(name = "ImageFrameProcessor")]
#[derive(Clone)]
pub struct PyImageFrameProcessor {
    inner: ImageFrameProcessor,
}

#[pymethods]
impl PyImageFrameProcessor {
    // TODO other methods

    #[new]
    pub fn new(input_image_properties: PyImageFrameProperties) -> PyResult<Self> {
        Ok(ImageFrameProcessor::new(input_image_properties.into()).into())
    }

    #[staticmethod]
    pub fn new_from_input_output_properties(
        input: PyImageFrameProperties,
        output: PyImageFrameProperties,
    ) -> PyResult<Self> {
        let result =
            ImageFrameProcessor::new_from_input_output_properties(&input.into(), &output.into());
        match result {
            Ok(inner) => Ok(PyImageFrameProcessor { inner }),
            Err(e) => Err(PyValueError::new_err(e.to_string())),
        }
    }

    //region Set Settings

    pub fn set_brightness_offset(&mut self, brightness_offset: i32) -> PyResult<()> {
        self.inner
            .set_brightness_offset(brightness_offset)
            .map_err(|e| PyValueError::new_err(e.to_string()))?;
        Ok(())
    }

    pub fn set_contrast_change(&mut self, contrast_change: f32) -> PyResult<()> {
        self.inner
            .set_contrast_change(contrast_change)
            .map_err(|e| PyValueError::new_err(e.to_string()))?;
        Ok(())
    }

    //endregion
}

impl From<ImageFrameProcessor> for PyImageFrameProcessor {
    fn from(inner: ImageFrameProcessor) -> Self {
        PyImageFrameProcessor { inner }
    }
}

impl From<PyImageFrameProcessor> for ImageFrameProcessor {
    fn from(val: PyImageFrameProcessor) -> Self {
        val.inner
    }
}
