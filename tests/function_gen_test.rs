use proffer::*;

fn normalize_whitespace(s: &str) -> String {
    s.split("\n")
        .map(|l| l.trim())
        .filter(|l| l.len() > 0)
        .collect::<String>()
}

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
        normalize_whitespace(expected),
        normalize_whitespace(&src_code)
    );
}

#[test]
fn function_gen_parameters() {
    let mut function = Function::new("foo");
    function.set_is_pub(true);
    function.add_parameter(Parameter::new("bar1", "usize"));
    function.add_parameter(Parameter::new("bar2", "&str"));
    let expected = r#"
        pub fn foo(bar1: usize, bar2: &str) -> ()
        {
        }
    "#;

    let src_code = function.generate();
    println!("{}", &src_code);
    assert_eq!(
        normalize_whitespace(expected),
        normalize_whitespace(&src_code)
    );
}

#[test]
fn function_with_generic() {
    let mut function = Function::new("foo");
    function.set_is_pub(true);
    function.add_parameter(Parameter::new("bar1", "T"));
    function.add_parameter(Parameter::new("bar2", "S"));
    function.add_generic(Generic::new("T", vec!["ToString", "Number"]));
    function.add_generic(Generic::new("S", vec!["Display"]));
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
        normalize_whitespace(expected),
        normalize_whitespace(&src_code)
    );
}

#[test]
fn function_with_generic_no_bounds() {
    let mut function = Function::new("foo");
    function.set_is_pub(true);
    function.add_parameter(Parameter::new("bar1", "T"));
    function.add_parameter(Parameter::new("bar2", "S"));
    function.add_generic(Generic::new("T", vec![]));
    function.add_generic(Generic::new("S", vec![]));
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
        normalize_whitespace(expected),
        normalize_whitespace(&src_code)
    );
}
