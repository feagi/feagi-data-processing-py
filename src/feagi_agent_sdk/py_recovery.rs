//! PyO3 wrappers for `feagi_agent::clients::recovery`.
//!
//! Every primitive here is a thin shim over the Rust source of truth.
//! No decision logic, transition detection, or threshold handling lives
//! in this file: that all happens in the underlying Rust types so all
//! SDKs (Python, Rust, future Java) share a single behavioral spec.

use feagi_agent::clients::recovery::{
    fetch_health_snapshot_blocking, HealthEvent, HealthFetchConfig, HealthSnapshot, HealthWatcher,
    ReconnectDecision, ReconnectPolicy, ReconnectPolicyConfig, RecoveryTrigger,
};
use pyo3::prelude::*;
use pyo3::types::PyList;
use std::time::Duration;

/// Python-facing snapshot of FEAGI health.
///
/// Field semantics follow [`feagi_agent::clients::recovery::HealthSnapshot`].
#[pyclass(name = "FeagiHealthSnapshot")]
#[derive(Clone)]
pub struct PyHealthSnapshot {
    pub(crate) inner: HealthSnapshot,
}

#[pymethods]
impl PyHealthSnapshot {
    #[new]
    #[pyo3(signature = (
        feagi_session,
        genome_num,
        genome_timestamp,
        genome_loading,
        genome_availability,
        brain_readiness,
    ))]
    fn new(
        feagi_session: Option<i64>,
        genome_num: Option<i32>,
        genome_timestamp: Option<i64>,
        genome_loading: bool,
        genome_availability: bool,
        brain_readiness: bool,
    ) -> Self {
        Self {
            inner: HealthSnapshot {
                feagi_session,
                genome_num,
                genome_timestamp,
                genome_loading,
                genome_availability,
                brain_readiness,
            },
        }
    }

    #[getter]
    fn feagi_session(&self) -> Option<i64> {
        self.inner.feagi_session
    }

    #[getter]
    fn genome_num(&self) -> Option<i32> {
        self.inner.genome_num
    }

    #[getter]
    fn genome_timestamp(&self) -> Option<i64> {
        self.inner.genome_timestamp
    }

    #[getter]
    fn genome_loading(&self) -> bool {
        self.inner.genome_loading
    }

    #[getter]
    fn genome_availability(&self) -> bool {
        self.inner.genome_availability
    }

    #[getter]
    fn brain_readiness(&self) -> bool {
        self.inner.brain_readiness
    }

    fn __repr__(&self) -> String {
        format!(
            "FeagiHealthSnapshot(session={:?}, genome_num={:?}, ts={:?}, loading={}, avail={}, ready={})",
            self.inner.feagi_session,
            self.inner.genome_num,
            self.inner.genome_timestamp,
            self.inner.genome_loading,
            self.inner.genome_availability,
            self.inner.brain_readiness,
        )
    }
}

/// Python-facing FEAGI health event.
///
/// Mirrors [`HealthEvent`] but is exposed as a single class with a
/// `kind` discriminator string so PyO3's enum-marshaling restrictions do
/// not force callers to learn variant subtypes. The mapping is:
/// - `feagi_unreachable`
/// - `feagi_back_online`
/// - `session_changed` -> `old_session`, `new_session`
/// - `genome_changed`  -> `old_genome_num`, `new_genome_num`,
///                        `old_genome_timestamp`, `new_genome_timestamp`
/// - `genome_load_started`
/// - `genome_load_completed`
/// - `brain_ready`
/// - `brain_lost`
#[pyclass(name = "FeagiHealthEvent")]
#[derive(Clone)]
pub struct PyHealthEvent {
    pub(crate) inner: HealthEvent,
}

#[pymethods]
impl PyHealthEvent {
    #[getter]
    fn kind(&self) -> &'static str {
        match self.inner {
            HealthEvent::FeagiUnreachable => "feagi_unreachable",
            HealthEvent::FeagiBackOnline => "feagi_back_online",
            HealthEvent::SessionChanged { .. } => "session_changed",
            HealthEvent::GenomeChanged { .. } => "genome_changed",
            HealthEvent::GenomeLoadStarted => "genome_load_started",
            HealthEvent::GenomeLoadCompleted => "genome_load_completed",
            HealthEvent::BrainReady => "brain_ready",
            HealthEvent::BrainLost => "brain_lost",
        }
    }

    #[getter]
    fn old_session(&self) -> Option<i64> {
        if let HealthEvent::SessionChanged { old, .. } = self.inner {
            Some(old)
        } else {
            None
        }
    }

    #[getter]
    fn new_session(&self) -> Option<i64> {
        if let HealthEvent::SessionChanged { new, .. } = self.inner {
            Some(new)
        } else {
            None
        }
    }

    #[getter]
    fn old_genome_num(&self) -> Option<i32> {
        if let HealthEvent::GenomeChanged { old_num, .. } = self.inner {
            old_num
        } else {
            None
        }
    }

    #[getter]
    fn new_genome_num(&self) -> Option<i32> {
        if let HealthEvent::GenomeChanged { new_num, .. } = self.inner {
            new_num
        } else {
            None
        }
    }

    #[getter]
    fn old_genome_timestamp(&self) -> Option<i64> {
        if let HealthEvent::GenomeChanged { old_timestamp, .. } = self.inner {
            old_timestamp
        } else {
            None
        }
    }

    #[getter]
    fn new_genome_timestamp(&self) -> Option<i64> {
        if let HealthEvent::GenomeChanged { new_timestamp, .. } = self.inner {
            new_timestamp
        } else {
            None
        }
    }

    fn __repr__(&self) -> String {
        format!("FeagiHealthEvent({:?})", self.inner)
    }
}

/// Diff successive [`PyHealthSnapshot`] observations into transition events.
#[pyclass(name = "FeagiHealthWatcher")]
pub struct PyHealthWatcher {
    inner: HealthWatcher,
}

#[pymethods]
impl PyHealthWatcher {
    #[new]
    fn new() -> Self {
        Self {
            inner: HealthWatcher::new(),
        }
    }

    /// Record a successful snapshot fetch and return any transitions.
    fn observe(&mut self, py: Python<'_>, snapshot: &PyHealthSnapshot) -> PyResult<Py<PyList>> {
        let events = self.inner.observe(snapshot.inner.clone());
        events_to_pylist(py, events)
    }

    /// Record that a snapshot fetch failed.
    fn observe_unreachable(&mut self, py: Python<'_>) -> PyResult<Py<PyList>> {
        let events = self.inner.observe_unreachable();
        events_to_pylist(py, events)
    }

    fn has_baseline(&self) -> bool {
        self.inner.has_baseline()
    }

    fn is_reachable(&self) -> bool {
        self.inner.is_reachable()
    }

    fn last_snapshot(&self) -> Option<PyHealthSnapshot> {
        self.inner
            .last_snapshot()
            .cloned()
            .map(|inner| PyHealthSnapshot { inner })
    }
}

fn events_to_pylist(py: Python<'_>, events: Vec<HealthEvent>) -> PyResult<Py<PyList>> {
    let wrapped: Vec<Py<PyHealthEvent>> = events
        .into_iter()
        .map(|inner| Py::new(py, PyHealthEvent { inner }))
        .collect::<PyResult<_>>()?;
    let list = PyList::new(py, wrapped)?;
    Ok(list.unbind())
}

/// Python-facing recovery trigger. Construct with the class methods
/// `health(event)` and `transport_send_failed()` rather than directly.
#[pyclass(name = "FeagiRecoveryTrigger")]
#[derive(Clone)]
pub struct PyRecoveryTrigger {
    pub(crate) inner: RecoveryTrigger,
}

#[pymethods]
impl PyRecoveryTrigger {
    #[staticmethod]
    fn health(event: &PyHealthEvent) -> Self {
        Self {
            inner: RecoveryTrigger::Health(event.inner.clone()),
        }
    }

    #[staticmethod]
    fn transport_send_failed() -> Self {
        Self {
            inner: RecoveryTrigger::TransportSendFailed,
        }
    }

    fn __repr__(&self) -> String {
        format!("FeagiRecoveryTrigger({:?})", self.inner)
    }
}

/// Caller-supplied configuration for [`PyReconnectPolicy`].
///
/// Every field is required at construction. The Python SDK does not
/// substitute defaults, matching the central-configuration policy.
#[pyclass(name = "FeagiReconnectPolicyConfig")]
#[derive(Clone)]
pub struct PyReconnectPolicyConfig {
    pub(crate) inner: ReconnectPolicyConfig,
}

#[pymethods]
impl PyReconnectPolicyConfig {
    #[new]
    #[pyo3(signature = (
        cooldown_ms,
        max_consecutive_failures,
        trigger_on_session_changed,
        trigger_on_genome_changed,
        trigger_on_genome_load_completed,
        trigger_on_back_online,
        trigger_on_brain_ready,
        trigger_on_transport_send_failed,
    ))]
    #[allow(clippy::too_many_arguments)]
    fn new(
        cooldown_ms: u64,
        max_consecutive_failures: u32,
        trigger_on_session_changed: bool,
        trigger_on_genome_changed: bool,
        trigger_on_genome_load_completed: bool,
        trigger_on_back_online: bool,
        trigger_on_brain_ready: bool,
        trigger_on_transport_send_failed: bool,
    ) -> Self {
        Self {
            inner: ReconnectPolicyConfig {
                cooldown_ms,
                max_consecutive_failures,
                trigger_on_session_changed,
                trigger_on_genome_changed,
                trigger_on_genome_load_completed,
                trigger_on_back_online,
                trigger_on_brain_ready,
                trigger_on_transport_send_failed,
            },
        }
    }

    fn __repr__(&self) -> String {
        format!("{:?}", self.inner)
    }
}

/// Python-facing decision returned by [`PyReconnectPolicy::decide`].
#[pyclass(name = "FeagiReconnectDecision")]
#[derive(Clone)]
pub struct PyReconnectDecision {
    inner: ReconnectDecision,
}

#[pymethods]
impl PyReconnectDecision {
    #[getter]
    fn kind(&self) -> &'static str {
        match self.inner {
            ReconnectDecision::Skip => "skip",
            ReconnectDecision::AttemptNow { .. } => "attempt_now",
            ReconnectDecision::RetryAfter { .. } => "retry_after",
            ReconnectDecision::GiveUp { .. } => "give_up",
        }
    }

    #[getter]
    fn reason(&self) -> Option<String> {
        match &self.inner {
            ReconnectDecision::AttemptNow { reason }
            | ReconnectDecision::RetryAfter { reason, .. } => Some(reason.clone()),
            _ => None,
        }
    }

    #[getter]
    fn wait_ms(&self) -> Option<u64> {
        if let ReconnectDecision::RetryAfter { wait_ms, .. } = self.inner {
            Some(wait_ms)
        } else {
            None
        }
    }

    #[getter]
    fn consecutive_failures(&self) -> Option<u32> {
        if let ReconnectDecision::GiveUp {
            consecutive_failures,
        } = self.inner
        {
            Some(consecutive_failures)
        } else {
            None
        }
    }

    fn is_attempt_now(&self) -> bool {
        matches!(self.inner, ReconnectDecision::AttemptNow { .. })
    }

    fn is_skip(&self) -> bool {
        matches!(self.inner, ReconnectDecision::Skip)
    }

    fn is_retry_after(&self) -> bool {
        matches!(self.inner, ReconnectDecision::RetryAfter { .. })
    }

    fn is_give_up(&self) -> bool {
        matches!(self.inner, ReconnectDecision::GiveUp { .. })
    }

    fn __repr__(&self) -> String {
        format!("{:?}", self.inner)
    }
}

/// Python-facing reconnect decision policy.
#[pyclass(name = "FeagiReconnectPolicy")]
pub struct PyReconnectPolicy {
    inner: ReconnectPolicy,
}

#[pymethods]
impl PyReconnectPolicy {
    #[new]
    fn new(config: &PyReconnectPolicyConfig) -> Self {
        Self {
            inner: ReconnectPolicy::new(config.inner.clone()),
        }
    }

    fn decide(&mut self, triggers: Vec<PyRecoveryTrigger>, now_ms: u64) -> PyReconnectDecision {
        let triggers: Vec<RecoveryTrigger> = triggers.into_iter().map(|t| t.inner).collect();
        PyReconnectDecision {
            inner: self.inner.decide(&triggers, now_ms),
        }
    }

    fn record_attempt_succeeded(&mut self) {
        self.inner.record_attempt_succeeded();
    }

    fn record_attempt_failed(&mut self) {
        self.inner.record_attempt_failed();
    }

    fn reset(&mut self) {
        self.inner.reset();
    }

    #[getter]
    fn consecutive_failures(&self) -> u32 {
        self.inner.consecutive_failures()
    }

    #[getter]
    fn last_attempt_at_ms(&self) -> Option<u64> {
        self.inner.last_attempt_at_ms()
    }

    fn __repr__(&self) -> String {
        format!(
            "FeagiReconnectPolicy(consecutive_failures={})",
            self.inner.consecutive_failures()
        )
    }
}

/// Caller-supplied HTTP configuration for [`fetch_health_snapshot`].
#[pyclass(name = "FeagiHealthFetchConfig")]
#[derive(Clone)]
pub struct PyHealthFetchConfig {
    feagi_api_host: String,
    feagi_api_port: u16,
    timeout_ms: u64,
}

#[pymethods]
impl PyHealthFetchConfig {
    #[new]
    fn new(feagi_api_host: String, feagi_api_port: u16, timeout_ms: u64) -> Self {
        Self {
            feagi_api_host,
            feagi_api_port,
            timeout_ms,
        }
    }

    fn __repr__(&self) -> String {
        format!(
            "FeagiHealthFetchConfig(host={}, port={}, timeout_ms={})",
            self.feagi_api_host, self.feagi_api_port, self.timeout_ms
        )
    }
}

/// Module-level: blocking fetch of a FEAGI health snapshot.
///
/// Returns a `FeagiHealthSnapshot` on success. Raises `RuntimeError` on
/// any HTTP failure (network down, parse failure, non-2xx). The watcher
/// distinguishes "fetch succeeded" from "fetch failed" via separate
/// `observe` and `observe_unreachable` calls, so callers should catch
/// here and forward to `observe_unreachable`.
#[pyfunction]
pub fn fetch_health_snapshot(config: &PyHealthFetchConfig) -> PyResult<PyHealthSnapshot> {
    let fetch_config = HealthFetchConfig::new(
        &config.feagi_api_host,
        config.feagi_api_port,
        Duration::from_millis(config.timeout_ms),
    );
    let snapshot = fetch_health_snapshot_blocking(&fetch_config)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;
    Ok(PyHealthSnapshot { inner: snapshot })
}

/// Register the recovery primitives with the existing `feagi_agent`
/// Python module.
pub fn register(submodule: &Bound<'_, PyModule>) -> PyResult<()> {
    submodule.add_class::<PyHealthSnapshot>()?;
    submodule.add_class::<PyHealthEvent>()?;
    submodule.add_class::<PyHealthWatcher>()?;
    submodule.add_class::<PyRecoveryTrigger>()?;
    submodule.add_class::<PyReconnectPolicyConfig>()?;
    submodule.add_class::<PyReconnectDecision>()?;
    submodule.add_class::<PyReconnectPolicy>()?;
    submodule.add_class::<PyHealthFetchConfig>()?;
    submodule.add_function(wrap_pyfunction!(fetch_health_snapshot, submodule)?)?;
    Ok(())
}
