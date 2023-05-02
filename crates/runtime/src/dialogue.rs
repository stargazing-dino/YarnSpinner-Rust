use std::fmt::Debug;
use yarn_slinger_core::prelude::*;

/// Co-ordinates the execution of Yarn programs.
#[non_exhaustive]
#[derive(Debug, Clone)]
pub struct Dialogue {
    /// Gets the [`Library`] that this Dialogue uses to locate functions.
    ///
    /// When the Dialogue is constructed, the Library is initialized with
    /// the built-in operators like `+`, `-`, and so on.
    pub library: Library,

    /// The object that provides access to storing and retrieving the values of variables.
    pub variable_storage: Box<dyn VariableStorage>,
}

impl Dialogue {
    /// Initializes a new instance of the [`Dialogue`] class.
    pub fn with_variable_storage(variable_storage: impl VariableStorage + 'static) -> Self {
        let mut library = Library::default();
        library.import(Library::standard_library());
        Self {
            variable_storage: Box::new(variable_storage),
            library,
        }
    }
}

/// Provides a mechanism for storing and retrieving instances
/// of the [`InternalValue`] type.
pub trait VariableStorage: Debug {
    fn clone_box(&self) -> Box<dyn VariableStorage>;
}

impl Clone for Box<dyn VariableStorage> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}

/// A simple concrete implementation of [`VariableStorage`]
/// that keeps all variables in memory.
#[derive(Debug, Clone, Default)]
pub struct MemoryVariableStore {}

impl VariableStorage for MemoryVariableStore {
    fn clone_box(&self) -> Box<dyn VariableStorage> {
        Box::new(self.clone())
    }
}
