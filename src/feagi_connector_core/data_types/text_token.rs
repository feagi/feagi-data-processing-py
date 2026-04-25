use crate::feagi_connector_core::data_types::PyMiscData;
use crate::feagi_data_structures::neurons_voxels::xyzp::PyNeuronVoxelXYZPArrays;
use feagi_sensorimotor::data_types::{
    decode_token_id_from_misc_data, decode_token_id_from_xyzp_bitplanes,
    encode_token_id_to_misc_data, encode_token_id_to_xyzp_bitplanes,
};
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use tokenizers::Tokenizer;

/// Shared codec for FEAGI text token streams.
///
/// This exposes the deterministic FEAGI-side transport encoding/decoding to Python:
/// token_id <-> (x=0,y=0,z-bitplanes) via either `MiscData` or raw XYZP arrays.
#[pyclass]
#[pyo3(name = "TextTokenCodec")]
pub struct PyTextTokenCodec;

#[pymethods]
impl PyTextTokenCodec {
    /// Encode a token id into a 1x1x`depth` `MiscData` bitplane buffer.
    ///
    /// Gap semantics are represented by absence of token; use `None` on the caller side
    /// rather than encoding a value of zero.
    #[staticmethod]
    pub fn encode_to_misc_data(token_id: u32, depth: u32) -> PyResult<PyMiscData> {
        let inner = encode_token_id_to_misc_data(token_id, depth)
            .map_err(|e| PyValueError::new_err(e.to_string()))?;
        Ok(PyMiscData { inner })
    }

    /// Decode a token id from a 1x1x`depth` `MiscData` bitplane buffer.
    ///
    /// Returns None for a gap (no token emitted).
    #[staticmethod]
    pub fn decode_from_misc_data(misc: &PyMiscData) -> PyResult<Option<u32>> {
        decode_token_id_from_misc_data(&misc.inner)
            .map_err(|e| PyValueError::new_err(e.to_string()))
    }

    /// Encode a token id into raw XYZP arrays (bitplanes along z, z=0 is MSB).
    #[staticmethod]
    pub fn encode_to_xyzp_arrays(token_id: u32, depth: u32) -> PyResult<PyNeuronVoxelXYZPArrays> {
        let inner = encode_token_id_to_xyzp_bitplanes(token_id, depth)
            .map_err(|e| PyValueError::new_err(e.to_string()))?;
        Ok(PyNeuronVoxelXYZPArrays { inner })
    }

    /// Decode a token id from raw XYZP arrays (bitplanes along z, z=0 is MSB).
    ///
    /// Returns None for a gap (no token emitted).
    #[staticmethod]
    pub fn decode_from_xyzp_arrays(
        xyzp: &PyNeuronVoxelXYZPArrays,
        depth: u32,
    ) -> PyResult<Option<u32>> {
        decode_token_id_from_xyzp_bitplanes(&xyzp.inner, depth)
            .map_err(|e| PyValueError::new_err(e.to_string()))
    }
}

/// GPT-2 tokenizer wrapper (Hugging Face `tokenizers` crate).
///
/// This is a *tokenizer* (text <-> token IDs). It is intentionally kept separate from the
/// FEAGI bitplane codec (`TextTokenCodec`) so clients can swap tokenizers per language later.
#[pyclass]
#[pyo3(name = "Gpt2Tokenizer")]
pub struct PyGpt2Tokenizer {
    inner: Tokenizer,
}

#[pymethods]
impl PyGpt2Tokenizer {
    /// Load a GPT-2 tokenizer from a pinned `tokenizer.json` file.
    #[staticmethod]
    pub fn from_file(tokenizer_json_path: String) -> PyResult<Self> {
        let inner = Tokenizer::from_file(&tokenizer_json_path)
            .map_err(|e| PyValueError::new_err(format!("Failed to load tokenizer.json: {e}")))?;
        Ok(Self { inner })
    }

    /// Encode text into token IDs.
    pub fn encode(&self, text: String) -> PyResult<Vec<u32>> {
        let enc = self
            .inner
            .encode(text, true)
            .map_err(|e| PyValueError::new_err(format!("Tokenizer encode failed: {e}")))?;
        Ok(enc.get_ids().to_vec())
    }

    /// Decode token IDs into text.
    pub fn decode(&self, token_ids: Vec<u32>, skip_special_tokens: bool) -> PyResult<String> {
        self.inner
            .decode(&token_ids, skip_special_tokens)
            .map_err(|e| PyValueError::new_err(format!("Tokenizer decode failed: {e}")))
    }

    /// Return the vocabulary size.
    pub fn vocab_size(&self) -> PyResult<usize> {
        Ok(self.inner.get_vocab_size(true))
    }
}
