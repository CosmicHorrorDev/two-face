use std::collections::BTreeMap;

use once_cell::sync::OnceCell;
use serde::{Deserialize, Serialize};
use syntect::{
    dumps::{dump_binary, from_binary},
    highlighting::{Theme, ThemeSet},
};

#[derive(Serialize, Deserialize)]
pub struct LazyThemeSet {
    // Can't be public since people can tweak `LazyTheme`'s internal data to get deserialization to
    // fail
    themes: BTreeMap<String, LazyTheme>,
}

impl LazyThemeSet {
    pub fn get(&self, name: &str) -> Option<&Theme> {
        self.themes.get(name).map(|lazy_theme| {
            lazy_theme
                .deserialized
                .get_or_init(|| lazy_theme.deserialize())
        })
    }

    pub fn theme_names(&self) -> impl Iterator<Item = &str> {
        self.themes.keys().map(|name| name.as_str())
    }
}

impl From<&ThemeSet> for LazyThemeSet {
    fn from(full_themes: &ThemeSet) -> Self {
        let themes = full_themes
            .themes
            .iter()
            .map(|(name, theme)| (name.to_owned(), theme.into()))
            .collect();
        Self { themes }
    }
}

impl From<&LazyThemeSet> for ThemeSet {
    fn from(lazy_themes: &LazyThemeSet) -> Self {
        let themes = lazy_themes
            .themes
            .iter()
            .map(|(name, lazy)| (name.to_owned(), lazy.into()))
            .collect();
        Self { themes }
    }
}

#[derive(Serialize, Deserialize)]
struct LazyTheme {
    serialized: Vec<u8>,

    #[serde(skip, default)]
    deserialized: OnceCell<Theme>,
}

impl LazyTheme {
    fn deserialize(&self) -> Theme {
        from_binary(&self.serialized)
    }
}

impl From<&LazyTheme> for Theme {
    fn from(lazy: &LazyTheme) -> Self {
        lazy.deserialize()
    }
}

impl From<&Theme> for LazyTheme {
    fn from(theme: &Theme) -> Self {
        let serialized = dump_binary(theme);
        Self {
            serialized,
            deserialized: OnceCell::new(),
        }
    }
}
