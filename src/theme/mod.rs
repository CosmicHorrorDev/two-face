//! Contains extra theme definitions and the [`LazyThemeSet`] type

mod core_types;

pub use core_types::LazyThemeSet;

use syntect::highlighting::{Theme, ThemeSet};

/// Returns a [`LazyThemeSet`] with even more popular theme definitions
///
/// Note: This includes all of `syntect`'s embedded theme definitions
///
/// # Example
///
/// ```
/// use two_face::theme;
///
/// let theme_set = theme::extra();
/// let nord = theme_set.get(theme::ThemeName::Nord);
/// ```
pub fn extra() -> EmbeddedLazyThemeSet {
    let theme_set =
        syntect::dumps::from_uncompressed_data(include_bytes!("../../generated/themes.bin"))
            .unwrap();
    EmbeddedLazyThemeSet(theme_set)
}

pub struct EmbeddedLazyThemeSet(LazyThemeSet);

impl EmbeddedLazyThemeSet {
    pub fn get(&self, name: ThemeName) -> &Theme {
        self.0.get(name.as_name()).unwrap()
    }

    pub fn theme_names(&self) -> &'static [ThemeName] {
        &[
            ThemeName::Leet,
            ThemeName::ColdarkCold,
            ThemeName::ColdarkDark,
            ThemeName::DarkNeon,
            ThemeName::Dracula,
            ThemeName::Github,
            ThemeName::MonokaiExtended,
            ThemeName::MonokaiExtendedBright,
            ThemeName::MonokaiExtendedLight,
            ThemeName::MonokaiExtendedOrigin,
            ThemeName::Nord,
            ThemeName::OneHalfDark,
            ThemeName::OneHalfLight,
            ThemeName::SolarizedDark,
            ThemeName::SolarizedLight,
            ThemeName::SubmlimeSnazzy,
            ThemeName::TwoDark,
            ThemeName::VisualStudioDarkPlus,
            ThemeName::Ansi,
            ThemeName::Base16,
            ThemeName::Base16_256,
            ThemeName::GruvboxDark,
            ThemeName::GruvboxLight,
            ThemeName::Zenburn,
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
#[derive(Clone, Copy, Debug)]
#[cfg_attr(test, derive(strum::EnumIter))]
pub enum ThemeName {
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

impl ThemeName {
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
    use super::*;

    use strum::IntoEnumIterator;

    #[test]
    fn theme_name_is_exhaustive() {
        let theme_set = extra();
        for theme_name in ThemeName::iter() {
            println!("Getting: {:?}", theme_name);
            let _ = theme_set.get(theme_name);
        }

        assert_eq!(theme_set.0.themes.len(), ThemeName::iter().len());
    }
}
