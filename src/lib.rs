#![warn(missing_docs)]
//!
//! Proffer is a code generation library to create code from other data structures
//! such as JSON or any other reason where generating raw source code is required.
//!
//! Example
//! -------
//!
//! ```
//! use proffer::*;
//!
//! let ipl = Impl::new("That")
//!     .add_generic(Generic::new("T").add_trait_bounds(vec!["ToString"]).to_owned())
//!     .add_function(
//!         Function::new("foo")
//!             .set_is_pub(true)
//!             .add_parameter(Parameter::new("bar1", "T"))
//!             .add_parameter(Parameter::new("bar2", "S"))
//!             .set_return_ty("T")
//!             .add_generic(Generic::new("S"))
//!             .set_body("bar1")
//!             .to_owned()
//!     ).to_owned();
//!
//! let expected = r#"
//!     impl<T> That<T>
//!         where
//!             T: ToString,
//!     {
//!         pub fn foo<S>(bar1: T, bar2: S) -> T
//!             where
//!                 S: ,
//!         {
//!             bar1
//!         }
//!     }
//! "#;
//!
//! let src_code = ipl.generate();
//! println!("{}", &src_code);
//!
//! assert_eq!(
//!     norm_whitespace(expected),
//!     norm_whitespace(&src_code)
//! )
//! ```
//!
//!

mod internal;

pub mod gen;
pub use gen::*;

pub mod traits;
pub use traits::*;

/// Helper function throughout tests and documentation
/// for comparing expected source code generated.
#[must_use]
pub fn norm_whitespace(s: &str) -> String {
    s.split('\n')
        .map(str::trim)
        .filter(|l| !l.is_empty())
        .collect::<Vec<&str>>()
        .join("\n")
}
