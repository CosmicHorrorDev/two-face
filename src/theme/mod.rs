//! Contains extra theme definitions and the [`LazyThemeSet`] type
//!
//! The extra themes are provided in an [`EmbeddedLazyThemeSet`] which is just a newtype around a
//! [`LazyThemeSet`], but with an exhastive enumeration of its themes through the
//! [`EmbeddedThemeName`] enum
//!
//! _Note: For visual examples of all of the embedded themes look at the docs for
//! [`EmbeddedThemeName`]_

mod core_types;

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
            EmbeddedThemeName::VisualStudioDarkPlus,
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

// NOTE: doc comment HTML is copied from the tests/docs_watchdog/theme.rs tests
/// An enum that represents all themes included in [`EmbeddedLazyThemeSet`]
///
/// A demo is included for how each theme highlights the following Elixir snippet
///
/// ```elixir
/// There currently is no ternary operator like  true ? "yes" : "no"
/// # So the following is suggested
/// "no" = if 1 == 0, do: "yes", else: "no"
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
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
    /// <span style="color:#747369;"># There currently is no ternary operator like  true ? &quot;yes&quot; : &quot;no&quot;
    /// </span><span style="color:#747369;"># So the following is suggested
    /// </span><span style="color:#d3d0c8;">&quot;</span><span style="color:#99cc99;">no</span><span style="color:#d3d0c8;">&quot; = </span><span style="color:#cc99cc;">if </span><span style="color:#f99157;">1 </span><span style="color:#d3d0c8;">== </span><span style="color:#f99157;">0</span><span style="color:#d3d0c8;">, </span><span style="color:#f99157;">do: </span><span style="color:#d3d0c8;">&quot;</span><span style="color:#99cc99;">yes</span><span style="color:#d3d0c8;">&quot;, </span><span style="color:#f99157;">else: </span><span style="color:#d3d0c8;">&quot;</span><span style="color:#99cc99;">no</span><span style="color:#d3d0c8;">&quot;
    /// </span></pre>
    Base16EightiesDark,
    /// Base16 Mocha Dark Theme
    ///
    /// <pre style="background-color:#3b3228;">
    /// <span style="color:#7e705a;"># There currently is no ternary operator like  true ? &quot;yes&quot; : &quot;no&quot;
    /// </span><span style="color:#7e705a;"># So the following is suggested
    /// </span><span style="color:#d0c8c6;">&quot;</span><span style="color:#beb55b;">no</span><span style="color:#d0c8c6;">&quot; = </span><span style="color:#a89bb9;">if </span><span style="color:#d28b71;">1 </span><span style="color:#d0c8c6;">== </span><span style="color:#d28b71;">0</span><span style="color:#d0c8c6;">, </span><span style="color:#d28b71;">do: </span><span style="color:#d0c8c6;">&quot;</span><span style="color:#beb55b;">yes</span><span style="color:#d0c8c6;">&quot;, </span><span style="color:#d28b71;">else: </span><span style="color:#d0c8c6;">&quot;</span><span style="color:#beb55b;">no</span><span style="color:#d0c8c6;">&quot;
    /// </span></pre>
    Base16MochaDark,
    /// Base16 Ocean Dark
    ///
    /// <pre style="background-color:#2b303b;">
    /// <span style="color:#65737e;"># There currently is no ternary operator like  true ? &quot;yes&quot; : &quot;no&quot;
    /// </span><span style="color:#65737e;"># So the following is suggested
    /// </span><span style="color:#c0c5ce;">&quot;</span><span style="color:#a3be8c;">no</span><span style="color:#c0c5ce;">&quot; = </span><span style="color:#b48ead;">if </span><span style="color:#d08770;">1 </span><span style="color:#c0c5ce;">== </span><span style="color:#d08770;">0</span><span style="color:#c0c5ce;">, </span><span style="color:#d08770;">do: </span><span style="color:#c0c5ce;">&quot;</span><span style="color:#a3be8c;">yes</span><span style="color:#c0c5ce;">&quot;, </span><span style="color:#d08770;">else: </span><span style="color:#c0c5ce;">&quot;</span><span style="color:#a3be8c;">no</span><span style="color:#c0c5ce;">&quot;
    /// </span></pre>
    Base16OceanDark,
    /// Base16 Ocean Light
    ///
    /// <pre style="background-color:#eff1f5;">
    /// <span style="color:#a7adba;"># There currently is no ternary operator like  true ? &quot;yes&quot; : &quot;no&quot;
    /// </span><span style="color:#a7adba;"># So the following is suggested
    /// </span><span style="color:#4f5b66;">&quot;</span><span style="color:#a3be8c;">no</span><span style="color:#4f5b66;">&quot; = </span><span style="color:#b48ead;">if </span><span style="color:#d08770;">1 </span><span style="color:#4f5b66;">== </span><span style="color:#d08770;">0</span><span style="color:#4f5b66;">, </span><span style="color:#d08770;">do: </span><span style="color:#4f5b66;">&quot;</span><span style="color:#a3be8c;">yes</span><span style="color:#4f5b66;">&quot;, </span><span style="color:#d08770;">else: </span><span style="color:#4f5b66;">&quot;</span><span style="color:#a3be8c;">no</span><span style="color:#4f5b66;">&quot;
    /// </span></pre>
    Base16OceanLight,
    /// Base16 256
    ///
    /// _Doesn't display as HTML well_
    Base16_256,
    /// Coldark-Cold
    ///
    /// <pre style="background-color:#e3eaf2;">
    /// <span style="color:#3c526d;"># There currently is no ternary operator like  true ? &quot;yes&quot; : &quot;no&quot;
    /// </span><span style="color:#3c526d;"># So the following is suggested
    /// </span><span style="color:#116b00;">&quot;no&quot; </span><span style="color:#a04900;">= if </span><span style="color:#755f00;">1 </span><span style="color:#a04900;">== </span><span style="color:#755f00;">0</span><span style="color:#111b27;">, </span><span style="color:#005a8e;">do</span><span style="color:#111b27;">: </span><span style="color:#116b00;">&quot;yes&quot;</span><span style="color:#111b27;">, </span><span style="color:#005a8e;">else</span><span style="color:#111b27;">: </span><span style="color:#116b00;">&quot;no&quot;
    /// </span></pre>
    ColdarkCold,
    /// Coldark-Dark
    ///
    /// <pre style="background-color:#111b27;">
    /// <span style="color:#8da1b9;"># There currently is no ternary operator like  true ? &quot;yes&quot; : &quot;no&quot;
    /// </span><span style="color:#8da1b9;"># So the following is suggested
    /// </span><span style="color:#91d076;">&quot;no&quot; </span><span style="color:#e9ae7e;">= if </span><span style="color:#e6d37a;">1 </span><span style="color:#e9ae7e;">== </span><span style="color:#e6d37a;">0</span><span style="color:#e3eaf2;">, </span><span style="color:#6cb8e6;">do</span><span style="color:#e3eaf2;">: </span><span style="color:#91d076;">&quot;yes&quot;</span><span style="color:#e3eaf2;">, </span><span style="color:#6cb8e6;">else</span><span style="color:#e3eaf2;">: </span><span style="color:#91d076;">&quot;no&quot;
    /// </span></pre>
    ColdarkDark,
    /// Dark Neon
    ///
    /// <pre style="background-color:#000000;">
    /// <span style="background-color:#212121;color:#7c7c7c;"># There currently is no ternary operator like  true ? &quot;yes&quot; : &quot;no&quot;
    /// </span><span style="background-color:#212121;color:#7c7c7c;"># So the following is suggested
    /// </span><span style="color:#ccff66;">&quot;no&quot; </span><span style="color:#aaaaaa;">= </span><span style="color:#66ccff;">if </span><span style="font-weight:bold;color:#ff73fd;">1 </span><span style="color:#aaaaaa;">== </span><span style="font-weight:bold;color:#ff73fd;">0</span><span style="color:#ffffff;">, </span><span style="color:#99cc99;">do: </span><span style="color:#ccff66;">&quot;yes&quot;</span><span style="color:#ffffff;">, </span><span style="color:#99cc99;">else: </span><span style="color:#ccff66;">&quot;no&quot;
    /// </span></pre>
    DarkNeon,
    /// Dracula
    ///
    /// <pre style="background-color:#282a36;">
    /// <span style="color:#6272a4;"># There currently is no ternary operator like  true ? &quot;yes&quot; : &quot;no&quot;
    /// </span><span style="color:#6272a4;"># So the following is suggested
    /// </span><span style="color:#f1fa8c;">&quot;no&quot; </span><span style="color:#ff79c6;">= if </span><span style="color:#bd93f9;">1 </span><span style="color:#ff79c6;">== </span><span style="color:#bd93f9;">0</span><span style="color:#f8f8f2;">, </span><span style="color:#bd93f9;">do: </span><span style="color:#f1fa8c;">&quot;yes&quot;</span><span style="color:#f8f8f2;">, </span><span style="color:#bd93f9;">else: </span><span style="color:#f1fa8c;">&quot;no&quot;
    /// </span></pre>
    Dracula,
    /// GitHub
    ///
    /// <pre style="background-color:#ffffff;">
    /// <span style="color:#969896;"># There currently is no ternary operator like  true ? &quot;yes&quot; : &quot;no&quot;
    /// </span><span style="color:#969896;"># So the following is suggested
    /// </span><span style="color:#183691;">&quot;no&quot; </span><span style="color:#a71d5d;">= if </span><span style="color:#0086b3;">1 </span><span style="color:#a71d5d;">== </span><span style="color:#0086b3;">0</span><span style="color:#333333;">, </span><span style="color:#0086b3;">do: </span><span style="color:#183691;">&quot;yes&quot;</span><span style="color:#333333;">, </span><span style="color:#0086b3;">else: </span><span style="color:#183691;">&quot;no&quot;
    /// </span></pre>
    Github,
    /// gruvbox (Dark)
    ///
    /// <pre style="background-color:#282828;">
    /// <span style="font-style:italic;color:#928374;"># There currently is no ternary operator like  true ? &quot;yes&quot; : &quot;no&quot;
    /// </span><span style="font-style:italic;color:#928374;"># So the following is suggested
    /// </span><span style="color:#fbf1c7;">&quot;</span><span style="color:#b8bb26;">no</span><span style="color:#fbf1c7;">&quot; </span><span style="color:#8ec07c;">= </span><span style="color:#fb4934;">if </span><span style="color:#d3869b;">1 </span><span style="color:#8ec07c;">== </span><span style="color:#d3869b;">0</span><span style="color:#fbf1c7;">, </span><span style="color:#d3869b;">do</span><span style="color:#fbf1c7;">: &quot;</span><span style="color:#b8bb26;">yes</span><span style="color:#fbf1c7;">&quot;, </span><span style="color:#d3869b;">else</span><span style="color:#fbf1c7;">: &quot;</span><span style="color:#b8bb26;">no</span><span style="color:#fbf1c7;">&quot;
    /// </span></pre>
    GruvboxDark,
    /// gruvbox (Light)
    ///
    /// <pre style="background-color:#fbf1c7;">
    /// <span style="font-style:italic;color:#928374;"># There currently is no ternary operator like  true ? &quot;yes&quot; : &quot;no&quot;
    /// </span><span style="font-style:italic;color:#928374;"># So the following is suggested
    /// </span><span style="color:#282828;">&quot;</span><span style="color:#79740e;">no</span><span style="color:#282828;">&quot; </span><span style="color:#427b58;">= </span><span style="color:#9d0006;">if </span><span style="color:#8f3f71;">1 </span><span style="color:#427b58;">== </span><span style="color:#8f3f71;">0</span><span style="color:#282828;">, </span><span style="color:#8f3f71;">do</span><span style="color:#282828;">: &quot;</span><span style="color:#79740e;">yes</span><span style="color:#282828;">&quot;, </span><span style="color:#8f3f71;">else</span><span style="color:#282828;">: &quot;</span><span style="color:#79740e;">no</span><span style="color:#282828;">&quot;
    /// </span></pre>
    GruvboxLight,
    /// Inspired GitHub
    ///
    /// <pre style="background-color:#ffffff;">
    /// <span style="font-style:italic;color:#969896;"># There currently is no ternary operator like  true ? &quot;yes&quot; : &quot;no&quot;
    /// </span><span style="font-style:italic;color:#969896;"># So the following is suggested
    /// </span><span style="color:#183691;">&quot;no&quot; </span><span style="font-weight:bold;color:#a71d5d;">= if </span><span style="color:#0086b3;">1 </span><span style="font-weight:bold;color:#a71d5d;">== </span><span style="color:#0086b3;">0</span><span style="color:#323232;">, </span><span style="color:#0086b3;">do: </span><span style="color:#183691;">&quot;yes&quot;</span><span style="color:#323232;">, </span><span style="color:#0086b3;">else: </span><span style="color:#183691;">&quot;no&quot;
    /// </span></pre>
    InspiredGithub,
    /// 1337
    ///
    /// <pre style="background-color:#191919;">
    /// <span style="color:#6d6d6d;"># There currently is no ternary operator like  true ? &quot;yes&quot; : &quot;no&quot;
    /// </span><span style="color:#6d6d6d;"># So the following is suggested
    /// </span><span style="color:#ffffff;">&quot;</span><span style="color:#fbe3bf;">no</span><span style="color:#ffffff;">&quot; </span><span style="color:#ff5e5e;">= if </span><span style="color:#fdb082;">1 </span><span style="color:#ff5e5e;">== </span><span style="color:#fdb082;">0</span><span style="color:#f8f8f2;">, </span><span style="color:#fdb082;">do: </span><span style="color:#ffffff;">&quot;</span><span style="color:#fbe3bf;">yes</span><span style="color:#ffffff;">&quot;</span><span style="color:#f8f8f2;">, </span><span style="color:#fdb082;">else: </span><span style="color:#ffffff;">&quot;</span><span style="color:#fbe3bf;">no</span><span style="color:#ffffff;">&quot;
    /// </span></pre>
    Leet,
    /// Monokai Extended
    ///
    /// <pre style="background-color:#222222;">
    /// <span style="color:#75715e;"># There currently is no ternary operator like  true ? &quot;yes&quot; : &quot;no&quot;
    /// </span><span style="color:#75715e;"># So the following is suggested
    /// </span><span style="color:#e6db74;">&quot;no&quot; </span><span style="color:#f92672;">= if </span><span style="color:#be84ff;">1 </span><span style="color:#f92672;">== </span><span style="color:#be84ff;">0</span><span style="color:#f8f8f2;">, </span><span style="color:#be84ff;">do: </span><span style="color:#e6db74;">&quot;yes&quot;</span><span style="color:#f8f8f2;">, </span><span style="color:#be84ff;">else: </span><span style="color:#e6db74;">&quot;no&quot;
    /// </span></pre>
    MonokaiExtended,
    /// Monokai Extended Bright
    ///
    /// <pre style="background-color:#272822;">
    /// <span style="color:#75715e;"># There currently is no ternary operator like  true ? &quot;yes&quot; : &quot;no&quot;
    /// </span><span style="color:#75715e;"># So the following is suggested
    /// </span><span style="color:#e6db74;">&quot;no&quot; </span><span style="color:#f92672;">= if </span><span style="color:#ae81ff;">1 </span><span style="color:#f92672;">== </span><span style="color:#ae81ff;">0</span><span style="color:#f8f8f2;">, </span><span style="color:#ae81ff;">do: </span><span style="color:#e6db74;">&quot;yes&quot;</span><span style="color:#f8f8f2;">, </span><span style="color:#ae81ff;">else: </span><span style="color:#e6db74;">&quot;no&quot;
    /// </span></pre>
    MonokaiExtendedBright,
    /// Monokai Extended Light
    ///
    /// <pre style="background-color:#fafafa;">
    /// <span style="color:#75715e;"># There currently is no ternary operator like  true ? &quot;yes&quot; : &quot;no&quot;
    /// </span><span style="color:#75715e;"># So the following is suggested
    /// </span><span style="color:#998f2f;">&quot;no&quot; </span><span style="color:#f9005a;">= if </span><span style="color:#684d99;">1 </span><span style="color:#f9005a;">== </span><span style="color:#684d99;">0</span><span style="color:#49483e;">, </span><span style="color:#684d99;">do: </span><span style="color:#998f2f;">&quot;yes&quot;</span><span style="color:#49483e;">, </span><span style="color:#684d99;">else: </span><span style="color:#998f2f;">&quot;no&quot;
    /// </span></pre>
    MonokaiExtendedLight,
    /// Monokai Extended Origin
    ///
    /// <pre style="background-color:#272822;">
    /// <span style="color:#75715e;"># There currently is no ternary operator like  true ? &quot;yes&quot; : &quot;no&quot;
    /// </span><span style="color:#75715e;"># So the following is suggested
    /// </span><span style="color:#e6db74;">&quot;no&quot; </span><span style="color:#f92672;">= if </span><span style="color:#ae81ff;">1 </span><span style="color:#f92672;">== </span><span style="color:#ae81ff;">0</span><span style="color:#f8f8f2;">, </span><span style="color:#ae81ff;">do: </span><span style="color:#e6db74;">&quot;yes&quot;</span><span style="color:#f8f8f2;">, </span><span style="color:#ae81ff;">else: </span><span style="color:#e6db74;">&quot;no&quot;
    /// </span></pre>
    MonokaiExtendedOrigin,
    /// Nord
    ///
    /// <pre style="background-color:#2e3440;">
    /// <span style="color:#616e88;"># There currently is no ternary operator like  true ? &quot;yes&quot; : &quot;no&quot;
    /// </span><span style="color:#616e88;"># So the following is suggested
    /// </span><span style="color:#a3be8c;">&quot;no&quot; </span><span style="color:#81a1c1;">= if </span><span style="color:#b48ead;">1 </span><span style="color:#81a1c1;">== </span><span style="color:#b48ead;">0</span><span style="color:#eceff4;">, </span><span style="color:#d8dee9;">do: </span><span style="color:#a3be8c;">&quot;yes&quot;</span><span style="color:#eceff4;">, </span><span style="color:#d8dee9;">else: </span><span style="color:#a3be8c;">&quot;no&quot;
    /// </span></pre>
    Nord,
    /// One Half Dark
    ///
    /// <pre style="background-color:#282c34;">
    /// <span style="color:#5c6370;"># There currently is no ternary operator like  true ? &quot;yes&quot; : &quot;no&quot;
    /// </span><span style="color:#5c6370;"># So the following is suggested
    /// </span><span style="color:#98c379;">&quot;no&quot; </span><span style="color:#c678dd;">= if </span><span style="color:#e5c07b;">1 </span><span style="color:#c678dd;">== </span><span style="color:#e5c07b;">0</span><span style="color:#dcdfe4;">, </span><span style="color:#e5c07b;">do: </span><span style="color:#98c379;">&quot;yes&quot;</span><span style="color:#dcdfe4;">, </span><span style="color:#e5c07b;">else: </span><span style="color:#98c379;">&quot;no&quot;
    /// </span></pre>
    OneHalfDark,
    /// One Half Light
    ///
    /// <pre style="background-color:#fafafa;">
    /// <span style="color:#a0a1a7;"># There currently is no ternary operator like  true ? &quot;yes&quot; : &quot;no&quot;
    /// </span><span style="color:#a0a1a7;"># So the following is suggested
    /// </span><span style="color:#50a14f;">&quot;no&quot; </span><span style="color:#a626a4;">= if </span><span style="color:#c18401;">1 </span><span style="color:#a626a4;">== </span><span style="color:#c18401;">0</span><span style="color:#383a42;">, </span><span style="color:#c18401;">do: </span><span style="color:#50a14f;">&quot;yes&quot;</span><span style="color:#383a42;">, </span><span style="color:#c18401;">else: </span><span style="color:#50a14f;">&quot;no&quot;
    /// </span></pre>
    OneHalfLight,
    /// Solarized (dark)
    ///
    /// <pre style="background-color:#002b36;">
    /// <span style="color:#586e75;"># There currently is no ternary operator like  true ? &quot;yes&quot; : &quot;no&quot;
    /// </span><span style="color:#586e75;"># So the following is suggested
    /// </span><span style="color:#839496;">&quot;</span><span style="color:#2aa198;">no</span><span style="color:#839496;">&quot; </span><span style="color:#657b83;">= </span><span style="color:#859900;">if </span><span style="color:#6c71c4;">1 </span><span style="color:#657b83;">== </span><span style="color:#6c71c4;">0</span><span style="color:#839496;">, </span><span style="color:#cb4b16;">do: </span><span style="color:#839496;">&quot;</span><span style="color:#2aa198;">yes</span><span style="color:#839496;">&quot;, </span><span style="color:#cb4b16;">else: </span><span style="color:#839496;">&quot;</span><span style="color:#2aa198;">no</span><span style="color:#839496;">&quot;
    /// </span></pre>
    SolarizedDark,
    /// Solarized (light)
    ///
    /// <pre style="background-color:#fdf6e3;">
    /// <span style="color:#93a1a1;"># There currently is no ternary operator like  true ? &quot;yes&quot; : &quot;no&quot;
    /// </span><span style="color:#93a1a1;"># So the following is suggested
    /// </span><span style="color:#839496;">&quot;</span><span style="color:#2aa198;">no</span><span style="color:#839496;">&quot; </span><span style="color:#657b83;">= </span><span style="color:#859900;">if </span><span style="color:#6c71c4;">1 </span><span style="color:#657b83;">== </span><span style="color:#6c71c4;">0</span><span style="color:#657b83;">, </span><span style="color:#cb4b16;">do: </span><span style="color:#839496;">&quot;</span><span style="color:#2aa198;">yes</span><span style="color:#839496;">&quot;</span><span style="color:#657b83;">, </span><span style="color:#cb4b16;">else: </span><span style="color:#839496;">&quot;</span><span style="color:#2aa198;">no</span><span style="color:#839496;">&quot;
    /// </span></pre>
    SolarizedLight,
    /// Sublime Snazzy
    ///
    /// <pre style="background-color:#282a36;">
    /// <span style="color:#686868;"># There currently is no ternary operator like  true ? &quot;yes&quot; : &quot;no&quot;
    /// </span><span style="color:#686868;"># So the following is suggested
    /// </span><span style="color:#f3f99d;">&quot;no&quot; </span><span style="color:#ff5c57;">= if </span><span style="color:#f1f1f0;">1 </span><span style="color:#ff5c57;">== </span><span style="color:#f1f1f0;">0</span><span style="color:#f8f8f2;">, </span><span style="color:#5af78e;">do: </span><span style="color:#f3f99d;">&quot;yes&quot;</span><span style="color:#f8f8f2;">, </span><span style="color:#5af78e;">else: </span><span style="color:#f3f99d;">&quot;no&quot;
    /// </span></pre>
    SublimeSnazzy,
    /// TwoDark
    ///
    /// <pre style="background-color:#282c34;">
    /// <span style="font-style:italic;color:#5c6370;"># There currently is no ternary operator like  true ? &quot;yes&quot; : &quot;no&quot;
    /// </span><span style="font-style:italic;color:#5c6370;"># So the following is suggested
    /// </span><span style="color:#98c379;">&quot;no&quot; </span><span style="color:#abb2bf;">= </span><span style="color:#c678dd;">if </span><span style="color:#d19a66;">1 </span><span style="color:#abb2bf;">== </span><span style="color:#d19a66;">0</span><span style="color:#abb2bf;">, </span><span style="color:#d19a66;">do: </span><span style="color:#98c379;">&quot;yes&quot;</span><span style="color:#abb2bf;">, </span><span style="color:#d19a66;">else: </span><span style="color:#98c379;">&quot;no&quot;
    /// </span></pre>
    TwoDark,
    /// Visual Studio Dark+
    ///
    /// <pre style="background-color:#1e1e1e;">
    /// <span style="color:#608b4e;"># There currently is no ternary operator like  true ? &quot;yes&quot; : &quot;no&quot;
    /// </span><span style="color:#608b4e;"># So the following is suggested
    /// </span><span style="color:#d69d85;">&quot;no&quot; </span><span style="color:#dcdcdc;">= </span><span style="color:#c586c0;">if </span><span style="color:#b5cea8;">1 </span><span style="color:#dcdcdc;">== </span><span style="color:#b5cea8;">0</span><span style="color:#dcdcdc;">, </span><span style="color:#b4cea8;">do: </span><span style="color:#d69d85;">&quot;yes&quot;</span><span style="color:#dcdcdc;">, </span><span style="color:#b4cea8;">else: </span><span style="color:#d69d85;">&quot;no&quot;
    /// </span></pre>
    VisualStudioDarkPlus,
    /// zenburn
    ///
    /// <pre style="background-color:#3f3f3f;">
    /// <span style="color:#a0cfa1;">#</span><span style="color:#87ae86;"> There currently is no ternary operator like  true ? &quot;yes&quot; : &quot;no&quot;
    /// </span><span style="color:#a0cfa1;">#</span><span style="color:#87ae86;"> So the following is suggested
    /// </span><span style="color:#d6d6d680;">&quot;</span><span style="color:#d68686;">no</span><span style="color:#d6d6d680;">&quot; </span><span style="color:#ececec;">= </span><span style="color:#fed6af;">if </span><span style="font-weight:bold;color:#87d6d5;">1 </span><span style="color:#ececec;">== </span><span style="font-weight:bold;color:#87d6d5;">0</span><span style="color:#dedede;">, </span><span style="font-weight:bold;color:#d58684;">do: </span><span style="color:#d6d6d680;">&quot;</span><span style="color:#d68686;">yes</span><span style="color:#d6d6d680;">&quot;</span><span style="color:#dedede;">, </span><span style="font-weight:bold;color:#d58684;">else: </span><span style="color:#d6d6d680;">&quot;</span><span style="color:#d68686;">no</span><span style="color:#d6d6d680;">&quot;
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
    /// assert_eq!(
    ///     EmbeddedThemeName::VisualStudioDarkPlus.as_name(),
    ///     "Visual Studio Dark+",
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
            Self::VisualStudioDarkPlus => "Visual Studio Dark+",
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
