use std::collections::BTreeMap;

use once_cell::sync::OnceCell;
use serde::{Deserialize, Serialize};
use syntect::{
    dumps::{dump_binary, from_binary},
    highlighting::{Theme, ThemeSet},
};

/// A [`ThemeSet`] that lazily deserializes/decompresses a single theme at a time
///
/// This pattern is already actually used by [`syntect::parsing::SyntaxSet`] which
/// deserializes/decompresses syntaxes on demand. Doing this has the benefit of lowering load times
/// when you're only using a few themes at the expense of slightly larger embedded size due to
/// worse compression
///
/// # Example
///
/// For convenience you can easily convert between a [`LazyThemeSet`] and a [`ThemeSet`]
///
/// ```
/// use two_face::theme::{extra, LazyThemeSet};
///
/// let theme_set: LazyThemeSet = LazyThemeSet::from(extra());
/// let syntect_theme_set = syntect::highlighting::ThemeSet::from(&theme_set);
/// ```
#[derive(Serialize, Deserialize)]
pub struct LazyThemeSet {
    // Can't be public since people can tweak `LazyTheme`'s internal data to get deserialization to
    // fail
    pub(crate) themes: BTreeMap<String, LazyTheme>,
}

impl LazyThemeSet {
    /// Access a single theme from the set
    ///
    /// Calling this multiple times for the same theme will only deserialize and decompress the
    /// theme once
    ///
    /// # Example
    ///
    /// ```
    /// use two_face::theme::{extra, LazyThemeSet};
    ///
    /// let theme_set = LazyThemeSet::from(extra());
    /// // Loads the theme
    /// let nord1 = theme_set.get("Nord").unwrap();
    /// // Reuses the same loaded theme
    /// let nord2 = theme_set.get("Nord").unwrap();
    /// ```
    pub fn get(&self, name: &str) -> Option<&Theme> {
        self.themes.get(name).map(|lazy_theme| {
            lazy_theme
                .deserialized
                .get_or_init(|| lazy_theme.deserialize())
        })
    }

    /// Iterate over all the theme names included in the set
    ///
    /// # Example
    ///
    /// ```
    /// use two_face::theme::{extra, LazyThemeSet};
    ///
    /// let theme_set = LazyThemeSet::from(extra());
    /// // Nord should be included
    /// assert!(theme_set.theme_names().find(|&name| name == "Nord").is_some());
    /// ```
    // TODO: use a nameable iterator here
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
pub(crate) struct LazyTheme {
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
