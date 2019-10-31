pub mod utilities;
use crate::utilities::Verify;

use proffer::*;

#[test]
fn basic_gen() {
    let tr8t = Trait::new("Foo").set_is_pub(true).to_owned();
    let expected = r#"
        pub trait Foo
        {
        }
    "#;

    let src_code = tr8t.generate_and_verify();
    println!("{}", &src_code);

    assert_eq!(norm_whitespace(expected), norm_whitespace(&src_code));
}

#[test]
fn gen_with_method_signatures() {
    let tr8t = Trait::new("Foo")
        .set_is_pub(true)
        .add_signature(FunctionSignature::new("foo"))
        .add_signature(FunctionSignature::new("bar"))
        .to_owned();
    let expected = r#"
        pub trait Foo
        {
            fn foo() -> ();
            fn bar() -> ();
        }
    "#;

    let src_code = tr8t.generate_and_verify();
    println!("{}", &src_code);

    assert_eq!(norm_whitespace(expected), norm_whitespace(&src_code));
}

#[test]
fn gen_with_generics() {
    let tr8t = Trait::new("Foo")
        .set_is_pub(true)
        .add_signature(
            FunctionSignature::new("foo")
                .add_parameter(Parameter::new("name", "T"))
                .to_owned(),
        )
        .add_signature(FunctionSignature::new("bar"))
        .add_generic(
            Generic::new("T")
                .add_trait_bounds(vec!["ToString"])
                .to_owned(),
        )
        .to_owned();
    let expected = r#"
        pub trait Foo<T>
            where
                T: ToString,
        {
            fn foo(name: T) -> ();
            fn bar() -> ();
        }
    "#;

    let src_code = tr8t.generate_and_verify();
    println!("{}", &src_code);

    assert_eq!(norm_whitespace(expected), norm_whitespace(&src_code));
}

#[test]
fn gen_with_associated_types() {
    let tr8t = Trait::new("Foo")
        .set_is_pub(true)
        .add_associated_type(AssociatedTypeDeclaration::new("FOO"))
        .add_associated_type(
            AssociatedTypeDeclaration::new("BAR")
                .add_trait_bounds(vec!["Debug"])
                .to_owned(),
        )
        .add_associated_type(
            AssociatedTypeDeclaration::new("BAZ")
                .add_trait_bounds(vec!["Debug", "Default"])
                .to_owned(),
        )
        .to_owned();
    let expected = r#"
        pub trait Foo
        {
            type FOO;
            type BAR: Debug;
            type BAZ: Debug + Default;
        }
    "#;

    let src_code = tr8t.generate_and_verify();
    println!("{}", &src_code);

    assert_eq!(norm_whitespace(expected), norm_whitespace(&src_code));
}

#[test]
fn gen_with_associated_type_annotations() {
    let tr8t = Trait::new("Foo")
        .set_is_pub(true)
        .add_associated_type(
            AssociatedTypeDeclaration::new("BAR")
                .add_annotation("#[bar]")
                .to_owned(),
        )
        .add_associated_type(
            AssociatedTypeDeclaration::new("BAZ")
                .add_annotation("#[bar]")
                .add_annotation("#[baz]")
                .to_owned(),
        )
        .to_owned();
    let expected = r#"
        pub trait Foo
        {
            #[bar]
            type BAR;
            #[bar]
            #[baz]
            type BAZ;
        }
    "#;

    let src_code = tr8t.generate_and_verify();
    println!("{}", &src_code);

    assert_eq!(norm_whitespace(expected), norm_whitespace(&src_code));
}
