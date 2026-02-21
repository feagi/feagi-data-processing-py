use pyo3::prelude::*;
use feagi_evolutionary::{
    load_genome_from_json, save_genome_to_json, validate_genome,
    validator::auto_fix_genome,
};

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
pub fn py_auto_fix_genome(genome_json: &str) -> PyResult<(String, usize)> {
    // Load genome from JSON
    let mut genome = load_genome_from_json(genome_json).map_err(|e| {
        PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
            "Failed to parse genome: {}",
            e
        ))
    })?;
    
    // Apply auto-fixes
    let fixes_applied = auto_fix_genome(&mut genome);
    
    // Convert back to JSON
    let fixed_json = save_genome_to_json(&genome).map_err(|e| {
        PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("Failed to serialize fixed genome: {}", e))
    })?;
    
    Ok((fixed_json, fixes_applied))
}

/// Register the genome validation module with Python
pub fn register_module(py: Python, parent_module: &Bound<'_, PyModule>) -> PyResult<()> {
    let genome_module = PyModule::new(py, "genome")?;
    
    genome_module.add_function(wrap_pyfunction!(py_validate_genome, &genome_module)?)?;
    genome_module.add_function(wrap_pyfunction!(py_auto_fix_genome, &genome_module)?)?;
    genome_module.add_class::<PyValidationResult>()?;
    
    parent_module.add_submodule(&genome_module)?;
    
    Ok(())
}

