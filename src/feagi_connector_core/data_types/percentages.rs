use crate::{__base_py_class_shared, create_pyclass};
use feagi_sensorimotor::data_types::Percentage;
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::{pymethods, PyResult};

// TODO port all remaining methods

//region Percentage (0 - 1)

create_pyclass!(PyPercentage, Percentage, "Percentage");

#[pymethods]
impl PyPercentage {
    // Note: Skip any unchecked methods

    //region Constructors

    #[staticmethod]
    pub fn new_from_0_1(value: f32) -> PyResult<PyPercentage> {
        Ok(PyPercentage {
            inner: Percentage::new_from_0_1(value)
                .map_err(|err| PyValueError::new_err(err.to_string()))?,
        })
    }

    #[staticmethod]
    pub fn new_from_interp_m1_1(value: f32) -> PyResult<PyPercentage> {
        Ok(PyPercentage {
            inner: Percentage::new_from_interp_m1_1(value)
                .map_err(|err| PyValueError::new_err(err.to_string()))?,
        })
    }

    #[staticmethod]
    pub fn new_from_u8_0_255(value: u8) -> PyResult<PyPercentage> {
        Ok(PyPercentage {
            inner: Percentage::new_from_u8_0_255(value)
                .map_err(|err| PyValueError::new_err(err.to_string()))?,
        })
    }

    #[staticmethod]
    pub fn new_from_0_100(value: f32) -> PyResult<PyPercentage> {
        Ok(PyPercentage {
            inner: Percentage::new_from_0_100(value)
                .map_err(|err| PyValueError::new_err(err.to_string()))?,
        })
    }

    #[staticmethod]
    pub fn new_from_linear_interp(
        value: f32,
        lower_range: f32,
        upper_range: f32,
    ) -> PyResult<PyPercentage> {
        if lower_range >= upper_range {
            return Err(PyValueError::new_err(
                "Lower range cannot be greater than upper range!",
            ));
        }

        Ok(PyPercentage {
            inner: Percentage::new_from_linear_interp(value, &(lower_range..upper_range))
                .map_err(|err| PyValueError::new_err(err.to_string()))?,
        })
    }

    //endregion

    //region Update

    pub fn inplace_update_from_0_1(&mut self, value: f32) -> PyResult<()> {
        self.inner
            .inplace_update_from_0_1(value)
            .map_err(|err| PyValueError::new_err(err.to_string()))?;
        Ok(())
    }

    pub fn inplace_update_u8_0_255(&mut self, value: u8) -> PyResult<()> {
        self.inner
            .inplace_update_u8_0_255(value)
            .map_err(|err| PyValueError::new_err(err.to_string()))?;
        Ok(())
    }

    pub fn inplace_update_0_100(&mut self, value: f32) -> PyResult<()> {
        self.inner
            .inplace_update_0_100(value)
            .map_err(|err| PyValueError::new_err(err.to_string()))?;
        Ok(())
    }

    pub fn inplace_update_linear_interp(
        &mut self,
        value: f32,
        lower_range: f32,
        upper_range: f32,
    ) -> PyResult<()> {
        if lower_range >= upper_range {
            return Err(PyValueError::new_err(
                "Lower range cannot be greater than upper range!",
            ));
        }
        self.inner
            .inplace_update_linear_interp(value, &(lower_range..upper_range))
            .map_err(|err| PyValueError::new_err(err.to_string()))?;
        Ok(())
    }

    //endregion

    //region Properties

    pub fn get_as_0_1(&self) -> f32 {
        self.inner.get_as_0_1()
    }

    pub fn get_as_u8(&self) -> u8 {
        self.inner.get_as_u8()
    }

    pub fn get_as_0_100(&self) -> f32 {
        self.inner.get_as_0_100()
    }

    //endregion
}

//endregion

//region SignedPercentage (-1 to 1)

create_pyclass!(
    PySignedPercentage,
    feagi_sensorimotor::data_types::SignedPercentage,
    "SignedPercentage"
);

#[pymethods]
impl PySignedPercentage {
    // Note: Skip any unchecked methods

    //region Constructors

    #[staticmethod]
    pub fn new_from_m1_1(value: f32) -> PyResult<PySignedPercentage> {
        Ok(PySignedPercentage {
            inner: feagi_sensorimotor::data_types::SignedPercentage::new_from_m1_1(value)
                .map_err(|err| PyValueError::new_err(err.to_string()))?,
        })
    }

    #[staticmethod]
    pub fn new_scaled_from_0_1(value: f32) -> PyResult<PySignedPercentage> {
        Ok(PySignedPercentage {
            inner: feagi_sensorimotor::data_types::SignedPercentage::new_scaled_from_0_1(value)
                .map_err(|err| PyValueError::new_err(err.to_string()))?,
        })
    }

    #[staticmethod]
    pub fn new_from_m100_100(value: f32) -> PyResult<PySignedPercentage> {
        Ok(PySignedPercentage {
            inner: feagi_sensorimotor::data_types::SignedPercentage::new_from_m100_100(value)
                .map_err(|err| PyValueError::new_err(err.to_string()))?,
        })
    }

    #[staticmethod]
    pub fn new_from_linear_interp(
        value: f32,
        lower_range: f32,
        upper_range: f32,
    ) -> PyResult<PySignedPercentage> {
        if lower_range >= upper_range {
            return Err(PyValueError::new_err(
                "Lower range cannot be greater than upper range!",
            ));
        }

        Ok(PySignedPercentage {
            inner: feagi_sensorimotor::data_types::SignedPercentage::new_from_linear_interp(
                value,
                &(lower_range..upper_range),
            )
            .map_err(|err| PyValueError::new_err(err.to_string()))?,
        })
    }

    //endregion

    //region Update

    pub fn inplace_update_from_m1_1(&mut self, value: f32) -> PyResult<()> {
        self.inner
            .inplace_update_from_m1_1(value)
            .map_err(|err| PyValueError::new_err(err.to_string()))?;
        Ok(())
    }

    pub fn inplace_update_m100_100(&mut self, value: f32) -> PyResult<()> {
        self.inner
            .inplace_update_m100_100(value)
            .map_err(|err| PyValueError::new_err(err.to_string()))?;
        Ok(())
    }

    pub fn inplace_update_linear_interp(
        &mut self,
        value: f32,
        lower_range: f32,
        upper_range: f32,
    ) -> PyResult<()> {
        if lower_range >= upper_range {
            return Err(PyValueError::new_err(
                "Lower range cannot be greater than upper range!",
            ));
        }
        self.inner
            .inplace_update_linear_interp(value, &(lower_range..upper_range))
            .map_err(|err| PyValueError::new_err(err.to_string()))?;
        Ok(())
    }

    //endregion

    //region Properties

    pub fn get_as_m1_1(&self) -> f32 {
        self.inner.get_as_m1_1()
    }

    pub fn get_as_m100_100(&self) -> f32 {
        self.inner.get_as_m100_100()
    }

    //endregion
}

//endregion

//region 2D Percentage Types

create_pyclass!(
    PyPercentage2D,
    feagi_sensorimotor::data_types::Percentage2D,
    "Percentage2D"
);

#[pymethods]
impl PyPercentage2D {
    #[new]
    pub fn new(a: PyPercentage, b: PyPercentage) -> Self {
        PyPercentage2D {
            inner: feagi_sensorimotor::data_types::Percentage2D::new(a.into(), b.into()),
        }
    }

    #[staticmethod]
    pub fn new_zero() -> Self {
        PyPercentage2D {
            inner: feagi_sensorimotor::data_types::Percentage2D::new_zero(),
        }
    }

    #[staticmethod]
    pub fn new_identical_percentages(percentage: PyPercentage) -> Self {
        PyPercentage2D {
            inner: feagi_sensorimotor::data_types::Percentage2D::new_identical_percentages(
                percentage.into(),
            ),
        }
    }

    #[getter]
    pub fn a(&self) -> PyPercentage {
        self.inner.a.into()
    }

    #[getter]
    pub fn b(&self) -> PyPercentage {
        self.inner.b.into()
    }

    #[setter]
    pub fn set_a(&mut self, value: PyPercentage) {
        self.inner.a = value.into();
    }

    #[setter]
    pub fn set_b(&mut self, value: PyPercentage) {
        self.inner.b = value.into();
    }
}

create_pyclass!(
    PySignedPercentage2D,
    feagi_sensorimotor::data_types::SignedPercentage2D,
    "SignedPercentage2D"
);

#[pymethods]
impl PySignedPercentage2D {
    #[new]
    pub fn new(a: PySignedPercentage, b: PySignedPercentage) -> Self {
        PySignedPercentage2D {
            inner: feagi_sensorimotor::data_types::SignedPercentage2D::new(a.into(), b.into()),
        }
    }

    #[staticmethod]
    pub fn new_zero() -> Self {
        PySignedPercentage2D {
            inner: feagi_sensorimotor::data_types::SignedPercentage2D::new_zero(),
        }
    }

    #[staticmethod]
    pub fn new_identical_percentages(percentage: PySignedPercentage) -> Self {
        PySignedPercentage2D {
            inner: feagi_sensorimotor::data_types::SignedPercentage2D::new_identical_percentages(
                percentage.into(),
            ),
        }
    }

    #[getter]
    pub fn a(&self) -> PySignedPercentage {
        self.inner.a.into()
    }

    #[getter]
    pub fn b(&self) -> PySignedPercentage {
        self.inner.b.into()
    }

    #[setter]
    pub fn set_a(&mut self, value: PySignedPercentage) {
        self.inner.a = value.into();
    }

    #[setter]
    pub fn set_b(&mut self, value: PySignedPercentage) {
        self.inner.b = value.into();
    }
}

//endregion

//region 3D Percentage Types

create_pyclass!(
    PyPercentage3D,
    feagi_sensorimotor::data_types::Percentage3D,
    "Percentage3D"
);

#[pymethods]
impl PyPercentage3D {
    #[new]
    pub fn new(a: PyPercentage, b: PyPercentage, c: PyPercentage) -> Self {
        PyPercentage3D {
            inner: feagi_sensorimotor::data_types::Percentage3D::new(a.into(), b.into(), c.into()),
        }
    }

    #[staticmethod]
    pub fn new_zero() -> Self {
        PyPercentage3D {
            inner: feagi_sensorimotor::data_types::Percentage3D::new_zero(),
        }
    }

    #[staticmethod]
    pub fn new_identical_percentages(percentage: PyPercentage) -> Self {
        PyPercentage3D {
            inner: feagi_sensorimotor::data_types::Percentage3D::new_identical_percentages(
                percentage.into(),
            ),
        }
    }

    #[getter]
    pub fn a(&self) -> PyPercentage {
        self.inner.a.into()
    }

    #[getter]
    pub fn b(&self) -> PyPercentage {
        self.inner.b.into()
    }

    #[getter]
    pub fn c(&self) -> PyPercentage {
        self.inner.c.into()
    }

    #[setter]
    pub fn set_a(&mut self, value: PyPercentage) {
        self.inner.a = value.into();
    }

    #[setter]
    pub fn set_b(&mut self, value: PyPercentage) {
        self.inner.b = value.into();
    }

    #[setter]
    pub fn set_c(&mut self, value: PyPercentage) {
        self.inner.c = value.into();
    }
}

create_pyclass!(
    PySignedPercentage3D,
    feagi_sensorimotor::data_types::SignedPercentage3D,
    "SignedPercentage3D"
);

#[pymethods]
impl PySignedPercentage3D {
    #[new]
    pub fn new(a: PySignedPercentage, b: PySignedPercentage, c: PySignedPercentage) -> Self {
        PySignedPercentage3D {
            inner: feagi_sensorimotor::data_types::SignedPercentage3D::new(
                a.into(),
                b.into(),
                c.into(),
            ),
        }
    }

    #[staticmethod]
    pub fn new_zero() -> Self {
        PySignedPercentage3D {
            inner: feagi_sensorimotor::data_types::SignedPercentage3D::new_zero(),
        }
    }

    #[staticmethod]
    pub fn new_identical_percentages(percentage: PySignedPercentage) -> Self {
        PySignedPercentage3D {
            inner: feagi_sensorimotor::data_types::SignedPercentage3D::new_identical_percentages(
                percentage.into(),
            ),
        }
    }

    #[getter]
    pub fn a(&self) -> PySignedPercentage {
        self.inner.a.into()
    }

    #[getter]
    pub fn b(&self) -> PySignedPercentage {
        self.inner.b.into()
    }

    #[getter]
    pub fn c(&self) -> PySignedPercentage {
        self.inner.c.into()
    }

    #[setter]
    pub fn set_a(&mut self, value: PySignedPercentage) {
        self.inner.a = value.into();
    }

    #[setter]
    pub fn set_b(&mut self, value: PySignedPercentage) {
        self.inner.b = value.into();
    }

    #[setter]
    pub fn set_c(&mut self, value: PySignedPercentage) {
        self.inner.c = value.into();
    }
}

//endregion

//region 4D Percentage Types

create_pyclass!(
    PyPercentage4D,
    feagi_sensorimotor::data_types::Percentage4D,
    "Percentage4D"
);

#[pymethods]
impl PyPercentage4D {
    #[new]
    pub fn new(a: PyPercentage, b: PyPercentage, c: PyPercentage, d: PyPercentage) -> Self {
        PyPercentage4D {
            inner: feagi_sensorimotor::data_types::Percentage4D::new(
                a.into(),
                b.into(),
                c.into(),
                d.into(),
            ),
        }
    }

    #[staticmethod]
    pub fn new_zero() -> Self {
        PyPercentage4D {
            inner: feagi_sensorimotor::data_types::Percentage4D::new_zero(),
        }
    }

    #[staticmethod]
    pub fn new_identical_percentages(percentage: PyPercentage) -> Self {
        PyPercentage4D {
            inner: feagi_sensorimotor::data_types::Percentage4D::new_identical_percentages(
                percentage.into(),
            ),
        }
    }

    #[getter]
    pub fn a(&self) -> PyPercentage {
        self.inner.a.into()
    }

    #[getter]
    pub fn b(&self) -> PyPercentage {
        self.inner.b.into()
    }

    #[getter]
    pub fn c(&self) -> PyPercentage {
        self.inner.c.into()
    }

    #[getter]
    pub fn d(&self) -> PyPercentage {
        self.inner.d.into()
    }

    #[setter]
    pub fn set_a(&mut self, value: PyPercentage) {
        self.inner.a = value.into();
    }

    #[setter]
    pub fn set_b(&mut self, value: PyPercentage) {
        self.inner.b = value.into();
    }

    #[setter]
    pub fn set_c(&mut self, value: PyPercentage) {
        self.inner.c = value.into();
    }

    #[setter]
    pub fn set_d(&mut self, value: PyPercentage) {
        self.inner.d = value.into();
    }
}

create_pyclass!(
    PySignedPercentage4D,
    feagi_sensorimotor::data_types::SignedPercentage4D,
    "SignedPercentage4D"
);

#[pymethods]
impl PySignedPercentage4D {
    #[new]
    pub fn new(
        a: PySignedPercentage,
        b: PySignedPercentage,
        c: PySignedPercentage,
        d: PySignedPercentage,
    ) -> Self {
        PySignedPercentage4D {
            inner: feagi_sensorimotor::data_types::SignedPercentage4D::new(
                a.into(),
                b.into(),
                c.into(),
                d.into(),
            ),
        }
    }

    #[staticmethod]
    pub fn new_zero() -> Self {
        PySignedPercentage4D {
            inner: feagi_sensorimotor::data_types::SignedPercentage4D::new_zero(),
        }
    }

    #[staticmethod]
    pub fn new_identical_percentages(percentage: PySignedPercentage) -> Self {
        PySignedPercentage4D {
            inner: feagi_sensorimotor::data_types::SignedPercentage4D::new_identical_percentages(
                percentage.into(),
            ),
        }
    }

    #[getter]
    pub fn a(&self) -> PySignedPercentage {
        self.inner.a.into()
    }

    #[getter]
    pub fn b(&self) -> PySignedPercentage {
        self.inner.b.into()
    }

    #[getter]
    pub fn c(&self) -> PySignedPercentage {
        self.inner.c.into()
    }

    #[getter]
    pub fn d(&self) -> PySignedPercentage {
        self.inner.d.into()
    }

    #[setter]
    pub fn set_a(&mut self, value: PySignedPercentage) {
        self.inner.a = value.into();
    }

    #[setter]
    pub fn set_b(&mut self, value: PySignedPercentage) {
        self.inner.b = value.into();
    }

    #[setter]
    pub fn set_c(&mut self, value: PySignedPercentage) {
        self.inner.c = value.into();
    }

    #[setter]
    pub fn set_d(&mut self, value: PySignedPercentage) {
        self.inner.d = value.into();
    }
}

//endregion
