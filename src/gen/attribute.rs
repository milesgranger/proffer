//!
//! Create attribute objects using the `Attribute` enum.
//!
//!
//! Example
//! -------
//! ```
//! use proffer::*;
//!
//! let a = Attribute::from("#![be_cool]");
//!
//! let src_code = a.generate();
//! let expected = "#![be_cool]";
//! assert_eq!(norm_whitespace(expected), norm_whitespace(&src_code))
//! ```

use crate::SrcCode;
use serde::{Deserialize, Serialize};

/// Represents a single Rust attribute to a module, function, etc.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Attribute {
    /// Attribute representing an attribute for an item. ie. `#[foo(bar)]`
    ItemAttr(String),
    /// Attribute representing a scoped level attribute. ie. `#![warn(...)]`
    ScopeAttr(String),
}

// TODO: Use TryFrom when https://github.com/rust-lang/rust/issues/50133 is resolved.
impl<S: ToString> From<S> for Attribute {
    fn from(attribute: S) -> Self {
        let attribute = attribute.to_string();

        if attribute.starts_with("#!") {
            Attribute::ScopeAttr(attribute)
        } else if attribute.starts_with('#') {
            Attribute::ItemAttr(attribute)
        } else {
            panic!("No Attribute match for '{}'", attribute)
        }
    }
}

impl SrcCode for Attribute {
    fn generate(&self) -> String {
        match self {
            Attribute::ScopeAttr(s) | Attribute::ItemAttr(s) => s.to_owned(),
        }
    }
}
