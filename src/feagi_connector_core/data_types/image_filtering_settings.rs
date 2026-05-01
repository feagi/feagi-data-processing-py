use pyo3::prelude::*;
use pyo3::{pymethods, PyResult};

use feagi_sensorimotor::data_types::ImageFilteringSettings;

use crate::feagi_connector_core::data_types::{PyPercentage, PyPercentage2D};
use crate::{__base_py_class_shared, create_pyclass};

create_pyclass!(
    PyImageFilteringSettings,
    ImageFilteringSettings,
    "ImageFilteringSettings"
);

#[pymethods]
impl PyImageFilteringSettings {
    #[new]
    fn new(
        brightness: PyPercentage,
        contrast: PyPercentage,
        per_pixel_diff_threshold: PyPercentage2D,
        image_diff_threshold: PyPercentage2D,
    ) -> PyResult<Self> {
        Ok(PyImageFilteringSettings::new_from_rust(
            ImageFilteringSettings::new(
                brightness.into(),
                contrast.into(),
                per_pixel_diff_threshold.into(),
                image_diff_threshold.into(),
            ),
        ))
    }

    #[staticmethod]
    fn default() -> Self {
        PyImageFilteringSettings {
            inner: ImageFilteringSettings::default(),
        }
    }
}
