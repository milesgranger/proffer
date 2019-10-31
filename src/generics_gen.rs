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
    pub(crate) generic: String,
    pub(crate) traits: Vec<String>,
}

impl Generic {
    /// Create a new `Generic`
    pub fn new<S: ToString>(id: S) -> Self {
        Self {
            generic: id.to_string(),
            ..Self::default()
        }
    }
}

impl internal::TraitBounds for Generic {
    fn trait_bounds(&mut self) -> &mut Vec<String> {
        &mut self.traits
    }
}

/// Represent a collection of trait bounds
#[derive(Serialize, Deserialize, Default, Clone)]
pub struct Generics {
    pub(crate) generics: Vec<Generic>,
}

impl internal::Generics for Generics {
    fn generics(&mut self) -> &mut Vec<Generic> {
        &mut self.generics
    }
}

impl Generics {
    /// Create a new collection of `Generic`s.
    pub fn new(generics: Vec<Generic>) -> Self {
        Self { generics }
    }

    /// Check how many generics are held here
    pub fn len(&self) -> usize {
        self.generics.len()
    }

    /// Determine if it doesn't have any generics.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl SrcCode for Generics {
    fn generate(&self) -> String {
        if !self.generics.is_empty() {
            let template = r#"<{{ generic_keys | join(sep=", ") }}>
                where
                    {% for generic in generics %}{{ generic.generic }}: {{ generic.traits | join(sep=" + ") }},
                    {% endfor %}
            "#;
            let mut context = Context::new();
            context.insert(
                "generic_keys",
                &self
                    .generics
                    .iter()
                    .map(|g| g.generic.clone())
                    .collect::<Vec<String>>(),
            );
            context.insert("generics", &self.generics);
            Tera::one_off(template, &context, false).unwrap()
        } else {
            "".to_string()
        }
    }
}
