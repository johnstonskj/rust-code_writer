/*!
One-line description.

More detailed description, with

# Example

*/

use crate::model::comments::HasDocumentation;
use crate::model::functions::FunctionDecl;
use crate::model::identity::{HasName, Identifier};
use crate::model::modules::{HasVisibility, Visibility};
use crate::model::properties::{HasProperties, IsOptional, Property};
use crate::model::values::{HasOptionalType, HasOptionalValue, HasType, Value, ValueType};
use crate::model::Builder;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Debug)]
pub struct Field {
    properties: Vec<Property>,
    visibility: Option<Visibility>,
    optional: bool,
    name: Identifier,
    documentation: Option<String>,
    value_type: ValueType,
    value: Option<Value>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum StructuredTypeKind {
    Exception,
    Structure,
    Union,
    Class,
    Interface,
    Service,
}

#[derive(Clone, Debug)]
pub struct StructuredType {
    properties: Vec<Property>,
    visibility: Option<Visibility>,
    kind: StructuredTypeKind,
    name: Identifier,
    documentation: Option<String>,
    extends: Vec<ValueType>,
    fields: Vec<Field>,
    methods: Vec<FunctionDecl>,
}

// ------------------------------------------------------------------------------------------------

#[derive(Clone, Debug)]
pub struct EnumerationVariant {
    properties: Vec<Property>,
    name: Identifier,
    documentation: Option<String>,
    value_type: Option<ValueType>,
    value: Option<Value>,
}

#[derive(Clone, Debug)]
pub struct Enumeration {
    properties: Vec<Property>,
    visibility: Option<Visibility>,
    name: Identifier,
    documentation: Option<String>,
    variants: Vec<EnumerationVariant>,
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

impl_has_properties!(Field);

impl_has_visibility!(Field);

impl_is_optional!(Field);

impl_has_name!(Field);

impl_has_documentation!(Field);

impl_has_type!(Field);

impl_has_optional_value!(Field);

impl Builder for Field {}

impl Field {
    pub fn new(name: Identifier, value_type: ValueType) -> Self {
        Self {
            properties: Default::default(),
            visibility: None,
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
            visibility: None,
            optional: false,
            name,
            documentation: None,
            value_type,
            value: Some(value),
        }
    }
}

// ------------------------------------------------------------------------------------------------

impl_has_properties!(StructuredType);

impl_has_visibility!(StructuredType);

impl_has_name!(StructuredType);

impl_has_documentation!(StructuredType);

impl Builder for StructuredType {}

impl StructuredType {
    pub fn new(name: Identifier, kind: StructuredTypeKind) -> Self {
        Self {
            properties: Default::default(),
            visibility: None,
            kind,
            name,
            documentation: None,
            extends: Default::default(),
            fields: Default::default(),
            methods: Default::default(),
        }
    }

    pub fn exception(name: Identifier) -> Self {
        Self::new(name, StructuredTypeKind::Exception)
    }

    pub fn structure(name: Identifier) -> Self {
        Self::new(name, StructuredTypeKind::Structure)
    }

    pub fn union(name: Identifier) -> Self {
        Self::new(name, StructuredTypeKind::Union)
    }

    pub fn class(name: Identifier) -> Self {
        Self::new(name, StructuredTypeKind::Class)
    }

    pub fn interface(name: Identifier) -> Self {
        Self::new(name, StructuredTypeKind::Interface)
    }

    pub fn service(name: Identifier) -> Self {
        Self::new(name, StructuredTypeKind::Service)
    }

    pub fn kind(&self) -> &StructuredTypeKind {
        &self.kind
    }

    pub fn set_kind(&mut self, kind: StructuredTypeKind) -> &mut Self {
        self.kind = kind;
        self
    }

    pub fn extends(&self) -> &Vec<ValueType> {
        &self.extends
    }

    pub fn set_extends(&mut self, extends: Vec<ValueType>) -> &mut Self {
        self.extends = extends;
        self
    }

    pub fn add_extend(&mut self, extend: ValueType) -> &mut Self {
        self.extends.push(extend);
        self
    }

    pub fn fields(&self) -> &Vec<Field> {
        &self.fields
    }

    pub fn set_fields(&mut self, fields: Vec<Field>) -> &mut Self {
        self.fields = fields;
        self
    }

    pub fn add_field(&mut self, field: Field) -> &mut Self {
        self.fields.push(field);
        self
    }

    pub fn methods(&self) -> &Vec<FunctionDecl> {
        &self.methods
    }

    pub fn set_methods(&mut self, methods: Vec<FunctionDecl>) -> &mut Self {
        self.methods = methods;
        self
    }

    pub fn add_method(&mut self, method: FunctionDecl) -> &mut Self {
        self.methods.push(method);
        self
    }
}

// ------------------------------------------------------------------------------------------------

impl_has_properties!(EnumerationVariant);

impl_has_name!(EnumerationVariant);

impl_has_documentation!(EnumerationVariant);

impl_has_optional_type!(EnumerationVariant);

impl_has_optional_value!(EnumerationVariant);

impl Builder for EnumerationVariant {}

impl EnumerationVariant {
    pub fn new(name: Identifier) -> Self {
        Self {
            properties: Default::default(),
            name,
            documentation: None,
            value_type: None,
            value: None,
        }
    }
}

// ------------------------------------------------------------------------------------------------

impl_has_properties!(Enumeration);

impl_has_visibility!(Enumeration);

impl_has_name!(Enumeration);

impl_has_documentation!(Enumeration);

impl Builder for Enumeration {}

impl Enumeration {
    pub fn new(name: Identifier) -> Self {
        Self {
            properties: Default::default(),
            visibility: None,
            name,
            documentation: None,
            variants: Default::default(),
        }
    }

    pub fn with_visibility(visibility: Visibility, name: Identifier) -> Self {
        Self {
            properties: Default::default(),
            visibility: Some(visibility),
            name,
            documentation: None,
            variants: Default::default(),
        }
    }

    pub fn variants(&self) -> &Vec<EnumerationVariant> {
        &self.variants
    }

    pub fn set_variants(&mut self, variants: Vec<EnumerationVariant>) -> &mut Self {
        self.variants = variants;
        self
    }

    pub fn add_variant(&mut self, variant: EnumerationVariant) -> &mut Self {
        self.variants.push(variant);
        self
    }

    pub fn add_named_variant(&mut self, variant: Identifier) -> &mut Self {
        self.variants.push(EnumerationVariant::new(variant));
        self
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
