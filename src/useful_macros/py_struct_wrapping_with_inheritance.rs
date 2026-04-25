/// These implementations do not need the parent to store a shared state in a box (child st
//region No Box (Child stores data)
/// Creates a parent class for python, when exposing rust structs that share a trait.
/// Doesn't do much on its own and not intended to be instantiated directly, either in rust or python.
/// Parameter 1: str parent class name as visible in python (ParentClass)
/// Parameter 2: identity name of this struct as visible in rust (PyParentClass)
#[macro_export]
macro_rules! create_trait_parent_pyclass {
    ($parent_class_name_in_python:expr, $py_class_parent_name_in_rust:ident) => {
        #[pyo3::pyclass(str, subclass)]
        #[pyo3(name = $parent_class_name_in_python)]
        #[derive(Debug, Clone)]
        pub struct $py_class_parent_name_in_rust {}

        impl $py_class_parent_name_in_rust {
            pub(crate) fn new_blank_parent() -> Self {
                $py_class_parent_name_in_rust {}
            }
        }

        impl std::fmt::Display for $py_class_parent_name_in_rust {
            fn fmt(
                &self,
                f: &mut std::fmt::Formatter<'_>,
            ) -> std::result::Result<(), std::fmt::Error> {
                write!(f, "{}", $parent_class_name_in_python)
            }
        }
    };
}

/// Creates a child class for python, with a parent defined by create_trait_parent_pyclass.
/// Has private method python_new_child_constructor that creates a tuple of (child_py_struct, parent_py_struct)
/// which you must use for your new function. All other constructors are to use python_etc_child_constructor and return
/// PyResult<Py<Self>>
/// Parameter 1: Identity Parent struct name as defined in rust (PyParentClass)
/// Parameter 2: Identity Child struct name (PyChildClass)
/// Parameter 3: str Child struct name (ChildClass)
/// Parameter 4: type representing rust type in inner for this py wrapper (RustStruct)
#[macro_export]
macro_rules! create_trait_child_pyclass {
    ($parent_pyclass_in_rust:ident, $py_class_name_in_rust:ident, $class_name_in_python_str:expr, $representing_rust_struct:ty) => {
        #[pyo3::pyclass(str, extends=$parent_pyclass_in_rust)]
        #[derive(Debug, Clone)]
        #[pyo3(name = $class_name_in_python_str)]
        pub struct $py_class_name_in_rust {
            pub inner: $representing_rust_struct,
        }

        // Require print support
        impl std::fmt::Display for $py_class_name_in_rust {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(f, "{}", self.inner.to_string())
            }
        }

        impl From<$representing_rust_struct> for $py_class_name_in_rust {
            fn from(rust_struct: $representing_rust_struct) -> Self {
                $py_class_name_in_rust { inner: rust_struct }
            }
        }

        impl From<$py_class_name_in_rust> for $representing_rust_struct {
            fn from(py: $py_class_name_in_rust) -> Self {
                py.inner
            }
        }

        impl $py_class_name_in_rust {
            /// You MUST use this for the new constructor, and ONLY for that usecase!. Your "new" must use this and return (PyChild, PyParent)
            fn python_new_child_constructor(
                child_rust_struct: $representing_rust_struct,
            ) -> (Self, $parent_pyclass_in_rust) {
                (
                    $py_class_name_in_rust {
                        inner: child_rust_struct,
                    },
                    $parent_pyclass_in_rust::new_blank_parent(),
                )
            }

            /// You MUST use for all other constructors except for "new"
            pub fn python_etc_child_constructor<'py>(
                _py: Python<'py>,
                child_rust_struct: $representing_rust_struct,
            ) -> PyResult<Py<Self>> {
                Python::attach(|py| {
                    Py::new(
                        py,
                        (
                            $py_class_name_in_rust {
                                inner: child_rust_struct,
                            },
                            $parent_pyclass_in_rust::new_blank_parent(),
                        ),
                    ) // TODO this is outdated
                })
            }

            /// Use this if you want to export this py-wrapped struct to python with proper inheritance
            #[allow(dead_code)]
            fn export_as_python_child<'py>(self, _py: Python<'py>) -> PyResult<Py<Self>> {
                Python::attach(|py| {
                    Py::new(py, (self, $parent_pyclass_in_rust::new_blank_parent())) // TODO this is outdated
                })
            }
        }
    };
}
//endregion

/// This implementation assumes the parent is a rust trait, implemented in python as a Box dyn. "clone_box" must be defined!
/// Parameter 1: str parent class name as visible in python ("ParentClass")
/// Parameter 2: identity name of this struct as visible in rust (PyParentClass)
/// Parameter 3: identity of the boxed rust trait type (RustTrait)
/// Parameter 4: list of (PyWrappedStruct, RustConcreteStruct) pairs for from_box_to_correct_child conversion
///
/// Example:
/// ```ignore
/// create_trait_parent_with_box_pyclass!(
///     "PipelineStageProperties",
///     PyPipelineStageProperties,
///     PipelineStageProperties,
///     [(PyImageQuickDiffStageProperties, ImageQuickDiffStageProperties),
///      (PyImageSegmentatorStageProperties, ImageFrameSegmentatorStageProperties)]
/// );
/// ```
//region Shared Parent Data
#[macro_export]
macro_rules! create_trait_parent_with_box_pyclass {
    ($parent_class_name_in_python_str:expr, $py_class_parent_name_in_rust:ident, $boxed_rust_type:ident, [$(($py_wrapped:ident, $rust_struct:ident)),* $(,)?]) => {

        #[pyo3::pyclass(str, subclass)]
        #[pyo3(name = $parent_class_name_in_python_str)]
        #[derive(Debug)]
        pub struct $py_class_parent_name_in_rust {
            pub inner: Box<dyn $boxed_rust_type + Send + Sync>,
        }

        // Manual implementation
        impl Clone for $py_class_parent_name_in_rust {
            fn clone(&self) -> Self {
                Self {
                    inner: self.inner.clone_box(),
                }
            }
        }

        // Require print support
        impl std::fmt::Display for $py_class_parent_name_in_rust {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
                write!(f, "{}", $parent_class_name_in_python_str)
            }
        }

        impl $py_class_parent_name_in_rust {
            // Do not allow direct instantiation from Python
            pub(crate) fn new_parent(boxed: Box<dyn $boxed_rust_type + Send + Sync>) -> Self {
                $py_class_parent_name_in_rust {
                    inner: boxed,
                }
            }

            pub(crate) fn py_any_to_box<'py>(_py: Python<'_>, py_any: &pyo3::Bound<'py, pyo3::PyAny>) -> Result<Box<dyn $boxed_rust_type + Send + Sync>, feagi_data_structures::FeagiDataError> {
                if let Ok(reference) = py_any.cast::<( $py_class_parent_name_in_rust )>() {
                    return Ok(reference.borrow().inner.clone_box());
                }
                Err(feagi_data_structures::FeagiDataError::BadParameters(format!("Unable to parse object as any child of {}!", $parent_class_name_in_python_str)))
            }

            /// Converts a boxed trait object to the correct Python child class.
            /// This is needed when returning stage properties from Rust to Python,
            /// ensuring the correct subclass type is returned rather than just the parent.
            pub(crate) fn from_box_to_correct_child(py: pyo3::Python<'_>, boxed: Box<dyn $boxed_rust_type + Send + Sync>) -> pyo3::PyResult<pyo3::Py<pyo3::PyAny>> {

                $(
                    if boxed.as_any().downcast_ref::<$rust_struct>().is_some() {
                        let (child, parent) = $py_wrapped::python_new_child_constructor(boxed);
                        return pyo3::Py::new(py, (child, parent)).map(|obj| obj.into_any());
                    }
                )*

                Err(feagi_data_structures::FeagiDataError::InternalError(
                    format!("Missing Definition for {} - unknown concrete type!", $parent_class_name_in_python_str)
                )).map_err(crate::py_error::PyFeagiError::from)?
            }

            /// Converts a vector of boxed trait objects to a vector of Python wrapped objects (as PyAny).
            /// Each element is converted to its correct Python child class.
            pub(crate) fn from_vec_box_to_vec_py(py: pyo3::Python<'_>, vec: Vec<Box<dyn $boxed_rust_type + Send + Sync>>) -> pyo3::PyResult<Vec<pyo3::Py<pyo3::PyAny>>> {
                vec.into_iter()
                    .map(|boxed| Self::from_box_to_correct_child(py, boxed))
                    .collect()
            }

            /// Converts a boxed trait object to a Python object typed as the parent class.
            /// Python will still see the actual child type - this just changes the Rust return type.
            pub(crate) fn from_box_to_parent_typed(py: pyo3::Python<'_>, boxed: Box<dyn $boxed_rust_type + Send + Sync>) -> pyo3::PyResult<pyo3::Py<$py_class_parent_name_in_rust>> {
                // Get as PyAny first, then extract as parent type
                // This is safe because all children extend the parent
                use crate::py_error::PyFeagiError;
                let py_any = Self::from_box_to_correct_child(py, boxed)?;
                if let Ok(extracted) = py_any.extract::<pyo3::Py<$py_class_parent_name_in_rust>>(py) {
                    return Ok(extracted)
                }
                Err(feagi_data_structures::FeagiDataError::InternalError(format!("Unable to extract the child of type {}", $parent_class_name_in_python_str))).map_err(PyFeagiError::from)?
            }

            /// Converts a vector of boxed trait objects to a vector typed as the parent class.
            /// Python will still see each element as its actual child type.
            pub(crate) fn from_vec_box_to_vec_parent_typed(py: pyo3::Python<'_>, vec: Vec<Box<dyn $boxed_rust_type + Send + Sync>>) -> pyo3::PyResult<Vec<pyo3::Py<$py_class_parent_name_in_rust>>> {
                vec.into_iter()
                    .map(|boxed| Self::from_box_to_parent_typed(py, boxed))
                    .collect()
            }

            /// Converts a Py<PyParentClass> to a boxed trait object.
            /// Clones the inner data from the Python object.
            pub(crate) fn from_py_to_box(py: pyo3::Python<'_>, py_obj: &pyo3::Py<$py_class_parent_name_in_rust>) -> Result<Box<dyn $boxed_rust_type + Send + Sync>, feagi_data_structures::FeagiDataError> {
                let bound = py_obj.bind(py);
                Ok(bound.borrow().inner.clone_box())
            }

            /// Converts a vector of Py<PyParentClass> to a vector of boxed trait objects.
            /// Clones the inner data from each Python object.
            pub(crate) fn from_vec_py_to_vec_box(py: pyo3::Python<'_>, vec: &[pyo3::Py<$py_class_parent_name_in_rust>]) -> Result<Vec<Box<dyn $boxed_rust_type + Send + Sync>>, feagi_data_structures::FeagiDataError> {
                vec.iter()
                    .map(|py_obj| Self::from_py_to_box(py, py_obj))
                    .collect()
            }

            /// Converts a vector of Bound PyAny references to a vector of boxed trait objects.
            /// Useful when receiving a list from Python.
            pub(crate) fn from_vec_py_any_to_vec_box<'py>(py: pyo3::Python<'py>, vec: Vec<pyo3::Bound<'py, pyo3::PyAny>>) -> Result<Vec<Box<dyn $boxed_rust_type + Send + Sync>>, feagi_data_structures::FeagiDataError> {
                vec.iter()
                    .map(|py_any| Self::py_any_to_box(py, py_any))
                    .collect()
            }
        }

    };
}

#[macro_export]
macro_rules! create_trait_child_with_box_pyclass {
    ($parent_pyclass_in_rust:ident, $py_class_name_in_rust:ident, $class_name_in_python_str:expr, $boxed_rust_type:ident, $rust_child_concrete_type:ident) => {
        #[pyo3::pyclass(str, extends=$parent_pyclass_in_rust)]
        #[derive(Debug, Clone)]
        #[pyo3(name = $class_name_in_python_str)]
        pub struct $py_class_name_in_rust {}

        // Require print support // TODO improve
        impl std::fmt::Display for $py_class_name_in_rust {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(f, "{}", $class_name_in_python_str)
            }
        }

        // Into gets weird due to parent owning data. skipping for now

        impl $py_class_name_in_rust {
            /// You MUST use this for the new constructor, and ONLY for that usecase!. Your "new" must use this and return (PyChild, PyParent)
            pub(crate) fn python_new_child_constructor(
                boxed_data: Box<dyn $boxed_rust_type + Send + Sync>,
            ) -> (Self, $parent_pyclass_in_rust) {
                (
                    $py_class_name_in_rust {},
                    $parent_pyclass_in_rust::new_parent(boxed_data),
                )
            }
        }

        impl $py_class_name_in_rust {
            // To use the following functions, call them from a a pymethods block with input "slf: PyRef<Self>" and in the format of "Self::get_parent_box(&slf)"

            fn get_parent_box<'a>(
                slf: &'a PyRef<'_, Self>,
            ) -> &'a Box<dyn $boxed_rust_type + Send + Sync> {
                let parent: &$parent_pyclass_in_rust = slf.as_ref();
                &parent.inner
            }

            fn get_parent_box_mut<'a>(
                slf: &'a mut PyRefMut<'_, Self>,
            ) -> &'a mut Box<dyn $boxed_rust_type + Send + Sync> {
                let parent: &mut $parent_pyclass_in_rust = slf.as_mut();
                &mut parent.inner
            }

            fn get_ref<'a>(
                slf: &'a PyRef<'_, Self>,
            ) -> Result<&'a $rust_child_concrete_type, feagi_data_structures::FeagiDataError> {
                let parent_box = Self::get_parent_box(slf);
                parent_box
                    .as_any()
                    .downcast_ref::<$rust_child_concrete_type>()
                    .ok_or_else(|| {
                        feagi_data_structures::FeagiDataError::InternalError(
                            "Type mismatch: expected unwrapped $py_class_name_in_rust".into(),
                        )
                    })
            }

            fn get_ref_mut<'a>(
                slf: &'a mut PyRefMut<'_, Self>,
            ) -> Result<&'a mut $rust_child_concrete_type, feagi_data_structures::FeagiDataError>
            {
                let parent_box = Self::get_parent_box_mut(slf);
                parent_box
                    .as_any_mut()
                    .downcast_mut::<$rust_child_concrete_type>()
                    .ok_or_else(|| {
                        feagi_data_structures::FeagiDataError::InternalError(
                            "Type mismatch: expected unwrapped $py_class_name_in_rust".into(),
                        )
                    })
            }
        }
    };
}

//endregion
