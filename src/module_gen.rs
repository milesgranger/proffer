//!
//! Create a `Module` which can hold any number of other `SrcCode` elements.
//!
//! Example
//! -------
//!
//! ```
//!  use proffer::*;
//!  let mut m = Module::new("foo");
//!  m.set_is_pub(true);
//!  m.add_trait(Trait::new("Bar", true));
//!  m.add_function(Function::new("foo", false));
//!  m.add_struct(Struct::new("Thingy", false));
//!  m.add_impl(Impl::new("Thingy", None));
//!  m.add_outer_annotation("#[special_outer_annotation]");
//!  m.add_inner_annotation("#![special_inner_annotation]");
//!  m.add_doc("//! Module level docs");
//!
//! let src_code = m.generate();
//!
//!  let expected = r#"
//!      #[special_outer_annotation]
//!      pub mod foo
//!      {
//!          #![special_inner_annotation]
//!          //! Module level docs
//!
//!          pub trait Bar
//!          {
//!          }
//!          fn foo() -> ()
//!          {
//!          }
//!          struct Thingy {
//!          }
//!          impl Thingy
//!          {
//!          }
//!
//!      }
//!  "#;
//!  println!("{}", &src_code);
//!  assert_eq!(
//!      norm_whitespace(expected), norm_whitespace(&src_code)
//!  )
//! ```
//!

use serde::Serialize;
use tera::{Context, Tera};

use crate::*;


/// Represent a module of code
#[derive(Default, Serialize)]
pub struct Module {
    name: String,
    is_pub: bool,
    traits: Vec<Trait>,
    functions: Vec<Function>,
    structs: Vec<Struct>,
    impls: Vec<Impl>,
    docs: Vec<String>,
    inner_annotations: Vec<String>,
    outer_annotations: Vec<String>,
}

impl Module {
    /// Create a new module
    pub fn new<S: ToString>(name: S) -> Self {
        let mut m = Module::default();
        m.name = name.to_string();
        m
    }
    /// Set if this module is public
    pub fn set_is_pub(&mut self, is_pub: bool) {
        self.is_pub = is_pub;
    }
    /// Add a function to the module
    pub fn add_function(&mut self, func: Function) {
        self.functions.push(func)
    }
    /// Add a trait to the module
    pub fn add_trait(&mut self, tr8t: Trait) {
        self.traits.push(tr8t)
    }
    /// Add a struct to the module
    pub fn add_struct(&mut self, stct: Struct) {
        self.structs.push(stct)
    }
    /// Add an impl block to the module
    pub fn add_impl(&mut self, iml: Impl) {
        self.impls.push(iml)
    }
    /// Add outer module annotations
    pub fn add_outer_annotation<S: ToString>(&mut self, ann: S) {
        self.outer_annotations.push(ann.to_string())
    }
    /// Add inner module annotations
    pub fn add_inner_annotation<S: ToString>(&mut self, ann: S) {
        self.inner_annotations.push(ann.to_string())
    }
    /// Add a doc string to this module
    pub fn add_doc<S: ToString>(&mut self, doc: S) {
        self.docs.push(doc.to_string())
    }
}


impl SrcCode for Module {
    fn generate(&self) -> String {
        let template = r#"
        {% for annotation in self.outer_annotations %}{{ annotation }}{% endfor %}
        {% if self.is_pub %}pub {% endif %}mod {{ self.name }}
        {
            {% for annotation in self.inner_annotations %}{{ annotation }}{% endfor %}
            {% for doc in self.docs %}{{ doc }}{% endfor %}
            {% for obj in objs %}{{ obj }}{% endfor %}
        }
        "#;

        let mut ctx = Context::new();
        ctx.insert("self", &self);

        let mut objs: Vec<String> = vec![];
        &self.traits.iter().for_each(|v| objs.push(v.generate()));
        &self.functions.iter().for_each(|v| objs.push(v.generate()));
        &self.structs.iter().for_each(|v| objs.push(v.generate()));
        &self.impls.iter().for_each(|v| objs.push(v.generate()));

        ctx.insert("objs", &objs);
        Tera::one_off(template, &ctx, false).unwrap()
    }
}

