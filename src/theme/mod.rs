//! Contains extra theme definitions and the [`LazyThemeSet`] type

// TODO: have an enum to enumerate themes? This allows for nicer programmatic access

pub use self::core_types::LazyThemeSet;

mod core_types;

/// Returns a [`LazyThemeSet`] with even more popular theme definitions
///
/// Note: This includes all of `syntect`'s embedded theme definitions
///
/// # Example
///
/// ```
/// let theme_set = two_face::theme::extra();
/// let nord = theme_set.get("Nord").unwrap();
/// ```
pub fn extra() -> LazyThemeSet {
    syntect::dumps::from_binary(include_bytes!("../../generated/themes.bin"))
}
