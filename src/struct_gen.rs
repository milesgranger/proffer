//!
//!
//! Create a `struct` object.
//!

use serde::Serialize;
use tera::{Context, Tera};

use crate::*;

/// Represents a `struct` in source code.
#[derive(Default, Serialize)]
pub struct Struct {
    is_pub: bool,
    name: String,
    fields: Vec<Field>,
    generics: Generics,
    docs: Vec<String>,
}

impl Struct {
    /// Create a new `Struct`
    pub fn new<S: ToString>(name: S, is_pub: bool) -> Self {
        let mut s = Struct::default();
        s.name = name.to_string();
        s.is_pub = is_pub;
        s
    }

    /// Add a new field to this struct
    pub fn add_field(&mut self, field: Field) {
        self.fields.push(field)
    }

    /// Add a trait bound to this struct
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
