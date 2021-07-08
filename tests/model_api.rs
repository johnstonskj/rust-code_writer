use code_writer::model::modules::Module;
use code_writer::model::{
    Builder, Field, HasDocumentation, HasProperties, HasVisibility, Identifier, Import, IsOptional,
    Property, StructuredType, Value, ValueType,
};

pub mod common;

#[test]
fn make_a_model() {
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
        .add_sub_module(
            Module::new(Identifier::new("countries"))
                .make_public()
                .build(),
        )
        .build();

    common::print_module(&module).unwrap();
}
