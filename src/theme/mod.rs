//! Contains extra theme definitions and the [`LazyThemeSet`] type

mod core_types;

pub use core_types::LazyThemeSet;

use syntect::highlighting::{Theme, ThemeSet};

/// Returns an [`EmbeddedLazyThemeSet`] with more popular theme definitions
///
/// Note: This includes all of `syntect`'s embedded theme definitions
///
/// # Example
///
/// ```
/// use two_face::theme::{extra, EmbeddedThemeName};
///
/// let theme_set = extra();
/// let nord = theme_set.get(EmbeddedThemeName::Nord);
/// ```
pub fn extra() -> EmbeddedLazyThemeSet {
    let theme_set =
        syntect::dumps::from_uncompressed_data(include_bytes!("../../generated/themes.bin"))
            .unwrap();
    EmbeddedLazyThemeSet(theme_set)
}

/// A `LazyThemeSet` where we know all themes that are included
pub struct EmbeddedLazyThemeSet(LazyThemeSet);

impl EmbeddedLazyThemeSet {
    /// Gets a single theme from the set
    ///
    /// An infallible version of [`LazyThemeSet::get()`]
    ///
    /// # Example
    ///
    /// ```
    /// use two_face::theme::{extra, EmbeddedThemeName};
    ///
    /// let theme_set = extra();
    /// // Loads the theme
    /// let nord1 = theme_set.get(EmbeddedThemeName::Nord);
    /// // Reuses the same loaded theme
    /// let nord2 = theme_set.get(EmbeddedThemeName::Nord);
    /// ```
    pub fn get(&self, name: EmbeddedThemeName) -> &Theme {
        self.0.get(name.as_name()).unwrap()
    }

    /// A listing of all the themes included in [`EmbeddedLazyThemeSet`]
    ///
    /// # Example
    ///
    /// ```
    /// use two_face::theme::{EmbeddedThemeName, EmbeddedLazyThemeSet};
    ///
    /// // Nord should be included
    /// assert!(EmbeddedLazyThemeSet::theme_names().contains(&EmbeddedThemeName::Nord));
    /// ```
    pub fn theme_names() -> &'static [EmbeddedThemeName] {
        &[
            EmbeddedThemeName::Leet,
            EmbeddedThemeName::ColdarkCold,
            EmbeddedThemeName::ColdarkDark,
            EmbeddedThemeName::DarkNeon,
            EmbeddedThemeName::Dracula,
            EmbeddedThemeName::Github,
            EmbeddedThemeName::MonokaiExtended,
            EmbeddedThemeName::MonokaiExtendedBright,
            EmbeddedThemeName::MonokaiExtendedLight,
            EmbeddedThemeName::MonokaiExtendedOrigin,
            EmbeddedThemeName::Nord,
            EmbeddedThemeName::OneHalfDark,
            EmbeddedThemeName::OneHalfLight,
            EmbeddedThemeName::SolarizedDark,
            EmbeddedThemeName::SolarizedLight,
            EmbeddedThemeName::SubmlimeSnazzy,
            EmbeddedThemeName::TwoDark,
            EmbeddedThemeName::VisualStudioDarkPlus,
            EmbeddedThemeName::Ansi,
            EmbeddedThemeName::Base16,
            EmbeddedThemeName::Base16_256,
            EmbeddedThemeName::GruvboxDark,
            EmbeddedThemeName::GruvboxLight,
            EmbeddedThemeName::Zenburn,
        ]
    }
}

impl From<EmbeddedLazyThemeSet> for LazyThemeSet {
    fn from(embedded: EmbeddedLazyThemeSet) -> Self {
        embedded.0
    }
}

impl From<&EmbeddedLazyThemeSet> for ThemeSet {
    fn from(embedded: &EmbeddedLazyThemeSet) -> Self {
        Self::from(&embedded.0)
    }
}

// TODO: include syntect's defaults in the dump too
/// An enum that represents all themes included in [`EmbeddedLazyThemeSet`]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(test, derive(strum::EnumIter))]
pub enum EmbeddedThemeName {
    Leet,
    ColdarkCold,
    ColdarkDark,
    DarkNeon,
    Dracula,
    Github,
    MonokaiExtended,
    MonokaiExtendedBright,
    MonokaiExtendedLight,
    MonokaiExtendedOrigin,
    Nord,
    OneHalfDark,
    OneHalfLight,
    SolarizedDark,
    SolarizedLight,
    SubmlimeSnazzy,
    TwoDark,
    VisualStudioDarkPlus,
    Ansi,
    Base16,
    Base16_256,
    GruvboxDark,
    GruvboxLight,
    Zenburn,
}

impl EmbeddedThemeName {
    pub fn as_name(self) -> &'static str {
        match self {
            Self::Leet => "1337",
            Self::ColdarkCold => "Coldark-Cold",
            Self::ColdarkDark => "Coldark-Dark",
            Self::DarkNeon => "DarkNeon",
            Self::Dracula => "Dracula",
            Self::Github => "GitHub",
            Self::MonokaiExtended => "Monokai Extended",
            Self::MonokaiExtendedBright => "Monokai Extended Bright",
            Self::MonokaiExtendedLight => "Monokai Extended Light",
            Self::MonokaiExtendedOrigin => "Monokai Extended Origin",
            Self::Nord => "Nord",
            Self::OneHalfDark => "OneHalfDark",
            Self::OneHalfLight => "OneHalfLight",
            Self::SolarizedDark => "Solarized (dark)",
            Self::SolarizedLight => "Solarized (light)",
            Self::SubmlimeSnazzy => "Sublime Snazzy",
            Self::TwoDark => "TwoDark",
            Self::VisualStudioDarkPlus => "Visual Studio Dark+",
            Self::Ansi => "ansi",
            Self::Base16 => "base16",
            Self::Base16_256 => "base16-256",
            Self::GruvboxDark => "gruvbox-dark",
            Self::GruvboxLight => "gruvbox-light",
            Self::Zenburn => "zenburn",
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use super::*;

    use strum::IntoEnumIterator;

    #[test]
    fn embedded_theme_is_exhaustive() {
        let theme_set = extra();
        for theme_name in EmbeddedThemeName::iter() {
            println!("Getting: {:?}", theme_name);
            let _ = theme_set.get(theme_name);
        }

        assert_eq!(theme_set.0.themes.len(), EmbeddedThemeName::iter().len());
        assert_eq!(
            EmbeddedLazyThemeSet::theme_names().len(),
            EmbeddedThemeName::iter().len()
        );

        let all_unique: BTreeSet<_> = EmbeddedLazyThemeSet::theme_names().iter().collect();
        assert_eq!(all_unique.len(), EmbeddedLazyThemeSet::theme_names().len());
    }
}
