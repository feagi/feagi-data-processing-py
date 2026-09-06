use pyo3::prelude::*;
use pyo3::types::PyModule;

/// Validate and migrate a connectome artifact using FEAGI core.
///
/// Returns immutable migrated bytes and a JSON compatibility report. Invalid,
/// corrupt, future-version, or semantically inconsistent artifacts raise
/// `ValueError` and produce no output artifact.
#[pyfunction]
#[pyo3(name = "validate_and_migrate_connectome")]
pub fn py_validate_and_migrate_connectome(connectome_bytes: &[u8]) -> PyResult<(Vec<u8>, String)> {
    let result =
        feagi_services::brain_artifact::validate_and_migrate_brain_artifact(connectome_bytes)
            .map_err(|error| {
                PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
                    "Connectome validation failed: {error}"
                ))
            })?;
    let report_json = serde_json::to_string(&result.report).map_err(|error| {
        PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!(
            "Failed to serialize connectome report: {error}"
        ))
    })?;
    Ok((result.artifact_bytes, report_json))
}

/// Register the authoritative connectome compatibility module with Python.
pub fn register_module(py: Python, parent_module: &Bound<'_, PyModule>) -> PyResult<()> {
    let connectome_module = PyModule::new(py, "connectome")?;
    connectome_module.add_function(wrap_pyfunction!(
        py_validate_and_migrate_connectome,
        &connectome_module
    )?)?;
    parent_module.add_submodule(&connectome_module)?;
    Ok(())
}
