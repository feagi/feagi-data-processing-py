use pyo3::{Bound, PyResult};
use pyo3::prelude::{PyModule, PyModuleMethods};

pub mod neuron_data;
use neuron_data::{PyCorticalMappedNeuronData, PyNeuronYXCPArrays};

pub fn register_neuron_state(parent_module: &Bound<'_, PyModule>) -> PyResult<()> {
    let child_module = PyModule::new(parent_module.py(), "neuron_state")?;

    register_neuron_data(&child_module)?;

    parent_module.add_submodule(&child_module)
}

pub fn register_neuron_data(parent_module: &Bound<'_, PyModule>) -> PyResult<()> {
    let child_module = PyModule::new(parent_module.py(), "neuron_data")?;

    child_module.add_class::<PyCorticalMappedNeuronData>()?;
    child_module.add_class::<PyNeuronYXCPArrays>()?;

    parent_module.add_submodule(&child_module)
}