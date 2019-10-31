# Proffer

[![CircleCI](https://circleci.com/gh/milesgranger/proffer.svg?style=svg)](https://circleci.com/gh/milesgranger/proffer)
[![crates.io](http://meritbadge.herokuapp.com/proffer)](https://crates.io/crates/proffer)
[![License](https://img.shields.io/badge/license-Unlicense-green.svg)](http://unlicense.org/)
[![Coverage Status](https://coveralls.io/repos/github/milesgranger/proffer/badge.svg?branch=master)](https://coveralls.io/github/milesgranger/proffer?branch=master)
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
    .add_generic(Generic::new("T").add_trait_bounds(vec!["ToString"]).to_owned())
    .add_function(
        Function::new("foo")
            .set_is_pub(true)
            .add_parameter(Parameter::new("bar1", "T"))
            .add_parameter(Parameter::new("bar2", "S"))
            .set_return_ty("T")
            .add_generic(Generic::new("S"))
            .set_body("bar")
            .to_owned()
    ).to_owned();

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
    norm_whitespace(expected),
    norm_whitespace(&src_code)
)
```

