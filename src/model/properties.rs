/*!
One-line description.

More detailed description, with

# Example

*/

use crate::model::identity::{HasName, Identifier};
use crate::model::values::{HasOptionalValue, Value};
use std::fmt::Debug;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Debug)]
pub struct Property {
    name: Identifier,
    value: Option<Value>,
}

pub trait HasProperties {
    fn properties(&self) -> &Vec<Property>;
    fn has_properties(&self) -> bool {
        !self.properties().is_empty()
    }
    fn set_properties(&mut self, properties: Vec<Property>) -> &mut Self
    where
        Self: Sized;
    fn add_property(&mut self, property: Property) -> &mut Self
    where
        Self: Sized;
}

// ------------------------------------------------------------------------------------------------

pub trait IsOptional {
    fn is_optional(&self) -> bool;
    fn optional(&mut self) -> &mut Self
    where
        Self: Sized;
    fn is_required(&self) -> bool;
    fn required(&mut self) -> &mut Self
    where
        Self: Sized;
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

impl_has_name!(Property);

impl_has_optional_value!(Property);

impl Property {
    pub fn new(name: Identifier) -> Self {
        Self { name, value: None }
    }

    pub fn with_value(name: Identifier, value: Value) -> Self {
        Self {
            name,
            value: Some(value),
        }
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
