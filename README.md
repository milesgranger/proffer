# Proffer

[![CircleCI](https://circleci.com/gh/milesgranger/proffer.svg?style=svg)](https://circleci.com/gh/milesgranger/proffer)
[![crates.io](http://meritbadge.herokuapp.com/proffer)](https://crates.io/crates/proffer)
[![License](https://img.shields.io/badge/license-Unlicense-green.svg)](http://unlicense.org/)
---

[API Documentation](https://docs.rs/proffer)

Code generation for Rust

Namely useful for generating source code
from other data such as JSON



Example
-------

See the documentation for more examples

```rust
use proffer::*;
let ipl = Impl::new("That")
    .add_generic(Generic::new("T").add_trait_bounds(vec!["ToString"]))
    .add_function(
        Function::new("foo")
            .set_is_pub(true)
            .add_parameter(Parameter::new("bar1", "T"))
            .add_parameter(Parameter::new("bar2", "S"))
            .set_return_ty("T")
            .add_generic(Generic::new("S"))
            .set_body("bar"),
    );

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

assert_eq!(
    normalize_whitespace(expected),
    normalize_whitespace(&src_code)
)
```

