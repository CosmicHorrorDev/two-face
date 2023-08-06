//! Contains acknowledgements for embedded data and all of their associated types

mod core_types;
pub use core_types::{Acknowledgements, License, LicenseType};

impl Acknowledgements {
    /// Returns all of the acknowledgements specifically for embedded syntax definitions
    pub fn for_syntaxes(&self) -> &[License] {
        &self.for_syntaxes
    }

    /// Returns all of the acknowledgements specifically for embedded theme definitions
    pub fn for_themes(&self) -> &[License] {
        &self.for_themes
    }
}

/// Returns all the [`Acknowledgements`] for embedded data
pub fn listing() -> Acknowledgements {
    syntect::dumps::from_binary(include_bytes!("../../generated/acknowledgements_full.bin",))
}
