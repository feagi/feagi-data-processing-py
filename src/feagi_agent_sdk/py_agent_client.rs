/*
 * PyO3 wrapper for AgentClient
 */

use pyo3::prelude::*;
use pyo3::types::{PyAny, PyBytes, PyList};
use super::py_agent_config::PyAgentConfig;
use std::sync::{Arc, Mutex};

#[pyclass(name = "PyAgentClient")]
pub struct PyAgentClient {
    inner: Arc<Mutex<feagi_agent::AgentClient>>,
}

#[pymethods]
impl PyAgentClient {
    #[new]
    fn new(config: &PyAgentConfig) -> PyResult<Self> {
        let client = feagi_agent::AgentClient::new(config.inner().clone())
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;
        
        Ok(PyAgentClient {
            inner: Arc::new(Mutex::new(client)),
        })
    }
    
    /// Connect and register with FEAGI
    fn connect(&mut self) -> PyResult<()> {
        let mut client = self.inner.lock()
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(
                format!("Lock poisoned: {}", e)
            ))?;
        
        client.connect()
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))
    }
    
    /// Send sensory data as list of (neuron_id, potential) tuples
    fn send_sensory_data(&self, _py: Python, neuron_pairs: Bound<'_, PyAny>) -> PyResult<()> {
        let list = neuron_pairs.cast::<PyList>()?;
        let mut pairs: Vec<(i32, f64)> = Vec::new();
        
        for item in list {
            let tuple = item.cast::<pyo3::types::PyTuple>()?;
            if tuple.len() != 2 {
                return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                    "Each item must be a (neuron_id, potential) tuple"
                ));
            }
            let neuron_id: i32 = tuple.get_item(0)?.extract()?;
            let potential: f64 = tuple.get_item(1)?.extract()?;
            pairs.push((neuron_id, potential));
        }
        
        let client = self.inner.lock()
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(
                format!("Lock poisoned: {}", e)
            ))?;
        
        // Send data (ZMQ sockets are not Sync, so we can't release GIL)
        client.send_sensory_data(pairs)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))
    }

    /// Send pre-serialized sensory bytes to FEAGI (FeagiByteContainer bytes).
    ///
    /// Real-time semantics: the underlying Rust client drops on backpressure instead of blocking.
    fn send_sensory_bytes(&self, _py: Python, payload: Bound<'_, PyAny>) -> PyResult<()> {
        let py_bytes = payload.cast::<PyBytes>()?;
        let bytes = py_bytes.as_bytes().to_vec();

        let client = self.inner.lock().map_err(|e| {
            PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("Lock poisoned: {}", e))
        })?;

        client
            .send_sensory_bytes(bytes)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))
    }
    
    /// Receive motor data (non-blocking, returns None if no data)
    /// Returns motor data as JSON string in format: {"motor": {"0": value, "1": value, ...}}
    fn receive_motor_data(&self, _py: Python) -> PyResult<Option<String>> {
        let client = self.inner.lock()
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(
                format!("Lock poisoned: {}", e)
            ))?;
        
        // Receive data (ZMQ sockets are not Sync, so we can't release GIL)
        match client.receive_motor_data() {
            Ok(Some(data)) => {
                // Return standard XYZP SoA format: {cortical_id: {x: [...], y: [...], z: [...], p: [...]}}
                use serde_json::json;
                
                let mut result = serde_json::Map::new();
                
                for (cortical_id, neuron_voxels) in data.mappings.iter() {
                    let (x_vec, y_vec, z_vec, p_vec) = neuron_voxels.borrow_xyzp_vectors();
                    
                    let mut area_data = serde_json::Map::new();
                    area_data.insert("x".to_string(), json!(x_vec));
                    area_data.insert("y".to_string(), json!(y_vec));
                    area_data.insert("z".to_string(), json!(z_vec));
                    area_data.insert("p".to_string(), json!(p_vec));
                    
                    // Use base64-encoded cortical ID so Python decode_motor_xyzp can parse
                    // group_id (byte 7) and emit "group:channel" keys for multi-group motors.
                    let cortical_id_str = cortical_id.as_base_64();
                    result.insert(cortical_id_str, serde_json::Value::Object(area_data));
                }
                
                // Return standard XYZP SoA JSON
                let response = serde_json::Value::Object(result);
                Ok(Some(response.to_string()))
            },
            Ok(None) => Ok(None),
            Err(e) => Err(PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string())),
        }
    }
    
    /// Check if agent is registered
    fn is_registered(&self) -> PyResult<bool> {
        let client = self.inner.lock()
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(
                format!("Lock poisoned: {}", e)
            ))?;
        Ok(client.is_registered())
    }
    
    /// Note: Disconnect happens automatically on drop, no explicit disconnect needed
    /// This method is kept for compatibility but is a no-op
    fn disconnect(&mut self) -> PyResult<()> {
        // Agent automatically deregisters on drop
        Ok(())
    }
    
    fn __repr__(&self) -> String {
        let registered = self.is_registered().unwrap_or(false);
        format!("PyAgentClient(registered={})", registered)
    }
}

