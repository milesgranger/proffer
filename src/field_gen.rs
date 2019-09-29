//!
//!
//! `Field` generation module, represents single fields within a `struct`
//!

use serde::Serialize;
use tera::{Context, Tera};

use crate::*;

/// Create a field
///
/// Example
/// -------
/// ```
/// use proffer::*;
///
/// let field = Field::new("foo", "usize", true)
///     .generate();
/// let expected = "pub foo: usize,";
///
/// assert_eq!(norm_whitespace(&field), norm_whitespace(expected));
/// ```
#[derive(Default, Serialize)]
pub struct Field {
    name: String,
    is_pub: bool,
    ty: String,
    annotations: Vec<String>,
    docs: Vec<String>,
}

impl Field {
    /// Create a new `Field`
    pub fn new<S: ToString>(name: S, ty: S, is_pub: bool) -> Self {
        let mut f = Field::default();
        f.name = name.to_string();
        f.ty = ty.to_string();
        f.is_pub = is_pub;
        f
    }

    /// Add a single field annotation. ie `#[serde(rename="something")`
    pub fn add_annotation<S: ToString>(&mut self, annotation: S) {
        self.annotations.push(annotation.to_string())
    }

    /// Add multiple field annotations at once.
    pub fn add_annotations<S: ToString, I: IntoIterator<Item = S>>(&mut self, annotations: I) {
        annotations.into_iter().for_each(|a| self.add_annotation(a))
    }

    /// Add a single documentation line for this field
    pub fn add_doc<S: ToString>(&mut self, doc: S) {
        self.docs.push(doc.to_string())
    }

    /// Add multiple documentation lines at once.
    pub fn add_docs<S: ToString, I: IntoIterator<Item = S>>(&mut self, docs: I) {
        docs.into_iter().for_each(|d| self.add_doc(d))
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
