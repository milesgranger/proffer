use proffer::*;

#[test]
fn gen_enum_basic() {
    let mut e = Enum::new("Foo");
    e.add_variant(Variant::new("A"));
    e.add_variant(Variant::new("B"));
    e.set_is_pub(true);
    let src_code = e.generate();
    println!("{}", &src_code);

    let expected = r#"
        pub enum Foo {
            A,
            B,
        }
    "#;

    assert_eq!(norm_whitespace(expected), norm_whitespace(&src_code))
}


#[test]
fn gen_enum_with_generic() {
    let mut e = Enum::new("Foo");

    e.add_variant(Variant::new("A"));

    let mut v = Variant::new("B");
    v.set_inner(Some("(T)"));
    e.add_variant(v);

    e.add_generic(Generic::new("T", vec![]));

    let src_code = e.generate();
    println!("{}", &src_code);

    let expected = r#"
        enum Foo<T>
            where
                T: ,
        {
            A,
            B(T),
        }
    "#;
    assert_eq!(norm_whitespace(expected), norm_whitespace(&src_code))
}
