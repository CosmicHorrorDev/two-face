//! Contains extra theme definitions and the [`LazyThemeSet`] type
//!
//! The extra themes are provided in an [`EmbeddedLazyThemeSet`] which is just a newtype around a
//! [`LazyThemeSet`], but with an exhastive enumeration of its themes through the
//! [`EmbeddedThemeName`] enum
//!
//! _Note: For visual examples of all of the embedded themes look at the docs for
//! [`EmbeddedThemeName`]_

mod core_types;

use std::{fmt, ops::Index};

pub use core_types::LazyThemeSet;

use syntect::highlighting::{Theme, ThemeSet};

/// Returns an [`EmbeddedLazyThemeSet`] with more popular theme definitions
///
/// These themes cover a variety of use-cases, so it's very likely that you'll only want to expose
/// a subset or tweak the values for specific themes depending on your usage. E.g.
/// `EmbeddedThemeName::{Ansi, Base16, Base16_256}` are all terminal related themes,
/// `EmbeddedThemeName::InspiredGithub` uses a full-white background which wouldn't be a good fit
/// for a static site generator, etc.
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

/// A [`LazyThemeSet`] where we know all of the themes that are included
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
            EmbeddedThemeName::Ansi,
            EmbeddedThemeName::Base16,
            EmbeddedThemeName::Base16EightiesDark,
            EmbeddedThemeName::Base16MochaDark,
            EmbeddedThemeName::Base16OceanDark,
            EmbeddedThemeName::Base16OceanLight,
            EmbeddedThemeName::Base16_256,
            EmbeddedThemeName::CatppuccinFrappe,
            EmbeddedThemeName::CatppuccinLatte,
            EmbeddedThemeName::CatppuccinMacchiato,
            EmbeddedThemeName::CatppuccinMocha,
            EmbeddedThemeName::ColdarkCold,
            EmbeddedThemeName::ColdarkDark,
            EmbeddedThemeName::DarkNeon,
            EmbeddedThemeName::Dracula,
            EmbeddedThemeName::Github,
            EmbeddedThemeName::GruvboxDark,
            EmbeddedThemeName::GruvboxLight,
            EmbeddedThemeName::InspiredGithub,
            EmbeddedThemeName::Leet,
            EmbeddedThemeName::MonokaiExtended,
            EmbeddedThemeName::MonokaiExtendedBright,
            EmbeddedThemeName::MonokaiExtendedLight,
            EmbeddedThemeName::MonokaiExtendedOrigin,
            EmbeddedThemeName::Nord,
            EmbeddedThemeName::OneHalfDark,
            EmbeddedThemeName::OneHalfLight,
            EmbeddedThemeName::SolarizedDark,
            EmbeddedThemeName::SolarizedLight,
            EmbeddedThemeName::SublimeSnazzy,
            EmbeddedThemeName::TwoDark,
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

impl From<EmbeddedLazyThemeSet> for ThemeSet {
    fn from(embedded: EmbeddedLazyThemeSet) -> Self {
        (&embedded).into()
    }
}

impl Index<EmbeddedThemeName> for EmbeddedLazyThemeSet {
    type Output = Theme;

    fn index(&self, theme_name: EmbeddedThemeName) -> &Self::Output {
        self.get(theme_name)
    }
}

// NOTE: doc comment HTML is copied from the tests/docs_watchdog/theme.rs tests
/// An enum that represents all themes included in [`EmbeddedLazyThemeSet`]
///
/// A demo is included for how each theme highlights the following Julia snippet adapted from the
/// Julia _Getting Started_ manual
///
/// ```julia
/// # sends a variety of values over a channel
/// function producer(c::Channel)
///     for n=1:4
///         put!(c, 2n)
///     end
///     put!(c, "stop")
/// end;
/// ```
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(test, derive(strum::EnumIter))]
pub enum EmbeddedThemeName {
    /// ANSI
    ///
    /// _Doesn't display as HTML well_
    Ansi,
    /// Base16
    ///
    /// _Doesn't display as HTML well_
    Base16,
    /// Base16 Eighties Dark
    ///
    /// <pre style="background-color:#2d2d2d;">
    /// <span style="color:#747369;"># sends a variety of values over a channel
    /// </span><span style="color:#d3d0c8;">function producer(c::</span><span style="color:#ffcc66;">Channel</span><span style="color:#d3d0c8;">)
    /// </span><span style="color:#d3d0c8;">    </span><span style="color:#cc99cc;">for</span><span style="color:#d3d0c8;"> n=</span><span style="color:#f99157;">1</span><span style="color:#d3d0c8;">:</span><span style="color:#f99157;">4
    /// </span><span style="color:#d3d0c8;">        put!(c, 2n)
    /// </span><span style="color:#d3d0c8;">    </span><span style="color:#cc99cc;">end
    /// </span><span style="color:#d3d0c8;">    put!(c, &quot;</span><span style="color:#99cc99;">stop</span><span style="color:#d3d0c8;">&quot;)
    /// </span><span style="color:#cc99cc;">end</span><span style="color:#d3d0c8;">;
    /// </span></pre>
    Base16EightiesDark,
    /// Base16 Mocha Dark Theme
    ///
    /// <pre style="background-color:#3b3228;">
    /// <span style="color:#7e705a;"># sends a variety of values over a channel
    /// </span><span style="color:#d0c8c6;">function producer(c::</span><span style="color:#f4bc87;">Channel</span><span style="color:#d0c8c6;">)
    /// </span><span style="color:#d0c8c6;">    </span><span style="color:#a89bb9;">for</span><span style="color:#d0c8c6;"> n=</span><span style="color:#d28b71;">1</span><span style="color:#d0c8c6;">:</span><span style="color:#d28b71;">4
    /// </span><span style="color:#d0c8c6;">        put!(c, 2n)
    /// </span><span style="color:#d0c8c6;">    </span><span style="color:#a89bb9;">end
    /// </span><span style="color:#d0c8c6;">    put!(c, &quot;</span><span style="color:#beb55b;">stop</span><span style="color:#d0c8c6;">&quot;)
    /// </span><span style="color:#a89bb9;">end</span><span style="color:#d0c8c6;">;
    /// </span></pre>
    Base16MochaDark,
    /// Base16 Ocean Dark
    ///
    /// <pre style="background-color:#2b303b;">
    /// <span style="color:#65737e;"># sends a variety of values over a channel
    /// </span><span style="color:#c0c5ce;">function producer(c::</span><span style="color:#ebcb8b;">Channel</span><span style="color:#c0c5ce;">)
    /// </span><span style="color:#c0c5ce;">    </span><span style="color:#b48ead;">for</span><span style="color:#c0c5ce;"> n=</span><span style="color:#d08770;">1</span><span style="color:#c0c5ce;">:</span><span style="color:#d08770;">4
    /// </span><span style="color:#c0c5ce;">        put!(c, 2n)
    /// </span><span style="color:#c0c5ce;">    </span><span style="color:#b48ead;">end
    /// </span><span style="color:#c0c5ce;">    put!(c, &quot;</span><span style="color:#a3be8c;">stop</span><span style="color:#c0c5ce;">&quot;)
    /// </span><span style="color:#b48ead;">end</span><span style="color:#c0c5ce;">;
    /// </span></pre>
    Base16OceanDark,
    /// Base16 Ocean Light
    ///
    /// <pre style="background-color:#eff1f5;">
    /// <span style="color:#a7adba;"># sends a variety of values over a channel
    /// </span><span style="color:#4f5b66;">function producer(c::</span><span style="color:#d08770;">Channel</span><span style="color:#4f5b66;">)
    /// </span><span style="color:#4f5b66;">    </span><span style="color:#b48ead;">for</span><span style="color:#4f5b66;"> n=</span><span style="color:#d08770;">1</span><span style="color:#4f5b66;">:</span><span style="color:#d08770;">4
    /// </span><span style="color:#4f5b66;">        put!(c, 2n)
    /// </span><span style="color:#4f5b66;">    </span><span style="color:#b48ead;">end
    /// </span><span style="color:#4f5b66;">    put!(c, &quot;</span><span style="color:#a3be8c;">stop</span><span style="color:#4f5b66;">&quot;)
    /// </span><span style="color:#b48ead;">end</span><span style="color:#4f5b66;">;
    /// </span></pre>
    Base16OceanLight,
    /// Base16 256
    ///
    /// _Doesn't display as HTML well_
    Base16_256,
    /// Catppuccin Frappe
    ///
    /// <pre style="background-color:#303446;">
    /// <span style="font-style:italic;color:#949cbb;"># sends a variety of values over a channel
    /// </span><span style="color:#c6d0f5;">function producer</span><span style="color:#949cbb;">(</span><span style="color:#c6d0f5;">c::</span><span style="font-style:italic;color:#e5c890;">Channel</span><span style="color:#949cbb;">)
    /// </span><span style="color:#c6d0f5;">    </span><span style="color:#ca9ee6;">for</span><span style="color:#c6d0f5;"> n</span><span style="color:#81c8be;">=</span><span style="color:#ef9f76;">1</span><span style="color:#c6d0f5;">:</span><span style="color:#ef9f76;">4
    /// </span><span style="color:#c6d0f5;">        put!</span><span style="color:#949cbb;">(</span><span style="color:#c6d0f5;">c</span><span style="color:#949cbb;">,</span><span style="color:#c6d0f5;"> 2n</span><span style="color:#949cbb;">)
    /// </span><span style="color:#c6d0f5;">    </span><span style="color:#ca9ee6;">end
    /// </span><span style="color:#c6d0f5;">    put!</span><span style="color:#949cbb;">(</span><span style="color:#c6d0f5;">c</span><span style="color:#949cbb;">, </span><span style="color:#a6d189;">&quot;stop&quot;</span><span style="color:#949cbb;">)
    /// </span><span style="color:#ca9ee6;">end</span><span style="color:#949cbb;">;
    /// </span></pre>
    CatppuccinFrappe,
    /// Catppuccin Latte
    ///
    /// <pre style="background-color:#eff1f5;">
    /// <span style="font-style:italic;color:#7c7f93;"># sends a variety of values over a channel
    /// </span><span style="color:#4c4f69;">function producer</span><span style="color:#7c7f93;">(</span><span style="color:#4c4f69;">c::</span><span style="font-style:italic;color:#df8e1d;">Channel</span><span style="color:#7c7f93;">)
    /// </span><span style="color:#4c4f69;">    </span><span style="color:#8839ef;">for</span><span style="color:#4c4f69;"> n</span><span style="color:#179299;">=</span><span style="color:#fe640b;">1</span><span style="color:#4c4f69;">:</span><span style="color:#fe640b;">4
    /// </span><span style="color:#4c4f69;">        put!</span><span style="color:#7c7f93;">(</span><span style="color:#4c4f69;">c</span><span style="color:#7c7f93;">,</span><span style="color:#4c4f69;"> 2n</span><span style="color:#7c7f93;">)
    /// </span><span style="color:#4c4f69;">    </span><span style="color:#8839ef;">end
    /// </span><span style="color:#4c4f69;">    put!</span><span style="color:#7c7f93;">(</span><span style="color:#4c4f69;">c</span><span style="color:#7c7f93;">, </span><span style="color:#40a02b;">&quot;stop&quot;</span><span style="color:#7c7f93;">)
    /// </span><span style="color:#8839ef;">end</span><span style="color:#7c7f93;">;
    /// </span></pre>
    CatppuccinLatte,
    /// Catppuccin Macchiato
    ///
    /// <pre style="background-color:#24273a;">
    /// <span style="font-style:italic;color:#939ab7;"># sends a variety of values over a channel
    /// </span><span style="color:#cad3f5;">function producer</span><span style="color:#939ab7;">(</span><span style="color:#cad3f5;">c::</span><span style="font-style:italic;color:#eed49f;">Channel</span><span style="color:#939ab7;">)
    /// </span><span style="color:#cad3f5;">    </span><span style="color:#c6a0f6;">for</span><span style="color:#cad3f5;"> n</span><span style="color:#8bd5ca;">=</span><span style="color:#f5a97f;">1</span><span style="color:#cad3f5;">:</span><span style="color:#f5a97f;">4
    /// </span><span style="color:#cad3f5;">        put!</span><span style="color:#939ab7;">(</span><span style="color:#cad3f5;">c</span><span style="color:#939ab7;">,</span><span style="color:#cad3f5;"> 2n</span><span style="color:#939ab7;">)
    /// </span><span style="color:#cad3f5;">    </span><span style="color:#c6a0f6;">end
    /// </span><span style="color:#cad3f5;">    put!</span><span style="color:#939ab7;">(</span><span style="color:#cad3f5;">c</span><span style="color:#939ab7;">, </span><span style="color:#a6da95;">&quot;stop&quot;</span><span style="color:#939ab7;">)
    /// </span><span style="color:#c6a0f6;">end</span><span style="color:#939ab7;">;
    /// </span></pre>
    CatppuccinMacchiato,
    /// Catppuccin Mocha
    ///
    /// <pre style="background-color:#1e1e2e;">
    /// <span style="font-style:italic;color:#9399b2;"># sends a variety of values over a channel
    /// </span><span style="color:#cdd6f4;">function producer</span><span style="color:#9399b2;">(</span><span style="color:#cdd6f4;">c::</span><span style="font-style:italic;color:#f9e2af;">Channel</span><span style="color:#9399b2;">)
    /// </span><span style="color:#cdd6f4;">    </span><span style="color:#cba6f7;">for</span><span style="color:#cdd6f4;"> n</span><span style="color:#94e2d5;">=</span><span style="color:#fab387;">1</span><span style="color:#cdd6f4;">:</span><span style="color:#fab387;">4
    /// </span><span style="color:#cdd6f4;">        put!</span><span style="color:#9399b2;">(</span><span style="color:#cdd6f4;">c</span><span style="color:#9399b2;">,</span><span style="color:#cdd6f4;"> 2n</span><span style="color:#9399b2;">)
    /// </span><span style="color:#cdd6f4;">    </span><span style="color:#cba6f7;">end
    /// </span><span style="color:#cdd6f4;">    put!</span><span style="color:#9399b2;">(</span><span style="color:#cdd6f4;">c</span><span style="color:#9399b2;">, </span><span style="color:#a6e3a1;">&quot;stop&quot;</span><span style="color:#9399b2;">)
    /// </span><span style="color:#cba6f7;">end</span><span style="color:#9399b2;">;
    /// </span></pre>
    CatppuccinMocha,
    /// Coldark-Cold
    ///
    /// <pre style="background-color:#e3eaf2;">
    /// <span style="color:#3c526d;"># sends a variety of values over a channel
    /// </span><span style="color:#111b27;">function producer(c::Channel)
    /// </span><span style="color:#111b27;">    </span><span style="color:#a04900;">for</span><span style="color:#111b27;"> n</span><span style="color:#a04900;">=</span><span style="color:#755f00;">1</span><span style="color:#111b27;">:</span><span style="color:#755f00;">4
    /// </span><span style="color:#111b27;">        put!(c, 2n)
    /// </span><span style="color:#111b27;">    </span><span style="color:#a04900;">end
    /// </span><span style="color:#111b27;">    put!(c, </span><span style="color:#116b00;">&quot;stop&quot;</span><span style="color:#111b27;">)
    /// </span><span style="color:#a04900;">end</span><span style="color:#111b27;">;
    /// </span></pre>
    ColdarkCold,
    /// Coldark-Dark
    ///
    /// <pre style="background-color:#111b27;">
    /// <span style="color:#8da1b9;"># sends a variety of values over a channel
    /// </span><span style="color:#e3eaf2;">function producer(c::Channel)
    /// </span><span style="color:#e3eaf2;">    </span><span style="color:#e9ae7e;">for</span><span style="color:#e3eaf2;"> n</span><span style="color:#e9ae7e;">=</span><span style="color:#e6d37a;">1</span><span style="color:#e3eaf2;">:</span><span style="color:#e6d37a;">4
    /// </span><span style="color:#e3eaf2;">        put!(c, 2n)
    /// </span><span style="color:#e3eaf2;">    </span><span style="color:#e9ae7e;">end
    /// </span><span style="color:#e3eaf2;">    put!(c, </span><span style="color:#91d076;">&quot;stop&quot;</span><span style="color:#e3eaf2;">)
    /// </span><span style="color:#e9ae7e;">end</span><span style="color:#e3eaf2;">;
    /// </span></pre>
    ColdarkDark,
    /// Dark Neon
    ///
    /// <pre style="background-color:#000000;">
    /// <span style="background-color:#212121;color:#7c7c7c;"># sends a variety of values over a channel
    /// </span><span style="color:#ffffff;">function producer(c::</span><span style="color:#f8f8f8;">Channel</span><span style="color:#ffffff;">)
    /// </span><span style="color:#ffffff;">    </span><span style="color:#66ccff;">for</span><span style="color:#ffffff;"> n</span><span style="color:#aaaaaa;">=</span><span style="font-weight:bold;color:#ff73fd;">1</span><span style="color:#ffffff;">:</span><span style="font-weight:bold;color:#ff73fd;">4
    /// </span><span style="color:#ffffff;">        put!(c, 2n)
    /// </span><span style="color:#ffffff;">    </span><span style="color:#66ccff;">end
    /// </span><span style="color:#ffffff;">    put!(c, </span><span style="color:#ccff66;">&quot;stop&quot;</span><span style="color:#ffffff;">)
    /// </span><span style="color:#66ccff;">end</span><span style="color:#ffffff;">;
    /// </span></pre>
    DarkNeon,
    /// Dracula
    ///
    /// <pre style="background-color:#282a36;">
    /// <span style="color:#6272a4;"># sends a variety of values over a channel
    /// </span><span style="color:#f8f8f2;">function producer(c::</span><span style="text-decoration:underline;color:#8be9fd;">Channel</span><span style="color:#f8f8f2;">)
    /// </span><span style="color:#f8f8f2;">    </span><span style="color:#ff79c6;">for</span><span style="color:#f8f8f2;"> n</span><span style="color:#ff79c6;">=</span><span style="color:#bd93f9;">1</span><span style="color:#f8f8f2;">:</span><span style="color:#bd93f9;">4
    /// </span><span style="color:#f8f8f2;">        put!(c, 2n)
    /// </span><span style="color:#f8f8f2;">    </span><span style="color:#ff79c6;">end
    /// </span><span style="color:#f8f8f2;">    put!(c, </span><span style="color:#f1fa8c;">&quot;stop&quot;</span><span style="color:#f8f8f2;">)
    /// </span><span style="color:#ff79c6;">end</span><span style="color:#f8f8f2;">;
    /// </span></pre>
    Dracula,
    /// GitHub
    ///
    /// <pre style="background-color:#ffffff;">
    /// <span style="color:#969896;"># sends a variety of values over a channel
    /// </span><span style="color:#333333;">function producer(c::Channel)
    /// </span><span style="color:#333333;">    </span><span style="color:#a71d5d;">for</span><span style="color:#333333;"> n</span><span style="color:#a71d5d;">=</span><span style="color:#0086b3;">1</span><span style="color:#333333;">:</span><span style="color:#0086b3;">4
    /// </span><span style="color:#333333;">        put!(c, 2n)
    /// </span><span style="color:#333333;">    </span><span style="color:#a71d5d;">end
    /// </span><span style="color:#333333;">    put!(c, </span><span style="color:#183691;">&quot;stop&quot;</span><span style="color:#333333;">)
    /// </span><span style="color:#a71d5d;">end</span><span style="color:#333333;">;
    /// </span></pre>
    Github,
    /// gruvbox (Dark)
    ///
    /// <pre style="background-color:#282828;">
    /// <span style="font-style:italic;color:#928374;"># sends a variety of values over a channel
    /// </span><span style="color:#fbf1c7;">function producer(c::</span><span style="color:#fabd2f;">Channel</span><span style="color:#fbf1c7;">)
    /// </span><span style="color:#fbf1c7;">    </span><span style="color:#fb4934;">for</span><span style="color:#fbf1c7;"> n</span><span style="color:#8ec07c;">=</span><span style="color:#d3869b;">1</span><span style="color:#fbf1c7;">:</span><span style="color:#d3869b;">4
    /// </span><span style="color:#fbf1c7;">        put!(c, 2n)
    /// </span><span style="color:#fbf1c7;">    </span><span style="color:#fb4934;">end
    /// </span><span style="color:#fbf1c7;">    put!(c, &quot;</span><span style="color:#b8bb26;">stop</span><span style="color:#fbf1c7;">&quot;)
    /// </span><span style="color:#fb4934;">end</span><span style="color:#fbf1c7;">;
    /// </span></pre>
    GruvboxDark,
    /// gruvbox (Light)
    ///
    /// <pre style="background-color:#fbf1c7;">
    /// <span style="font-style:italic;color:#928374;"># sends a variety of values over a channel
    /// </span><span style="color:#282828;">function producer(c::</span><span style="color:#b57614;">Channel</span><span style="color:#282828;">)
    /// </span><span style="color:#282828;">    </span><span style="color:#9d0006;">for</span><span style="color:#282828;"> n</span><span style="color:#427b58;">=</span><span style="color:#8f3f71;">1</span><span style="color:#282828;">:</span><span style="color:#8f3f71;">4
    /// </span><span style="color:#282828;">        put!(c, 2n)
    /// </span><span style="color:#282828;">    </span><span style="color:#9d0006;">end
    /// </span><span style="color:#282828;">    put!(c, &quot;</span><span style="color:#79740e;">stop</span><span style="color:#282828;">&quot;)
    /// </span><span style="color:#9d0006;">end</span><span style="color:#282828;">;
    /// </span></pre>
    GruvboxLight,
    /// Inspired GitHub
    ///
    /// <pre style="background-color:#ffffff;">
    /// <span style="font-style:italic;color:#969896;"># sends a variety of values over a channel
    /// </span><span style="color:#323232;">function producer(c::</span><span style="color:#0086b3;">Channel</span><span style="color:#323232;">)
    /// </span><span style="color:#323232;">    </span><span style="font-weight:bold;color:#a71d5d;">for</span><span style="color:#323232;"> n</span><span style="font-weight:bold;color:#a71d5d;">=</span><span style="color:#0086b3;">1</span><span style="color:#323232;">:</span><span style="color:#0086b3;">4
    /// </span><span style="color:#323232;">        put!(c, 2n)
    /// </span><span style="color:#323232;">    </span><span style="font-weight:bold;color:#a71d5d;">end
    /// </span><span style="color:#323232;">    put!(c, </span><span style="color:#183691;">&quot;stop&quot;</span><span style="color:#323232;">)
    /// </span><span style="font-weight:bold;color:#a71d5d;">end</span><span style="color:#323232;">;
    /// </span></pre>
    InspiredGithub,
    /// 1337
    ///
    /// <pre style="background-color:#191919;">
    /// <span style="color:#6d6d6d;"># sends a variety of values over a channel
    /// </span><span style="color:#f8f8f2;">function producer(c::</span><span style="text-decoration:underline;color:#8cdaff;">Channel</span><span style="color:#f8f8f2;">)
    /// </span><span style="color:#f8f8f2;">    </span><span style="color:#ff5e5e;">for</span><span style="color:#f8f8f2;"> n</span><span style="color:#ff5e5e;">=</span><span style="color:#fdb082;">1</span><span style="color:#f8f8f2;">:</span><span style="color:#fdb082;">4
    /// </span><span style="color:#f8f8f2;">        put!(c, 2n)
    /// </span><span style="color:#f8f8f2;">    </span><span style="color:#ff5e5e;">end
    /// </span><span style="color:#f8f8f2;">    put!(c, </span><span style="color:#ffffff;">&quot;</span><span style="color:#fbe3bf;">stop</span><span style="color:#ffffff;">&quot;</span><span style="color:#f8f8f2;">)
    /// </span><span style="color:#ff5e5e;">end</span><span style="color:#f8f8f2;">;
    /// </span></pre>
    Leet,
    /// Monokai Extended
    ///
    /// <pre style="background-color:#222222;">
    /// <span style="color:#75715e;"># sends a variety of values over a channel
    /// </span><span style="color:#f8f8f2;">function producer(c::</span><span style="text-decoration:underline;color:#66d9ef;">Channel</span><span style="color:#f8f8f2;">)
    /// </span><span style="color:#f8f8f2;">    </span><span style="color:#f92672;">for</span><span style="color:#f8f8f2;"> n</span><span style="color:#f92672;">=</span><span style="color:#be84ff;">1</span><span style="color:#f8f8f2;">:</span><span style="color:#be84ff;">4
    /// </span><span style="color:#f8f8f2;">        put!(c, 2n)
    /// </span><span style="color:#f8f8f2;">    </span><span style="color:#f92672;">end
    /// </span><span style="color:#f8f8f2;">    put!(c, </span><span style="color:#e6db74;">&quot;stop&quot;</span><span style="color:#f8f8f2;">)
    /// </span><span style="color:#f92672;">end</span><span style="color:#f8f8f2;">;
    /// </span></pre>
    MonokaiExtended,
    /// Monokai Extended Bright
    ///
    /// <pre style="background-color:#272822;">
    /// <span style="color:#75715e;"># sends a variety of values over a channel
    /// </span><span style="color:#f8f8f2;">function producer(c::</span><span style="text-decoration:underline;color:#a6e22e;">Channel</span><span style="color:#f8f8f2;">)
    /// </span><span style="color:#f8f8f2;">    </span><span style="color:#f92672;">for</span><span style="color:#f8f8f2;"> n</span><span style="color:#f92672;">=</span><span style="color:#ae81ff;">1</span><span style="color:#f8f8f2;">:</span><span style="color:#ae81ff;">4
    /// </span><span style="color:#f8f8f2;">        put!(c, 2n)
    /// </span><span style="color:#f8f8f2;">    </span><span style="color:#f92672;">end
    /// </span><span style="color:#f8f8f2;">    put!(c, </span><span style="color:#e6db74;">&quot;stop&quot;</span><span style="color:#f8f8f2;">)
    /// </span><span style="color:#f92672;">end</span><span style="color:#f8f8f2;">;
    /// </span></pre>
    MonokaiExtendedBright,
    /// Monokai Extended Light
    ///
    /// <pre style="background-color:#fafafa;">
    /// <span style="color:#75715e;"># sends a variety of values over a channel
    /// </span><span style="color:#49483e;">function producer(c::</span><span style="text-decoration:underline;color:#679c00;">Channel</span><span style="color:#49483e;">)
    /// </span><span style="color:#49483e;">    </span><span style="color:#f9005a;">for</span><span style="color:#49483e;"> n</span><span style="color:#f9005a;">=</span><span style="color:#684d99;">1</span><span style="color:#49483e;">:</span><span style="color:#684d99;">4
    /// </span><span style="color:#49483e;">        put!(c, 2n)
    /// </span><span style="color:#49483e;">    </span><span style="color:#f9005a;">end
    /// </span><span style="color:#49483e;">    put!(c, </span><span style="color:#998f2f;">&quot;stop&quot;</span><span style="color:#49483e;">)
    /// </span><span style="color:#f9005a;">end</span><span style="color:#49483e;">;
    /// </span></pre>
    MonokaiExtendedLight,
    /// Monokai Extended Origin
    ///
    /// <pre style="background-color:#272822;">
    /// <span style="color:#75715e;"># sends a variety of values over a channel
    /// </span><span style="color:#f8f8f2;">function producer(c::</span><span style="text-decoration:underline;color:#a6e22e;">Channel</span><span style="color:#f8f8f2;">)
    /// </span><span style="color:#f8f8f2;">    </span><span style="color:#f92672;">for</span><span style="color:#f8f8f2;"> n</span><span style="color:#f92672;">=</span><span style="color:#ae81ff;">1</span><span style="color:#f8f8f2;">:</span><span style="color:#ae81ff;">4
    /// </span><span style="color:#f8f8f2;">        put!(c, 2n)
    /// </span><span style="color:#f8f8f2;">    </span><span style="color:#f92672;">end
    /// </span><span style="color:#f8f8f2;">    put!(c, </span><span style="color:#e6db74;">&quot;stop&quot;</span><span style="color:#f8f8f2;">)
    /// </span><span style="color:#f92672;">end</span><span style="color:#f8f8f2;">;
    /// </span></pre>
    MonokaiExtendedOrigin,
    /// Nord
    ///
    /// <pre style="background-color:#2e3440;">
    /// <span style="color:#616e88;"># sends a variety of values over a channel
    /// </span><span style="color:#d8dee9;">function producer(c::</span><span style="color:#8fbcbb;">Channel</span><span style="color:#d8dee9;">)
    /// </span><span style="color:#d8dee9;">    </span><span style="color:#81a1c1;">for</span><span style="color:#d8dee9;"> n</span><span style="color:#81a1c1;">=</span><span style="color:#b48ead;">1</span><span style="color:#d8dee9;">:</span><span style="color:#b48ead;">4
    /// </span><span style="color:#d8dee9;">        put!(c</span><span style="color:#eceff4;">,</span><span style="color:#d8dee9;"> 2n)
    /// </span><span style="color:#d8dee9;">    </span><span style="color:#81a1c1;">end
    /// </span><span style="color:#d8dee9;">    put!(c</span><span style="color:#eceff4;">, </span><span style="color:#a3be8c;">&quot;stop&quot;</span><span style="color:#d8dee9;">)
    /// </span><span style="color:#81a1c1;">end</span><span style="color:#eceff4;">;
    /// </span></pre>
    Nord,
    /// One Half Dark
    ///
    /// <pre style="background-color:#282c34;">
    /// <span style="color:#5c6370;"># sends a variety of values over a channel
    /// </span><span style="color:#dcdfe4;">function producer(c::</span><span style="color:#e5c07b;">Channel</span><span style="color:#dcdfe4;">)
    /// </span><span style="color:#dcdfe4;">    </span><span style="color:#c678dd;">for</span><span style="color:#dcdfe4;"> n</span><span style="color:#c678dd;">=</span><span style="color:#e5c07b;">1</span><span style="color:#dcdfe4;">:</span><span style="color:#e5c07b;">4
    /// </span><span style="color:#dcdfe4;">        put!(c, 2n)
    /// </span><span style="color:#dcdfe4;">    </span><span style="color:#c678dd;">end
    /// </span><span style="color:#dcdfe4;">    put!(c, </span><span style="color:#98c379;">&quot;stop&quot;</span><span style="color:#dcdfe4;">)
    /// </span><span style="color:#c678dd;">end</span><span style="color:#dcdfe4;">;
    /// </span></pre>
    OneHalfDark,
    /// One Half Light
    ///
    /// <pre style="background-color:#fafafa;">
    /// <span style="color:#a0a1a7;"># sends a variety of values over a channel
    /// </span><span style="color:#383a42;">function producer(c::</span><span style="color:#c18401;">Channel</span><span style="color:#383a42;">)
    /// </span><span style="color:#383a42;">    </span><span style="color:#a626a4;">for</span><span style="color:#383a42;"> n</span><span style="color:#a626a4;">=</span><span style="color:#c18401;">1</span><span style="color:#383a42;">:</span><span style="color:#c18401;">4
    /// </span><span style="color:#383a42;">        put!(c, 2n)
    /// </span><span style="color:#383a42;">    </span><span style="color:#a626a4;">end
    /// </span><span style="color:#383a42;">    put!(c, </span><span style="color:#50a14f;">&quot;stop&quot;</span><span style="color:#383a42;">)
    /// </span><span style="color:#a626a4;">end</span><span style="color:#383a42;">;
    /// </span></pre>
    OneHalfLight,
    /// Solarized (dark)
    ///
    /// <pre style="background-color:#002b36;">
    /// <span style="color:#586e75;"># sends a variety of values over a channel
    /// </span><span style="color:#839496;">function producer</span><span style="color:#657b83;">(</span><span style="color:#839496;">c::</span><span style="color:#b58900;">Channel</span><span style="color:#657b83;">)
    /// </span><span style="color:#839496;">    </span><span style="color:#859900;">for</span><span style="color:#839496;"> n</span><span style="color:#657b83;">=</span><span style="color:#6c71c4;">1</span><span style="color:#839496;">:</span><span style="color:#6c71c4;">4
    /// </span><span style="color:#839496;">        put!</span><span style="color:#657b83;">(</span><span style="color:#839496;">c, 2n</span><span style="color:#657b83;">)
    /// </span><span style="color:#839496;">    </span><span style="color:#859900;">end
    /// </span><span style="color:#839496;">    put!</span><span style="color:#657b83;">(</span><span style="color:#839496;">c, &quot;</span><span style="color:#2aa198;">stop</span><span style="color:#839496;">&quot;</span><span style="color:#657b83;">)
    /// </span><span style="color:#859900;">end</span><span style="color:#839496;">;
    /// </span></pre>
    SolarizedDark,
    /// Solarized (light)
    ///
    /// <pre style="background-color:#fdf6e3;">
    /// <span style="color:#93a1a1;"># sends a variety of values over a channel
    /// </span><span style="color:#657b83;">function producer(c::</span><span style="color:#b58900;">Channel</span><span style="color:#657b83;">)
    /// </span><span style="color:#657b83;">    </span><span style="color:#859900;">for</span><span style="color:#657b83;"> n=</span><span style="color:#6c71c4;">1</span><span style="color:#657b83;">:</span><span style="color:#6c71c4;">4
    /// </span><span style="color:#657b83;">        put!(c, 2n)
    /// </span><span style="color:#657b83;">    </span><span style="color:#859900;">end
    /// </span><span style="color:#657b83;">    put!(c, </span><span style="color:#839496;">&quot;</span><span style="color:#2aa198;">stop</span><span style="color:#839496;">&quot;</span><span style="color:#657b83;">)
    /// </span><span style="color:#859900;">end</span><span style="color:#657b83;">;
    /// </span></pre>
    SolarizedLight,
    /// Sublime Snazzy
    ///
    /// <pre style="background-color:#282a36;">
    /// <span style="color:#686868;"># sends a variety of values over a channel
    /// </span><span style="color:#f8f8f2;">function producer(c::</span><span style="text-decoration:underline;color:#9aedfe;">Channel</span><span style="color:#f8f8f2;">)
    /// </span><span style="color:#f8f8f2;">    </span><span style="color:#ff5c57;">for</span><span style="color:#f8f8f2;"> n</span><span style="color:#ff5c57;">=</span><span style="color:#f1f1f0;">1</span><span style="color:#f8f8f2;">:</span><span style="color:#f1f1f0;">4
    /// </span><span style="color:#f8f8f2;">        put!(c, 2n)
    /// </span><span style="color:#f8f8f2;">    </span><span style="color:#ff5c57;">end
    /// </span><span style="color:#f8f8f2;">    put!(c, </span><span style="color:#f3f99d;">&quot;stop&quot;</span><span style="color:#f8f8f2;">)
    /// </span><span style="color:#ff5c57;">end</span><span style="color:#f8f8f2;">;
    /// </span></pre>
    SublimeSnazzy,
    /// TwoDark
    ///
    /// <pre style="background-color:#282c34;">
    /// <span style="font-style:italic;color:#5c6370;"># sends a variety of values over a channel
    /// </span><span style="color:#abb2bf;">function producer(c::</span><span style="color:#e5c07b;">Channel</span><span style="color:#abb2bf;">)
    /// </span><span style="color:#abb2bf;">    </span><span style="color:#c678dd;">for</span><span style="color:#abb2bf;"> n=</span><span style="color:#d19a66;">1</span><span style="color:#abb2bf;">:</span><span style="color:#d19a66;">4
    /// </span><span style="color:#abb2bf;">        put!(c, 2n)
    /// </span><span style="color:#abb2bf;">    </span><span style="color:#c678dd;">end
    /// </span><span style="color:#abb2bf;">    put!(c, </span><span style="color:#98c379;">&quot;stop&quot;</span><span style="color:#abb2bf;">)
    /// </span><span style="color:#c678dd;">end</span><span style="color:#abb2bf;">;
    /// </span></pre>
    TwoDark,
    /// zenburn
    ///
    /// <pre style="background-color:#3f3f3f;">
    /// <span style="color:#a0cfa1;">#</span><span style="color:#87ae86;"> sends a variety of values over a channel
    /// </span><span style="color:#dedede;">function producer(c::Channel)
    /// </span><span style="color:#dedede;">    </span><span style="color:#fed6af;">for</span><span style="color:#dedede;"> n</span><span style="color:#ececec;">=</span><span style="color:#87d6d5;">1</span><span style="color:#dedede;">:</span><span style="color:#87d6d5;">4
    /// </span><span style="color:#dedede;">        put!(c, 2n)
    /// </span><span style="color:#dedede;">    </span><span style="color:#fed6af;">end
    /// </span><span style="color:#dedede;">    put!(c, </span><span style="color:#d6d6d680;">&quot;</span><span style="color:#d68686;">stop</span><span style="color:#d6d6d680;">&quot;</span><span style="color:#dedede;">)
    /// </span><span style="color:#fed6af;">end</span><span style="color:#dedede;">;
    /// </span></pre>
    Zenburn,
}

impl EmbeddedThemeName {
    /// The name of each embedded theme
    ///
    /// This matches the key used for each theme in [`ThemeSet`]'s `themes`
    ///
    /// ```
    /// use two_face::theme::EmbeddedThemeName;
    ///
    /// assert_eq!(
    ///     EmbeddedThemeName::Leet.as_name(),
    ///     "1337",
    /// );
    /// // `.as_name()` is used for `Display` too!
    /// assert_eq!(
    ///     EmbeddedThemeName::SolarizedDark.to_string(),
    ///     "Solarized (dark)",
    /// );
    /// ```
    pub fn as_name(self) -> &'static str {
        match self {
            Self::Ansi => "ansi",
            Self::Base16 => "base16",
            Self::Base16EightiesDark => "base16-eighties.dark",
            Self::Base16MochaDark => "base16-mocha.dark",
            Self::Base16OceanDark => "base16-ocean.dark",
            Self::Base16OceanLight => "base16-ocean.light",
            Self::Base16_256 => "base16-256",
            Self::CatppuccinFrappe => "Catppuccin Frappe",
            Self::CatppuccinLatte => "Catppuccin Latte",
            Self::CatppuccinMacchiato => "Catppuccin Macchiato",
            Self::CatppuccinMocha => "Catppuccin Mocha",
            Self::ColdarkCold => "Coldark-Cold",
            Self::ColdarkDark => "Coldark-Dark",
            Self::DarkNeon => "DarkNeon",
            Self::Dracula => "Dracula",
            Self::Github => "GitHub",
            Self::GruvboxDark => "gruvbox-dark",
            Self::GruvboxLight => "gruvbox-light",
            Self::InspiredGithub => "InspiredGitHub",
            Self::Leet => "1337",
            Self::MonokaiExtended => "Monokai Extended",
            Self::MonokaiExtendedBright => "Monokai Extended Bright",
            Self::MonokaiExtendedLight => "Monokai Extended Light",
            Self::MonokaiExtendedOrigin => "Monokai Extended Origin",
            Self::Nord => "Nord",
            Self::OneHalfDark => "OneHalfDark",
            Self::OneHalfLight => "OneHalfLight",
            Self::SolarizedDark => "Solarized (dark)",
            Self::SolarizedLight => "Solarized (light)",
            Self::SublimeSnazzy => "Sublime Snazzy",
            Self::TwoDark => "TwoDark",
            Self::Zenburn => "zenburn",
        }
    }
}

impl fmt::Display for EmbeddedThemeName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_name())
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use super::*;

    use strum::IntoEnumIterator;

    #[test]
    fn theme_set_roundtrip() {
        let theme_set: ThemeSet = extra().into();
        let lazy: LazyThemeSet = (&theme_set).into();
        let theme_set_again: ThemeSet = lazy.into();
        let eq = theme_set
            .themes
            .into_iter()
            .zip(theme_set_again.themes.into_iter())
            .all(|(pair1, pair2)| pair1 == pair2);
        assert!(eq);
    }

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
