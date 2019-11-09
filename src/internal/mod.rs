use crate::{Attribute, Field, Generic};

/// Internal trait to get access to the container storing the attributes.
/// Used for the generic implementation of `AttributeExt`
pub trait Attributes {
    fn attributes_mut(&mut self) -> &mut Vec<Attribute>;
}

/// Internal trait to get access to the container storing the fields.
/// Used for the generic implementation of `FieldExt`
pub trait Fields {
    fn fields_mut(&mut self) -> &mut Vec<Field>;
}

/// Internal trait to get access to the container storing the generics.
/// Used for the generic implementation of `GenericExt`
pub trait Generics {
    fn generics_mut(&mut self) -> &mut Vec<Generic>;
    fn generics(&self) -> &[Generic];
}

/// Internal trait to get access to the container storing the trait bounds.
/// Used for the generic implementation of `TraitBoundExt`
pub trait TraitBounds {
    fn trait_bounds_mut(&mut self) -> &mut Vec<String>;
}

/// Internal trait to get access to the container storing the documentation.
/// Used for the generic implementation of `TraitBoundExt`
pub trait Docs {
    fn docs_mut(&mut self) -> &mut Vec<String>;
}
