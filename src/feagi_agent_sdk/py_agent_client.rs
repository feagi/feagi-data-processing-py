/*
 * PyO3 wrapper for AgentClient on top of FEAGI 2.0 client primitives.
 */

use super::py_agent_config::{AgentConfigCompat, PyAgentConfig};
use super::py_agent_type::AgentTypeCompat;
use feagi_agent::clients::{AgentRegistrationStatus, CommandControlAgent};
use feagi_agent::command_and_control::agent_embodiment_configuration_message::AgentEmbodimentConfigurationMessage;
use feagi_agent::command_and_control::FeagiMessage;
use feagi_agent::{AgentCapabilities, AgentDescriptor, AuthToken};
use feagi_sensorimotor::configuration::jsonable::JSONInputOutputDefinition;
use feagi_data_structures::genomic::cortical_area::CorticalID;
use feagi_data_structures::neuron_voxels::xyzp::{
    CorticalMappedXYZPNeuronVoxels, NeuronVoxelXYZPArrays,
};
use feagi_io::protocol_implementations::websocket::WebSocketUrl;
use feagi_io::protocol_implementations::zmq::ZmqUrl;
use feagi_io::traits_and_enums::client::{
    FeagiClientPusher, FeagiClientSubscriber,
};
use feagi_io::traits_and_enums::shared::{FeagiEndpointState, TransportProtocolEndpoint};
use feagi_io::AgentID;
use feagi_serialization::FeagiByteContainer;
use pyo3::prelude::*;
use pyo3::types::{PyAny, PyBytes, PyList, PyTuple};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

struct AgentClientCompat {
    config: AgentConfigCompat,
    command_control: Option<CommandControlAgent>,
    sensory_client: Option<Box<dyn FeagiClientPusher>>,
    motor_client: Option<Box<dyn FeagiClientSubscriber>>,
    registered: bool,
    last_heartbeat_sent_at: Option<Instant>,
    /// AgentID (base64) assigned at registration; used for device_registrations import
    registration_agent_id_b64: Option<String>,
}

impl AgentClientCompat {
    fn new(config: AgentConfigCompat) -> Self {
        Self {
            config,
            command_control: None,
            sensory_client: None,
            motor_client: None,
            registered: false,
            last_heartbeat_sent_at: None,
            registration_agent_id_b64: None,
        }
    }

    fn parse_endpoint(endpoint: &str) -> Result<TransportProtocolEndpoint, String> {
        if endpoint.starts_with("tcp://") {
            let parsed = ZmqUrl::new(endpoint).map_err(|e| e.to_string())?;
            Ok(TransportProtocolEndpoint::Zmq(parsed))
        } else if endpoint.starts_with("ws://") || endpoint.starts_with("wss://") {
            let parsed = WebSocketUrl::new(endpoint).map_err(|e| e.to_string())?;
            Ok(TransportProtocolEndpoint::WebSocket(parsed))
        } else {
            Err(format!(
                "Unsupported endpoint scheme for '{}'; expected tcp://, ws://, or wss://",
                endpoint
            ))
        }
    }

    fn requested_capabilities(agent_type: AgentTypeCompat) -> Vec<AgentCapabilities> {
        match agent_type {
            AgentTypeCompat::Sensory => vec![AgentCapabilities::SendSensorData],
            AgentTypeCompat::Motor => vec![AgentCapabilities::ReceiveMotorData],
            AgentTypeCompat::Both => vec![
                AgentCapabilities::SendSensorData,
                AgentCapabilities::ReceiveMotorData,
            ],
        }
    }

    fn connect(&mut self) -> Result<(), String> {
        if self.registered {
            return Ok(());
        }

        let endpoint = Self::parse_endpoint(&self.config.registration_endpoint)?;
        let requester = endpoint
            .try_create_boxed_client_requester_properties()
            .map_err(|e| e.to_string())?;
        let mut control = CommandControlAgent::new(requester);
        control.request_connect().map_err(|e| e.to_string())?;

        let descriptor_cfg = self
            .config
            .descriptor
            .as_ref()
            .ok_or_else(|| "Agent descriptor is not configured".to_string())?;
        let descriptor = AgentDescriptor::new(
            &descriptor_cfg.manufacturer,
            &descriptor_cfg.agent_name,
            descriptor_cfg.agent_version,
        )
        .map_err(|e| e.to_string())?;
        let auth_bytes = self
            .config
            .auth_token
            .ok_or_else(|| "Auth token is not configured".to_string())?;
        let auth = AuthToken::new(auth_bytes);
        let requested_capabilities = Self::requested_capabilities(self.config.agent_type);

        let connect_timeout_ms = self
            .config
            .connection_timeout_ms
            .ok_or_else(|| "connection_timeout_ms is not configured".to_string())?;
        let registration_retries = self
            .config
            .registration_retries
            .ok_or_else(|| "registration_retries is not configured".to_string())?;
        let timeout_ms = connect_timeout_ms.saturating_mul(registration_retries as u64).max(1);
        let deadline = Instant::now() + Duration::from_millis(timeout_ms);
        let mut sent_registration = false;

        let mut session_id: Option<AgentID> = None;
        let mut endpoints: Option<std::collections::HashMap<AgentCapabilities, TransportProtocolEndpoint>> = None;

        while Instant::now() < deadline {
            let (state, _message) = control.poll_for_messages().map_err(|e| e.to_string())?;

            if !sent_registration
                && matches!(
                    state,
                    FeagiEndpointState::ActiveWaiting | FeagiEndpointState::ActiveHasData
                )
            {
                control
                    .request_registration(
                        descriptor.clone(),
                        auth.clone(),
                        requested_capabilities.clone(),
                    )
                    .map_err(|e| e.to_string())?;
                sent_registration = true;
            }

            if let AgentRegistrationStatus::Registered(id, map) = control.registration_status() {
                session_id = Some(*id);
                endpoints = Some(map.clone());
                break;
            }

            std::thread::yield_now();
        }

        let session_id = session_id.ok_or_else(|| "Registration timed out".to_string())?;
        let endpoint_map = endpoints.ok_or_else(|| "No endpoint map after registration".to_string())?;

        if matches!(
            self.config.agent_type,
            AgentTypeCompat::Sensory | AgentTypeCompat::Both
        ) {
            let sensory_endpoint = endpoint_map
                .get(&AgentCapabilities::SendSensorData)
                .cloned()
                .ok_or_else(|| "FEAGI registration did not provide sensory endpoint".to_string())?;
            let props = sensory_endpoint
                .try_create_boxed_client_pusher_properties()
                .map_err(|e| e.to_string())?;
            let mut sensory_client = props.as_boxed_client_pusher();
            sensory_client.request_connect().map_err(|e| e.to_string())?;
            self.wait_until_active_waiting_pusher(sensory_client.as_mut(), deadline)?;
            self.sensory_client = Some(sensory_client);
        }

        if matches!(
            self.config.agent_type,
            AgentTypeCompat::Motor | AgentTypeCompat::Both
        ) {
            let motor_endpoint = endpoint_map
                .get(&AgentCapabilities::ReceiveMotorData)
                .cloned()
                .ok_or_else(|| "FEAGI registration did not provide motor endpoint".to_string())?;
            let props = motor_endpoint
                .try_create_boxed_client_subscriber_properties()
                .map_err(|e| e.to_string())?;
            let mut motor_client = props.as_boxed_client_subscriber();
            motor_client.request_connect().map_err(|e| e.to_string())?;
            self.wait_until_active_waiting_subscriber(motor_client.as_mut(), deadline)?;
            self.motor_client = Some(motor_client);
        }

        self.registration_agent_id_b64 = Some(session_id.to_base64());
        self.command_control = Some(control);
        self.registered = true;
        self.last_heartbeat_sent_at = Some(Instant::now());
        Ok(())
    }

    /// Send device registrations (AgentConfiguration) to FEAGI over the command/control channel.
    /// Must be called after connect() when the agent has motors registered.
    fn send_device_configuration(&mut self, device_registrations_json: &str) -> Result<(), String> {
        if !self.registered {
            return Err("Agent is not registered; call connect() first".to_string());
        }
        let control = self
            .command_control
            .as_mut()
            .ok_or_else(|| "Command control channel not available".to_string())?;
        let device_def: JSONInputOutputDefinition = serde_json::from_str(device_registrations_json)
            .map_err(|e| format!("Invalid device_registrations JSON: {}", e))?;
        let message = FeagiMessage::AgentConfiguration(
            AgentEmbodimentConfigurationMessage::AgentConfigurationDetails(device_def),
        );
        control
            .send_message(message, 0)
            .map_err(|e| format!("Failed to send device configuration: {}", e))?;
        Ok(())
    }

    fn wait_until_active_waiting_pusher(
        &self,
        pusher: &mut dyn FeagiClientPusher,
        deadline: Instant,
    ) -> Result<(), String> {
        while Instant::now() < deadline {
            match pusher.poll() {
                FeagiEndpointState::ActiveWaiting => return Ok(()),
                FeagiEndpointState::ActiveHasData => {
                    return Err("Sensory pusher entered invalid ActiveHasData state".to_string());
                }
                FeagiEndpointState::Errored(err) => {
                    return Err(format!("Sensory connect failed: {}", err));
                }
                _ => {
                    std::thread::yield_now();
                }
            }
        }
        Err("Timed out waiting for sensory channel connection".to_string())
    }

    fn wait_until_active_waiting_subscriber(
        &self,
        subscriber: &mut dyn FeagiClientSubscriber,
        deadline: Instant,
    ) -> Result<(), String> {
        while Instant::now() < deadline {
            match subscriber.poll() {
                FeagiEndpointState::ActiveWaiting | FeagiEndpointState::ActiveHasData => {
                    return Ok(());
                }
                FeagiEndpointState::Errored(err) => {
                    return Err(format!("Motor connect failed: {}", err));
                }
                _ => {
                    std::thread::yield_now();
                }
            }
        }
        Err("Timed out waiting for motor channel connection".to_string())
    }

    fn send_sensory_bytes(&mut self, bytes: &[u8]) -> Result<(), String> {
        if !self.registered {
            return Err("Agent is not registered".to_string());
        }
        self.maybe_send_heartbeat()?;
        let sensory_client = self
            .sensory_client
            .as_mut()
            .ok_or_else(|| "Sensory socket is not available for this agent type".to_string())?;

        match sensory_client.poll() {
            FeagiEndpointState::ActiveWaiting => {}
            FeagiEndpointState::Pending => return Err("Sensory socket is pending".to_string()),
            FeagiEndpointState::Inactive => return Err("Sensory socket is inactive".to_string()),
            FeagiEndpointState::ActiveHasData => {
                return Err("Sensory socket entered invalid ActiveHasData state".to_string());
            }
            FeagiEndpointState::Errored(err) => {
                return Err(format!("Sensory socket errored: {}", err));
            }
        }

        sensory_client.publish_data(bytes).map_err(|e| e.to_string())
    }

    fn encode_sensory_pairs_to_container(
        &self,
        pairs: &[(i32, f64)],
    ) -> Result<FeagiByteContainer, String> {
        let vision = self
            .config
            .vision_capability
            .as_ref()
            .ok_or_else(|| "Vision capability is required for send_sensory_data".to_string())?;

        let cortical_area = vision.cortical_area.as_ref().ok_or_else(|| {
            "send_sensory_data requires with_vision_capability(target_cortical_area)".to_string()
        })?;
        if cortical_area.is_empty() {
            return Err("target_cortical_area cannot be empty".to_string());
        }
        if vision.width == 0 || vision.height == 0 || vision.channels == 0 {
            return Err("Vision dimensions and channels must all be > 0".to_string());
        }

        let mut id_bytes = [b' '; 8];
        let area_bytes = cortical_area.as_bytes();
        let copy_len = area_bytes.len().min(8);
        id_bytes[..copy_len].copy_from_slice(&area_bytes[..copy_len]);
        let cortical_id =
            CorticalID::try_from_bytes(&id_bytes).map_err(|e| format!("Invalid cortical area id: {:?}", e))?;

        let width = vision.width as u32;
        let height = vision.height as u32;
        let channels = vision.channels as u32;
        let plane = width
            .checked_mul(height)
            .ok_or_else(|| "Vision width*height overflowed u32".to_string())?;
        let max_index = plane
            .checked_mul(channels)
            .ok_or_else(|| "Vision width*height*channels overflowed u32".to_string())?;

        let mut x_coords = Vec::with_capacity(pairs.len());
        let mut y_coords = Vec::with_capacity(pairs.len());
        let mut z_coords = Vec::with_capacity(pairs.len());
        let mut potentials = Vec::with_capacity(pairs.len());

        for (neuron_id, potential) in pairs.iter().copied() {
            if neuron_id < 0 {
                return Err(format!("Neuron id must be non-negative, got {}", neuron_id));
            }
            let idx = neuron_id as u32;
            if idx >= max_index {
                return Err(format!(
                    "Neuron id {} is out of bounds for configured vision shape {}x{}x{}",
                    idx, width, height, channels
                ));
            }
            let z = idx / plane;
            let in_plane = idx % plane;
            let y = in_plane / width;
            let x = in_plane % width;

            x_coords.push(x);
            y_coords.push(y);
            z_coords.push(z);
            potentials.push(potential as f32);
        }

        let arrays = NeuronVoxelXYZPArrays::new_from_vectors(x_coords, y_coords, z_coords, potentials)
            .map_err(|e| format!("Failed to create neuron arrays: {}", e))?;
        let mut mapped = CorticalMappedXYZPNeuronVoxels::new();
        mapped.insert(cortical_id, arrays);

        let mut container = FeagiByteContainer::new_empty();
        container
            .overwrite_byte_data_with_single_struct_data(&mapped, 0)
            .map_err(|e| format!("Failed to serialize sensory data: {}", e))?;
        Ok(container)
    }

    fn receive_motor_data(&mut self) -> Result<Option<String>, String> {
        if !self.registered {
            return Err("Agent is not registered".to_string());
        }
        self.maybe_send_heartbeat()?;
        let motor_client = self
            .motor_client
            .as_mut()
            .ok_or_else(|| "Motor socket is not available for this agent type".to_string())?;

        let raw_data: Vec<u8> = match motor_client.poll() {
            FeagiEndpointState::ActiveHasData => motor_client
                .consume_retrieved_data()
                .map_err(|e| e.to_string())?
                .to_vec(),
            FeagiEndpointState::ActiveWaiting | FeagiEndpointState::Pending | FeagiEndpointState::Inactive => {
                return Ok(None)
            }
            FeagiEndpointState::Errored(err) => {
                return Err(format!("Motor socket errored: {}", err));
            }
        };

        let mut buffer = FeagiByteContainer::new_empty();
        buffer
            .try_write_data_by_copy_and_verify(&raw_data)
            .map_err(|e| e.to_string())?;

        let structure_count = buffer
            .try_get_number_contained_structures()
            .map_err(|e| e.to_string())?;
        if structure_count == 0 {
            return Ok(None);
        }

        let boxed_struct = buffer
            .try_create_new_struct_from_index(0)
            .map_err(|e| e.to_string())?;
        let motor_data = boxed_struct
            .as_any()
            .downcast_ref::<CorticalMappedXYZPNeuronVoxels>()
            .ok_or_else(|| "Received motor payload is not CorticalMappedXYZPNeuronVoxels".to_string())?;

        let mut result = serde_json::Map::new();
        for (cortical_id, neuron_voxels) in motor_data.mappings.iter() {
            let (x_vec, y_vec, z_vec, p_vec) = neuron_voxels.borrow_xyzp_vectors();
            let mut area_data = serde_json::Map::new();
            area_data.insert("x".to_string(), serde_json::json!(x_vec));
            area_data.insert("y".to_string(), serde_json::json!(y_vec));
            area_data.insert("z".to_string(), serde_json::json!(z_vec));
            area_data.insert("p".to_string(), serde_json::json!(p_vec));
            result.insert(cortical_id.as_base_64(), serde_json::Value::Object(area_data));
        }

        Ok(Some(serde_json::Value::Object(result).to_string()))
    }

    fn maybe_send_heartbeat(&mut self) -> Result<(), String> {
        if !self.registered {
            return Ok(());
        }
        let interval_secs = self
            .config
            .heartbeat_interval_secs
            .ok_or_else(|| "heartbeat_interval_secs is not configured".to_string())?;
        if interval_secs <= 0.0 {
            return Ok(());
        }

        let should_send = match self.last_heartbeat_sent_at {
            None => true,
            Some(last) => last.elapsed() >= Duration::from_secs_f64(interval_secs),
        };
        if !should_send {
            return Ok(());
        }

        let control = self
            .command_control
            .as_mut()
            .ok_or_else(|| "Command control channel is not initialized".to_string())?;
        control.send_heartbeat().map_err(|e| e.to_string())?;
        self.last_heartbeat_sent_at = Some(Instant::now());
        Ok(())
    }

    fn disconnect(&mut self) -> Result<(), String> {
        if !self.registered {
            return Ok(());
        }

        let timeout_ms = self
            .config
            .connection_timeout_ms
            .ok_or_else(|| "connection_timeout_ms is not configured".to_string())?;
        let deadline = Instant::now() + Duration::from_millis(timeout_ms.max(1));

        if let Some(control) = self.command_control.as_mut() {
            control
                .request_deregistration(Some("python-sdk disconnect".to_string()))
                .map_err(|e| e.to_string())?;

            while Instant::now() < deadline {
                let _ = control.poll_for_messages().map_err(|e| e.to_string())?;
                if matches!(control.registration_status(), AgentRegistrationStatus::NotRegistered) {
                    break;
                }
                std::thread::yield_now();
            }
            if let Err(err) = control.request_disconnect() {
                let msg = err.to_string();
                if !msg.contains("Cannot disconnect: client is not in Active state") {
                    return Err(msg);
                }
            }
        }

        if let Some(sensory) = self.sensory_client.as_mut() {
            if let Err(err) = sensory.request_disconnect() {
                let msg = err.to_string();
                if !msg.contains("not in Active state") {
                    return Err(msg);
                }
            }
            let _ = sensory.poll();
        }
        if let Some(motor) = self.motor_client.as_mut() {
            if let Err(err) = motor.request_disconnect() {
                let msg = err.to_string();
                if !msg.contains("not in Active state") {
                    return Err(msg);
                }
            }
            let _ = motor.poll();
        }

        self.sensory_client = None;
        self.motor_client = None;
        self.command_control = None;
        self.registered = false;
        self.last_heartbeat_sent_at = None;
        Ok(())
    }
}

#[pyclass(name = "PyAgentClient")]
pub struct PyAgentClient {
    inner: Arc<Mutex<AgentClientCompat>>,
}

#[pymethods]
impl PyAgentClient {
    #[new]
    fn new(config: &PyAgentConfig) -> PyResult<Self> {
        config
            .validate_internal()
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e))?;
        Ok(PyAgentClient {
            inner: Arc::new(Mutex::new(AgentClientCompat::new(config.inner().clone()))),
        })
    }

    fn connect(&mut self) -> PyResult<()> {
        let mut client = self.inner.lock().map_err(|e| {
            PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("Lock poisoned: {}", e))
        })?;
        client
            .connect()
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e))
    }

    fn send_sensory_data(&self, _py: Python, neuron_pairs: Bound<'_, PyAny>) -> PyResult<()> {
        let list = neuron_pairs.cast::<PyList>()?;
        let mut pairs: Vec<(i32, f64)> = Vec::new();
        for item in list {
            let tuple = item.cast::<PyTuple>()?;
            if tuple.len() != 2 {
                return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                    "Each item must be a (neuron_id, potential) tuple",
                ));
            }
            let neuron_id: i32 = tuple.get_item(0)?.extract()?;
            let potential: f64 = tuple.get_item(1)?.extract()?;
            pairs.push((neuron_id, potential));
        }

        let mut client = self.inner.lock().map_err(|e| {
            PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("Lock poisoned: {}", e))
        })?;
        let container = client
            .encode_sensory_pairs_to_container(&pairs)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e))?;
        client
            .send_sensory_bytes(container.get_byte_ref())
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e))
    }

    fn send_sensory_bytes(&self, _py: Python, payload: Bound<'_, PyAny>) -> PyResult<()> {
        let py_bytes = payload.cast::<PyBytes>()?;
        let bytes = py_bytes.as_bytes().to_vec();
        let mut client = self.inner.lock().map_err(|e| {
            PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("Lock poisoned: {}", e))
        })?;
        client
            .send_sensory_bytes(&bytes)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e))
    }

    fn receive_motor_data(&self, _py: Python) -> PyResult<Option<String>> {
        let mut client = self.inner.lock().map_err(|e| {
            PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("Lock poisoned: {}", e))
        })?;
        client
            .receive_motor_data()
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e))
    }

    fn is_registered(&self) -> PyResult<bool> {
        let mut client = self.inner.lock().map_err(|e| {
            PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("Lock poisoned: {}", e))
        })?;
        client
            .maybe_send_heartbeat()
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e))?;
        Ok(client.registered)
    }

    /// Return AgentID (base64) assigned at registration, or None if not connected.
    fn get_registration_agent_id_b64(&self) -> PyResult<Option<String>> {
        let client = self.inner.lock().map_err(|e| {
            PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("Lock poisoned: {}", e))
        })?;
        Ok(client.registration_agent_id_b64.clone())
    }

    /// Send device registrations to FEAGI over ZMQ (AgentConfiguration message).
    /// Call after connect() when the agent has motors. Replaces HTTP device_registrations import.
    fn send_device_configuration(&self, device_registrations_json: &str) -> PyResult<()> {
        let mut client = self.inner.lock().map_err(|e| {
            PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("Lock poisoned: {}", e))
        })?;
        client
            .send_device_configuration(device_registrations_json)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e))
    }

    fn disconnect(&mut self) -> PyResult<()> {
        let mut client = self.inner.lock().map_err(|e| {
            PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("Lock poisoned: {}", e))
        })?;
        client
            .disconnect()
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e))
    }

    fn __repr__(&self) -> String {
        let registered = self.is_registered().unwrap_or(false);
        format!("PyAgentClient(registered={})", registered)
    }
}

#[cfg(test)]
mod tests {
    use super::AgentClientCompat;
    use crate::feagi_agent_sdk::py_agent_config::{
        AgentConfigCompat, AgentDescriptorCompat, VisionCapabilityCompat,
    };
    use crate::feagi_agent_sdk::py_agent_type::AgentTypeCompat;

    fn configured_client() -> AgentClientCompat {
        let mut cfg = AgentConfigCompat::new("agent-1".to_string(), AgentTypeCompat::Sensory);
        cfg.registration_endpoint = "tcp://127.0.0.1:30001".to_string();
        cfg.sensory_endpoint = "tcp://127.0.0.1:5558".to_string();
        cfg.connection_timeout_ms = Some(1000);
        cfg.registration_retries = Some(3);
        cfg.heartbeat_interval_secs = Some(1.0);
        cfg.descriptor = Some(AgentDescriptorCompat {
            manufacturer: "Neuraville".to_string(),
            agent_name: "agent-1".to_string(),
            agent_version: 1,
        });
        cfg.auth_token = Some([9_u8; 32]);
        cfg.vision_capability = Some(VisionCapabilityCompat {
            modality: "vision".to_string(),
            width: 2,
            height: 2,
            channels: 1,
            cortical_area: Some("iv00".to_string()),
            unit: None,
            group: None,
        });
        AgentClientCompat::new(cfg)
    }

    #[test]
    fn encode_sensory_pairs_to_container_produces_single_structure() {
        let client = configured_client();
        let container = client
            .encode_sensory_pairs_to_container(&[(0, 0.1), (1, 0.2), (2, 0.3)])
            .expect("encoding should succeed");

        let count = container
            .try_get_number_contained_structures()
            .expect("container read should succeed");
        assert_eq!(count, 1);
    }

    #[test]
    fn encode_sensory_pairs_to_container_rejects_out_of_bounds_neuron() {
        let client = configured_client();
        let err = client
            .encode_sensory_pairs_to_container(&[(4, 0.5)])
            .expect_err("expected bounds check failure");
        assert!(err.contains("out of bounds"));
    }
}

