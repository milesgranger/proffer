//!
//! Create `impl` blocks for functions, traits, and other objects.
//!

use serde::Serialize;

use crate::traits::SrcCode;
use crate::{internal, AssociatedTypeDefinition, Function, Generic, Generics, SrcCodeVec, Trait};
use tera::{Context, Tera};

/// Represents an `impl` block
#[derive(Serialize, Default, Clone)]
pub struct Impl {
    generics: Generics,
    impl_trait: Option<Trait>,
    functions: Vec<Function>,
    obj_name: String,
    associated_types: Vec<AssociatedTypeDefinition>,
}

impl Impl {
    /// Create a new impl block
    pub fn new(obj_name: impl ToString) -> Self {
        Self {
            obj_name: obj_name.to_string(),
            ..Self::default()
        }
    }

    /// Set if this `impl` is implementing a `Trait` for an object.
    pub fn set_impl_trait(&mut self, impl_trait: Option<Trait>) -> &mut Self {
        self.impl_trait = impl_trait;
        self
    }

    /// Add a function to this `Impl` block
    pub fn add_function(&mut self, func: Function) -> &mut Self {
        self.functions.push(func);
        self
    }

    /// Add a associated type to this `Impl` block
    pub fn add_associated_type(&mut self, associated_type: AssociatedTypeDefinition) -> &mut Self {
        self.associated_types.push(associated_type);
        self
    }
}

impl internal::Generics for Impl {
    fn generics(&mut self) -> &mut Vec<Generic> {
        self.generics.generics()
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
                {% for associated_type in associated_types %}{{ associated_type }}{% endfor %}
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
                .map_or_else(|| "".to_string(), |t| t.name.clone()),
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
        context.insert("functions", &self.functions.to_src_vec());
        context.insert("associated_types", &self.associated_types.to_src_vec());
        Tera::one_off(template, &context, false).unwrap()
    }
}
