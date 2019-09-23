use tera::{Context, Tera};
use serde::{Serialize};

use crate::prelude::*;
use crate::traits::SrcCode;

#[derive(Default, Serialize)]
pub struct Field {
    pub is_pub: bool,
    pub name: String,
    pub ty: String,
    pub annotations: Vec<String>,
    pub docs: Vec<String>,
}

impl Field {
    pub fn new<S: ToString>(name: S, ty: S, is_pub: bool) -> Self {
        let mut f = Field::default();
        f.name = name.to_string();
        f.ty = ty.to_string();
        f.is_pub = is_pub;
        f
    }
}

impl SrcCode for Field {
    fn generate(&self) -> String {
        let template = r#"
            {% for doc in field.docs %}{{ doc }}{% endfor %}
            {% for annotation in field.annotations %}{{ annotation }}{% endfor %}
            {% if field.is_pub %}pub{% endif %} {{ field.name }}: {{ field.ty }},
        "#;
        let mut context = Context::new();
        context.insert("field", &self);
        Tera::one_off(template, &context, false).unwrap()
    }
}
