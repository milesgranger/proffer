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
        .add_attribute("#[special_outer_attribute]")
        .add_attribute("#![special_inner_attribute]")
        .add_doc("//! Module level docs")
        .add_use_statement("use super::*;")
        .add_enum(Enum::new("EnumThingy"))
        .to_owned();
    let src_code = m.generate_and_verify();

    let expected = r#"
        #[special_outer_attribute]
        pub mod foo
        {
            #![special_inner_attribute]
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
fn test_get_submodules_of_module() {
    let mut module = Module::new("Foo");

    assert!(module.get_submodule("Bar1").is_none());
    module.add_submodule(Module::new("Bar1"));
    assert!(module.get_submodule("Bar1").is_some());

    assert!(module.get_submodule_mut("Bar2").is_none());
    module.add_submodule(Module::new("Bar2"));
    assert!(module.get_submodule_mut("Bar2").is_some());
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
                .add_attribute("#[special_outer_attribute]")
                .add_attribute("#![special_inner_attribute]")
                .add_doc("//! Module level docs")
                .add_enum(Enum::new("EnumThingy"))
                .to_owned(),
        )
        .to_owned();
    let src_code = m.generate_and_verify();

    let expected = r#"
        pub mod upper_module
        {

            #[special_outer_attribute]
            pub mod foo
            {
                #![special_inner_attribute]
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
