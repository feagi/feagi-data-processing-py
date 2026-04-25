//! Python binding for the composite [`feagi_sensorimotor::data_types::RawIMU`] sensor reading.
//!
//! `RawIMU` is a single cache value that bundles three independent 3-axis signed
//! percentage readings (accelerometer, gyroscope, magnetometer) for a single
//! IMU channel. It is the Python-facing data carrier for the `sensor_raw_imu_*`
//! family of methods on [`crate::feagi_connector_core::PyConnectorAgent`].
//!
//! Sub-component ordering is contractual and MUST stay in lock-step with the
//! `sensor_cortical_units!` template entry, the encoder
//! ([`feagi_sensorimotor::neuron_voxel_coding::xyzp::encoders::RawIMUNeuronVoxelXYZPEncoder`]),
//! and the canonical index constants in
//! [`feagi_sensorimotor::data_types::raw_imu`]:
//!
//! - sub-area 0 -> accelerometer
//! - sub-area 1 -> gyroscope
//! - sub-area 2 -> magnetometer

use crate::feagi_connector_core::data_types::PySignedPercentage3D;
use crate::{__base_py_class_shared, create_pyclass};
use feagi_sensorimotor::data_types::{RawIMU, SignedPercentage3D};
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::{pymethods, PyResult};

create_pyclass!(PyRawIMU, RawIMU, "RawIMU");

#[pymethods]
impl PyRawIMU {
    /// Construct a `RawIMU` from three pre-built `SignedPercentage3D` readings.
    ///
    /// Order is fixed: `(accelerometer, gyroscope, magnetometer)`.
    #[new]
    pub fn new(
        accelerometer: PySignedPercentage3D,
        gyroscope: PySignedPercentage3D,
        magnetometer: PySignedPercentage3D,
    ) -> Self {
        PyRawIMU {
            inner: RawIMU::new(accelerometer.into(), gyroscope.into(), magnetometer.into()),
        }
    }

    /// Construct a zero-initialised `RawIMU` (all axes for all three sensors == 0).
    #[staticmethod]
    pub fn new_zero() -> Self {
        PyRawIMU {
            inner: RawIMU::new_zero(),
        }
    }

    /// Construct from raw `[x, y, z]` triples in the inclusive range `[-1.0, 1.0]`.
    ///
    /// Values outside the range cause a `ValueError`. Order is fixed:
    /// `(accelerometer, gyroscope, magnetometer)`.
    #[staticmethod]
    pub fn try_from_axis_triples(
        accelerometer_xyz: (f32, f32, f32),
        gyroscope_xyz: (f32, f32, f32),
        magnetometer_xyz: (f32, f32, f32),
    ) -> PyResult<Self> {
        let inner =
            RawIMU::try_from_axis_triples(accelerometer_xyz, gyroscope_xyz, magnetometer_xyz)
                .map_err(|err| PyValueError::new_err(err.to_string()))?;
        Ok(PyRawIMU { inner })
    }

    //region Component getters

    /// Return the current accelerometer 3D reading.
    #[getter]
    pub fn accelerometer(&self) -> PySignedPercentage3D {
        (*self.inner.get_accelerometer()).into()
    }

    /// Return the current gyroscope 3D reading.
    #[getter]
    pub fn gyroscope(&self) -> PySignedPercentage3D {
        (*self.inner.get_gyroscope()).into()
    }

    /// Return the current magnetometer 3D reading.
    #[getter]
    pub fn magnetometer(&self) -> PySignedPercentage3D {
        (*self.inner.get_magnetometer()).into()
    }

    //endregion

    //region Component setters (in-place, single-component)

    /// Replace only the accelerometer component; gyroscope and magnetometer
    /// stay untouched.
    pub fn inplace_set_accelerometer(&mut self, value: PySignedPercentage3D) {
        let v: SignedPercentage3D = value.into();
        self.inner.set_accelerometer(v);
    }

    /// Replace only the gyroscope component; accelerometer and magnetometer
    /// stay untouched.
    pub fn inplace_set_gyroscope(&mut self, value: PySignedPercentage3D) {
        let v: SignedPercentage3D = value.into();
        self.inner.set_gyroscope(v);
    }

    /// Replace only the magnetometer component; accelerometer and gyroscope
    /// stay untouched.
    pub fn inplace_set_magnetometer(&mut self, value: PySignedPercentage3D) {
        let v: SignedPercentage3D = value.into();
        self.inner.set_magnetometer(v);
    }

    //endregion
}
