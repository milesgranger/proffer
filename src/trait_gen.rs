//!
//!
//! Create a `trait` definition
//!
use serde::Serialize;

use crate::traits::SrcCode;
use crate::{internal, AssociatedTypeDeclaration, FunctionSignature, Generic, Generics};
use tera::{Context, Tera};

/// Represents a `trait` block.
///
/// Example
/// -------
/// ```
/// use proffer::*;
/// let tr8t = Trait::new("Foo")
///     .add_signature(FunctionSignature::new("bar"))
///     .to_owned();
/// let expected = r#"
///     trait Foo
///     {
///         fn bar() -> ();
///     }
/// "#;
/// assert_eq!(
///     norm_whitespace(expected),
///     norm_whitespace(tr8t.generate().as_str())
/// )
/// ```
#[derive(Serialize, Default, Clone)]
pub struct Trait {
    pub(crate) name: String,
    pub(crate) is_pub: bool,
    generics: Generics,
    signatures: Vec<FunctionSignature>,
    associated_types: Vec<AssociatedTypeDeclaration>,
}

impl Trait {
    /// Create a new `trait`
    pub fn new<S: ToString>(name: S) -> Self {
        Self {
            name: name.to_string(),
            ..Self::default()
        }
    }

    /// Add a new signature requirement to this trait.
    pub fn add_signature(&mut self, signature: FunctionSignature) -> &mut Self {
        self.signatures.push(signature);
        self
    }

    /// Set if this is a `pub` trait
    pub fn set_is_pub(&mut self, is_pub: bool) -> &mut Self {
        self.is_pub = is_pub;
        self
    }

    /// Add a associated type to this trait.
    pub fn add_associated_type(&mut self, associated_type: AssociatedTypeDeclaration) -> &mut Self {
        self.associated_types.push(associated_type);
        self
    }
}

impl internal::Generics for Trait {
    fn generics(&mut self) -> &mut Vec<Generic> {
        self.generics.generics()
    }
}

impl SrcCode for Trait {
    fn generate(&self) -> String {
        let template = r#"
            {% if self.is_pub %}pub {% endif %}trait {{ self.name }}{% if has_generics %}{{ generic_bounds }}{% endif %}
            {
                {% for associated_type in associated_types %}{{ associated_type }}{% endfor %}
                {% for signature in signatures %}{{ signature }};{% endfor %}
            }
        "#;
        let mut context = Context::new();
        context.insert("self", &self);
        context.insert(
            "signatures",
            &self
                .signatures
                .iter()
                .map(|s| s.generate())
                .collect::<Vec<String>>(),
        );
        context.insert(
            "associated_types",
            &self
                .associated_types
                .iter()
                .map(|t| t.generate())
                .collect::<Vec<String>>(),
        );
        context.insert("has_generics", &!self.generics.is_empty());
        context.insert("generic_bounds", &self.generics.generate());
        Tera::one_off(template, &context, false).unwrap()
    }
}
