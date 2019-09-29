use serde::Serialize;
use tera::{Context, Tera};

use crate::traits::SrcCode;
use crate::{Generic, Generics};

/// Represents a function or method. Determined if any `Parameter` contains `self`
#[derive(Default, Serialize)]
pub struct Function {
    pub signature: FunctionSignature,
    pub body: FunctionBody,
}

#[derive(Default, Serialize)]
pub struct FunctionSignature {
    pub name: String,
    pub is_pub: bool,
    pub parameters: Vec<Parameter>,
    pub generics: Generics,
    pub return_ty: Option<String>,
}

impl FunctionSignature {
    pub fn new<S: ToString>(name: S, is_pub: bool) -> Self {
        let mut f = Self::default();
        f.name = name.to_string();
        f.is_pub = is_pub;
        f
    }
    pub fn add_parameter(&mut self, param: Parameter) {
        self.parameters.push(param)
    }
}

impl SrcCode for FunctionSignature {
    fn generate(&self) -> String {
        let template = r#"
        {% if self.is_pub %}pub {% endif %}fn {{ self.name }}{% if has_generics %}<{{ generic_keys | join(sep=", ") }}>{% endif %}({{ parameters | join(sep=", ") }}) -> {{ return_ty }}{% if has_generics %}
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

#[derive(Default, Serialize)]
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
    pub fn new<S: ToString>(name: S, is_pub: bool) -> Self {
        let mut f = Self::default();
        f.signature.name = name.to_string();
        f.signature.is_pub = is_pub;
        f
    }
    pub fn add_parameter(&mut self, param: Parameter) {
        self.signature.parameters.push(param)
    }
    pub fn add_generic(&mut self, generic: Generic) {
        self.signature.generics.add_generic(generic)
    }
    pub fn set_return_ty<S: ToString>(&mut self, ty: S) {
        self.signature.return_ty = Some(ty.to_string());
    }
    pub fn set_body<S: Into<FunctionBody>>(&mut self, body: S) {
        self.body = body.into()
    }
}

#[derive(Serialize, Default)]
pub struct Parameter {
    pub name: String,
    pub ty: String,
}
impl Parameter {
    pub fn new<S: ToString>(name: S, ty: S) -> Self {
        Self {
            name: name.to_string(),
            ty: ty.to_string(),
        }
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
