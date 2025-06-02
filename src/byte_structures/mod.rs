pub mod serializers;
mod deserializers;

use pyo3::pyclass;
use feagi_core_data_structures_and_processing::byte_structures::FeagiByteStructureType;

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
