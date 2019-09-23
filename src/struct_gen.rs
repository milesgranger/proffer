use tera::{Context, Tera};
use serde::{Serialize};

use crate::{Field};
use crate::traits::SrcCode;


#[derive(Default, Serialize)]
pub struct Struct {
    pub is_pub: bool,
    pub name: String,
    pub fields: Vec<Field>,
    pub docs: Vec<String>
}

impl Struct {
    pub fn new<S: ToString>(name: S, is_pub: bool) -> Self {
        let mut s = Struct::default();
        s.name = name.to_string();
        s.is_pub = is_pub;
        s
    }

    pub fn field(&mut self, field: Field) {
        self.fields.push(field)
    }
}

impl SrcCode for Struct {
    fn generate(&self) -> String {
        let template = r#"
         {% if struct.is_pub %}pub{% endif %} struct {{ struct.name }} {
            {% for field in fields %}{{field}}{% endfor %}
         }
        "#;
        let mut context = Context::new();
        context.insert("struct", &self);

        let fields = self.fields.iter()
            .map(|f| f.generate())
            .collect::<Vec<String>>();
        context.insert("fields", &fields);

        Tera::one_off(template, &context, false).unwrap()
    }
}


#[cfg(test)]
mod tests {

    use crate::*;

    #[test]
    fn test_struct_gen() {
        let mut struct_ = Struct::new("Basic", true);

        let mut f = Field::new("field1", "String", true);
        f.annotations.push("#[serde = w]".to_string());
        f.docs.push("/// Some example documentation".to_string());
        struct_.field(f);

        struct_.field(Field::new("field2", "usize", true));
        let expected = r#"
            pub struct Basic {
                pub field: usize
            }
        "#.to_owned();
        println!("{}", struct_.generate());
        //assert_eq!(src_code, expected);
    }
}