
// Note: While you can divide up derive blocks in multiple lines, you cannot with pyclass blocks

// TODO instead of having so many macros, we should be having some sort of configuration into one

/// Takes the Pyclass internal name, and the rust type, to crate a basic
/// wrapper of the rust struct as inner
#[macro_export]
macro_rules! create_pyclass {
    ($py_wrapped_name:ident, $rust_name:ty, $py_name:expr) => {
        #[pyclass(str)]
        #[pyo3(name = $py_name)]
        #[derive(Debug, Clone)]
        pub struct $py_wrapped_name {
            pub inner: $rust_name,
        }

        __base_py_class_shared!($py_wrapped_name, $rust_name, $py_name);

        impl From<&$rust_name> for $py_wrapped_name {
            fn from(reference: &$rust_name) -> Self {
                $py_wrapped_name { inner: reference.clone() }
            }
        }

        impl $py_wrapped_name {
            pub fn copy_out_from_bound(bounded: &pyo3::Bound< $py_wrapped_name>) -> $rust_name { // needs clone
                bounded.borrow().inner.clone()
            }
        }
    };
}

/// Takes the Pyclass internal name, and the rust type, to crate a basic
/// wrapper of the rust struct as inner. allows comparison if equal
#[macro_export]
macro_rules! create_pyclass_with_equal {
    ($py_wrapped_name:ident, $rust_name:ty, $py_name:expr) => {
        #[pyclass(str, eq)]
        #[pyo3(name = $py_name)]
        #[derive(Debug, Clone)]

        pub struct $py_wrapped_name {
            pub inner: $rust_name,
        }

        impl From<&$rust_name> for $py_wrapped_name {
            fn from(inner: &$rust_name) -> Self {
                $py_wrapped_name { inner.clone() }
            }
        }

         __base_py_class_shared!($py_wrapped_name, $rust_name, $py_name);
    };
}

/// Takes the Pyclass internal name, and the rust type, to crate a basic
/// wrapper of the rust struct as inner. Allows Hashing
#[macro_export]
macro_rules! create_pyclass_with_hash {
    ($py_wrapped_name:ident, $rust_name:ty, $py_name:expr) => {
        #[pyclass(str, hash)]
        #[pyo3(name = $py_name)]
        #[derive(Debug, Clone)]
        pub struct $py_wrapped_name {
            pub inner: $rust_name,
        }

        impl From<&$rust_name> for $py_wrapped_name {
            fn from(inner: &$rust_name) -> Self {
                $py_wrapped_name { inner.clone() }
            }
        }

         __base_py_class_shared!($py_wrapped_name, $rust_name, $py_name);
    };
}

/// Takes the Pyclass internal name, and the rust type, to crate a basic
/// wrapper of the rust struct as inner. Uniquely lacks clone support, limiting its uses
#[macro_export]
macro_rules! create_pyclass_no_clone {
    ($py_wrapped_name:ident, $rust_name:ty, $py_name:expr) => {

        #[pyclass(str)]
        #[pyo3(name = $py_name)]
        #[derive(Debug)]
        pub struct $py_wrapped_name {
            pub inner: $rust_name,
        }

        __base_py_class_shared!($py_wrapped_name, $rust_name, $py_name);
    };
}

/// Like create_pyclass_no_clone but for inner types that are not Sync, Display, or Debug.
/// Uses #[pyclass(str, unsendable)] and a fixed display string (the py_name).
#[macro_export]
macro_rules! create_pyclass_no_clone_unsendable {
    ($py_wrapped_name:ident, $rust_name:ty, $py_name:expr) => {
        #[pyclass(str, unsendable)]
        #[pyo3(name = $py_name)]
        pub struct $py_wrapped_name {
            pub inner: $rust_name,
        }

        impl std::fmt::Debug for $py_wrapped_name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", $py_name)
            }
        }

        impl std::fmt::Display for $py_wrapped_name {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(f, "{}", $py_name)
            }
        }

        impl From<$rust_name> for $py_wrapped_name {
            fn from(inner: $rust_name) -> Self {
                $py_wrapped_name { inner }
            }
        }

        impl From<$py_wrapped_name> for $rust_name {
            fn from(py: $py_wrapped_name) -> Self {
                py.inner
            }
        }

        impl $py_wrapped_name {
            #[allow(dead_code)]
            pub(crate) fn new_from_rust(rust_struct: $rust_name) -> Self {
                $py_wrapped_name { inner: rust_struct }
            }

            pub fn wrap_to_bound_any(py: pyo3::Python<'_>, rust_struct: $rust_name) -> pyo3::PyResult<pyo3::Bound<'_, pyo3::PyAny>> {
                pyo3::Bound::new(py, Self { inner: rust_struct }).map(|b| b.into_any())
            }

            pub fn wrap_self_into_bound_any(self, py: pyo3::Python<'_>) -> pyo3::PyResult<pyo3::Bound<'_, pyo3::PyAny>> {
                pyo3::Bound::new(py, self).map(|b| b.into_any())
            }

            pub fn try_extract_from_bound_any<'py>(obj: pyo3::Bound<'py, pyo3::PyAny>) -> Result<pyo3::PyRef<'py, Self>, feagi_data_structures::FeagiDataError> {
                use pyo3::prelude::PyAnyMethods;
                use feagi_data_structures::FeagiDataError;
                obj.cast::<Self>()
                    .map_err(|_| FeagiDataError::BadParameters(format!("Expected {} but got {:?}", $py_name, obj.get_type())))?
                    .try_borrow()
                    .map_err(|e| FeagiDataError::BadParameters(format!("Failed to borrow: {}", e)))
            }
        }

        #[pyo3::pymethods]
        impl $py_wrapped_name {
            fn as_any<'py>(slf: pyo3::Bound<'py, Self>) -> pyo3::Py<pyo3::PyAny> {
                slf.unbind().into_any()
            }
        }
    };
}


// NOTE: technically #[macro_export] is required for visibility
/// Shared implementation of base py classes
#[macro_export]
macro_rules! __base_py_class_shared {
    ($py_wrapped_name:ident, $rust_name:ty, $py_name:expr) => {
        // Require print support
        impl std::fmt::Display for $py_wrapped_name {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(f, "{}", self.inner.to_string())
            }
        }

        impl From<$rust_name> for $py_wrapped_name {
            fn from(inner: $rust_name) -> Self {
                $py_wrapped_name { inner }
            }
        }

        impl From<$py_wrapped_name> for $rust_name {
            fn from(py: $py_wrapped_name) -> Self {
                py.inner
            }
        }


        impl $py_wrapped_name {
            /// Create Python wrapped instance of the given Rust structure
            #[allow(dead_code)]
            pub(crate) fn new_from_rust(rust_struct: $rust_name) -> Self {
                $py_wrapped_name {inner: rust_struct}
            }

            /*
            pub fn wrap_to_py_any(py: Python<'_>, rust_struct: $rust_name) -> PyResult<Py<PyAny>> {
                let obj: Bound<'_, PyAny> =  Py::new(py, Self {inner: rust_struct})?.bind(py);
                Ok(obj.unbind())
            }
             */
            /// Static wrapping an existing rust non-py wrapped instance directly to PyAny. Used only in specific contexts
            pub fn wrap_to_bound_any(py: Python<'_>, rust_struct: $rust_name) -> PyResult<Bound<'_, PyAny>> {
                Bound::new(py, Self {inner: rust_struct} ).map(|b| b.into_any())
            }

            /// Wraps self into into a Bound<PyAny>. Used only in specific contexts
            pub fn wrap_self_into_bound_any(self, py: Python<'_>) -> PyResult<Bound<'_, PyAny>> {
                Bound::new(py, self).map(|b| b.into_any())
            }

            /// Attempt to downcast as a reference to the python wrapper.
            /// Use &ref.inner to borrow the rust data.
            /// Note that doing if-elif-else chains will have to be the main way to get out a type from multiple.
            pub fn try_extract_from_bound_any<'py>(obj: pyo3::Bound<'py, pyo3::PyAny>) -> Result<pyo3::PyRef<'py, Self>, feagi_data_structures::FeagiDataError> {
                use pyo3::prelude::PyAnyMethods;
                use feagi_data_structures::FeagiDataError;
                obj.cast::<Self>()
                    .map_err(|_| FeagiDataError::BadParameters(format!("Expected {} but got {:?}", $py_name, obj.get_type())))?
                    .try_borrow()
                    .map_err(|e| FeagiDataError::BadParameters(format!("Failed to borrow: {}", e)))
            }

        }

        #[pyo3::pymethods]
        impl $py_wrapped_name {
            fn as_any<'py>(slf: Bound<'py, Self>) -> Py<PyAny> {
                slf.unbind().into_any()
            }
        }
    };
}

