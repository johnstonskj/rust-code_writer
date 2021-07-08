/*!
This crate provides a simple type model that can be used for code generation, along with traits
implemented for specific language options.

The goal is not to provide a faithful AST for any specific language, but rather to provide a simple
model that can generate _enough_ code for tools that define types and API bindings.

# Example

The following example builds a single module, with a single struct type.

```rust
use code_writer::model::modules::Module;
use code_writer::model::{
    Builder, Field, HasDocumentation, HasProperties, HasVisibility, Identifier, Import, IsOptional,
    Property, StructuredType, Value, ValueType,
};

let module = Module::new(Identifier::new("address"))
    .set_documentation("This module provides a simple address type.")
    .add_import(Import::with_items(
        Identifier::new("serde").into(),
        vec![
            Identifier::new("Deserialize").into(),
            Identifier::new("Serialize").into(),
        ],
    ))
    .add_structure(
        StructuredType::structure(Identifier::new("Address"))
            .make_public()
            .set_documentation("A Locale-neutral address type.")
            .add_property(Property::with_value(
                Identifier::new("derive"),
                vec![
                    Value::from("Clone"),
                    Value::from("Debug"),
                    Value::from("PartialEq"),
                    Value::from("Deserialize"),
                    Value::from("Serialize"),
                ]
                .into(),
            ))
            .add_field(
                Field::new(Identifier::new("property_number"), ValueType::string())
                    .set_documentation("The number of the property on a street.")
                    .make_public()
                    .required()
                    .build(),
            )
            .add_field(
                Field::new(Identifier::new("property_street"), ValueType::string())
                    .set_documentation("The street the property is on.")
                    .make_public()
                    .required()
                    .build(),
            )
            .add_field(
                Field::new(Identifier::new("unit_number"), ValueType::string())
                    .set_documentation("The unit within the property.")
                    .make_public()
                    .optional()
                    .build(),
            )
            .add_field(
                Field::new(Identifier::new("city"), ValueType::string())
                    .set_documentation("The city in which the property exists.")
                    .make_public()
                    .required()
                    .build(),
            )
            .add_field(
                Field::new(Identifier::new("region"), ValueType::string())
                    .set_documentation("An optional region such as state, county, etc.")
                    .make_public()
                    .optional()
                    .build(),
            )
            .add_field(
                Field::new(Identifier::new("postal_code"), ValueType::string())
                    .set_documentation("A country-specific postal code.")
                    .make_public()
                    .required()
                    .build(),
            )
            .add_field(
                Field::new(Identifier::new("country"), ValueType::string())
                    .set_documentation("The country in which the property exists.")
                    .make_public()
                    .required()
                    .build(),
            )
            .build(),
    )
    .build();
```

This will generate the following code when using the `RustWriter`.

```rust,ignore
/*!
This module provides a simple address type.
*/

use serde::{Deserialize, Serialize};

/**
A Locale-neutral address type.
*/
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct Address {
    /**
    The number of the property on a street.
    */
    pub property_number: String,
    /**
    The street the property is on.
    */
    pub property_street: String,
    /**
    The unit within the property.
    */
    pub unit_number: Option<String>,
    /**
    The city in which the property exists.
    */
    pub city: String,
    /**
    An optional region such as state, county, etc.
    */
    pub region: Option<String>,
    /**
    A country-specific postal code.
    */
    pub postal_code: String,
    /**
    The country in which the property exists.
    */
    pub country: String,
}
```
*/

#[macro_use]
extern crate error_chain;

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------

pub mod error;

pub mod language;

pub mod model;

pub mod writer;
