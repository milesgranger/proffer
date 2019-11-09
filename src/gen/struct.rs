//!
//!
//! Create a `struct` object.
//!

use serde::Serialize;
use tera::{Context, Tera};

use crate::*;

/// Represents a `struct` in source code.
///
/// Example
/// -------
/// ```
/// use proffer::*;
///
/// let s = Struct::new("FooBar")
///     .set_is_pub(true)
///     .add_field(
///         Field::new("foo", "String")
///     )
///     .add_attribute("#[derive(Clone)]")
///     .to_owned();
/// ```
///
#[derive(Default, Serialize, Clone)]
pub struct Struct {
    is_pub: bool,
    name: String,
    fields: Vec<Field>,
    generics: Vec<Generic>,
    docs: Vec<String>,
    attributes: Vec<Attribute>,
}

impl Struct {
    /// Create a new `Struct`
    pub fn new(name: impl ToString) -> Self {
        Self {
            name: name.to_string(),
            ..Self::default()
        }
    }

    /// Set if this struct is `pub`
    pub fn set_is_pub(&mut self, is_pub: bool) -> &mut Self {
        self.is_pub = is_pub;
        self
    }
}

impl internal::Fields for Struct {
    fn fields_mut(&mut self) -> &mut Vec<Field> {
        &mut self.fields
    }
}

impl internal::Attributes for Struct {
    fn attributes_mut(&mut self) -> &mut Vec<Attribute> {
        &mut self.attributes
    }
}

impl internal::Generics for Struct {
    fn generics_mut(&mut self) -> &mut Vec<Generic> {
        &mut self.generics
    }
    fn generics(&self) -> &[Generic] {
        self.generics.as_slice()
    }
}

impl internal::Docs for Struct {
    fn docs_mut(&mut self) -> &mut Vec<String> {
        &mut self.docs
    }
}

impl SrcCode for Struct {
    fn generate(&self) -> String {
        let template = r#"
        {{ struct.docs | join(sep="
        ") }}
        {{ attributes | join(sep="
        ") }}
        {% if struct.is_pub %}pub {% endif %}struct {{ struct.name }}{{ generics }}
        {
            {% for field in fields %}{{ field }}{% endfor %}
        }
        "#;
        let mut context = Context::new();
        context.insert("struct", &self);
        context.insert("fields", &self.fields.to_src_vec());
        context.insert("generics", &self.generics.generate());
        context.insert("attributes", &self.attributes.to_src_vec());
        Tera::one_off(template, &context, false).unwrap()
    }
}
