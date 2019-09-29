use proffer::*;

fn normalize_whitespace(s: &str) -> String {
    s.split("\n")
        .map(|l| l.trim())
        .filter(|l| l.len() > 0)
        .collect::<String>()
}

#[test]
fn basic_gen() {
    let mut tr8t = Trait::new("Foo");
    tr8t.set_is_pub(true);
    let expected = r#"
        pub trait Foo
        {
        }
    "#;

    let src_code = tr8t.generate();
    println!("{}", &src_code);

    assert_eq!(
        normalize_whitespace(expected),
        normalize_whitespace(&src_code)
    );
}

#[test]
fn gen_with_method_signatures() {
    let mut tr8t = Trait::new("Foo");
    tr8t.set_is_pub(true);
    tr8t.add_signature(FunctionSignature::new("foo"));
    tr8t.add_signature(FunctionSignature::new("bar"));
    let expected = r#"
        pub trait Foo
        {
            fn foo() -> ();
            fn bar() -> ();
        }
    "#;

    let src_code = tr8t.generate();
    println!("{}", &src_code);

    assert_eq!(
        normalize_whitespace(expected),
        normalize_whitespace(&src_code)
    );
}

#[test]
fn gen_with_generics() {
    let mut tr8t = Trait::new("Foo");
    tr8t.set_is_pub(true);

    let mut fs = FunctionSignature::new("foo");
    fs.add_parameter(Parameter::new("name", "T"));
    tr8t.add_signature(fs);
    tr8t.add_signature(FunctionSignature::new("bar"));
    tr8t.add_generic(Generic::new("T", vec!["ToString"]));
    let expected = r#"
        pub trait Foo<T>
            where
                T: ToString,
        {
            fn foo(name: T) -> ();
            fn bar() -> ();
        }
    "#;

    let src_code = tr8t.generate();
    println!("{}", &src_code);

    assert_eq!(
        normalize_whitespace(expected),
        normalize_whitespace(&src_code)
    );
}
