pub mod utilities;
use crate::utilities::Verify;

use proffer::*;

#[test]
fn basic_gen() {
    let struct_ = Struct::new("Basic")
        .set_is_pub(true)
        .add_attribute("#[derive(Clone)]")
        .add_field(
            Field::new("field1", "String")
                .set_is_pub(true)
                .add_attribute("#[serde = w]")
                .add_doc("/// Some example documentation")
                .add_docs(vec!["/// Another line", "/// and another"])
                .to_owned(),
        )
        .add_field(Field::new("field2", "usize"))
        .to_owned();
    let expected = r#"
        #[derive(Clone)]
        pub struct Basic
        {
            /// Some example documentation
            /// Another line
            /// and another
            #[serde = w]
            pub field1: String,
            field2: usize,
        }
        "#;
    let src_code = struct_.generate_and_verify();
    println!("{}", &src_code);
    assert_eq!(norm_whitespace(expected), norm_whitespace(&src_code));
}

#[test]
fn generic_gen() {
    let s = Struct::new("Generic")
        .set_is_pub(true)
        .add_generic(
            Generic::new("T")
                .add_trait_bounds(vec!["ToString"])
                .to_owned(),
        )
        .add_generic(
            Generic::new("S")
                .add_trait_bounds(vec!["ToString", "Number"])
                .to_owned(),
        )
        .add_fields(&[Field::new("field1", "S"), Field::new("field2", "T")])
        .to_owned();
    let src_code = s.generate_and_verify();
    println!("{}", &src_code);
    let expected = r#"
        pub struct Generic<T, S>
            where
                T: ToString,
                S: ToString + Number,
        {
            field1: S,
            field2: T,
        }
    "#;
    assert_eq!(norm_whitespace(expected), norm_whitespace(&src_code));
}

#[test]
fn gen_with_doc() {
    let struct_ = Struct::new("Basic")
        .set_is_pub(true)
        .add_doc("/// Some example documentation")
        .add_docs(vec!["/// Another line", "/// and another"])
        .to_owned();
    let expected = r#"
        /// Some example documentation
        /// Another line
        /// and another
        pub struct Basic
        {
        }
        "#;
    let src_code = struct_.generate_and_verify();
    println!("{}", &src_code);
    assert_eq!(norm_whitespace(expected), norm_whitespace(&src_code));
}
