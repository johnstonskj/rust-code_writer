/*!
One-line description.

More detailed description, with

# Example

*/

// ------------------------------------------------------------------------------------------------
// Macros
// ------------------------------------------------------------------------------------------------

macro_rules! impl_has_documentation {
    ($name:ty) => {
        impl HasDocumentation for $name {
            fn documentation(&self) -> &Option<String> {
                &self.documentation
            }

            fn set_documentation(&mut self, documentation: &str) -> &mut Self {
                self.documentation = Some(documentation.to_string());
                self
            }

            fn unset_documentation(&mut self) -> &mut Self {
                self.documentation = None;
                self
            }
        }
    };
}

macro_rules! impl_has_properties {
    ($name:ty) => {
        impl HasProperties for $name {
            fn properties(&self) -> &Vec<Property> {
                &self.properties
            }

            fn set_properties(&mut self, properties: Vec<Property>) -> &mut Self {
                self.properties = properties;
                self
            }

            fn add_property(&mut self, property: Property) -> &mut Self {
                self.properties.push(property);
                self
            }
        }
    };
}

macro_rules! impl_has_visibility {
    ($name:ty) => {
        impl HasVisibility for $name {
            fn visibility(&self) -> &Option<Visibility> {
                &self.visibility
            }

            fn set_visibility(&mut self, visibility: Visibility) -> &mut Self {
                self.visibility = Some(visibility);
                self
            }

            fn unset_visibility(&mut self) -> &mut Self {
                self.visibility = None;
                self
            }
        }
    };
}

macro_rules! impl_has_name {
    ($name:ty) => {
        impl HasName for $name {
            fn name(&self) -> &Identifier {
                &self.name
            }

            fn set_name(&mut self, name: Identifier) -> &mut Self {
                self.name = name;
                self
            }
        }
    };
}

macro_rules! impl_has_optional_value {
    ($name:ty) => {
        impl HasOptionalValue for $name {
            fn value(&self) -> &Option<Value> {
                &self.value
            }

            fn set_value(&mut self, value: Value) -> &mut Self {
                self.value = Some(value);
                self
            }

            fn unset_value(&mut self) -> &mut Self {
                self.value = None;
                self
            }
        }
    };
}

macro_rules! impl_has_value {
    ($name:ty) => {
        impl HasValue for $name {
            fn value(&self) -> &Value {
                &self.value
            }

            fn set_value(&mut self, value: Value) -> &mut Self {
                self.value = value;
                self
            }
        }
    };
}

macro_rules! impl_has_optional_type {
    ($name:ty) => {
        impl HasOptionalType for $name {
            fn value_type(&self) -> &Option<ValueType> {
                &self.value_type
            }

            fn set_value_type(&mut self, value_type: ValueType) -> &mut Self {
                self.value_type = Some(value_type);
                self
            }

            fn unset_value_type(&mut self) -> &mut Self {
                self.value_type = None;
                self
            }
        }
    };
}

macro_rules! impl_has_type {
    ($name:ty) => {
        impl HasType for $name {
            fn value_type(&self) -> &ValueType {
                &self.value_type
            }

            fn set_value_type(&mut self, value_type: ValueType) -> &mut Self {
                self.value_type = value_type;
                self
            }
        }
    };
}

macro_rules! impl_is_optional {
    ($name:ty) => {
        impl IsOptional for $name {
            fn is_optional(&self) -> bool {
                self.optional
            }

            fn optional(&mut self) -> &mut Self {
                self.optional = true;
                self
            }

            fn is_required(&self) -> bool {
                !self.is_optional()
            }

            fn required(&mut self) -> &mut Self {
                self.optional = false;
                self
            }
        }
    };
}
