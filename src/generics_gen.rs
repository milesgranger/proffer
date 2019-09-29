use serde::{Deserialize, Serialize};
use tera::{Context, Tera};

use crate::traits::SrcCode;

#[derive(Serialize, Deserialize, Default)]
pub struct Generic {
    pub generic: String,
    pub traits: Vec<String>,
}

impl Generic {
    pub fn new<S: ToString>(id: S, traits: Vec<S>) -> Self {
        Self {
            generic: id.to_string(),
            traits: traits.into_iter().map(|s| s.to_string()).collect(),
        }
    }
}

#[derive(Serialize, Deserialize, Default)]
pub struct Generics {
    pub generics: Vec<Generic>,
}

impl Generics {
    pub fn new(generics: Vec<Generic>) -> Self {
        Self { generics }
    }
    pub fn add_generic(&mut self, generic: Generic) {
        self.generics.push(generic)
    }
    pub fn len(&self) -> usize {
        self.generics.len()
    }
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl SrcCode for Generics {
    fn generate(&self) -> String {
        if self.generics.len() > 0 {
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
