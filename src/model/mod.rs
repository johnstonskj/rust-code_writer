/*!
One-line description.

More detailed description, with

# Example

*/

// use ...

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

pub trait Builder: Clone {
    fn build(&mut self) -> Self {
        self.clone()
    }
}

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------

#[macro_use]
mod macros;

pub mod comments;
pub use comments::{Comment, HasDocumentation};

pub mod identity;
pub use identity::{HasName, Identifier};

pub mod functions;
pub use functions::{FunctionDecl, Parameter};

pub mod modules;
pub use modules::{
    HasVisibility, Import, ImportItem, Module, ModuleContent, TypeAlias, Visibility,
};

pub mod properties;
pub use properties::{HasProperties, IsOptional, Property};

pub mod structured_types;
pub use structured_types::{
    Enumeration, EnumerationVariant, Field, StructuredType, StructuredTypeKind,
};

pub mod values;
pub use values::{
    HasOptionalType, HasOptionalValue, HasType, HasValue, KnownType, NamedValue, Value, ValueType,
};
