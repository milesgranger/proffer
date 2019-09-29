use serde::Serialize;
use tera::{Context, Tera};

use crate::traits::SrcCode;
use crate::{Field, Generic, Generics, Impl};

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
    pub fn add_generic(&mut self, generic: Generic) {
        self.generics.add_generic(generic)
    }
}

impl SrcCode for Struct {
    fn generate(&self) -> String {
        let template = r#"
        {% if struct.is_pub %}pub {% endif %}struct {{ struct.name }}{{ generics }} {
            {% for field in fields %}{{ field }}{% endfor %}
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
        context.insert("generics", &self.generics.generate());
        Tera::one_off(template, &context, false).unwrap()
    }
}
