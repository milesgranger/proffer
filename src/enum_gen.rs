//!
//! Create `enum` objects
//!
//!
//! Example
//! -------
//! ```
//! use proffer::*;
//!
//! let e = Enum::new("Foo")
//!     .add_variant(Variant::new("A"))
//!     .add_variant(Variant::new("B").set_inner(Some("(T)")).to_owned())
//!     .add_generic(Generic::new("T"))
//!     .to_owned();
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
#[derive(Default, Serialize, Clone)]
pub struct Enum {
    name: String,
    generics: Generics,
    is_pub: bool,
    variants: Vec<Variant>,
}

/// Represent an enum variant/arm
#[derive(Default, Serialize, Clone)]
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
    pub fn set_is_pub(&mut self, is_pub: bool) -> &mut Self {
        self.is_pub = is_pub;
        self
    }
    /// Add a variant
    pub fn add_variant(&mut self, variant: Variant) -> &mut Self {
        self.variants.push(variant);
        self
    }
    /// Add a generic bound to this Enum
    pub fn add_generic(&mut self, generic: Generic) -> &mut Self {
        self.generics.add_generic(generic);
        self
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
    pub fn set_inner<S: ToString>(&mut self, inner: Option<S>) -> &mut Self {
        self.inner = inner.map(|s| s.to_string());
        self
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
