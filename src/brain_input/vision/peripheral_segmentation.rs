use pyo3::prelude::*;
use pyo3::exceptions::PyValueError;
use super::single_frame_processing::*;
use crate::neuron_state::neuron_data::{PyCorticalMappedNeuronData};
use feagi_core_data_structures_and_processing::brain_input::vision::peripheral_segmentation::{SegmentedVisionCenterProperties, SegmentedVisionFrame, SegmentedVisionTargetResolutions};
use feagi_core_data_structures_and_processing::brain_input::vision::single_frame_processing::ColorSpace;
use ndarray::Array3;
use numpy::PyReadonlyArray3;
use pyo3::types::{PyBytes};

use crate::brain_input::vision::image_frame::PyImageFrame;

#[pyclass]
#[pyo3(name = "SegmentedVisionFrame")]
pub struct PySegmentedVisionFrame{
    inner: SegmentedVisionFrame,
}

#[pymethods]
impl PySegmentedVisionFrame {
    #[new]
    pub fn new(input: PyImageFrame, center_properties: PySegmentedVisionCenterProperties, segment_resolutions: PySegmentedVisionTargetResolutions) -> PyResult<Self> {
        let result = SegmentedVisionFrame::new(&input.inner, &center_properties.inner, &segment_resolutions.inner);
        match result {
            Ok(inner) => Ok(PySegmentedVisionFrame {inner}),
            Err(msg) => Err(PyErr::new::<PyValueError, _>(msg.to_string()))
        }
    }

    #[staticmethod]
    pub fn new_no_segment_test_temp(input: PyImageFrame, center_properties: PySegmentedVisionCenterProperties, segment_resolutions: PySegmentedVisionTargetResolutions) -> PyResult<Self> {
        let result = SegmentedVisionFrame::new_no_segment_test_temp(&input.inner, &center_properties.inner, &segment_resolutions.inner);
        match result {
            Ok(inner) => Ok(PySegmentedVisionFrame {inner}),
            Err(msg) => Err(PyErr::new::<PyValueError, _>(msg.to_string()))
        }
    }

    /*
    pub fn update_in_place(&mut self, source_frame: PyImageFrame) -> PyResult<()> {
        let result = self.inner.update_in_place(&source_frame.inner);
        match result {
            Ok(()) => Ok(()),
            Err(msg) => Err(PyErr::new::<PyValueError, _>(msg))
        }
    }
    */

    pub fn export_as_cortical_mapped_neuron_data(&mut self, camera_index: u8)-> PyResult<PyCorticalMappedNeuronData> {
        let result = self.inner.export_as_cortical_mapped_neuron_data(camera_index);
        match result {
            Ok(cortical_data) => Ok(PyCorticalMappedNeuronData{inner: cortical_data}),
            Err(msg) => Err(PyErr::new::<PyValueError, _>(msg.to_string()))
        }
    }


    
    pub fn get_center_image_frame(&self) -> PyImageFrame {
        PyImageFrame {inner: self.inner.get_center_image_frame().clone()}
    }
}