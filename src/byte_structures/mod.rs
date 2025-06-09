//pub mod serializers;
//mod deserializers;

pub mod feagi_byte_structure;

use pyo3::{pyclass, pymethods, PyResult};
use feagi_core_data_structures_and_processing::byte_structures::FeagiByteStructureType;
use feagi_core_data_structures_and_processing::error::DataProcessingError;
use pyo3::exceptions::PyValueError;
use crate::byte_structures::feagi_byte_structure::PyFeagiByteStructure;

#[pyclass(eq, eq_int)]
#[derive(PartialEq, Clone)]
#[pyo3(name = "FeagiByteStructureType")]
pub enum PyFeagiByteStructureType{
    JSON = 1,
    MultiStructHolder = 9,
    NeuronCategoricalXYZP = 11,
}

impl PyFeagiByteStructureType{
    pub fn from_base(e: FeagiByteStructureType) -> Self{
        match e {
            FeagiByteStructureType::JSON => PyFeagiByteStructureType::JSON,
            FeagiByteStructureType::MultiStructHolder => PyFeagiByteStructureType::MultiStructHolder,
            FeagiByteStructureType::NeuronCategoricalXYZP => PyFeagiByteStructureType::NeuronCategoricalXYZP,
        }
    }

    pub fn to_base(e: PyFeagiByteStructureType) -> FeagiByteStructureType{
        match e {
            PyFeagiByteStructureType::JSON => FeagiByteStructureType::JSON,
            PyFeagiByteStructureType::MultiStructHolder => FeagiByteStructureType::MultiStructHolder,
            PyFeagiByteStructureType::NeuronCategoricalXYZP => FeagiByteStructureType::NeuronCategoricalXYZP,
        }
    }
}

#[pyclass(subclass)]
#[pyo3(name = "FeagiByteStructureCompatible")]
pub struct PyFeagiByteStructureCompatible {}

#[pymethods]
impl PyFeagiByteStructureCompatible {
    
    #[getter]
    pub fn struct_type(&self) -> PyFeagiByteStructureType {
        PyFeagiByteStructureType::from_base(FeagiByteStructureType::JSON) // This is a overridden placeholder
    }
    
    pub fn version(&self) -> u8 { 0 } // This is a overridden placeholder
    
    #[staticmethod]
    pub fn new_from_feagi_byte_structure(_byte_structure: PyFeagiByteStructure) -> PyResult<Self> where Self: Sized {
        Err(PyValueError::new_err("Not properly overridden PyFeagiByteStructureCompatible abstract member!"))
    }
    
    pub fn as_new_feagi_byte_structure(&self) -> PyResult<PyFeagiByteStructure> {
        Err(PyValueError::new_err("Not properly overridden PyFeagiByteStructureCompatible abstract member!"))
    }
}