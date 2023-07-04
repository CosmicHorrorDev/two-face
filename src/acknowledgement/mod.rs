//! Contains acknowledgements for embedded data and all of their associated types

mod core_types;
pub use core_types::{Acknowledgements, License, LicenseType};

impl Acknowledgements {
    #[cfg_attr(docsrs, doc(cfg(feature = "extra-syntax")))]
    #[cfg(feature = "extra-syntax")]
    /// Returns all of the acknowledgements specifically for embedded syntax definitions
    pub fn for_syntaxes(&self) -> &[License] {
        &self.for_syntaxes
    }

    #[cfg_attr(docsrs, doc(cfg(feature = "extra-theme")))]
    #[cfg(feature = "extra-theme")]
    /// Returns all of the acknowledgements specifically for embedded theme definitions
    pub fn for_themes(&self) -> &[License] {
        &self.for_themes
    }
}

/// Returns all the [`Acknowledgements`] for embedded data
pub fn listing() -> Acknowledgements {
    listing_()
}

#[cfg(all(feature = "extra-syntax", feature = "extra-theme"))]
fn listing_() -> Acknowledgements {
    // The generated data is a bit smaller when we can bundle it together
    syntect::dumps::from_binary(include_bytes!("../../generated/acknowledgements_full.bin",))
}

#[cfg(not(all(feature = "extra-syntax", feature = "extra-theme")))]
fn listing_() -> Acknowledgements {
    #[cfg(feature = "extra-syntax")]
    let for_syntaxes = syntect::dumps::from_binary(include_bytes!(
        "../../generated/acknowledgements_syntaxes.bin",
    ));
    #[cfg(not(feature = "extra-syntax"))]
    let for_syntaxes = Vec::new();

    #[cfg(feature = "extra-theme")]
    let for_themes =
        syntect::dumps::from_binary(include_bytes!("../../generated/acknowledgements_theme.bin"));
    #[cfg(not(feature = "extra-theme"))]
    let for_themes = Vec::new();

    Acknowledgements {
        for_themes,
        for_syntaxes,
    }
}
