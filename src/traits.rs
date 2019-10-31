//!
//! Trait(s) specific to code generation objects within this crate.
//!

pub mod annotations;
pub mod docs;
pub mod fields;
pub mod generics;
pub mod trait_bounds;

pub use annotations::*;
pub use docs::*;
pub use fields::*;
pub use generics::*;
pub use trait_bounds::*;

/// Trait implemented for elements representing the ability to render as
/// raw source code.
pub trait SrcCode {
    /// Given current configuration, give the resulting source code.
    #[must_use]
    fn generate(&self) -> String;
}
