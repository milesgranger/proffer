use proffer::*;

#[test]
fn test_module_basic() {
    let mut m = Module::new("foo");
    m.set_is_pub(true);

    let mut t = Trait::new("Bar");
    t.set_is_pub(true);
    m.add_trait(t);

    m.add_function(Function::new("foo"));
    m.add_struct(Struct::new("Thingy"));
    m.add_impl(Impl::new("Thingy"));
    m.add_outer_annotation("#[special_outer_annotation]");
    m.add_inner_annotation("#![special_inner_annotation]");
    m.add_doc("//! Module level docs");
    m.add_enum(Enum::new("EnumThingy"));
    let src_code = m.generate();

    let expected = r#"
        #[special_outer_annotation]
        pub mod foo
        {
            #![special_inner_annotation]
            //! Module level docs

            pub trait Bar
            {
            }
            fn foo() -> ()
            {
            }
            struct Thingy {
            }
            impl Thingy
            {
            }
            enum EnumThingy {
            }

        }
    "#;
    println!("{}", &src_code);
    assert_eq!(norm_whitespace(expected), norm_whitespace(&src_code))
}
