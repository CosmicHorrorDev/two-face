use two_face::theme::EmbeddedThemeName;

const SAMPLE_ELIXIR: &str = r#"# sends a variety of values over a channel
function producer(c::Channel)
    for n=1:4
        put!(c, 2n)
    end
    put!(c, "stop")
end;
"#;

fn sample_html(name: EmbeddedThemeName) -> String {
    let syn_set = two_face::syntax::extra_newlines();
    let theme_set = two_face::theme::extra();

    let syn_ref = syn_set.find_syntax_by_extension("ex").unwrap();
    let theme = theme_set.get(name);
    syntect::html::highlighted_html_for_string(SAMPLE_ELIXIR, &syn_set, syn_ref, theme).unwrap()
}

#[test]
fn ansi() {
    insta::assert_snapshot!(
        sample_html(EmbeddedThemeName::Ansi),
        @r#"
    <pre style="background-color:#000000;">
    <span style="color:#02000000;"># sends a variety of values over a channel
    </span><span style="color:#00000001;">function producer(c::</span><span style="color:#03000000;">Channel</span><span style="color:#00000001;">)
    </span><span style="color:#00000001;">    </span><span style="color:#05000000;">for</span><span style="color:#00000001;"> n</span><span style="color:#05000000;">=</span><span style="color:#03000000;">1</span><span style="color:#00000001;">:</span><span style="color:#03000000;">4
    </span><span style="color:#00000001;">        put!(c, 2n)
    </span><span style="color:#00000001;">    </span><span style="color:#05000000;">end
    </span><span style="color:#00000001;">    put!(c, </span><span style="color:#02000000;">&quot;stop&quot;</span><span style="color:#00000001;">)
    </span><span style="color:#05000000;">end</span><span style="color:#00000001;">;
    </span></pre>
    "#
    );
}

#[test]
fn base16() {
    insta::assert_snapshot!(
        sample_html(EmbeddedThemeName::Base16),
        @r#"
    <pre style="background-color:#000000;">
    <span style="color:#08000000;"># sends a variety of values over a channel
    </span><span style="color:#07000000;">function producer(c::</span><span style="color:#03000000;">Channel</span><span style="color:#07000000;">)
    </span><span style="color:#07000000;">    </span><span style="color:#05000000;">for</span><span style="color:#07000000;"> n=</span><span style="color:#09000000;">1</span><span style="color:#07000000;">:</span><span style="color:#09000000;">4
    </span><span style="color:#07000000;">        put!(c, 2n)
    </span><span style="color:#07000000;">    </span><span style="color:#05000000;">end
    </span><span style="color:#07000000;">    put!(c, &quot;</span><span style="color:#02000000;">stop</span><span style="color:#07000000;">&quot;)
    </span><span style="color:#05000000;">end</span><span style="color:#07000000;">;
    </span></pre>
    "#
    );
}

#[test]
fn base16_eighties_dark() {
    insta::assert_snapshot!(
        sample_html(EmbeddedThemeName::Base16EightiesDark),
        @r#"
    <pre style="background-color:#2d2d2d;">
    <span style="color:#747369;"># sends a variety of values over a channel
    </span><span style="color:#d3d0c8;">function producer(c::</span><span style="color:#ffcc66;">Channel</span><span style="color:#d3d0c8;">)
    </span><span style="color:#d3d0c8;">    </span><span style="color:#cc99cc;">for</span><span style="color:#d3d0c8;"> n=</span><span style="color:#f99157;">1</span><span style="color:#d3d0c8;">:</span><span style="color:#f99157;">4
    </span><span style="color:#d3d0c8;">        put!(c, 2n)
    </span><span style="color:#d3d0c8;">    </span><span style="color:#cc99cc;">end
    </span><span style="color:#d3d0c8;">    put!(c, &quot;</span><span style="color:#99cc99;">stop</span><span style="color:#d3d0c8;">&quot;)
    </span><span style="color:#cc99cc;">end</span><span style="color:#d3d0c8;">;
    </span></pre>
    "#
    );
}

#[test]
fn base16_mocha_dark() {
    insta::assert_snapshot!(
        sample_html(EmbeddedThemeName::Base16MochaDark),
        @r#"
    <pre style="background-color:#3b3228;">
    <span style="color:#7e705a;"># sends a variety of values over a channel
    </span><span style="color:#d0c8c6;">function producer(c::</span><span style="color:#f4bc87;">Channel</span><span style="color:#d0c8c6;">)
    </span><span style="color:#d0c8c6;">    </span><span style="color:#a89bb9;">for</span><span style="color:#d0c8c6;"> n=</span><span style="color:#d28b71;">1</span><span style="color:#d0c8c6;">:</span><span style="color:#d28b71;">4
    </span><span style="color:#d0c8c6;">        put!(c, 2n)
    </span><span style="color:#d0c8c6;">    </span><span style="color:#a89bb9;">end
    </span><span style="color:#d0c8c6;">    put!(c, &quot;</span><span style="color:#beb55b;">stop</span><span style="color:#d0c8c6;">&quot;)
    </span><span style="color:#a89bb9;">end</span><span style="color:#d0c8c6;">;
    </span></pre>
    "#
    );
}

#[test]
fn base16_ocean_dark() {
    insta::assert_snapshot!(
        sample_html(EmbeddedThemeName::Base16OceanDark),
        @r#"
    <pre style="background-color:#2b303b;">
    <span style="color:#65737e;"># sends a variety of values over a channel
    </span><span style="color:#c0c5ce;">function producer(c::</span><span style="color:#ebcb8b;">Channel</span><span style="color:#c0c5ce;">)
    </span><span style="color:#c0c5ce;">    </span><span style="color:#b48ead;">for</span><span style="color:#c0c5ce;"> n=</span><span style="color:#d08770;">1</span><span style="color:#c0c5ce;">:</span><span style="color:#d08770;">4
    </span><span style="color:#c0c5ce;">        put!(c, 2n)
    </span><span style="color:#c0c5ce;">    </span><span style="color:#b48ead;">end
    </span><span style="color:#c0c5ce;">    put!(c, &quot;</span><span style="color:#a3be8c;">stop</span><span style="color:#c0c5ce;">&quot;)
    </span><span style="color:#b48ead;">end</span><span style="color:#c0c5ce;">;
    </span></pre>
    "#
    );
}

#[test]
fn base16_ocean_light() {
    insta::assert_snapshot!(
        sample_html(EmbeddedThemeName::Base16OceanLight),
        @r#"
    <pre style="background-color:#eff1f5;">
    <span style="color:#a7adba;"># sends a variety of values over a channel
    </span><span style="color:#4f5b66;">function producer(c::</span><span style="color:#d08770;">Channel</span><span style="color:#4f5b66;">)
    </span><span style="color:#4f5b66;">    </span><span style="color:#b48ead;">for</span><span style="color:#4f5b66;"> n=</span><span style="color:#d08770;">1</span><span style="color:#4f5b66;">:</span><span style="color:#d08770;">4
    </span><span style="color:#4f5b66;">        put!(c, 2n)
    </span><span style="color:#4f5b66;">    </span><span style="color:#b48ead;">end
    </span><span style="color:#4f5b66;">    put!(c, &quot;</span><span style="color:#a3be8c;">stop</span><span style="color:#4f5b66;">&quot;)
    </span><span style="color:#b48ead;">end</span><span style="color:#4f5b66;">;
    </span></pre>
    "#
    );
}

#[test]
fn base16_256() {
    insta::assert_snapshot!(
        sample_html(EmbeddedThemeName::Base16_256),
        @r#"
    <pre style="background-color:#000000;">
    <span style="color:#08000000;"># sends a variety of values over a channel
    </span><span style="color:#07000000;">function producer(c::</span><span style="color:#03000000;">Channel</span><span style="color:#07000000;">)
    </span><span style="color:#07000000;">    </span><span style="color:#05000000;">for</span><span style="color:#07000000;"> n=</span><span style="color:#10000000;">1</span><span style="color:#07000000;">:</span><span style="color:#10000000;">4
    </span><span style="color:#07000000;">        put!(c, 2n)
    </span><span style="color:#07000000;">    </span><span style="color:#05000000;">end
    </span><span style="color:#07000000;">    put!(c, &quot;</span><span style="color:#02000000;">stop</span><span style="color:#07000000;">&quot;)
    </span><span style="color:#05000000;">end</span><span style="color:#07000000;">;
    </span></pre>
    "#
    );
}

#[test]
fn catppuccin_frappe() {
    insta::assert_snapshot!(
        sample_html(EmbeddedThemeName::CatppuccinFrappe),
        @r#"
    <pre style="background-color:#303446;">
    <span style="font-style:italic;color:#949cbb;"># sends a variety of values over a channel
    </span><span style="color:#c6d0f5;">function producer</span><span style="color:#949cbb;">(</span><span style="color:#c6d0f5;">c::</span><span style="font-style:italic;color:#e5c890;">Channel</span><span style="color:#949cbb;">)
    </span><span style="color:#c6d0f5;">    </span><span style="color:#ca9ee6;">for</span><span style="color:#c6d0f5;"> n</span><span style="color:#81c8be;">=</span><span style="color:#ef9f76;">1</span><span style="color:#c6d0f5;">:</span><span style="color:#ef9f76;">4
    </span><span style="color:#c6d0f5;">        put!</span><span style="color:#949cbb;">(</span><span style="color:#c6d0f5;">c</span><span style="color:#949cbb;">,</span><span style="color:#c6d0f5;"> 2n</span><span style="color:#949cbb;">)
    </span><span style="color:#c6d0f5;">    </span><span style="color:#ca9ee6;">end
    </span><span style="color:#c6d0f5;">    put!</span><span style="color:#949cbb;">(</span><span style="color:#c6d0f5;">c</span><span style="color:#949cbb;">, </span><span style="color:#a6d189;">&quot;stop&quot;</span><span style="color:#949cbb;">)
    </span><span style="color:#ca9ee6;">end</span><span style="color:#949cbb;">;
    </span></pre>
    "#
    );
}

#[test]
fn catppuccin_latte() {
    insta::assert_snapshot!(sample_html(EmbeddedThemeName::CatppuccinLatte),
        @r#"
    <pre style="background-color:#eff1f5;">
    <span style="font-style:italic;color:#7c7f93;"># sends a variety of values over a channel
    </span><span style="color:#4c4f69;">function producer</span><span style="color:#7c7f93;">(</span><span style="color:#4c4f69;">c::</span><span style="font-style:italic;color:#df8e1d;">Channel</span><span style="color:#7c7f93;">)
    </span><span style="color:#4c4f69;">    </span><span style="color:#8839ef;">for</span><span style="color:#4c4f69;"> n</span><span style="color:#179299;">=</span><span style="color:#fe640b;">1</span><span style="color:#4c4f69;">:</span><span style="color:#fe640b;">4
    </span><span style="color:#4c4f69;">        put!</span><span style="color:#7c7f93;">(</span><span style="color:#4c4f69;">c</span><span style="color:#7c7f93;">,</span><span style="color:#4c4f69;"> 2n</span><span style="color:#7c7f93;">)
    </span><span style="color:#4c4f69;">    </span><span style="color:#8839ef;">end
    </span><span style="color:#4c4f69;">    put!</span><span style="color:#7c7f93;">(</span><span style="color:#4c4f69;">c</span><span style="color:#7c7f93;">, </span><span style="color:#40a02b;">&quot;stop&quot;</span><span style="color:#7c7f93;">)
    </span><span style="color:#8839ef;">end</span><span style="color:#7c7f93;">;
    </span></pre>
    "#
    );
}

#[test]
fn catppuccin_macchiato() {
    insta::assert_snapshot!(sample_html(EmbeddedThemeName::CatppuccinMacchiato),
        @r#"
    <pre style="background-color:#24273a;">
    <span style="font-style:italic;color:#939ab7;"># sends a variety of values over a channel
    </span><span style="color:#cad3f5;">function producer</span><span style="color:#939ab7;">(</span><span style="color:#cad3f5;">c::</span><span style="font-style:italic;color:#eed49f;">Channel</span><span style="color:#939ab7;">)
    </span><span style="color:#cad3f5;">    </span><span style="color:#c6a0f6;">for</span><span style="color:#cad3f5;"> n</span><span style="color:#8bd5ca;">=</span><span style="color:#f5a97f;">1</span><span style="color:#cad3f5;">:</span><span style="color:#f5a97f;">4
    </span><span style="color:#cad3f5;">        put!</span><span style="color:#939ab7;">(</span><span style="color:#cad3f5;">c</span><span style="color:#939ab7;">,</span><span style="color:#cad3f5;"> 2n</span><span style="color:#939ab7;">)
    </span><span style="color:#cad3f5;">    </span><span style="color:#c6a0f6;">end
    </span><span style="color:#cad3f5;">    put!</span><span style="color:#939ab7;">(</span><span style="color:#cad3f5;">c</span><span style="color:#939ab7;">, </span><span style="color:#a6da95;">&quot;stop&quot;</span><span style="color:#939ab7;">)
    </span><span style="color:#c6a0f6;">end</span><span style="color:#939ab7;">;
    </span></pre>
    "#
    );
}

#[test]
fn catppuccin_mocha() {
    insta::assert_snapshot!(sample_html(EmbeddedThemeName::CatppuccinMocha),
        @r#"
    <pre style="background-color:#1e1e2e;">
    <span style="font-style:italic;color:#9399b2;"># sends a variety of values over a channel
    </span><span style="color:#cdd6f4;">function producer</span><span style="color:#9399b2;">(</span><span style="color:#cdd6f4;">c::</span><span style="font-style:italic;color:#f9e2af;">Channel</span><span style="color:#9399b2;">)
    </span><span style="color:#cdd6f4;">    </span><span style="color:#cba6f7;">for</span><span style="color:#cdd6f4;"> n</span><span style="color:#94e2d5;">=</span><span style="color:#fab387;">1</span><span style="color:#cdd6f4;">:</span><span style="color:#fab387;">4
    </span><span style="color:#cdd6f4;">        put!</span><span style="color:#9399b2;">(</span><span style="color:#cdd6f4;">c</span><span style="color:#9399b2;">,</span><span style="color:#cdd6f4;"> 2n</span><span style="color:#9399b2;">)
    </span><span style="color:#cdd6f4;">    </span><span style="color:#cba6f7;">end
    </span><span style="color:#cdd6f4;">    put!</span><span style="color:#9399b2;">(</span><span style="color:#cdd6f4;">c</span><span style="color:#9399b2;">, </span><span style="color:#a6e3a1;">&quot;stop&quot;</span><span style="color:#9399b2;">)
    </span><span style="color:#cba6f7;">end</span><span style="color:#9399b2;">;
    </span></pre>
    "#
    );
}

#[test]
fn coldark_cold() {
    insta::assert_snapshot!(
        sample_html(EmbeddedThemeName::ColdarkCold),
        @r#"
    <pre style="background-color:#e3eaf2;">
    <span style="color:#3c526d;"># sends a variety of values over a channel
    </span><span style="color:#111b27;">function producer(c::Channel)
    </span><span style="color:#111b27;">    </span><span style="color:#a04900;">for</span><span style="color:#111b27;"> n</span><span style="color:#a04900;">=</span><span style="color:#755f00;">1</span><span style="color:#111b27;">:</span><span style="color:#755f00;">4
    </span><span style="color:#111b27;">        put!(c, 2n)
    </span><span style="color:#111b27;">    </span><span style="color:#a04900;">end
    </span><span style="color:#111b27;">    put!(c, </span><span style="color:#116b00;">&quot;stop&quot;</span><span style="color:#111b27;">)
    </span><span style="color:#a04900;">end</span><span style="color:#111b27;">;
    </span></pre>
    "#
    );
}

#[test]
fn coldark_dark() {
    insta::assert_snapshot!(
        sample_html(EmbeddedThemeName::ColdarkDark),
        @r#"
    <pre style="background-color:#111b27;">
    <span style="color:#8da1b9;"># sends a variety of values over a channel
    </span><span style="color:#e3eaf2;">function producer(c::Channel)
    </span><span style="color:#e3eaf2;">    </span><span style="color:#e9ae7e;">for</span><span style="color:#e3eaf2;"> n</span><span style="color:#e9ae7e;">=</span><span style="color:#e6d37a;">1</span><span style="color:#e3eaf2;">:</span><span style="color:#e6d37a;">4
    </span><span style="color:#e3eaf2;">        put!(c, 2n)
    </span><span style="color:#e3eaf2;">    </span><span style="color:#e9ae7e;">end
    </span><span style="color:#e3eaf2;">    put!(c, </span><span style="color:#91d076;">&quot;stop&quot;</span><span style="color:#e3eaf2;">)
    </span><span style="color:#e9ae7e;">end</span><span style="color:#e3eaf2;">;
    </span></pre>
    "#
    );
}

#[test]
fn dark_neon() {
    insta::assert_snapshot!(
        sample_html(EmbeddedThemeName::DarkNeon),
        @r#"
    <pre style="background-color:#000000;">
    <span style="background-color:#212121;color:#7c7c7c;"># sends a variety of values over a channel
    </span><span style="color:#ffffff;">function producer(c::</span><span style="color:#f8f8f8;">Channel</span><span style="color:#ffffff;">)
    </span><span style="color:#ffffff;">    </span><span style="color:#66ccff;">for</span><span style="color:#ffffff;"> n</span><span style="color:#aaaaaa;">=</span><span style="font-weight:bold;color:#ff73fd;">1</span><span style="color:#ffffff;">:</span><span style="font-weight:bold;color:#ff73fd;">4
    </span><span style="color:#ffffff;">        put!(c, 2n)
    </span><span style="color:#ffffff;">    </span><span style="color:#66ccff;">end
    </span><span style="color:#ffffff;">    put!(c, </span><span style="color:#ccff66;">&quot;stop&quot;</span><span style="color:#ffffff;">)
    </span><span style="color:#66ccff;">end</span><span style="color:#ffffff;">;
    </span></pre>
    "#
    );
}

#[test]
fn dracula() {
    insta::assert_snapshot!(
        sample_html(EmbeddedThemeName::Dracula),
        @r#"
    <pre style="background-color:#282a36;">
    <span style="color:#6272a4;"># sends a variety of values over a channel
    </span><span style="color:#f8f8f2;">function producer(c::</span><span style="text-decoration:underline;color:#8be9fd;">Channel</span><span style="color:#f8f8f2;">)
    </span><span style="color:#f8f8f2;">    </span><span style="color:#ff79c6;">for</span><span style="color:#f8f8f2;"> n</span><span style="color:#ff79c6;">=</span><span style="color:#bd93f9;">1</span><span style="color:#f8f8f2;">:</span><span style="color:#bd93f9;">4
    </span><span style="color:#f8f8f2;">        put!(c, 2n)
    </span><span style="color:#f8f8f2;">    </span><span style="color:#ff79c6;">end
    </span><span style="color:#f8f8f2;">    put!(c, </span><span style="color:#f1fa8c;">&quot;stop&quot;</span><span style="color:#f8f8f2;">)
    </span><span style="color:#ff79c6;">end</span><span style="color:#f8f8f2;">;
    </span></pre>
    "#
    );
}

#[test]
fn github() {
    insta::assert_snapshot!(
        sample_html(EmbeddedThemeName::Github),
        @r#"
    <pre style="background-color:#ffffff;">
    <span style="color:#969896;"># sends a variety of values over a channel
    </span><span style="color:#333333;">function producer(c::Channel)
    </span><span style="color:#333333;">    </span><span style="color:#a71d5d;">for</span><span style="color:#333333;"> n</span><span style="color:#a71d5d;">=</span><span style="color:#0086b3;">1</span><span style="color:#333333;">:</span><span style="color:#0086b3;">4
    </span><span style="color:#333333;">        put!(c, 2n)
    </span><span style="color:#333333;">    </span><span style="color:#a71d5d;">end
    </span><span style="color:#333333;">    put!(c, </span><span style="color:#183691;">&quot;stop&quot;</span><span style="color:#333333;">)
    </span><span style="color:#a71d5d;">end</span><span style="color:#333333;">;
    </span></pre>
    "#
    );
}

#[test]
fn gruvbox_dark() {
    insta::assert_snapshot!(
        sample_html(EmbeddedThemeName::GruvboxDark),
        @r#"
    <pre style="background-color:#282828;">
    <span style="font-style:italic;color:#928374;"># sends a variety of values over a channel
    </span><span style="color:#fbf1c7;">function producer(c::</span><span style="color:#fabd2f;">Channel</span><span style="color:#fbf1c7;">)
    </span><span style="color:#fbf1c7;">    </span><span style="color:#fb4934;">for</span><span style="color:#fbf1c7;"> n</span><span style="color:#8ec07c;">=</span><span style="color:#d3869b;">1</span><span style="color:#fbf1c7;">:</span><span style="color:#d3869b;">4
    </span><span style="color:#fbf1c7;">        put!(c, 2n)
    </span><span style="color:#fbf1c7;">    </span><span style="color:#fb4934;">end
    </span><span style="color:#fbf1c7;">    put!(c, &quot;</span><span style="color:#b8bb26;">stop</span><span style="color:#fbf1c7;">&quot;)
    </span><span style="color:#fb4934;">end</span><span style="color:#fbf1c7;">;
    </span></pre>
    "#
    );
}

#[test]
fn gruvbox_light() {
    insta::assert_snapshot!(
        sample_html(EmbeddedThemeName::GruvboxLight),
        @r#"
    <pre style="background-color:#fbf1c7;">
    <span style="font-style:italic;color:#928374;"># sends a variety of values over a channel
    </span><span style="color:#282828;">function producer(c::</span><span style="color:#b57614;">Channel</span><span style="color:#282828;">)
    </span><span style="color:#282828;">    </span><span style="color:#9d0006;">for</span><span style="color:#282828;"> n</span><span style="color:#427b58;">=</span><span style="color:#8f3f71;">1</span><span style="color:#282828;">:</span><span style="color:#8f3f71;">4
    </span><span style="color:#282828;">        put!(c, 2n)
    </span><span style="color:#282828;">    </span><span style="color:#9d0006;">end
    </span><span style="color:#282828;">    put!(c, &quot;</span><span style="color:#79740e;">stop</span><span style="color:#282828;">&quot;)
    </span><span style="color:#9d0006;">end</span><span style="color:#282828;">;
    </span></pre>
    "#
    );
}

#[test]
fn inspired_github() {
    insta::assert_snapshot!(
        sample_html(EmbeddedThemeName::InspiredGithub),
        @r#"
    <pre style="background-color:#ffffff;">
    <span style="font-style:italic;color:#969896;"># sends a variety of values over a channel
    </span><span style="color:#323232;">function producer(c::</span><span style="color:#0086b3;">Channel</span><span style="color:#323232;">)
    </span><span style="color:#323232;">    </span><span style="font-weight:bold;color:#a71d5d;">for</span><span style="color:#323232;"> n</span><span style="font-weight:bold;color:#a71d5d;">=</span><span style="color:#0086b3;">1</span><span style="color:#323232;">:</span><span style="color:#0086b3;">4
    </span><span style="color:#323232;">        put!(c, 2n)
    </span><span style="color:#323232;">    </span><span style="font-weight:bold;color:#a71d5d;">end
    </span><span style="color:#323232;">    put!(c, </span><span style="color:#183691;">&quot;stop&quot;</span><span style="color:#323232;">)
    </span><span style="font-weight:bold;color:#a71d5d;">end</span><span style="color:#323232;">;
    </span></pre>
    "#
    );
}

#[test]
fn leet() {
    insta::assert_snapshot!(
        sample_html(EmbeddedThemeName::Leet),
        @r#"
    <pre style="background-color:#191919;">
    <span style="color:#6d6d6d;"># sends a variety of values over a channel
    </span><span style="color:#f8f8f2;">function producer(c::</span><span style="text-decoration:underline;color:#8cdaff;">Channel</span><span style="color:#f8f8f2;">)
    </span><span style="color:#f8f8f2;">    </span><span style="color:#ff5e5e;">for</span><span style="color:#f8f8f2;"> n</span><span style="color:#ff5e5e;">=</span><span style="color:#fdb082;">1</span><span style="color:#f8f8f2;">:</span><span style="color:#fdb082;">4
    </span><span style="color:#f8f8f2;">        put!(c, 2n)
    </span><span style="color:#f8f8f2;">    </span><span style="color:#ff5e5e;">end
    </span><span style="color:#f8f8f2;">    put!(c, </span><span style="color:#ffffff;">&quot;</span><span style="color:#fbe3bf;">stop</span><span style="color:#ffffff;">&quot;</span><span style="color:#f8f8f2;">)
    </span><span style="color:#ff5e5e;">end</span><span style="color:#f8f8f2;">;
    </span></pre>
    "#
    );
}

#[test]
fn monokai_extended() {
    insta::assert_snapshot!(
        sample_html(EmbeddedThemeName::MonokaiExtended),
        @r#"
    <pre style="background-color:#222222;">
    <span style="color:#75715e;"># sends a variety of values over a channel
    </span><span style="color:#f8f8f2;">function producer(c::</span><span style="text-decoration:underline;color:#66d9ef;">Channel</span><span style="color:#f8f8f2;">)
    </span><span style="color:#f8f8f2;">    </span><span style="color:#f92672;">for</span><span style="color:#f8f8f2;"> n</span><span style="color:#f92672;">=</span><span style="color:#be84ff;">1</span><span style="color:#f8f8f2;">:</span><span style="color:#be84ff;">4
    </span><span style="color:#f8f8f2;">        put!(c, 2n)
    </span><span style="color:#f8f8f2;">    </span><span style="color:#f92672;">end
    </span><span style="color:#f8f8f2;">    put!(c, </span><span style="color:#e6db74;">&quot;stop&quot;</span><span style="color:#f8f8f2;">)
    </span><span style="color:#f92672;">end</span><span style="color:#f8f8f2;">;
    </span></pre>
    "#
    );
}

#[test]
fn monokai_extended_bright() {
    insta::assert_snapshot!(
        sample_html(EmbeddedThemeName::MonokaiExtendedBright),
        @r#"
    <pre style="background-color:#272822;">
    <span style="color:#75715e;"># sends a variety of values over a channel
    </span><span style="color:#f8f8f2;">function producer(c::</span><span style="text-decoration:underline;color:#a6e22e;">Channel</span><span style="color:#f8f8f2;">)
    </span><span style="color:#f8f8f2;">    </span><span style="color:#f92672;">for</span><span style="color:#f8f8f2;"> n</span><span style="color:#f92672;">=</span><span style="color:#ae81ff;">1</span><span style="color:#f8f8f2;">:</span><span style="color:#ae81ff;">4
    </span><span style="color:#f8f8f2;">        put!(c, 2n)
    </span><span style="color:#f8f8f2;">    </span><span style="color:#f92672;">end
    </span><span style="color:#f8f8f2;">    put!(c, </span><span style="color:#e6db74;">&quot;stop&quot;</span><span style="color:#f8f8f2;">)
    </span><span style="color:#f92672;">end</span><span style="color:#f8f8f2;">;
    </span></pre>
    "#
    );
}

#[test]
fn monokai_extended_light() {
    insta::assert_snapshot!(
        sample_html(EmbeddedThemeName::MonokaiExtendedLight),
        @r#"
    <pre style="background-color:#fafafa;">
    <span style="color:#75715e;"># sends a variety of values over a channel
    </span><span style="color:#49483e;">function producer(c::</span><span style="text-decoration:underline;color:#679c00;">Channel</span><span style="color:#49483e;">)
    </span><span style="color:#49483e;">    </span><span style="color:#f9005a;">for</span><span style="color:#49483e;"> n</span><span style="color:#f9005a;">=</span><span style="color:#684d99;">1</span><span style="color:#49483e;">:</span><span style="color:#684d99;">4
    </span><span style="color:#49483e;">        put!(c, 2n)
    </span><span style="color:#49483e;">    </span><span style="color:#f9005a;">end
    </span><span style="color:#49483e;">    put!(c, </span><span style="color:#998f2f;">&quot;stop&quot;</span><span style="color:#49483e;">)
    </span><span style="color:#f9005a;">end</span><span style="color:#49483e;">;
    </span></pre>
    "#
    );
}

#[test]
fn monokai_extended_origin() {
    insta::assert_snapshot!(
        sample_html(EmbeddedThemeName::MonokaiExtendedOrigin),
        @r#"
    <pre style="background-color:#272822;">
    <span style="color:#75715e;"># sends a variety of values over a channel
    </span><span style="color:#f8f8f2;">function producer(c::</span><span style="text-decoration:underline;color:#a6e22e;">Channel</span><span style="color:#f8f8f2;">)
    </span><span style="color:#f8f8f2;">    </span><span style="color:#f92672;">for</span><span style="color:#f8f8f2;"> n</span><span style="color:#f92672;">=</span><span style="color:#ae81ff;">1</span><span style="color:#f8f8f2;">:</span><span style="color:#ae81ff;">4
    </span><span style="color:#f8f8f2;">        put!(c, 2n)
    </span><span style="color:#f8f8f2;">    </span><span style="color:#f92672;">end
    </span><span style="color:#f8f8f2;">    put!(c, </span><span style="color:#e6db74;">&quot;stop&quot;</span><span style="color:#f8f8f2;">)
    </span><span style="color:#f92672;">end</span><span style="color:#f8f8f2;">;
    </span></pre>
    "#
    );
}

#[test]
fn nord() {
    insta::assert_snapshot!(
        sample_html(EmbeddedThemeName::Nord),
        @r#"
    <pre style="background-color:#2e3440;">
    <span style="color:#616e88;"># sends a variety of values over a channel
    </span><span style="color:#d8dee9;">function producer(c::</span><span style="color:#8fbcbb;">Channel</span><span style="color:#d8dee9;">)
    </span><span style="color:#d8dee9;">    </span><span style="color:#81a1c1;">for</span><span style="color:#d8dee9;"> n</span><span style="color:#81a1c1;">=</span><span style="color:#b48ead;">1</span><span style="color:#d8dee9;">:</span><span style="color:#b48ead;">4
    </span><span style="color:#d8dee9;">        put!(c</span><span style="color:#eceff4;">,</span><span style="color:#d8dee9;"> 2n)
    </span><span style="color:#d8dee9;">    </span><span style="color:#81a1c1;">end
    </span><span style="color:#d8dee9;">    put!(c</span><span style="color:#eceff4;">, </span><span style="color:#a3be8c;">&quot;stop&quot;</span><span style="color:#d8dee9;">)
    </span><span style="color:#81a1c1;">end</span><span style="color:#eceff4;">;
    </span></pre>
    "#
    );
}

#[test]
fn one_half_dark() {
    insta::assert_snapshot!(
        sample_html(EmbeddedThemeName::OneHalfDark),
        @r#"
    <pre style="background-color:#282c34;">
    <span style="color:#5c6370;"># sends a variety of values over a channel
    </span><span style="color:#dcdfe4;">function producer(c::</span><span style="color:#e5c07b;">Channel</span><span style="color:#dcdfe4;">)
    </span><span style="color:#dcdfe4;">    </span><span style="color:#c678dd;">for</span><span style="color:#dcdfe4;"> n</span><span style="color:#c678dd;">=</span><span style="color:#e5c07b;">1</span><span style="color:#dcdfe4;">:</span><span style="color:#e5c07b;">4
    </span><span style="color:#dcdfe4;">        put!(c, 2n)
    </span><span style="color:#dcdfe4;">    </span><span style="color:#c678dd;">end
    </span><span style="color:#dcdfe4;">    put!(c, </span><span style="color:#98c379;">&quot;stop&quot;</span><span style="color:#dcdfe4;">)
    </span><span style="color:#c678dd;">end</span><span style="color:#dcdfe4;">;
    </span></pre>
    "#
    );
}

#[test]
fn one_half_light() {
    insta::assert_snapshot!(
        sample_html(EmbeddedThemeName::OneHalfLight),
        @r#"
    <pre style="background-color:#fafafa;">
    <span style="color:#a0a1a7;"># sends a variety of values over a channel
    </span><span style="color:#383a42;">function producer(c::</span><span style="color:#c18401;">Channel</span><span style="color:#383a42;">)
    </span><span style="color:#383a42;">    </span><span style="color:#a626a4;">for</span><span style="color:#383a42;"> n</span><span style="color:#a626a4;">=</span><span style="color:#c18401;">1</span><span style="color:#383a42;">:</span><span style="color:#c18401;">4
    </span><span style="color:#383a42;">        put!(c, 2n)
    </span><span style="color:#383a42;">    </span><span style="color:#a626a4;">end
    </span><span style="color:#383a42;">    put!(c, </span><span style="color:#50a14f;">&quot;stop&quot;</span><span style="color:#383a42;">)
    </span><span style="color:#a626a4;">end</span><span style="color:#383a42;">;
    </span></pre>
    "#
    );
}

#[test]
fn solarized_dark() {
    insta::assert_snapshot!(
        sample_html(EmbeddedThemeName::SolarizedDark),
        @r#"
    <pre style="background-color:#002b36;">
    <span style="color:#586e75;"># sends a variety of values over a channel
    </span><span style="color:#839496;">function producer</span><span style="color:#657b83;">(</span><span style="color:#839496;">c::</span><span style="color:#b58900;">Channel</span><span style="color:#657b83;">)
    </span><span style="color:#839496;">    </span><span style="color:#859900;">for</span><span style="color:#839496;"> n</span><span style="color:#657b83;">=</span><span style="color:#6c71c4;">1</span><span style="color:#839496;">:</span><span style="color:#6c71c4;">4
    </span><span style="color:#839496;">        put!</span><span style="color:#657b83;">(</span><span style="color:#839496;">c, 2n</span><span style="color:#657b83;">)
    </span><span style="color:#839496;">    </span><span style="color:#859900;">end
    </span><span style="color:#839496;">    put!</span><span style="color:#657b83;">(</span><span style="color:#839496;">c, &quot;</span><span style="color:#2aa198;">stop</span><span style="color:#839496;">&quot;</span><span style="color:#657b83;">)
    </span><span style="color:#859900;">end</span><span style="color:#839496;">;
    </span></pre>
    "#
    );
}

#[test]
fn solarized_light() {
    insta::assert_snapshot!(
        sample_html(EmbeddedThemeName::SolarizedLight),
        @r#"
    <pre style="background-color:#fdf6e3;">
    <span style="color:#93a1a1;"># sends a variety of values over a channel
    </span><span style="color:#657b83;">function producer(c::</span><span style="color:#b58900;">Channel</span><span style="color:#657b83;">)
    </span><span style="color:#657b83;">    </span><span style="color:#859900;">for</span><span style="color:#657b83;"> n=</span><span style="color:#6c71c4;">1</span><span style="color:#657b83;">:</span><span style="color:#6c71c4;">4
    </span><span style="color:#657b83;">        put!(c, 2n)
    </span><span style="color:#657b83;">    </span><span style="color:#859900;">end
    </span><span style="color:#657b83;">    put!(c, </span><span style="color:#839496;">&quot;</span><span style="color:#2aa198;">stop</span><span style="color:#839496;">&quot;</span><span style="color:#657b83;">)
    </span><span style="color:#859900;">end</span><span style="color:#657b83;">;
    </span></pre>
    "#
    );
}

#[test]
fn sublime_snazzy() {
    insta::assert_snapshot!(
        sample_html(EmbeddedThemeName::SublimeSnazzy),
        @r#"
    <pre style="background-color:#282a36;">
    <span style="color:#686868;"># sends a variety of values over a channel
    </span><span style="color:#f8f8f2;">function producer(c::</span><span style="text-decoration:underline;color:#9aedfe;">Channel</span><span style="color:#f8f8f2;">)
    </span><span style="color:#f8f8f2;">    </span><span style="color:#ff5c57;">for</span><span style="color:#f8f8f2;"> n</span><span style="color:#ff5c57;">=</span><span style="color:#f1f1f0;">1</span><span style="color:#f8f8f2;">:</span><span style="color:#f1f1f0;">4
    </span><span style="color:#f8f8f2;">        put!(c, 2n)
    </span><span style="color:#f8f8f2;">    </span><span style="color:#ff5c57;">end
    </span><span style="color:#f8f8f2;">    put!(c, </span><span style="color:#f3f99d;">&quot;stop&quot;</span><span style="color:#f8f8f2;">)
    </span><span style="color:#ff5c57;">end</span><span style="color:#f8f8f2;">;
    </span></pre>
    "#
    );
}

#[test]
fn two_dark() {
    insta::assert_snapshot!(
        sample_html(EmbeddedThemeName::TwoDark),
        @r#"
    <pre style="background-color:#282c34;">
    <span style="font-style:italic;color:#5c6370;"># sends a variety of values over a channel
    </span><span style="color:#abb2bf;">function producer(c::</span><span style="color:#e5c07b;">Channel</span><span style="color:#abb2bf;">)
    </span><span style="color:#abb2bf;">    </span><span style="color:#c678dd;">for</span><span style="color:#abb2bf;"> n=</span><span style="color:#d19a66;">1</span><span style="color:#abb2bf;">:</span><span style="color:#d19a66;">4
    </span><span style="color:#abb2bf;">        put!(c, 2n)
    </span><span style="color:#abb2bf;">    </span><span style="color:#c678dd;">end
    </span><span style="color:#abb2bf;">    put!(c, </span><span style="color:#98c379;">&quot;stop&quot;</span><span style="color:#abb2bf;">)
    </span><span style="color:#c678dd;">end</span><span style="color:#abb2bf;">;
    </span></pre>
    "#
    );
}

#[test]
fn zenburn() {
    insta::assert_snapshot!(
        sample_html(EmbeddedThemeName::Zenburn),
        @r#"
    <pre style="background-color:#3f3f3f;">
    <span style="color:#a0cfa1;">#</span><span style="color:#87ae86;"> sends a variety of values over a channel
    </span><span style="color:#dedede;">function producer(c::Channel)
    </span><span style="color:#dedede;">    </span><span style="color:#fed6af;">for</span><span style="color:#dedede;"> n</span><span style="color:#ececec;">=</span><span style="color:#87d6d5;">1</span><span style="color:#dedede;">:</span><span style="color:#87d6d5;">4
    </span><span style="color:#dedede;">        put!(c, 2n)
    </span><span style="color:#dedede;">    </span><span style="color:#fed6af;">end
    </span><span style="color:#dedede;">    put!(c, </span><span style="color:#d6d6d680;">&quot;</span><span style="color:#d68686;">stop</span><span style="color:#d6d6d680;">&quot;</span><span style="color:#dedede;">)
    </span><span style="color:#fed6af;">end</span><span style="color:#dedede;">;
    </span></pre>
    "#
    );
}
