//! Re-exports from the `gen` submodules.

pub mod associated_types;
pub mod r#enum;
pub mod field;
pub mod function;
pub mod generics;
pub mod r#impl;
pub mod module;
pub mod r#struct;
pub mod r#trait;

pub use associated_types::*;
pub use r#enum::*;
pub use field::*;
pub use function::*;
pub use generics::*;
pub use r#impl::*;
pub use module::*;
pub use r#struct::*;
pub use r#trait::*;
