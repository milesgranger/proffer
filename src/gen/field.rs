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
/// let field = Field::new("foo", "usize")
///     .generate();
/// let expected = "foo: usize,";
///
/// assert_eq!(norm_whitespace(&field), norm_whitespace(expected));
/// ```
#[derive(Default, Serialize, Clone)]
pub struct Field {
    name: String,
    is_pub: bool,
    ty: String,
    annotations: Vec<String>,
    docs: Vec<String>,
}

impl Field {
    /// Create a new `Field`
    pub fn new(name: impl ToString, ty: impl ToString) -> Self {
        Self {
            name: name.to_string(),
            ty: ty.to_string(),
            ..Self::default()
        }
    }

    /// Set if this is public
    pub fn set_is_pub(&mut self, is_pub: bool) -> &mut Self {
        self.is_pub = is_pub;
        self
    }
}

impl internal::Annotations for Field {
    fn annotations_mut(&mut self) -> &mut Vec<String> {
        &mut self.annotations
    }
}

impl internal::Docs for Field {
    fn docs_mut(&mut self) -> &mut Vec<String> {
        &mut self.docs
    }
}

impl SrcCode for Field {
    fn generate(&self) -> String {
        let template = r#"
            {{ field.docs | join(sep="
            ") }}
            {% for annotation in field.annotations %}{{ annotation }}{% endfor %}
            {% if field.is_pub %}pub{% endif %} {{ field.name }}: {{ field.ty }},
        "#;
        let mut context = Context::new();
        context.insert("field", &self);
        Tera::one_off(template, &context, false).unwrap()
    }
}
