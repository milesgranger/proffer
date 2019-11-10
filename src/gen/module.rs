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
//!     .add_attribute("#[special_outer_attribute]")
//!     .add_attribute("#![special_inner_attribute]")
//!     .add_doc("//! Module level docs")
//!     .to_owned();
//!
//! let src_code = m.generate();
//!
//!  let expected = r#"
//!      #[special_outer_attribute]
//!      pub mod foo
//!      {
//!          #![special_inner_attribute]
//!          //! Module level docs
//!
//!          trait Bar
//!          {
//!          }
//!
//!          fn foo() -> ()
//!          {
//!          }
//!
//!          struct Thingy
//!          {
//!          }
//!
//!          impl Thingy
//!          {
//!          }
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
use std::borrow::Borrow;
use std::collections::HashMap;
use std::hash::Hash;

/// Represent a module of code
///
/// Example
/// -------
/// ```
/// use proffer::*;
///
/// let m = Module::new("foo")
///        .set_is_pub(true)
///        .add_trait(Trait::new("Bar").set_is_pub(true).to_owned())
///        .add_function(Function::new("foo"))
///        .add_struct(Struct::new("Thingy"))
///        .add_impl(Impl::new("Thingy"))
///        .add_attribute("#[special_outer_attribute]")
///        .add_attribute("#![special_inner_attribute]")
///        .add_doc("//! Module level docs")
///        .add_use_statement("use super::*;")
///        .add_enum(Enum::new("EnumThingy"))
///        .to_owned();
/// ```
#[derive(Default, Serialize, Clone)]
pub struct Module {
    name: String,
    is_pub: bool,
    traits: Vec<Trait>,
    functions: Vec<Function>,
    structs: Vec<Struct>,
    impls: Vec<Impl>,
    enums: Vec<Enum>,
    docs: Vec<String>,
    sub_modules: HashMap<String, Module>,
    attributes: Vec<Attribute>,
    use_stmts: Vec<String>,
}

impl Module {
    /// Create a new module
    pub fn new(name: impl ToString) -> Self {
        Self {
            name: name.to_string(),
            ..Self::default()
        }
    }
    /// Set if this module is public
    pub fn set_is_pub(&mut self, is_pub: bool) -> &mut Self {
        self.is_pub = is_pub;
        self
    }
    /// Add submodule
    pub fn add_submodule(&mut self, module: Module) -> &mut Self {
        self.sub_modules.insert(module.name.clone(), module);
        self
    }
    /// Get a mutable reference to a submodule of this module
    pub fn get_submodule_mut<Q>(&mut self, name: &Q) -> Option<&mut Module>
    where
        String: Borrow<Q>,
        Q: ?Sized + Hash + Eq,
    {
        self.sub_modules.get_mut(name)
    }
    /// Get a reference to a submodule of this module.
    pub fn get_submodule<Q>(&self, name: &Q) -> Option<&Module>
    where
        String: Borrow<Q>,
        Q: ?Sized + Hash + Eq,
    {
        self.sub_modules.get(name)
    }
    /// Add a function to the module
    pub fn add_function(&mut self, func: Function) -> &mut Self {
        self.functions.push(func);
        self
    }
    /// Add a trait to the module
    pub fn add_trait(&mut self, tr8t: Trait) -> &mut Self {
        self.traits.push(tr8t);
        self
    }
    /// Add a struct to the module
    pub fn add_struct(&mut self, stct: Struct) -> &mut Self {
        self.structs.push(stct);
        self
    }
    /// Add an impl block to the module
    pub fn add_impl(&mut self, iml: Impl) -> &mut Self {
        self.impls.push(iml);
        self
    }
    /// Add a `use` statement or similar module level statements
    pub fn add_use_statement(&mut self, stmt: impl ToString) -> &mut Self {
        self.use_stmts.push(stmt.to_string());
        self
    }
    /// Add an enum to the module
    pub fn add_enum(&mut self, enumm: Enum) -> &mut Self {
        self.enums.push(enumm);
        self
    }
}

impl internal::Attributes for Module {
    fn attributes_mut(&mut self) -> &mut Vec<Attribute> {
        &mut self.attributes
    }
}

impl internal::Docs for Module {
    fn docs_mut(&mut self) -> &mut Vec<String> {
        &mut self.docs
    }
}

impl SrcCode for Module {
    fn generate(&self) -> String {
        let template = r#"
        {{ item_attributes | join(sep="
        ") }}
        {% if self.is_pub %}pub {% endif %}mod {{ self.name }}
        {
            {{ scope_attributes | join(sep="
            ") }}
            {% for doc in self.docs %}{{ doc }}{% endfor %}

            {% for stmt in self.use_stmts %}{{ stmt }}{% endfor %}
            {% for obj in objs %}{{ obj }}{% endfor %}
            {% for sub_mod in submodules %}{{ sub_mod }}{% endfor %}
        }
        "#;

        let mut ctx = Context::new();
        ctx.insert("self", &self);
        ctx.insert(
            "item_attributes",
            &self
                .attributes
                .iter()
                .filter_map(|ann| match ann {
                    Attribute::ItemAttr(a) => Some(a),
                    Attribute::ScopeAttr(_) => None,
                })
                .collect::<Vec<&String>>(),
        );
        ctx.insert(
            "scope_attributes",
            &self
                .attributes
                .iter()
                .filter_map(|ann| match ann {
                    Attribute::ItemAttr(_) => None,
                    Attribute::ScopeAttr(a) => Some(a),
                })
                .collect::<Vec<&String>>(),
        );
        let mut objs: Vec<String> = vec![];
        self.traits.iter().for_each(|v| objs.push(v.generate()));
        self.functions.iter().for_each(|v| objs.push(v.generate()));
        self.structs.iter().for_each(|v| objs.push(v.generate()));
        self.impls.iter().for_each(|v| objs.push(v.generate()));
        self.enums.iter().for_each(|v| objs.push(v.generate()));
        ctx.insert("objs", &objs);

        ctx.insert(
            "submodules",
            &self
                .sub_modules
                .values()
                .map(|m| m.generate())
                .collect::<Vec<String>>(),
        );
        Tera::one_off(template, &ctx, false).unwrap()
    }
}
