use crate::{Field, Generic};

/// Internal trait to get access to the container storing the annotations.
/// Used for the generic implementation of `AnnotationExt`
pub trait Annotations {
    fn annotations(&mut self) -> &mut Vec<String>;
}

/// Internal trait to get access to the container storing the inner and outer annotations.
/// Used for the generic implementation of `InnerAndOuterAnnotationExt`
pub trait InnerAndOuterAnnotations {
    fn inner_annotations(&mut self) -> &mut Vec<String>;
    fn outer_annotations(&mut self) -> &mut Vec<String>;
}

/// Internal trait to get access to the container storing the fields.
/// Used for the generic implementation of `FieldExt`
pub trait Fields {
    fn fields(&mut self) -> &mut Vec<Field>;
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
    fn trait_bounds(&mut self) -> &mut Vec<String>;
}

/// Internal trait to get access to the container storing the documentation.
/// Used for the generic implementation of `TraitBoundExt`
pub trait Docs {
    fn docs(&mut self) -> &mut Vec<String>;
}
