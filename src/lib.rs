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
//! let mut ipl = Impl::new("That", None);
//! ipl.add_generic(Generic::new("T", vec!["ToString"]));
//!
//! let mut method = Function::new("foo", true);
//! method.add_parameter(Parameter::new("bar1", "T"));
//! method.add_parameter(Parameter::new("bar2", "S"));
//! method.set_return_ty("T");
//! method.add_generic(Generic::new("S", vec![]));
//! method.set_body("bar");
//!
//! ipl.add_function(method);
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
//!             bar
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

pub mod field_gen;
pub mod function_gen;
pub mod generics_gen;
pub mod impl_gen;
pub mod struct_gen;
pub mod trait_gen;
pub mod traits;

pub use field_gen::*;
pub use function_gen::*;
pub use generics_gen::*;
pub use impl_gen::*;
pub use struct_gen::*;
pub use trait_gen::*;
pub use traits::SrcCode;

/// Helper function throughout tests and documentation
/// for comparing expected source code generated.
pub fn norm_whitespace(s: &str) -> String {
    s.split("\n")
        .map(|l| l.trim())
        .filter(|l| l.len() > 0)
        .collect::<String>()
}
