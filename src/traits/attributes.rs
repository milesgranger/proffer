//!
//! Traits for attributes
//!

use crate::internal::Attributes;
use crate::Attribute;

/// Provides methods to add attributes to elements.
pub trait AttributeExt {
    /// Add a single attribute.
    fn add_attribute(&mut self, attribute: impl Into<Attribute>) -> &mut Self;

    /// Add multiple attributes at once.
    fn add_attributes(
        &mut self,
        attributes: impl IntoIterator<Item = impl Into<Attribute>>,
    ) -> &mut Self;
}

impl<T: Attributes> AttributeExt for T {
    /// Add a single attribute.
    fn add_attribute(&mut self, attribute: impl Into<Attribute>) -> &mut Self {
        self.attributes_mut().push(attribute.into());
        self
    }

    /// Add multiple attributes at once.
    fn add_attributes(
        &mut self,
        attributes: impl IntoIterator<Item = impl Into<Attribute>>,
    ) -> &mut Self {
        self.attributes_mut()
            .extend(attributes.into_iter().map(|a| a.into()));
        self
    }
}
