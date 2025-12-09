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

/// Returns a link to a page listing acknowledgements for all syntax and theme definitions
///
/// Available without having to bundle all of the acknowledgement info in your binary
///
/// ```
/// assert_eq!(
///     two_face::acknowledgement::url(),
///     "https://github.com/CosmicHorrorDev/two-face/blob/v0.5.0-rc1/generated/acknowledgements_full.md"
/// );
/// ```
pub const fn url() -> &'static str {
    concat!(
        "https://github.com/CosmicHorrorDev/two-face/blob/v",
        env!("CARGO_PKG_VERSION"),
        "/generated/acknowledgements_full.md",
    )
}
