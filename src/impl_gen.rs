//!
//! Create `impl` blocks for functions, traits, and other objects.
//!

use serde::Serialize;

use crate::traits::SrcCode;
use crate::{Function, Generic, Generics, Trait};
use tera::{Context, Tera};

/// Represents an `impl` block
#[derive(Serialize, Default)]
pub struct Impl {
    generics: Generics,
    impl_trait: Option<Trait>,
    functions: Vec<Function>,
    obj_name: String,
}

impl Impl {
    /// Create a new impl block
    pub fn new<S: ToString>(obj_name: S) -> Self {
        let mut mpl = Self::default();
        mpl.obj_name = obj_name.to_string();
        mpl
    }

    /// Set if this `impl` is implementing a `Trait` for an object.
    pub fn set_impl_trait(&mut self, impl_trait: Option<Trait>) {
        self.impl_trait = impl_trait
    }

    /// Add a function to this `Impl` block
    pub fn add_function(&mut self, func: Function) {
        self.functions.push(func)
    }

    /// Add a generic to this `Impl` block
    pub fn add_generic(&mut self, generic: Generic) {
        self.generics.add_generic(generic)
    }
}

impl SrcCode for Impl {
    fn generate(&self) -> String {
        let template = r#"
            impl{% if has_generics %}<{{ generic_keys | join(sep=", ") }}>{% endif %} {% if has_trait %}{{ trait_name }} for {% endif %}{{ self.obj_name }}{% if has_generics %}<{{ generic_keys | join(sep=", ") }}>{% endif %}
                {% if has_generics %}
                where
                    {% for generic in generics %}{{ generic.generic }}: {{ generic.traits | join(sep=" + ") }},
                    {% endfor %}
                {% endif %}
            {
                {% for function in functions %}
                    {{ function }}
                {% endfor %}
            }
        "#;
        let mut context = Context::new();
        context.insert("self", &self);
        context.insert("has_trait", &self.impl_trait.is_some());
        context.insert(
            "trait_name",
            &self
                .impl_trait
                .as_ref()
                .map(|t| t.name.clone())
                .unwrap_or("".to_string()),
        );
        context.insert("has_generics", &!self.generics.is_empty());
        context.insert("generics", &self.generics.generics);
        context.insert(
            "generic_keys",
            &self
                .generics
                .generics
                .iter()
                .map(|g| g.generic.clone())
                .collect::<Vec<String>>(),
        );
        context.insert(
            "functions",
            &self
                .functions
                .iter()
                .map(|f| f.generate())
                .collect::<Vec<String>>(),
        );
        Tera::one_off(template, &context, false).unwrap()
    }
}
