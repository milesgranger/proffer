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

/// Trait to help collecting `Vec<impl SrcCode>` into `Vec<String>` via `.generate()`
pub trait SrcCodeVec {
    /// Convert the current `Vec<impl SrcCode>` into `Vec<String>`
    fn to_src_vec(&self) -> Vec<String>;
}

impl<T: SrcCode> SrcCodeVec for Vec<T> {
    fn to_src_vec(&self) -> Vec<String> {
        self.iter().map(SrcCode::generate).collect()
    }
}
