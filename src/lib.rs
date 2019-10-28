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

pub mod associated_types_gen;
pub mod enum_gen;
pub mod field_gen;
pub mod function_gen;
pub mod generics_gen;
pub mod impl_gen;
pub mod module_gen;
pub mod struct_gen;
pub mod trait_gen;
pub mod traits;

pub use associated_types_gen::*;
pub use enum_gen::*;
pub use field_gen::*;
pub use function_gen::*;
pub use generics_gen::*;
pub use impl_gen::*;
pub use module_gen::*;
pub use struct_gen::*;
pub use trait_gen::*;
pub use traits::SrcCode;

/// Helper function throughout tests and documentation
/// for comparing expected source code generated.
#[must_use]
pub fn norm_whitespace(s: &str) -> String {
    s.split('\n')
        .map(str::trim)
        .filter(|l| !l.is_empty())
        .collect::<String>()
}
