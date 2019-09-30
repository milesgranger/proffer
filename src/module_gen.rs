//!
//! Create a `Module` which can hold any number of other `SrcCode` elements.
//!
//! Example
//! -------
//!
//! ```
//!  use proffer::*;
//!  let m = Module::new("foo")
//!     .set_is_pub(true)
//!     .add_trait(Trait::new("Bar"))
//!     .add_function(Function::new("foo"))
//!     .add_struct(Struct::new("Thingy"))
//!     .add_impl(Impl::new("Thingy"))
//!     .add_outer_annotation("#[special_outer_annotation]")
//!     .add_inner_annotation("#![special_inner_annotation]")
//!     .add_doc("//! Module level docs");
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
//!          trait Bar
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
    enums: Vec<Enum>,
    docs: Vec<String>,
    sub_modules: Vec<Module>,
    inner_annotations: Vec<String>,
    outer_annotations: Vec<String>,
    use_stmts: Vec<String>,
}

impl Module {
    /// Create a new module
    pub fn new<S: ToString>(name: S) -> Self {
        let mut m = Module::default();
        m.name = name.to_string();
        m
    }
    /// Set if this module is public
    pub fn set_is_pub(mut self, is_pub: bool) -> Self {
        self.is_pub = is_pub;
        self
    }
    /// Add submodule
    pub fn add_submodule(mut self, module: Module) -> Self {
        self.sub_modules.push(module);
        self
    }
    /// Add a function to the module
    pub fn add_function(mut self, func: Function) -> Self {
        self.functions.push(func);
        self
    }
    /// Add a trait to the module
    pub fn add_trait(mut self, tr8t: Trait) -> Self {
        self.traits.push(tr8t);
        self
    }
    /// Add a struct to the module
    pub fn add_struct(mut self, stct: Struct) -> Self {
        self.structs.push(stct);
        self
    }
    /// Add an impl block to the module
    pub fn add_impl(mut self, iml: Impl) -> Self {
        self.impls.push(iml);
        self
    }
    /// Add a `use` statement or similar module level statements
    pub fn add_use_statement<S: ToString>(mut self, stmt: S) -> Self {
        self.use_stmts.push(stmt.to_string());
        self
    }
    /// Add outer module annotations
    pub fn add_outer_annotation<S: ToString>(mut self, ann: S) -> Self {
        self.outer_annotations.push(ann.to_string());
        self
    }
    /// Add inner module annotations
    pub fn add_inner_annotation<S: ToString>(mut self, ann: S) -> Self {
        self.inner_annotations.push(ann.to_string());
        self
    }
    /// Add a doc string to this module
    pub fn add_doc<S: ToString>(mut self, doc: S) -> Self {
        self.docs.push(doc.to_string());
        self
    }
    /// Add an enum to the module
    pub fn add_enum(mut self, enumm: Enum) -> Self {
        self.enums.push(enumm);
        self
    }
}

impl SrcCode for Module {
    fn generate(&self) -> String {
        let template = r#"
        {% for annotation in self.outer_annotations %}{{ annotation }}{% endfor %}
        {% if self.is_pub %}pub {% endif %}mod {{ self.name }}
        {
            {% for stmt in self.use_stmts %}{{ stmt }}{% endfor %}

            {% for annotation in self.inner_annotations %}{{ annotation }}{% endfor %}
            {% for doc in self.docs %}{{ doc }}{% endfor %}
            {% for obj in objs %}{{ obj }}{% endfor %}
            {% for sub_mod in submodules %}{{ sub_mod }}{% endfor %}
        }
        "#;

        let mut ctx = Context::new();
        ctx.insert("self", &self);

        let mut objs: Vec<String> = vec![];
        &self.traits.iter().for_each(|v| objs.push(v.generate()));
        &self.functions.iter().for_each(|v| objs.push(v.generate()));
        &self.structs.iter().for_each(|v| objs.push(v.generate()));
        &self.impls.iter().for_each(|v| objs.push(v.generate()));
        &self.enums.iter().for_each(|v| objs.push(v.generate()));
        ctx.insert("objs", &objs);

        ctx.insert(
            "submodules",
            &self
                .sub_modules
                .iter()
                .map(|m| m.generate())
                .collect::<Vec<String>>(),
        );
        Tera::one_off(template, &ctx, false).unwrap()
    }
}
