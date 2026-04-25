// TODO can we automate the creation of enum keys?

/// Allows easy wrapping of a flat enum
/// keys must be added as static methods to a pymethods block.
/// Add #[allow(non_snake_case)] to prevent linter warnings
/// Parameter 1: PyStruct name
/// Parameter 2: Wrapping Rust Struct Type
/// Parameter 3: Enum name as seen by python (str)
#[macro_export]
macro_rules! wrap_flat_enum {
    ($py_wrapped_name:ident, $rust_name:ty, $py_name:expr) => {
        #[pyclass(str, eq, hash, frozen)]
        #[pyo3(name = $py_name)]
        #[derive(Debug, Clone, PartialEq, Hash)]
        pub struct $py_wrapped_name {
            pub inner: $rust_name,
        }

        __base_py_class_shared!($py_wrapped_name, $rust_name, $py_name);

        impl $py_wrapped_name {
            pub fn from_bound(bound: &pyo3::Bound<$py_wrapped_name>) -> $rust_name {
                bound.get().inner
            }
        }
    };
}

// For now these are duplicates, however future automation may make a divergence of functionality

/// Allows easy wrapping of a flat enum
/// keys must be added as static methods to a pymethods block.
/// Add #[allow(non_snake_case)] to prevent linter warnings
/// Parameter 1: PyStruct name
/// Parameter 2: Wrapping Rust Struct Type
/// Parameter 3: Enum name as seen by python (str)
#[macro_export]
macro_rules! wrap_layered_enum {
    ($py_wrapped_name:ident, $rust_name:ty, $py_name:expr) => {
        //#[pyo3::pyclass(str, eq, hash, frozen)]
        #[pyo3::pyclass(str, eq)]
        #[pyo3(name = $py_name)]
        #[derive(Debug, Clone, PartialEq)]
        //#[derive(Hash)]
        pub struct $py_wrapped_name {
            pub inner: $rust_name,
        }

        __base_py_class_shared!($py_wrapped_name, $rust_name, $py_name);

        impl $py_wrapped_name {}
    };
}
