use crate::feagi_connector_core::data_types::{
    PyGazeProperties, PyImageFilteringSettings, PyImageFrame, PyMiscData, PyPercentage,
    PyPercentage2D, PyPercentage3D, PyPercentage4D, PyRawIMU, PySegmentedImageFrame,
    PySignedPercentage, PySignedPercentage2D, PySignedPercentage3D, PySignedPercentage4D,
};
use feagi_data_structures::FeagiDataError;
use feagi_sensorimotor::wrapped_io_data::WrappedIOData;
use pyo3::exceptions::PyTypeError;
use pyo3::prelude::*;
use pyo3::types::PyBool;
use pyo3::{IntoPyObjectExt, PyResult};

/// Convert a cached motor (or other) [`WrappedIOData`] into the corresponding Python connector types.
pub fn wrapped_io_data_to_py_object(
    py: Python<'_>,
    wrapped_iodata: &WrappedIOData,
) -> PyResult<Py<PyAny>> {
    match wrapped_iodata {
        WrappedIOData::Boolean(boolean) => boolean.into_py_any(py),
        WrappedIOData::Percentage(percentage) => {
            let py_percentage = PyPercentage::from(percentage.clone());
            py_percentage.into_py_any(py)
        }
        WrappedIOData::Percentage_2D(percentage_2d) => {
            let py_percentage_2d = PyPercentage2D::from(percentage_2d.clone());
            py_percentage_2d.into_py_any(py)
        }
        WrappedIOData::Percentage_3D(percentage_3d) => {
            let py_percentage_3d = PyPercentage3D::from(percentage_3d.clone());
            py_percentage_3d.into_py_any(py)
        }
        WrappedIOData::Percentage_4D(percentage_4d) => {
            let py_percentage_4d = PyPercentage4D::from(percentage_4d.clone());
            py_percentage_4d.into_py_any(py)
        }
        WrappedIOData::SignedPercentage(signed_percentage) => {
            let py_signed_percentage = PySignedPercentage::from(signed_percentage.clone());
            py_signed_percentage.into_py_any(py)
        }
        WrappedIOData::SignedPercentage_2D(signed_percentage_2d) => {
            let py_signed_percentage_2d = PySignedPercentage2D::from(signed_percentage_2d.clone());
            py_signed_percentage_2d.into_py_any(py)
        }
        WrappedIOData::SignedPercentage_3D(signed_percentage_3d) => {
            let py_signed_percentage_3d = PySignedPercentage3D::from(signed_percentage_3d.clone());
            py_signed_percentage_3d.into_py_any(py)
        }
        WrappedIOData::SignedPercentage_4D(signed_percentage_4d) => {
            let py_signed_percentage_4d = PySignedPercentage4D::from(signed_percentage_4d.clone());
            py_signed_percentage_4d.into_py_any(py)
        }
        WrappedIOData::ImageFrame(frame) => {
            let py_frame = PyImageFrame::from(frame.clone());
            py_frame.into_py_any(py)
        }
        WrappedIOData::SegmentedImageFrame(segmented_frame) => {
            let py_segmented_frame = PySegmentedImageFrame::from(segmented_frame.clone());
            py_segmented_frame.into_py_any(py)
        }
        WrappedIOData::RawIMU(raw_imu) => {
            let py_raw_imu = PyRawIMU::from(*raw_imu);
            py_raw_imu.into_py_any(py)
        }
        WrappedIOData::MiscData(misc_data) => {
            let py_misc_data = PyMiscData::from(misc_data.clone());
            py_misc_data.into_py_any(py)
        }
        WrappedIOData::GazeProperties(gaze_properties) => {
            let py_gaze_properties = PyGazeProperties::from(gaze_properties.clone());
            py_gaze_properties.into_py_any(py)
        }
        WrappedIOData::ImageFilteringSettings(settings) => {
            let py_settings = PyImageFilteringSettings::from(settings.clone());
            py_settings.into_py_any(py)
        }
        WrappedIOData::PoseEstimationData(_) => Err(PyTypeError::new_err(
            "PoseEstimationData is not yet exposed in feagi_rust_py_libs Python bindings.",
        )),
    }
}

pub fn py_any_to_wrapped_io_data<'py>(
    _py: Python<'_>,
    py_wrapped: &Bound<'py, PyAny>,
) -> Result<WrappedIOData, FeagiDataError> {
    // Yes this is an if else chain. But this is the way the docs suggested for best performance https://pyo3.rs/main/performance.html#extract-versus-cast
    // Other ideas are welcome

    if let Ok(reference) = py_wrapped.cast::<PyImageFrame>() {
        let image_frame = &reference.borrow().inner;
        return Ok(WrappedIOData::ImageFrame(image_frame.clone()));
    } else if let Ok(reference) = py_wrapped.cast::<PySegmentedImageFrame>() {
        let segmented_frame = &reference.borrow().inner;
        return Ok(WrappedIOData::SegmentedImageFrame(segmented_frame.clone()));
    } else if let Ok(reference) = py_wrapped.cast::<PyRawIMU>() {
        // Cast Raw IMU before the more generic 3D types: it's a distinct composite,
        // not a SignedPercentage3D, so we must catch it explicitly here.
        let raw_imu = &reference.borrow().inner;
        return Ok(WrappedIOData::RawIMU(*raw_imu));
    } else if let Ok(reference) = py_wrapped.cast::<PyGazeProperties>() {
        let gaze_properties = &reference.borrow().inner;
        return Ok(WrappedIOData::GazeProperties(gaze_properties.clone()));
    } else if let Ok(reference) = py_wrapped.cast::<PyImageFilteringSettings>() {
        let settings = &reference.borrow().inner;
        return Ok(WrappedIOData::ImageFilteringSettings(settings.clone()));
    } else if let Ok(reference) = py_wrapped.cast::<PyMiscData>() {
        let misc_data = &reference.borrow().inner;
        return Ok(WrappedIOData::MiscData(misc_data.clone()));
    } else if let Ok(reference) = py_wrapped.cast::<PySignedPercentage4D>() {
        let percentage = &reference.borrow().inner;
        return Ok(WrappedIOData::SignedPercentage_4D(percentage.clone()));
    } else if let Ok(reference) = py_wrapped.cast::<PySignedPercentage3D>() {
        let percentage = &reference.borrow().inner;
        return Ok(WrappedIOData::SignedPercentage_3D(percentage.clone()));
    } else if let Ok(reference) = py_wrapped.cast::<PySignedPercentage2D>() {
        let percentage = &reference.borrow().inner;
        return Ok(WrappedIOData::SignedPercentage_2D(percentage.clone()));
    } else if let Ok(reference) = py_wrapped.cast::<PySignedPercentage>() {
        let percentage = &reference.borrow().inner;
        return Ok(WrappedIOData::SignedPercentage(percentage.clone()));
    } else if let Ok(reference) = py_wrapped.cast::<PyPercentage4D>() {
        let percentage = &reference.borrow().inner;
        return Ok(WrappedIOData::Percentage_4D(percentage.clone()));
    } else if let Ok(reference) = py_wrapped.cast::<PyPercentage3D>() {
        let percentage = &reference.borrow().inner;
        return Ok(WrappedIOData::Percentage_3D(percentage.clone()));
    } else if let Ok(reference) = py_wrapped.cast::<PyPercentage2D>() {
        let percentage = &reference.borrow().inner;
        return Ok(WrappedIOData::Percentage_2D(percentage.clone()));
    } else if let Ok(reference) = py_wrapped.cast::<PyPercentage>() {
        let percentage = &reference.borrow().inner;
        return Ok(WrappedIOData::Percentage(percentage.clone()));
    } else if let Ok(reference) = py_wrapped.cast::<PyBool>() {
        let boolean: bool = reference.is_true(); // lol
        return Ok(WrappedIOData::Boolean(boolean));
    }

    Err(FeagiDataError::BadParameters(
        "Unable to parse object as any supported wrapped io data!".into(),
    ))
}
