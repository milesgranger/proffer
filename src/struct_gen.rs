use serde::Serialize;
use tera::{Context, Tera};

use crate::traits::SrcCode;
use crate::Field;
use std::collections::HashMap;

type Generics = HashMap<String, Vec<String>>;

#[derive(Default, Serialize)]
pub struct Struct {
    pub is_pub: bool,
    pub name: String,
    pub fields: Vec<Field>,
    pub generics: Generics,
    pub docs: Vec<String>,
}

impl Struct {
    pub fn new<S: ToString>(name: S, is_pub: bool) -> Self {
        let mut s = Struct::default();
        s.name = name.to_string();
        s.is_pub = is_pub;
        s
    }

    pub fn add_field(&mut self, field: Field) {
        self.fields.push(field)
    }
    pub fn add_generic<S: ToString>(&mut self, id: S, traits: Vec<S>) {
        self.generics.insert(
            id.to_string(),
            traits.into_iter().map(|s| s.to_string()).collect(),
        );
    }
}

impl SrcCode for Struct {
    fn generate(&self) -> String {
        let template = r#"
         {% if struct.is_pub %}pub{% endif %} struct {{ struct.name }}{% if has_generics %}<{% for generic, _ in struct.generics %}{{generic}},{% endfor %}>{% endif %}
         {% if has_generics %}  where
            {% for generic_id, generics in struct.generics %}{{ generic_id }}: {% for g in generics %}{{ g }}{% endfor %},
            {% endfor %}{% endif %}
         {
            {% for field in fields %}{{field}}{% endfor %}
         }
        "#;
        let mut context = Context::new();
        context.insert("struct", &self);

        let fields = self
            .fields
            .iter()
            .map(|f| f.generate())
            .collect::<Vec<String>>();
        context.insert("fields", &fields);
        context.insert("has_generics", &(self.generics.len() > 0));

        Tera::one_off(template, &context, false).unwrap()
    }
}
