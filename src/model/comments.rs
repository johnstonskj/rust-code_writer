/*!
One-line description.

More detailed description, with

# Example

*/

// use ...

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

use crate::model::Builder;
use std::fmt::Debug;

#[derive(Clone, Debug)]
pub struct Comment {
    text: String,
    is_block: bool,
}

// ------------------------------------------------------------------------------------------------

pub trait HasDocumentation {
    fn documentation(&self) -> &Option<String>;
    fn has_documentation(&self) -> bool {
        self.documentation().is_some()
    }
    fn set_documentation(&mut self, documentation: &str) -> &mut Self
    where
        Self: Sized;
    fn unset_documentation(&mut self) -> &mut Self
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

impl Builder for Comment {}

impl Comment {
    pub fn new(text: &str) -> Self {
        Self {
            text: text.to_string(),
            is_block: true,
        }
    }

    pub fn line(text: &str) -> Self {
        Self {
            text: text.to_string(),
            is_block: false,
        }
    }

    pub fn block(text: &str) -> Self {
        Self {
            text: text.to_string(),
            is_block: true,
        }
    }

    pub fn text(&self) -> &String {
        &self.text
    }

    pub fn set_text(&mut self, text: &str) -> &mut Self {
        self.text = text.to_string();
        self
    }

    pub fn is_line(&self) -> bool {
        !self.is_block()
    }

    pub fn is_block(&self) -> bool {
        self.is_block
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
