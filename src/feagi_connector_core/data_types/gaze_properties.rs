use crate::feagi_connector_core::data_types::{PyPercentage, PyPercentage2D};
use crate::{__base_py_class_shared, create_pyclass};
use feagi_sensorimotor::data_types::GazeProperties;
use pyo3::prelude::*;
use pyo3::{pymethods, PyResult};

create_pyclass!(PyGazeProperties, GazeProperties, "GazeProperties");

#[pymethods]
impl PyGazeProperties {
    #[new]
    fn new(
        eccentricity_center_xy: PyPercentage2D,
        modularity_size: PyPercentage,
    ) -> PyResult<Self> {
        Ok(PyGazeProperties::new_from_rust(GazeProperties::new(
            eccentricity_center_xy.into(),
            modularity_size.into(),
        )))
    }

    #[staticmethod]
    fn create_default_centered() -> Self {
        PyGazeProperties {
            inner: GazeProperties::create_default_centered(),
        }
    }
}
