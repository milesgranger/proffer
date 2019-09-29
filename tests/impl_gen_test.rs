use proffer::*;

fn normalize_whitespace(s: &str) -> String {
    s.split("\n")
        .map(|l| l.trim())
        .filter(|l| l.len() > 0)
        .collect::<String>()
}

#[test]
fn impl_basic_gen_with_trait() {
    let ipl = Impl::new("That").set_impl_trait(Some(Trait::new("This")));
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

    let ipl = ipl.add_function(Function::new("foo"));

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
    let ipl = Impl::new("That")
        .add_generic(Generic::new("T", vec!["ToString"]))
        .add_function(
            Function::new("foo")
                .set_is_pub(true)
                .add_parameter(Parameter::new("bar1", "T"))
                .add_parameter(Parameter::new("bar2", "S"))
                .set_return_ty("T")
                .add_generic(Generic::new("S", vec![]))
                .set_body("bar"),
        );

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
