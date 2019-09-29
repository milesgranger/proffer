use proffer::*;

fn normalize_whitespace(s: &str) -> String {
    s.split("\n")
        .map(|l| l.trim())
        .filter(|l| l.len() > 0)
        .collect::<String>()
}

#[test]
fn basic_gen() {
    let mut struct_ = Struct::new("Basic");
    struct_.set_is_pub(true);

    let mut f = Field::new("field1", "String");
    f.set_is_pub(true);
    f.add_annotation("#[serde = w]");
    f.add_doc("/// Some example documentation");
    f.add_docs(vec!["/// Another line", "/// and another"]);
    struct_.add_field(f);

    struct_.add_field(Field::new("field2", "usize"));
    let expected = r#"
        pub struct Basic {
            /// Some example documentation
            /// Another line
            /// and another
            #[serde = w]
            pub field1: String,
            field2: usize,
         }
        "#
    .to_owned();
    let src_code = struct_.generate();
    println!("{}", &src_code);
    assert_eq!(
        normalize_whitespace(&src_code),
        normalize_whitespace(&expected)
    );
}

#[test]
fn generic_gen() {
    let mut s = Struct::new("Generic");
    s.set_is_pub(true);
    s.add_generic(Generic::new("T", vec!["ToString"]));
    s.add_generic(Generic::new("S", vec!["ToString", "Number"]));
    s.add_field(Field::new("field1", "S"));
    s.add_field(Field::new("field2", "T"));
    let src_code = s.generate();
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
    let src_code = s.generate();
    assert_eq!(
        normalize_whitespace(&src_code),
        normalize_whitespace(&expected)
    );

    // Add basic impl block.
}
