/*!
One-line description.

More detailed description, with

# Example

*/

use crate::model::comments::HasDocumentation;
use crate::model::identity::{HasName, Identifier};
use crate::model::modules::{HasVisibility, Visibility};
use crate::model::properties::{HasProperties, IsOptional, Property};
use crate::model::values::{HasOptionalType, HasOptionalValue, HasType, Value, ValueType};
use crate::model::Builder;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Debug)]
pub struct Parameter {
    properties: Vec<Property>,
    optional: bool,
    name: Identifier,
    documentation: Option<String>,
    value_type: ValueType,
    value: Option<Value>,
}

#[derive(Clone, Debug)]
pub struct FunctionDecl {
    properties: Vec<Property>,
    visibility: Option<Visibility>,
    name: Identifier,
    documentation: Option<String>,
    parameters: Vec<Parameter>,
    value_type: Option<ValueType>,
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

impl_has_properties!(Parameter);

impl_is_optional!(Parameter);

impl_has_name!(Parameter);

impl_has_documentation!(Parameter);

impl_has_type!(Parameter);

impl_has_optional_value!(Parameter);

impl Builder for Parameter {}

impl Parameter {
    pub fn new(name: Identifier, value_type: ValueType) -> Self {
        Self {
            properties: Default::default(),
            optional: false,
            name,
            documentation: None,
            value_type,
            value: None,
        }
    }

    pub fn with_value(name: Identifier, value_type: ValueType, value: Value) -> Self {
        Self {
            properties: Default::default(),
            optional: false,
            name,
            documentation: None,
            value_type,
            value: Some(value),
        }
    }
}

// ------------------------------------------------------------------------------------------------

impl_has_properties!(FunctionDecl);

impl_has_visibility!(FunctionDecl);

impl_has_name!(FunctionDecl);

impl_has_documentation!(FunctionDecl);

impl_has_optional_type!(FunctionDecl);

impl Builder for FunctionDecl {}

impl FunctionDecl {
    pub fn new(name: Identifier) -> Self {
        Self {
            properties: Default::default(),
            visibility: None,
            name,
            documentation: None,
            parameters: Default::default(),
            value_type: None,
        }
    }

    pub fn with_visibility(visibility: Visibility, name: Identifier) -> Self {
        Self {
            properties: Default::default(),
            visibility: Some(visibility),
            name,
            documentation: None,
            parameters: Default::default(),
            value_type: None,
        }
    }

    pub fn parameters(&self) -> &Vec<Parameter> {
        &self.parameters
    }

    pub fn set_parameters(&mut self, parameters: Vec<Parameter>) -> &mut Self {
        self.parameters = parameters;
        self
    }

    pub fn add_parameter(&mut self, parameter: Parameter) -> &mut Self {
        self.parameters.push(parameter);
        self
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
