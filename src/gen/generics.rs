//!
//!
//! Create a single or collection of generics/trait bounds for functions and other
//! objects.
//!

use serde::{Deserialize, Serialize};
use tera::{Context, Tera};

use crate::internal;
use crate::traits::SrcCode;

/// Represent a single trait bound
#[derive(Serialize, Deserialize, Default, Clone)]
pub struct Generic {
    name: String,
    traits: Vec<String>,
}

impl Generic {
    /// Create a new `Generic`
    pub fn new(name: impl ToString) -> Self {
        Self {
            name: name.to_string(),
            ..Self::default()
        }
    }

    /// Get the name of the generic
    pub fn name(&self) -> &str {
        self.name.as_str()
    }
}

impl internal::TraitBounds for Generic {
    fn trait_bounds_mut(&mut self) -> &mut Vec<String> {
        &mut self.traits
    }
}

impl SrcCode for Vec<Generic> {
    fn generate(&self) -> String {
        if !self.is_empty() {
            let template = r#"<{{ generic_keys | join(sep=", ") }}>
                where
                    {% for generic in generics %}{{ generic.name }}: {{ generic.traits | join(sep=" + ") }},
                    {% endfor %}
            "#;
            let mut context = Context::new();
            context.insert(
                "generic_keys",
                &self.iter().map(|g| g.name()).collect::<Vec<&str>>(),
            );
            context.insert("generics", &self);
            Tera::one_off(template, &context, false).unwrap()
        } else {
            "".to_string()
        }
    }
}
