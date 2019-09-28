use proffer::*;

fn normalize_whitespace(s: &str) -> String {
    s.split("\n")
        .map(|l| l.trim())
        .filter(|l| l.len() > 0)
        .collect::<String>()
}

#[test]
fn test_basic_gen() {
    let mut struct_ = Struct::new("Basic", true);

    let mut f = Field::new("field1", "String", true);
    f.annotations.push("#[serde = w]".to_string());
    f.docs.push("/// Some example documentation".to_string());
    struct_.add_field(f);

    struct_.add_field(Field::new("field2", "usize", true));
    let expected = r#"
        pub struct Basic {
            /// Some example documentation
            #[serde = w]
            pub field1: String,
            pub field2: usize,
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
fn test_generic_gen() {
    let mut s = Struct::new("Generic", true);
    s.add_generic(Generic::new("T", vec!["ToString"]));
    s.add_generic(Generic::new("S", vec!["ToString", "Number"]));
    s.add_field(Field::new("field1", "S", false));
    s.add_field(Field::new("field2", "T", false));
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
}
