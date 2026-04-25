use std::fmt::Debug;

use crate::{__base_py_class_shared, create_pyclass};
use feagi_data_structures::neuron_voxels::xyzp::NeuronVoxelXYZP;
use pyo3::prelude::*;
use pyo3::{pyclass, pymethods};

create_pyclass!(PyNeuronVoxelXYZP, NeuronVoxelXYZP, "PyNeuronVoxelXYZP");

#[pymethods]
impl PyNeuronVoxelXYZP {
    #[new]
    pub fn new(x: u32, y: u32, z: u32, p: f32) -> Self {
        PyNeuronVoxelXYZP {
            inner: NeuronVoxelXYZP::new(x, y, z, p),
        }
    }

    pub fn as_tuple(&self) -> (u32, u32, u32, f32) {
        self.inner.as_tuple()
    }
}
