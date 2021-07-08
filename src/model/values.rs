/*!
One-line description.

More detailed description, with

# Example

*/

use crate::model::comments::HasDocumentation;
use crate::model::identity::{HasName, Identifier};
use crate::model::modules::{HasVisibility, Visibility};
use crate::model::properties::{HasProperties, Property};
use crate::model::Builder;
use std::collections::HashMap;
use std::fmt::Debug;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Debug, PartialEq)]
pub enum KnownType {
    I8,
    U8,
    I16,
    U16,
    I32,
    U32,
    I64,
    U64,
    F32,
    F64,
    Boolean,
    Char,
    String,
}

#[derive(Clone, Debug)]
pub enum ValueType {
    Known(KnownType),
    Reference(Identifier),
    Array(Box<ValueType>),
    Set(Box<ValueType>),
    Map(Box<ValueType>, Box<ValueType>),
    Constrained(Identifier, Vec<ValueType>),
    Generic(Identifier, Vec<ValueType>),
    Function(Vec<ValueType>, Option<Box<ValueType>>),
}

pub trait HasOptionalType {
    fn value_type(&self) -> &Option<ValueType>;
    fn has_value_type(&self) -> bool {
        self.value_type().is_some()
    }
    fn set_value_type(&mut self, value_type: ValueType) -> &mut Self
    where
        Self: Sized;
    fn unset_value_type(&mut self) -> &mut Self
    where
        Self: Sized;
}

pub trait HasType {
    fn value_type(&self) -> &ValueType;
    fn set_value_type(&mut self, value_type: ValueType) -> &mut Self
    where
        Self: Sized;
}

// ------------------------------------------------------------------------------------------------

#[derive(Clone, Debug)]
pub enum Value {
    I8(i8),
    U8(u8),
    I16(i16),
    U16(u16),
    I32(i32),
    U32(u32),
    I64(i64),
    U64(u64),
    F32(f32),
    F64(f64),
    Boolean(bool),
    Char(char),
    String(String),
    Values(Vec<Value>),
    NamedValues(HashMap<Value, Value>),
    Identifier(Identifier),
}

pub trait HasValue: Clone + Debug {
    fn value(&self) -> &Value;
    fn set_value(&mut self, value: Value) -> &mut Self;
}

pub trait HasOptionalValue: Clone + Debug {
    fn value(&self) -> &Option<Value>;
    fn has_value(&self) -> bool {
        self.value().is_some()
    }
    fn set_value(&mut self, value: Value) -> &mut Self;
    fn unset_value(&mut self) -> &mut Self;
}

// ------------------------------------------------------------------------------------------------

#[derive(Clone, Debug)]
pub struct NamedValue {
    properties: Vec<Property>,
    visibility: Option<Visibility>,
    name: Identifier,
    documentation: Option<String>,
    value_type: ValueType,
    value: Value,
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

impl ValueType {
    pub fn i8() -> Self {
        Self::Known(KnownType::I8)
    }

    pub fn u8() -> Self {
        Self::Known(KnownType::U8)
    }

    pub fn i16() -> Self {
        Self::Known(KnownType::I16)
    }

    pub fn u16() -> Self {
        Self::Known(KnownType::U16)
    }

    pub fn i32() -> Self {
        Self::Known(KnownType::I32)
    }

    pub fn u32() -> Self {
        Self::Known(KnownType::U32)
    }

    pub fn i64() -> Self {
        Self::Known(KnownType::I64)
    }

    pub fn u64() -> Self {
        Self::Known(KnownType::U64)
    }

    pub fn f32() -> Self {
        Self::Known(KnownType::F32)
    }

    pub fn f64() -> Self {
        Self::Known(KnownType::F64)
    }

    pub fn boolean() -> Self {
        Self::Known(KnownType::Boolean)
    }

    pub fn character() -> Self {
        Self::Known(KnownType::Char)
    }

    pub fn string() -> Self {
        Self::Known(KnownType::String)
    }

    pub fn array_of(member_type: ValueType) -> Self {
        Self::Array(Box::new(member_type))
    }

    pub fn set_of(member_type: ValueType) -> Self {
        Self::Array(Box::new(member_type))
    }

    pub fn map_of(key_type: ValueType, value_type: ValueType) -> Self {
        Self::Map(Box::new(key_type), Box::new(value_type))
    }
}

// ------------------------------------------------------------------------------------------------

impl From<i8> for Value {
    fn from(v: i8) -> Self {
        Self::I8(v)
    }
}

impl From<u8> for Value {
    fn from(v: u8) -> Self {
        Self::U8(v)
    }
}

impl From<i16> for Value {
    fn from(v: i16) -> Self {
        Self::I16(v)
    }
}

impl From<u16> for Value {
    fn from(v: u16) -> Self {
        Self::U16(v)
    }
}

impl From<u32> for Value {
    fn from(v: u32) -> Self {
        Self::U32(v)
    }
}

impl From<i32> for Value {
    fn from(v: i32) -> Self {
        Self::I32(v)
    }
}

impl From<i64> for Value {
    fn from(v: i64) -> Self {
        Self::I64(v)
    }
}

impl From<u64> for Value {
    fn from(v: u64) -> Self {
        Self::U64(v)
    }
}

impl From<f32> for Value {
    fn from(v: f32) -> Self {
        Self::F32(v)
    }
}

impl From<f64> for Value {
    fn from(v: f64) -> Self {
        Self::F64(v)
    }
}

impl From<bool> for Value {
    fn from(v: bool) -> Self {
        Self::Boolean(v)
    }
}

impl From<char> for Value {
    fn from(v: char) -> Self {
        Self::Char(v)
    }
}

impl From<String> for Value {
    fn from(v: String) -> Self {
        Self::String(v)
    }
}

impl From<&str> for Value {
    fn from(v: &str) -> Self {
        Self::String(v.to_string())
    }
}

impl From<Identifier> for Value {
    fn from(v: Identifier) -> Self {
        Self::Identifier(v)
    }
}

impl From<Vec<Value>> for Value {
    fn from(v: Vec<Value>) -> Self {
        Self::Values(v)
    }
}

impl From<HashMap<Value, Value>> for Value {
    fn from(v: HashMap<Value, Value>) -> Self {
        Self::NamedValues(v)
    }
}

impl Value {
    pub fn value_type(&self) -> Option<ValueType> {
        match self {
            Value::I8(_) => Some(ValueType::Known(KnownType::I8)),
            Value::U8(_) => Some(ValueType::Known(KnownType::U8)),

            Value::I16(_) => Some(ValueType::Known(KnownType::I16)),
            Value::U16(_) => Some(ValueType::Known(KnownType::U16)),
            Value::I32(_) => Some(ValueType::Known(KnownType::I32)),
            Value::U32(_) => Some(ValueType::Known(KnownType::U32)),
            Value::I64(_) => Some(ValueType::Known(KnownType::I64)),
            Value::U64(_) => Some(ValueType::Known(KnownType::U64)),
            Value::F32(_) => Some(ValueType::Known(KnownType::F32)),
            Value::F64(_) => Some(ValueType::Known(KnownType::F64)),
            Value::Boolean(_) => Some(ValueType::Known(KnownType::Boolean)),
            Value::Char(_) => Some(ValueType::Known(KnownType::Char)),
            Value::String(_) => Some(ValueType::Known(KnownType::String)),
            Value::Values(_) => None,
            Value::NamedValues(_) => None,
            Value::Identifier(_) => None,
        }
    }
}

// ------------------------------------------------------------------------------------------------

impl_has_properties!(NamedValue);

impl_has_visibility!(NamedValue);

impl_has_name!(NamedValue);

impl_has_documentation!(NamedValue);

impl_has_type!(NamedValue);

impl_has_value!(NamedValue);

impl Builder for NamedValue {}

impl NamedValue {
    pub fn new(name: Identifier, value_type: ValueType, value: Value) -> Self {
        Self {
            properties: Default::default(),
            visibility: None,
            name,
            documentation: None,
            value_type,
            value,
        }
    }

    pub fn i8(name: Identifier, value: i8) -> Self {
        let value = Value::I8(value);
        Self::new(name, value.value_type().unwrap(), value)
    }

    pub fn u8(name: Identifier, value: u8) -> Self {
        let value = Value::U8(value);
        Self::new(name, value.value_type().unwrap(), value)
    }

    pub fn i16(name: Identifier, value: i16) -> Self {
        let value = Value::I16(value);
        Self::new(name, value.value_type().unwrap(), value)
    }

    pub fn u16(name: Identifier, value: u16) -> Self {
        let value = Value::U16(value);
        Self::new(name, value.value_type().unwrap(), value)
    }

    pub fn i32(name: Identifier, value: i32) -> Self {
        let value = Value::I32(value);
        Self::new(name, value.value_type().unwrap(), value)
    }

    pub fn u32(name: Identifier, value: u32) -> Self {
        let value = Value::U32(value);
        Self::new(name, value.value_type().unwrap(), value)
    }

    pub fn i64(name: Identifier, value: i64) -> Self {
        let value = Value::I64(value);
        Self::new(name, value.value_type().unwrap(), value)
    }

    pub fn u64(name: Identifier, value: u64) -> Self {
        let value = Value::U64(value);
        Self::new(name, value.value_type().unwrap(), value)
    }

    pub fn f32(name: Identifier, value: f32) -> Self {
        let value = Value::F32(value);
        Self::new(name, value.value_type().unwrap(), value)
    }

    pub fn f64(name: Identifier, value: f64) -> Self {
        let value = Value::F64(value);
        Self::new(name, value.value_type().unwrap(), value)
    }

    pub fn boolean(name: Identifier, value: bool) -> Self {
        let value = Value::Boolean(value);
        Self::new(name, value.value_type().unwrap(), value)
    }

    pub fn character(name: Identifier, value: char) -> Self {
        let value = Value::Char(value);
        Self::new(name, value.value_type().unwrap(), value)
    }

    pub fn string(name: Identifier, value: &str) -> Self {
        let value = Value::String(value.to_string());
        Self::new(name, value.value_type().unwrap(), value)
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
