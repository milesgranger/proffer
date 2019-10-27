//!
//! Trait(s) specific to code generation objects within this crate.
//!

/// Trait implemented for elements representing the ability to render as
/// raw source code.
pub trait SrcCode {
    /// Given current configuration, give the resulting source code.
    #[must_use]
    fn generate(&self) -> String;
}
