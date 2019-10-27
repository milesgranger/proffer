use proffer::*;

#[test]
fn function_gen_basic() {
    let function = Function::new("foo");

    let expected = r#"
        fn foo() -> ()
        {
        }
    "#;

    let src_code = function.generate();
    println!("{}", &src_code);
    assert_eq!(
        norm_whitespace(expected),
        norm_whitespace(&src_code)
    );
}

#[test]
fn function_gen_parameters() {
    let function = Function::new("foo")
        .set_is_pub(true)
        .add_parameter(Parameter::new("bar1", "usize"))
        .add_parameter(Parameter::new("bar2", "&str"))
        .to_owned();
    let expected = r#"
        pub fn foo(bar1: usize, bar2: &str) -> ()
        {
        }
    "#;

    let src_code = function.generate();
    println!("{}", &src_code);
    assert_eq!(
        norm_whitespace(expected),
        norm_whitespace(&src_code)
    );
}

#[test]
fn function_with_generic() {
    let function = Function::new("foo")
        .set_is_pub(true)
        .add_parameter(Parameter::new("bar1", "T"))
        .add_parameter(Parameter::new("bar2", "S"))
        .add_generic(
            Generic::new("T")
                .add_trait_bounds(vec!["ToString", "Number"])
                .to_owned(),
        )
        .add_generic(
            Generic::new("S")
                .add_trait_bounds(vec!["Display"])
                .to_owned(),
        )
        .to_owned();
    let expected = r#"
        pub fn foo<T, S>(bar1: T, bar2: S) -> ()
            where
                T: ToString + Number,
                S: Display,
        {
        }
    "#;

    let src_code = function.generate();
    println!("{}", &src_code);
    assert_eq!(
        norm_whitespace(expected),
        norm_whitespace(&src_code)
    );
}

#[test]
fn function_with_generic_no_bounds() {
    let function = Function::new("foo")
        .set_is_pub(true)
        .add_parameter(Parameter::new("bar1", "T"))
        .add_parameter(Parameter::new("bar2", "S"))
        .add_generic(Generic::new("T"))
        .add_generic(Generic::new("S"))
        .to_owned();
    let expected = r#"
        pub fn foo<T, S>(bar1: T, bar2: S) -> ()
            where
                T: ,
                S: ,
        {
        }
    "#;

    let src_code = function.generate();
    println!("{}", &src_code);
    assert_eq!(
        norm_whitespace(expected),
        norm_whitespace(&src_code)
    );
}

#[test]
fn function_with_async() {
    let function = Function::new("foo")
        .set_is_pub(true)
        .set_is_async(true)
        .to_owned();

    let expected = r#"
        pub async fn foo() -> ()
        {
        }
    "#;

    let src_code = function.generate();
    println!("{}", &src_code);
    assert_eq!(
        norm_whitespace(expected),
        norm_whitespace(&src_code)
    );
}

#[test]
fn function_gen_parameter_annotations() {
    let function = Function::new("foo")
        .set_is_pub(true)
        .add_parameter(Parameter::new("bar1", "usize")
            .add_annotation("#[foo]")
            .to_owned())
        .add_parameter(Parameter::new("bar2", "&str")
            .add_annotation("#[foo]")
            .add_annotation("#[bar]")
            .to_owned())
        .to_owned();
    let expected = r#"
        pub fn foo(#[foo] bar1: usize, #[foo] #[bar] bar2: &str) -> ()
        {
        }
    "#;

    let src_code = function.generate();
    println!("{}", &src_code);
    assert_eq!(
        norm_whitespace(expected),
        norm_whitespace(&src_code)
    );
}

#[test]
fn function_with_annotations() {
    let function = Function::new("foo")
        .add_outer_annotation("#[foo]")
        .add_outer_annotation("#[bar]")
        .add_inner_annotation("#![foo]")
        .add_inner_annotation("#![bar]")
        .set_body("//body")
        .to_owned();

    let expected = r#"
        #[foo]
        #[bar]
        fn foo() -> ()
        {
            #![foo]
            #![bar]
            //body
        }
    "#;

    let src_code = function.generate();
    println!("{}", &src_code);
    assert_eq!(
        norm_whitespace(expected),
        norm_whitespace(&src_code)
    );
}