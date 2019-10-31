//!
//!
//! Create a associated type for traits and trait implementations.
//!

use serde::{Deserialize, Serialize};
use tera::{Context, Tera};

use crate::internal;
use crate::traits::SrcCode;

/// Represent the declaration of a associated type in a trait
#[derive(Serialize, Deserialize, Default, Clone)]
pub struct AssociatedTypeDeclaration {
    pub(crate) name: String,
    pub(crate) traits: Vec<String>,
    annotations: Vec<String>,
}

impl AssociatedTypeDeclaration {
    /// Create a new `AssociatedTypeDeclaration`
    pub fn new<S: ToString>(name: S) -> Self {
        Self {
            name: name.to_string(),
            ..Self::default()
        }
    }
}

impl internal::Annotations for AssociatedTypeDeclaration {
    fn annotations(&mut self) -> &mut Vec<String> {
        &mut self.annotations
    }
}

impl internal::TraitBounds for AssociatedTypeDeclaration {
    fn trait_bounds(&mut self) -> &mut Vec<String> {
        &mut self.traits
    }
}

impl SrcCode for AssociatedTypeDeclaration {
    fn generate(&self) -> String {
        let template = r#"{% if has_annotations %}{{ self.annotations | join(sep="
        ") }}
        {% endif %}type {{ self.name }}{% if has_traits %}: {{ self.traits | join(sep=" + ") }}{% endif %};
        "#;
        let mut context = Context::new();
        context.insert("self", &self);
        context.insert("has_traits", &!self.traits.is_empty());
        context.insert("has_annotations", &!self.annotations.is_empty());
        Tera::one_off(template, &context, false).unwrap()
    }
}

/// Represent the definition of a associated type in a trait implementation
#[derive(Serialize, Deserialize, Default, Clone)]
pub struct AssociatedTypeDefinition {
    pub(crate) name: String,
    pub(crate) implementer: String,
    annotations: Vec<String>,
}

impl AssociatedTypeDefinition {
    /// Create a new `AssociatedTypeDefinition`
    pub fn new<N: ToString, I: ToString>(name: N, implementer: I) -> Self {
        AssociatedTypeDefinition {
            name: name.to_string(),
            implementer: implementer.to_string(),
            ..Self::default()
        }
    }
}

impl internal::Annotations for AssociatedTypeDefinition {
    fn annotations(&mut self) -> &mut Vec<String> {
        &mut self.annotations
    }
}

impl SrcCode for AssociatedTypeDefinition {
    fn generate(&self) -> String {
        let template = r#"{% if has_annotations %}{{ self.annotations | join(sep="
        ") }}
        {% endif %}type {{ self.name }} = {{ self.implementer }};
        "#;
        let mut context = Context::new();
        context.insert("self", &self);
        context.insert("has_annotations", &!self.annotations.is_empty());
        Tera::one_off(template, &context, false).unwrap()
    }
}
