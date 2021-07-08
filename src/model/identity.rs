/*!
One-line description.

More detailed description, with

# Example

*/

use crate::error::{Error, ErrorKind};
use std::fmt::{Debug, Display, Formatter};
use std::str::FromStr;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Identifier(String);

pub trait HasName {
    fn name(&self) -> &Identifier;
    fn set_name(&mut self, name: Identifier) -> &mut Self
    where
        Self: Sized;
    fn same_name_as(&self, other: &Self) -> bool {
        self.name() == other.name()
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Namespace(Vec<Identifier>);

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Display for Identifier {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl AsRef<str> for Identifier {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl FromStr for Identifier {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            Err(ErrorKind::InvalidIdentifierValue(s.to_string()).into())
        } else {
            Ok(Self(s.to_string()))
        }
    }
}

impl Identifier {
    pub fn new(id: &str) -> Self {
        Self::from_str(id).unwrap()
    }
}

// ------------------------------------------------------------------------------------------------

impl From<Identifier> for Namespace {
    fn from(path: Identifier) -> Self {
        Self::new(vec![path])
    }
}

impl From<Vec<Identifier>> for Namespace {
    fn from(path: Vec<Identifier>) -> Self {
        Self::new(path)
    }
}

impl Namespace {
    pub fn new(path: Vec<Identifier>) -> Self {
        assert!(!path.is_empty());
        Self(path)
    }

    pub fn with(&self, name: Identifier) -> Self {
        let mut new_namespace = self.clone();
        new_namespace.push(name);
        new_namespace
    }

    pub fn path(&self) -> &Vec<Identifier> {
        &self.0
    }

    pub fn pop(&mut self) -> Option<Identifier> {
        self.0.pop()
    }

    pub fn push(&mut self, name: Identifier) {
        self.0.push(name);
    }

    pub fn contains(&self, name: &Identifier) -> bool {
        self.0.contains(name)
    }

    pub fn join(&self, separator: &str) -> String {
        self.0
            .iter()
            .map(|i| i.to_string())
            .collect::<Vec<String>>()
            .join(separator)
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
