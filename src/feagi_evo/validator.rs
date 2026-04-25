use feagi_evolutionary::{
    load_genome_from_json, load_genome_with_report, save_genome_to_json, validate_genome,
    validator::auto_fix_genome,
};
use pyo3::prelude::*;

/// Validation result returned to Python
#[pyclass]
#[derive(Clone)]
pub struct PyValidationResult {
    #[pyo3(get)]
    pub valid: bool,
    #[pyo3(get)]
    pub errors: Vec<String>,
    #[pyo3(get)]
    pub warnings: Vec<String>,
}

#[pymethods]
impl PyValidationResult {
    fn __repr__(&self) -> String {
        format!(
            "ValidationResult(valid={}, errors={}, warnings={})",
            self.valid,
            self.errors.len(),
            self.warnings.len()
        )
    }

    fn __str__(&self) -> String {
        let mut output = String::new();
        output.push_str(&format!("Valid: {}\n", self.valid));

        if !self.errors.is_empty() {
            output.push_str("\nErrors:\n");
            for error in &self.errors {
                output.push_str(&format!("  - {}\n", error));
            }
        }

        if !self.warnings.is_empty() {
            output.push_str("\nWarnings:\n");
            for warning in &self.warnings {
                output.push_str(&format!("  - {}\n", warning));
            }
        }

        output
    }
}

/// Validate a genome from JSON string
///
/// # Arguments
/// * `genome_json` - JSON string containing genome data (must follow FEAGI 2.0 format)
///
/// # Returns
/// * `PyValidationResult` - Validation result with errors and warnings
///
/// # Example
/// ```python
/// from feagi_rust_py_libs.genome import validate_genome
/// import json
///
/// genome = {
///     "version": "2.0",
///     "genome_id": "g-test123",
///     "blueprint": { ... },
///     "neuron_morphologies": { ... },
///     "physiology": { ... }
/// }
///
/// result = validate_genome(json.dumps(genome))
/// if not result.valid:
///     for error in result.errors:
///         print(f"ERROR: {error}")
/// ```
#[pyfunction]
#[pyo3(name = "validate_genome")]
pub fn py_validate_genome(genome_json: &str) -> PyResult<PyValidationResult> {
    // Load genome from JSON
    let genome = load_genome_from_json(genome_json).map_err(|e| {
        PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
            "Failed to parse genome (invalid format): {}",
            e
        ))
    })?;

    // Validate
    let result = validate_genome(&genome);

    Ok(PyValidationResult {
        valid: result.valid,
        errors: result.errors,
        warnings: result.warnings,
    })
}

/// Auto-fix common genome issues (zero dimensions, missing physiology, etc.)
///
/// Takes a genome JSON string, fixes issues, and returns the fixed JSON string.
///
/// # Arguments
/// * `genome_json` - JSON string containing genome data
///
/// # Returns
/// * `tuple` - (fixed_json_string, num_fixes_applied)
///
/// # Example
/// ```python
/// from feagi_rust_py_libs.genome import auto_fix_genome
/// import json
///
/// genome = { ... }  # Genome with issues
/// fixed_json, fixes_applied = auto_fix_genome(json.dumps(genome))
/// genome = json.loads(fixed_json)
/// print(f"Applied {fixes_applied} automatic fixes")
/// ```
#[pyfunction]
#[pyo3(name = "auto_fix_genome")]
pub fn py_auto_fix_genome(genome_json: &str) -> PyResult<(String, usize)> {
    // Load genome from JSON
    let mut genome = load_genome_from_json(genome_json).map_err(|e| {
        PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Failed to parse genome: {}", e))
    })?;

    // Apply auto-fixes
    let fixes_applied = auto_fix_genome(&mut genome);

    // Convert back to JSON
    let fixed_json = save_genome_to_json(&genome).map_err(|e| {
        PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!(
            "Failed to serialize fixed genome: {}",
            e
        ))
    })?;

    Ok((fixed_json, fixes_applied))
}

// ============================================================================
// New schema-versioning aware API (feagi-evolutionary ChainResult bindings).
//
// The legacy `validate_genome` / `auto_fix_genome` functions above operate
// on the already-parsed `RuntimeGenome`, which loses the raw chain report
// (which migrators ran, which normalizers ran, blocking vs advisory
// diagnostics). The new `validate_and_repair_genome` function below
// exposes the full chain report directly from `load_genome_with_report`
// so consumers (nrs-composer, feagi-desktop) can surface the exact
// information the loader saw to users.
// ============================================================================

/// Diagnostics from a single migrator hop (`vN -> vN+1`).
#[pyclass]
#[derive(Clone)]
pub struct PyMigrationStepDiagnostics {
    #[pyo3(get)]
    pub from_version: u32,
    #[pyo3(get)]
    pub to_version: u32,
    #[pyo3(get)]
    pub transformations: Vec<String>,
}

#[pymethods]
impl PyMigrationStepDiagnostics {
    fn __repr__(&self) -> String {
        format!(
            "MigrationStepDiagnostics(from_version={}, to_version={}, transformations={})",
            self.from_version,
            self.to_version,
            self.transformations.len()
        )
    }
}

/// Diagnostics from a single normalizer run (in-version cleanup).
#[pyclass]
#[derive(Clone)]
pub struct PyNormalizationDiagnostics {
    #[pyo3(get)]
    pub schema_version: u32,
    #[pyo3(get)]
    pub transformations: Vec<String>,
}

#[pymethods]
impl PyNormalizationDiagnostics {
    fn __repr__(&self) -> String {
        format!(
            "NormalizationDiagnostics(schema_version={}, transformations={})",
            self.schema_version,
            self.transformations.len()
        )
    }
}

/// Full chain report from `validate_and_repair_genome`.
#[pyclass]
#[derive(Clone)]
pub struct PyChainResult {
    #[pyo3(get)]
    pub from_version: u32,
    #[pyo3(get)]
    pub to_version: u32,
    #[pyo3(get)]
    pub migrators_applied: Vec<String>,
    #[pyo3(get)]
    pub normalizers_applied: Vec<String>,
    #[pyo3(get)]
    pub per_step_diagnostics: Vec<PyMigrationStepDiagnostics>,
    #[pyo3(get)]
    pub per_normalizer_diagnostics: Vec<PyNormalizationDiagnostics>,
    #[pyo3(get)]
    pub advisory_warnings: Vec<String>,
    #[pyo3(get)]
    pub blocking_errors: Vec<String>,
}

#[pymethods]
impl PyChainResult {
    /// True when the final-version validator reported zero errors.
    /// Consumers typically feed this bit into `genome_validity` in their
    /// health/UI surface.
    #[getter]
    fn is_blocking_clean(&self) -> bool {
        self.blocking_errors.is_empty()
    }

    fn __repr__(&self) -> String {
        format!(
            "ChainResult(from=v{}, to=v{}, migrators={:?}, normalizers={:?}, advisories={}, blocking={})",
            self.from_version,
            self.to_version,
            self.migrators_applied,
            self.normalizers_applied,
            self.advisory_warnings.len(),
            self.blocking_errors.len()
        )
    }
}

/// Validate and repair a genome, returning the repaired JSON alongside
/// the full chain report.
///
/// Policy: the library does **not** turn validation errors into
/// exceptions. The function returns successfully with a repaired genome
/// and a `PyChainResult`. `blocking_errors` is non-empty when the latest
/// validator found issues; consumers decide whether to surface them to
/// the user, load in degraded mode, or reject the load entirely. Only
/// hard I/O / JSON parse / structural failures raise `PyValueError`.
///
/// # Arguments
/// * `genome_json` - JSON string containing genome data.
///
/// # Returns
/// * `(repaired_json: str, report: PyChainResult)`
///
/// # Example
/// ```python
/// from feagi_rust_py_libs.genome import validate_and_repair_genome
/// repaired_json, report = validate_and_repair_genome(original_json)
/// if not report.is_blocking_clean:
///     for err in report.blocking_errors:
///         print(f"BLOCKING: {err}")
/// ```
#[pyfunction]
#[pyo3(name = "validate_and_repair_genome")]
pub fn py_validate_and_repair_genome(genome_json: &str) -> PyResult<(String, PyChainResult)> {
    let (genome, report) = load_genome_with_report(genome_json).map_err(|e| {
        PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
            "Failed to parse genome (invalid format): {}",
            e
        ))
    })?;

    let repaired_json = save_genome_to_json(&genome).map_err(|e| {
        PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!(
            "Failed to serialize repaired genome: {}",
            e
        ))
    })?;

    let per_step_diagnostics = report
        .per_step_diagnostics
        .iter()
        .map(|d| PyMigrationStepDiagnostics {
            from_version: d.from_version.as_u32(),
            to_version: d.to_version.as_u32(),
            transformations: d.transformations.clone(),
        })
        .collect();

    let per_normalizer_diagnostics = report
        .per_normalizer_diagnostics
        .iter()
        .map(|d| PyNormalizationDiagnostics {
            schema_version: d.schema_version.as_u32(),
            transformations: d.transformations.clone(),
        })
        .collect();

    let py_report = PyChainResult {
        from_version: report.from_version.as_u32(),
        to_version: report.to_version.as_u32(),
        migrators_applied: report
            .migrators_applied
            .iter()
            .map(|s| s.to_string())
            .collect(),
        normalizers_applied: report
            .normalizers_applied
            .iter()
            .map(|s| s.to_string())
            .collect(),
        per_step_diagnostics,
        per_normalizer_diagnostics,
        advisory_warnings: report.advisory_warnings.clone(),
        blocking_errors: report.blocking_errors.clone(),
    };

    Ok((repaired_json, py_report))
}

/// Register the genome validation module with Python
pub fn register_module(py: Python, parent_module: &Bound<'_, PyModule>) -> PyResult<()> {
    let genome_module = PyModule::new(py, "genome")?;

    genome_module.add_function(wrap_pyfunction!(py_validate_genome, &genome_module)?)?;
    genome_module.add_function(wrap_pyfunction!(py_auto_fix_genome, &genome_module)?)?;
    genome_module.add_function(wrap_pyfunction!(
        py_validate_and_repair_genome,
        &genome_module
    )?)?;
    genome_module.add_class::<PyValidationResult>()?;
    genome_module.add_class::<PyMigrationStepDiagnostics>()?;
    genome_module.add_class::<PyNormalizationDiagnostics>()?;
    genome_module.add_class::<PyChainResult>()?;

    parent_module.add_submodule(&genome_module)?;

    Ok(())
}
