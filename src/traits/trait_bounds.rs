//!
//! Traits for trait bounds
//!

use crate::internal::TraitBounds;

/// Provides methods to add trait bounds to elements.
pub trait TraitBoundExt {
    /// Add a single trait bound.
    fn add_trait_bound(&mut self, trait_bound: impl ToString) -> &mut Self;

    /// Add multiple trait bounds at once.
    fn add_trait_bounds(
        &mut self,
        trait_bounds: impl IntoIterator<Item = impl ToString>,
    ) -> &mut Self;
}

impl<T: TraitBounds> TraitBoundExt for T {
    /// Add a single trait bound.
    fn add_trait_bound(&mut self, trait_bound: impl ToString) -> &mut Self {
        self.trait_bounds_mut().push(trait_bound.to_string());
        self
    }

    /// Add multiple trait bounds at once.
    fn add_trait_bounds(
        &mut self,
        trait_bounds: impl IntoIterator<Item = impl ToString>,
    ) -> &mut Self {
        self.trait_bounds_mut()
            .extend(trait_bounds.into_iter().map(|t| t.to_string()));
        self
    }
}
