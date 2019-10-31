//!
//! Traits for generics
//!

use crate::internal::Generics;
use crate::Generic;

/// Provides methods to add generics to elements.
pub trait GenericExt {
    /// Add a single generic.
    fn add_generic(&mut self, generic: Generic) -> &mut Self;

    /// Add multiple generics at once.
    fn add_generics<'a>(&mut self, generics: impl IntoIterator<Item = &'a Generic>) -> &mut Self;
}

impl<T: Generics> GenericExt for T {
    /// Add a single generic.
    fn add_generic(&mut self, generic: Generic) -> &mut Self {
        self.generics().push(generic);
        self
    }

    /// Add multiple generics at once.
    fn add_generics<'a>(&mut self, generics: impl IntoIterator<Item = &'a Generic>) -> &mut Self {
        self.generics()
            .extend(generics.into_iter().map(ToOwned::to_owned));
        self
    }
}
