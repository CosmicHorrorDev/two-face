//! Contains extra theme definitions and the [`LazyThemeSet`] type

// TODO: just `include!()` this file instead?
pub use self::core_types::LazyThemeSet;

mod core_types;

use syntect::highlighting::Theme;

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

impl LazyThemeSet {
    // TODO: rename
    // TODO: Maybe don't need regular `.get()` anymore
    pub fn get_theme(&self, name: ThemeName) -> &Theme {
        self.get(name.as_name()).unwrap()
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
    GitHub,
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
            Self::GitHub => "GitHub",
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
            let _ = theme_set.get_theme(theme_name);
        }

        assert_eq!(theme_set.themes.len(), ThemeName::iter().len());
    }
}
