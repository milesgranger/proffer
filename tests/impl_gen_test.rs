use proffer::*;

fn normalize_whitespace(s: &str) -> String {
    s.split("\n")
        .map(|l| l.trim())
        .filter(|l| l.len() > 0)
        .collect::<String>()
}

#[test]
fn impl_basic_gen_with_trait() {
    let mut ipl = Impl::new("That");
    ipl.set_impl_trait(Some(Trait::new("This")));
    let expected = r#"
        impl This for That
        {
        }
    "#;

    let src_code = ipl.generate();
    println!("{}", &src_code);

    assert_eq!(
        normalize_whitespace(expected),
        normalize_whitespace(&src_code)
    );

    // Add a function to the impl
    let expected = r#"
        impl This for That
        {
            fn foo() -> ()
            {
            }
        }
    "#;

    ipl.add_function(Function::new("foo"));

    let src_code = ipl.generate();
    println!("{}", &src_code);

    assert_eq!(
        normalize_whitespace(expected),
        normalize_whitespace(&src_code)
    );
}

#[test]
fn impl_basic_gen_without_trait() {
    let ipl = Impl::new("That");

    let expected = r#"
        impl That
        {
        }
    "#;

    let src_code = ipl.generate();
    println!("{}", &src_code);

    assert_eq!(
        normalize_whitespace(expected),
        normalize_whitespace(&src_code)
    )
}

#[test]
fn impl_with_generics() {
    let mut ipl = Impl::new("That");
    ipl.add_generic(Generic::new("T", vec!["ToString"]));

    let mut method = Function::new("foo");
    method.set_is_pub(true);
    method.add_parameter(Parameter::new("bar1", "T"));
    method.add_parameter(Parameter::new("bar2", "S"));
    method.set_return_ty("T");
    method.add_generic(Generic::new("S", vec![]));
    method.set_body("bar");
    ipl.add_function(method);

    let expected = r#"
        impl<T> That<T>
            where
                T: ToString,
        {
            pub fn foo<S>(bar1: T, bar2: S) -> T
                where
                    S: ,
            {
                bar
            }
        }
    "#;

    let src_code = ipl.generate();
    println!("{}", &src_code);

    assert_eq!(
        normalize_whitespace(expected),
        normalize_whitespace(&src_code)
    )
}
