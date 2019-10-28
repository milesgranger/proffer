use proffer::*;

#[test]
fn impl_basic_gen_with_trait() {
    let mut ipl = Impl::new("That")
        .set_impl_trait(Some(Trait::new("This")))
        .to_owned();
    let expected = r#"
        impl This for That
        {
        }
    "#;

    let src_code = ipl.generate();
    println!("{}", &src_code);

    assert_eq!(norm_whitespace(expected), norm_whitespace(&src_code));

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

    assert_eq!(norm_whitespace(expected), norm_whitespace(&src_code));
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

    assert_eq!(norm_whitespace(expected), norm_whitespace(&src_code))
}

#[test]
fn impl_with_generics() {
    let ipl = Impl::new("That")
        .add_generic(
            Generic::new("T")
                .add_trait_bounds(vec!["ToString"])
                .to_owned(),
        )
        .add_function(
            Function::new("foo")
                .set_is_pub(true)
                .add_parameter(Parameter::new("bar1", "T"))
                .add_parameter(Parameter::new("bar2", "S"))
                .set_return_ty("T")
                .add_generic(Generic::new("S"))
                .set_body("bar")
                .to_owned(),
        )
        .to_owned();

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

    assert_eq!(norm_whitespace(expected), norm_whitespace(&src_code))
}

#[test]
fn impl_with_associated_types() {
    let ipl = Impl::new("That")
        .set_impl_trait(Some(Trait::new("This")))
        .add_associated_type(AssociatedTypeDefinition::new("FOO", "Bar"))
        .add_associated_type(AssociatedTypeDefinition::new("BAR", "Foo"))
        .to_owned();
    let expected = r#"
        impl This for That
        {
            type FOO = Bar;
            type BAR = Foo;
        }
    "#;

    let src_code = ipl.generate();
    println!("{}", &src_code);

    assert_eq!(norm_whitespace(expected), norm_whitespace(&src_code));
}

#[test]
fn impl_with_associated_type_annotations() {
    let ipl = Impl::new("That")
        .set_impl_trait(Some(Trait::new("This")))
        .add_associated_type(
            AssociatedTypeDefinition::new("FOO", "Bar")
                .add_annotation("#[foo]")
                .to_owned(),
        )
        .add_associated_type(
            AssociatedTypeDefinition::new("BAR", "Foo")
                .add_annotation("#[foo]")
                .add_annotation("#[bar]")
                .to_owned(),
        )
        .to_owned();
    let expected = r#"
        impl This for That
        {
            #[foo]
            type FOO = Bar;
            #[foo]
            #[bar]
            type BAR = Foo;
        }
    "#;

    let src_code = ipl.generate();
    println!("{}", &src_code);

    assert_eq!(norm_whitespace(expected), norm_whitespace(&src_code));
}
