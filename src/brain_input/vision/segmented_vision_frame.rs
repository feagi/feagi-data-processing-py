use pyo3::prelude::*;
use pyo3::exceptions::PyValueError;
use feagi_core_data_structures_and_processing::brain_input::vision::segmented_vision_frame::SegmentedVisionFrame;
use feagi_core_data_structures_and_processing::brain_input::vision::descriptors::*;
use feagi_core_data_structures_and_processing::cortical_data::CorticalID;


use super::image_frame::PyImageFrame;
use super::descriptors::*;
use crate::cortical_data::PyCorticalID;
use crate::neuron_data::PyCorticalMappedNeuronData;


#[pyclass]
#[pyo3(name = "SegmentedVisionFrame")]
pub struct PySegmentedVisionFrame{
    inner: SegmentedVisionFrame,
}

#[pymethods]
impl PySegmentedVisionFrame {
    #[new]
    pub fn new(
        segment_resolutions: &PySegmentedVisionTargetResolutions,
        segment_color_channels: PyChannelFormat,
        segment_color_space: PyColorSpace,
        input_frames_source_width_height: (usize, usize)
    ) -> PyResult<Self> {
        match SegmentedVisionFrame::new(
            &segment_resolutions.inner,
            &segment_color_channels.into(),
            &segment_color_space.into(),
            input_frames_source_width_height
        ) {
            Ok(inner) => Ok(PySegmentedVisionFrame { inner }),
            Err(err) => Err(PyErr::new::<PyValueError, _>(err.to_string())),
        }
    }

    #[staticmethod]
    pub fn create_ordered_cortical_ids(camera_index: u8, is_grayscale: bool) -> PyResult<Vec<PyCorticalID>> {
        match SegmentedVisionFrame::create_ordered_cortical_ids(camera_index, is_grayscale) {
            Ok(cortical_ids) => {
                let py_cortical_ids: Vec<PyCorticalID> = cortical_ids
                    .into_iter()
                    .map(|id| PyCorticalID { inner: id })
                    .collect();
                Ok(py_cortical_ids)
            },
            Err(err) => Err(PyErr::new::<PyValueError, _>(err.to_string())),
        }
    }

    #[getter]
    pub fn color_space(&self) -> PyColorSpace {
        match self.inner.get_color_space() {
            ColorSpace::Linear => PyColorSpace::Linear,
            ColorSpace::Gamma => PyColorSpace::Gamma,
        }
    }

    #[getter]
    pub fn color_channels(&self) -> PyChannelFormat {
        match self.inner.get_color_channels() {
            ChannelFormat::GrayScale => PyChannelFormat::GrayScale,
            ChannelFormat::RG => PyChannelFormat::RG,
            ChannelFormat::RGB => PyChannelFormat::RGB,
            ChannelFormat::RGBA => PyChannelFormat::RGBA,
        }
    }

    pub fn update_segments(
        &mut self,
        source_frame: &PyImageFrame,
        center_properties: &PySegmentedVisionCenterProperties
    ) -> PyResult<()> {
        match self.inner.update_segments(&source_frame.inner, center_properties.inner) {
            Ok(_) => Ok(()),
            Err(err) => Err(PyErr::new::<PyValueError, _>(err.to_string())),
        }
    }

    pub fn export_as_new_cortical_mapped_neuron_data(&mut self, camera_index: u8) -> PyResult<PyCorticalMappedNeuronData> {
        match self.inner.export_as_new_cortical_mapped_neuron_data(camera_index) {
            Ok(neuron_data) => Ok(PyCorticalMappedNeuronData { inner: neuron_data }),
            Err(err) => Err(PyErr::new::<PyValueError, _>(err.to_string())),
        }
    }

    pub fn inplace_export_cortical_mapped_neuron_data(
        &mut self,
        ordered_cortical_ids: Vec<PyCorticalID>,
        all_mapped_neuron_data: &mut PyCorticalMappedNeuronData
    ) -> PyResult<()> {
        if ordered_cortical_ids.len() != 9 {
            return Err(PyErr::new::<PyValueError, _>("ordered_cortical_ids must contain exactly 9 elements".to_string()));
        }

        // Convert Vec<PyCorticalID> to [CorticalID; 9]
        let mut cortical_ids_array: [CorticalID; 9] = [
            ordered_cortical_ids[0].inner.clone(),
            ordered_cortical_ids[1].inner.clone(),
            ordered_cortical_ids[2].inner.clone(),
            ordered_cortical_ids[3].inner.clone(),
            ordered_cortical_ids[4].inner.clone(),
            ordered_cortical_ids[5].inner.clone(),
            ordered_cortical_ids[6].inner.clone(),
            ordered_cortical_ids[7].inner.clone(),
            ordered_cortical_ids[8].inner.clone(),
        ];

        match self.inner.inplace_export_cortical_mapped_neuron_data(&cortical_ids_array, &mut all_mapped_neuron_data.inner) {
            Ok(_) => Ok(()),
            Err(err) => Err(PyErr::new::<PyValueError, _>(err.to_string())),
        }
    }
}