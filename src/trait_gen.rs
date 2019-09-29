//!
//!
//! Create a `trait` definition
//!
use serde::Serialize;

use crate::traits::SrcCode;
use crate::{FunctionSignature, Generic, Generics};
use tera::{Context, Tera};

/// Represents a `trait` block.
///
/// Example
/// -------
/// ```
/// use proffer::*;
/// let mut tr8t = Trait::new("Foo", true);
/// tr8t.add_signature(FunctionSignature::new("bar", false));
/// let expected = r#"
///     pub trait Foo
///     {
///         fn bar() -> ();
///     }
/// "#;
/// assert_eq!(
///     norm_whitespace(expected),
///     norm_whitespace(tr8t.generate().as_str())
/// )
/// ```
#[derive(Serialize, Default)]
pub struct Trait {
    pub(crate) name: String,
    pub(crate) is_pub: bool,
    generics: Generics,
    signatures: Vec<FunctionSignature>,
}

impl Trait {
    /// Create a new `trait`
    pub fn new<S: ToString>(name: S, is_pub: bool) -> Self {
        let mut t = Self::default();
        t.name = name.to_string();
        t.is_pub = is_pub;
        t
    }

    /// Add a new signature requirement to this trait.
    pub fn add_signature(&mut self, signature: FunctionSignature) {
        self.signatures.push(signature)
    }

    /// Add a generic bound to this trait.
    pub fn add_generic(&mut self, generic: Generic) {
        self.generics.add_generic(generic)
    }
}

impl SrcCode for Trait {
    fn generate(&self) -> String {
        let template = r#"
            {% if self.is_pub %}pub {% endif %}trait {{ self.name }}{% if has_generics %}{{ generic_bounds }}{% endif %}
            {
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
        context.insert("has_generics", &!self.generics.is_empty());
        context.insert("generic_bounds", &self.generics.generate());
        Tera::one_off(template, &context, false).unwrap()
    }
}
