//!
//! Create `enum` objects
//!
//!
//! Example
//! -------
//! ```
//! use proffer::*;
//!
//! let mut e = Enum::new("Foo");
//!
//! e.add_variant(Variant::new("A"));
//!
//! let mut v = Variant::new("B");
//! v.set_inner(Some("(T)"));
//! e.add_variant(v);
//!
//! e.add_generic(Generic::new("T", vec![]));
//!
//! let src_code = e.generate();
//! let expected = r#"
//!     enum Foo<T>
//!         where
//!             T: ,
//!     {
//!         A,
//!         B(T),
//!     }
//! "#;
//! assert_eq!(norm_whitespace(expected), norm_whitespace(&src_code))
//! ```

use serde::Serialize;

use crate::*;
use tera::{Context, Tera};

/// Represent an `enum` object
#[derive(Default, Serialize)]
pub struct Enum {
    name: String,
    generics: Generics,
    is_pub: bool,
    variants: Vec<Variant>,
}

/// Represent an enum variant/arm
#[derive(Default, Serialize)]
pub struct Variant {
    name: String,
    inner: Option<String>,
}

impl Enum {
    /// Create a new `Enum`
    pub fn new<S: ToString>(name: S) -> Self {
        let mut e = Enum::default();
        e.name = name.to_string();
        e
    }
    /// Set if this is public
    pub fn set_is_pub(&mut self, is_pub: bool) {
        self.is_pub = is_pub;
    }
    /// Add a variant
    pub fn add_variant(&mut self, variant: Variant) {
        self.variants.push(variant)
    }
    /// Add a generic bound to this Enum
    pub fn add_generic(&mut self, generic: Generic) {
        self.generics.add_generic(generic)
    }
}

impl Variant {
    /// Create a new variant to add to an `Enum`
    pub fn new<S: ToString>(name: S) -> Self {
        let mut v = Variant::default();
        v.name = name.to_string();
        v
    }
    /// Set the inner portion of this variant, expected to be valid Rust source code.
    pub fn set_inner<S: ToString>(&mut self, inner: Option<S>) {
        self.inner = inner.map(|s| s.to_string());
    }
}

impl SrcCode for Variant {
    fn generate(&self) -> String {
        let template = r#"{{ self.name }}{{ inner }}"#;
        let mut ctx = Context::new();
        ctx.insert("self", &self);
        ctx.insert("inner", &self.inner.as_ref().unwrap_or(&"".to_string()));
        Tera::one_off(template, &ctx, false).unwrap()
    }
}

impl SrcCode for Enum {
    fn generate(&self) -> String {
        let template = r#"
            {% if self.is_pub %}pub {% endif %}enum {{ self.name }}{{ generics }} {
                {% for variant in variants %}{{ variant }},
                {% endfor %}
            }
        "#;
        let mut ctx = Context::new();
        ctx.insert("self", &self);
        ctx.insert("generics", &self.generics.generate());
        ctx.insert(
            "variants",
            &self
                .variants
                .iter()
                .map(|v| v.generate())
                .collect::<Vec<String>>(),
        );
        Tera::one_off(template, &ctx, false).unwrap()
    }
}
