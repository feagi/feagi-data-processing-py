use crate::{__base_py_class_shared, create_pyclass};
use feagi_sensorimotor::data_types::descriptors::MiscDataDimensions;
use feagi_sensorimotor::data_types::MiscData;
use numpy::{PyArray3, PyReadonlyArray3};
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;

create_pyclass!(PyMiscData, MiscData, "MiscData");

#[pymethods]
impl PyMiscData {
    #[new]
    pub fn new(x: u32, y: u32, z: u32) -> PyResult<PyMiscData> {
        let resolution = MiscDataDimensions::new(x, y, z).unwrap_or_else(|e| panic!("{}", e));
        Ok(PyMiscData {
            inner: MiscData::new(&resolution).unwrap(),
        })
    }

    #[staticmethod]
    pub fn new_from_array(input: PyReadonlyArray3<f32>, _py: Python) -> PyResult<PyMiscData> {
        let array = input.as_array().to_owned();
        match MiscData::new_with_data(array.into()) {
            // TODO is this into a good idea?
            Ok(inner) => Ok(PyMiscData { inner }),
            Err(err) => Err(PyErr::new::<PyValueError, _>(err.to_string())),
        }
    }

    pub fn copy_to_numpy_array<'py>(&self, py: Python) -> PyResult<Py<PyArray3<f32>>> {
        Ok(Py::from(PyArray3::from_array(
            py,
            &self.inner.get_internal_data(),
        )))
    }

    pub fn blank_data(&mut self) -> PyResult<()> {
        self.inner.blank_data();
        Ok(())
    }
}
