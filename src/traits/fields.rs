//!
//! Traits for fields
//!

use crate::internal::Fields;
use crate::Field;

/// Provides methods to add fields to elements.
pub trait FieldExt {
    /// Add a single field.
    fn add_field(&mut self, field: Field) -> &mut Self;

    /// Add multiple fields at once.
    fn add_fields<'a>(&mut self, fields: impl IntoIterator<Item = &'a Field>) -> &mut Self;
}

impl<T: Fields> FieldExt for T {
    /// Add a single field.
    fn add_field(&mut self, field: Field) -> &mut Self {
        self.fields().push(field);
        self
    }

    /// Add multiple fields at once.
    fn add_fields<'a>(&mut self, fields: impl IntoIterator<Item = &'a Field>) -> &mut Self {
        self.fields()
            .extend(fields.into_iter().map(ToOwned::to_owned));
        self
    }
}
