//!
//!
//! Function pieces, specifically `Function` which is composed of `FunctionSignature`
//! and `FunctionBody`. Naturally, a `Function` can be used as a "method" for another
//! object, by specifying `self` (or some variant of it) as the first `Parameter` to
//! a `Function` object.
//!

use serde::Serialize;
use tera::{Context, Tera};

use crate::traits::SrcCode;
use crate::{Generic, Generics};

/// Represents a function or method. Determined if any `Parameter` contains `self`
#[derive(Default, Serialize, Clone)]
pub struct Function {
    signature: FunctionSignature,
    body: FunctionBody,
}

/// Represents a function/method signature in source code
#[derive(Default, Serialize, Clone)]
pub struct FunctionSignature {
    name: String,
    is_pub: bool,
    is_async: bool,
    parameters: Vec<Parameter>,
    generics: Generics,
    return_ty: Option<String>,
}

impl FunctionSignature {
    /// Create a new function signature.
    pub fn new<S: ToString>(name: S) -> Self {
        let mut f = Self::default();
        f.name = name.to_string();
        f
    }

    /// Set this function as `async`
    pub fn set_is_async(&mut self, is_async: bool) -> &mut Self {
        self.is_async = is_async;
        self
    }

    /// Add a parameter to this signature
    pub fn add_parameter(&mut self, param: Parameter) -> &mut Self {
        self.parameters.push(param);
        self
    }

    /// Add a generic to this signature
    pub fn add_generic(&mut self, generic: Generic) -> &mut Self {
        self.generics.add_generic(generic);
        self
    }

    /// Set a return type, if `None` will result in `()` type.
    pub fn set_return_ty<S: ToString>(&mut self, ty: Option<S>) -> &mut Self {
        self.return_ty = ty.map(|s| s.to_string());
        self
    }

    /// Set if this signature should be prefixed with `pub`
    pub fn set_is_pub(&mut self, is_pub: bool) -> &mut Self {
        self.is_pub = is_pub;
        self
    }

    /// Set the name of this function.
    pub fn set_name<S: ToString>(&mut self, name: S) -> &mut Self {
        self.name = name.to_string();
        self
    }
}

impl SrcCode for FunctionSignature {
    fn generate(&self) -> String {
        let template = r#"
        {% if self.is_pub %}pub {% endif %}{% if self.is_async %}async {% endif %}fn {{ self.name }}{% if has_generics %}<{{ generic_keys | join(sep=", ") }}>{% endif %}({{ parameters | join(sep=", ") }}) -> {{ return_ty }}{% if has_generics %}
            where
                {% for generic in generics %}{{ generic.generic }}: {{ generic.traits | join(sep=" + ") }},
                {% endfor %}{% endif %}"#;
        let mut context = Context::new();
        context.insert("self", &self);
        context.insert(
            "return_ty",
            &self.return_ty.as_ref().unwrap_or(&"()".to_string()),
        );
        context.insert("has_generics", &!self.generics.is_empty());
        context.insert("generics", &self.generics.generics);
        context.insert(
            "generic_keys",
            &self
                .generics
                .generics
                .iter()
                .map(|g| g.generic.clone())
                .collect::<Vec<String>>(),
        );
        context.insert(
            "parameters",
            &self
                .parameters
                .iter()
                .map(|param| format!("{}: {}", param.name, param.ty))
                .collect::<Vec<String>>(),
        );
        Tera::one_off(template, &context, false).unwrap()
    }
}

/// Represents the function/method's body
#[derive(Default, Serialize, Clone)]
pub struct FunctionBody {
    body: String,
}

impl SrcCode for FunctionBody {
    fn generate(&self) -> String {
        let template = r#"
            {{ body }}
        "#;
        let mut ctx = Context::new();
        ctx.insert("body", &self.body);
        Tera::one_off(template, &ctx, false).unwrap()
    }
}

impl<S> From<S> for FunctionBody
where
    S: ToString,
{
    fn from(body: S) -> FunctionBody {
        FunctionBody {
            body: body.to_string(),
        }
    }
}

impl Function {
    /// Create a new function
    pub fn new<S: ToString>(name: S) -> Self {
        let mut f = Self::default();
        f.signature.name = name.to_string();
        f
    }

    /// Add a new parameter to this function
    pub fn add_parameter(&mut self, param: Parameter) -> &mut Self {
        self.signature.parameters.push(param);
        self
    }
    /// Add a new trait bound generic to this function
    pub fn add_generic(&mut self, generic: Generic) -> &mut Self {
        self.signature.generics.add_generic(generic);
        self
    }
    /// Set the return type of this function
    pub fn set_return_ty<S: ToString>(&mut self, ty: S) -> &mut Self {
        self.signature.return_ty = Some(ty.to_string());
        self
    }
    /// Set if this function is public
    pub fn set_is_pub(&mut self, is_pub: bool) -> &mut Self {
        self.signature.set_is_pub(is_pub);
        self
    }
    /// Set if this function is async
    pub fn set_is_async(&mut self, is_async: bool) -> &mut Self {
        self.signature.set_is_async(is_async);
        self
    }
    /// Set the body of the function, this should be valid Rust source code syntax.
    pub fn set_body<S: Into<FunctionBody>>(&mut self, body: S) -> &mut Self {
        self.body = body.into();
        self
    }
}

/// Represents a single parameter to a `Function`
#[derive(Serialize, Default, Clone)]
pub struct Parameter {
    name: String,
    ty: String,
}
impl Parameter {
    /// Create a new parameter
    ///
    /// Example
    /// -------
    /// ```
    /// use proffer::*;
    ///
    /// let param = Parameter::new("foo", "usize").generate();
    /// let expected = "foo: usize";
    /// assert_eq!(expected, &param);
    /// ```
    ///
    pub fn new<S: ToString>(name: S, ty: S) -> Self {
        Self {
            name: name.to_string(),
            ty: ty.to_string(),
        }
    }
}

impl SrcCode for Parameter {
    fn generate(&self) -> String {
        let template = "{{ self.name }}: {{ self.ty }}";
        let mut ctx = Context::new();
        ctx.insert("self", &self);
        Tera::one_off(template, &ctx, false).unwrap()
    }
}

impl SrcCode for Function {
    fn generate(&self) -> String {
        let template = r#"
        {{ function_signature }}
        {
            {{ body }}
        }
        "#;
        let mut context = Context::new();
        context.insert("body", &self.body.generate());
        context.insert("function_signature", &self.signature.generate());
        Tera::one_off(template, &context, false).unwrap()
    }
}
