pub mod utilities;
use crate::utilities::Verify;

use proffer::*;

#[test]
fn test_module_basic() {
    let m = Module::new("foo")
        .set_is_pub(true)
        .add_trait(Trait::new("Bar").set_is_pub(true).to_owned())
        .add_function(Function::new("foo"))
        .add_struct(Struct::new("Thingy"))
        .add_impl(Impl::new("Thingy"))
        .add_outer_annotation("#[special_outer_annotation]")
        .add_inner_annotation("#![special_inner_annotation]")
        .add_doc("//! Module level docs")
        .add_use_statement("use super::*;")
        .add_enum(Enum::new("EnumThingy"))
        .to_owned();
    let src_code = m.generate_and_verify();

    let expected = r#"
        #[special_outer_annotation]
        pub mod foo
        {
            #![special_inner_annotation]
            //! Module level docs

            use super::*;

            pub trait Bar
            {
            }

            fn foo() -> ()
            {
            }

            struct Thingy
            {
            }

            impl Thingy
            {
            }

            enum EnumThingy
            {
            }
        }
    "#;
    println!("{}", &src_code);
    assert_eq!(norm_whitespace(expected), norm_whitespace(&src_code));
}

#[test]
fn test_module_submodule() {
    let m = Module::new("upper_module")
        .set_is_pub(true)
        .add_submodule(
            Module::new("foo")
                .set_is_pub(true)
                .add_trait(Trait::new("Bar").set_is_pub(true).to_owned())
                .add_function(Function::new("foo"))
                .add_struct(Struct::new("Thingy"))
                .add_impl(Impl::new("Thingy"))
                .add_outer_annotation("#[special_outer_annotation]")
                .add_inner_annotation("#![special_inner_annotation]")
                .add_doc("//! Module level docs")
                .add_enum(Enum::new("EnumThingy"))
                .to_owned(),
        )
        .to_owned();
    let src_code = m.generate_and_verify();

    let expected = r#"
        pub mod upper_module
        {

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

                struct Thingy
                {
                }

                impl Thingy
                {
                }

                enum EnumThingy
                {
                }
            }
        }
    "#;
    println!("{}", &src_code);
    assert_eq!(norm_whitespace(expected), norm_whitespace(&src_code));
}
