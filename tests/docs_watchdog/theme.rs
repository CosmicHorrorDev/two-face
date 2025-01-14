use two_face::theme::EmbeddedThemeName;

const SAMPLE_ELIXIR: &str = r#"# There currently is no ternary operator like  true ? "yes" : "no"
# So the following is suggested
"no" = if 1 == 0, do: "yes", else: "no"
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
        @r###"
    <pre style="background-color:#000000;">
    <span style="color:#02000000;"># There currently is no ternary operator like  true ? &quot;yes&quot; : &quot;no&quot;
    </span><span style="color:#02000000;"># So the following is suggested
    </span><span style="color:#02000000;">&quot;no&quot; </span><span style="color:#05000000;">= if </span><span style="color:#03000000;">1 </span><span style="color:#05000000;">== </span><span style="color:#03000000;">0</span><span style="color:#00000001;">, </span><span style="color:#03000000;">do: </span><span style="color:#02000000;">&quot;yes&quot;</span><span style="color:#00000001;">, </span><span style="color:#03000000;">else: </span><span style="color:#02000000;">&quot;no&quot;
    </span></pre>
    "###
    );
}

#[test]
fn base16() {
    insta::assert_snapshot!(
        sample_html(EmbeddedThemeName::Base16),
        @r###"
    <pre style="background-color:#000000;">
    <span style="color:#08000000;"># There currently is no ternary operator like  true ? &quot;yes&quot; : &quot;no&quot;
    </span><span style="color:#08000000;"># So the following is suggested
    </span><span style="color:#07000000;">&quot;</span><span style="color:#02000000;">no</span><span style="color:#07000000;">&quot; = </span><span style="color:#05000000;">if </span><span style="color:#09000000;">1 </span><span style="color:#07000000;">== </span><span style="color:#09000000;">0</span><span style="color:#07000000;">, </span><span style="color:#09000000;">do: </span><span style="color:#07000000;">&quot;</span><span style="color:#02000000;">yes</span><span style="color:#07000000;">&quot;, </span><span style="color:#09000000;">else: </span><span style="color:#07000000;">&quot;</span><span style="color:#02000000;">no</span><span style="color:#07000000;">&quot;
    </span></pre>
    "###
    );
}

#[test]
fn base16_eighties_dark() {
    insta::assert_snapshot!(
        sample_html(EmbeddedThemeName::Base16EightiesDark),
        @r###"
    <pre style="background-color:#2d2d2d;">
    <span style="color:#747369;"># There currently is no ternary operator like  true ? &quot;yes&quot; : &quot;no&quot;
    </span><span style="color:#747369;"># So the following is suggested
    </span><span style="color:#d3d0c8;">&quot;</span><span style="color:#99cc99;">no</span><span style="color:#d3d0c8;">&quot; = </span><span style="color:#cc99cc;">if </span><span style="color:#f99157;">1 </span><span style="color:#d3d0c8;">== </span><span style="color:#f99157;">0</span><span style="color:#d3d0c8;">, </span><span style="color:#f99157;">do: </span><span style="color:#d3d0c8;">&quot;</span><span style="color:#99cc99;">yes</span><span style="color:#d3d0c8;">&quot;, </span><span style="color:#f99157;">else: </span><span style="color:#d3d0c8;">&quot;</span><span style="color:#99cc99;">no</span><span style="color:#d3d0c8;">&quot;
    </span></pre>
    "###
    );
}

#[test]
fn base16_mocha_dark() {
    insta::assert_snapshot!(
        sample_html(EmbeddedThemeName::Base16MochaDark),
        @r###"
    <pre style="background-color:#3b3228;">
    <span style="color:#7e705a;"># There currently is no ternary operator like  true ? &quot;yes&quot; : &quot;no&quot;
    </span><span style="color:#7e705a;"># So the following is suggested
    </span><span style="color:#d0c8c6;">&quot;</span><span style="color:#beb55b;">no</span><span style="color:#d0c8c6;">&quot; = </span><span style="color:#a89bb9;">if </span><span style="color:#d28b71;">1 </span><span style="color:#d0c8c6;">== </span><span style="color:#d28b71;">0</span><span style="color:#d0c8c6;">, </span><span style="color:#d28b71;">do: </span><span style="color:#d0c8c6;">&quot;</span><span style="color:#beb55b;">yes</span><span style="color:#d0c8c6;">&quot;, </span><span style="color:#d28b71;">else: </span><span style="color:#d0c8c6;">&quot;</span><span style="color:#beb55b;">no</span><span style="color:#d0c8c6;">&quot;
    </span></pre>
    "###
    );
}

#[test]
fn base16_ocean_dark() {
    insta::assert_snapshot!(
        sample_html(EmbeddedThemeName::Base16OceanDark),
        @r###"
    <pre style="background-color:#2b303b;">
    <span style="color:#65737e;"># There currently is no ternary operator like  true ? &quot;yes&quot; : &quot;no&quot;
    </span><span style="color:#65737e;"># So the following is suggested
    </span><span style="color:#c0c5ce;">&quot;</span><span style="color:#a3be8c;">no</span><span style="color:#c0c5ce;">&quot; = </span><span style="color:#b48ead;">if </span><span style="color:#d08770;">1 </span><span style="color:#c0c5ce;">== </span><span style="color:#d08770;">0</span><span style="color:#c0c5ce;">, </span><span style="color:#d08770;">do: </span><span style="color:#c0c5ce;">&quot;</span><span style="color:#a3be8c;">yes</span><span style="color:#c0c5ce;">&quot;, </span><span style="color:#d08770;">else: </span><span style="color:#c0c5ce;">&quot;</span><span style="color:#a3be8c;">no</span><span style="color:#c0c5ce;">&quot;
    </span></pre>
    "###
    );
}

#[test]
fn base16_ocean_light() {
    insta::assert_snapshot!(
        sample_html(EmbeddedThemeName::Base16OceanLight),
        @r###"
    <pre style="background-color:#eff1f5;">
    <span style="color:#a7adba;"># There currently is no ternary operator like  true ? &quot;yes&quot; : &quot;no&quot;
    </span><span style="color:#a7adba;"># So the following is suggested
    </span><span style="color:#4f5b66;">&quot;</span><span style="color:#a3be8c;">no</span><span style="color:#4f5b66;">&quot; = </span><span style="color:#b48ead;">if </span><span style="color:#d08770;">1 </span><span style="color:#4f5b66;">== </span><span style="color:#d08770;">0</span><span style="color:#4f5b66;">, </span><span style="color:#d08770;">do: </span><span style="color:#4f5b66;">&quot;</span><span style="color:#a3be8c;">yes</span><span style="color:#4f5b66;">&quot;, </span><span style="color:#d08770;">else: </span><span style="color:#4f5b66;">&quot;</span><span style="color:#a3be8c;">no</span><span style="color:#4f5b66;">&quot;
    </span></pre>
    "###
    );
}

#[test]
fn base16_256() {
    insta::assert_snapshot!(
        sample_html(EmbeddedThemeName::Base16_256),
        @r###"
    <pre style="background-color:#000000;">
    <span style="color:#08000000;"># There currently is no ternary operator like  true ? &quot;yes&quot; : &quot;no&quot;
    </span><span style="color:#08000000;"># So the following is suggested
    </span><span style="color:#07000000;">&quot;</span><span style="color:#02000000;">no</span><span style="color:#07000000;">&quot; = </span><span style="color:#05000000;">if </span><span style="color:#10000000;">1 </span><span style="color:#07000000;">== </span><span style="color:#10000000;">0</span><span style="color:#07000000;">, </span><span style="color:#10000000;">do: </span><span style="color:#07000000;">&quot;</span><span style="color:#02000000;">yes</span><span style="color:#07000000;">&quot;, </span><span style="color:#10000000;">else: </span><span style="color:#07000000;">&quot;</span><span style="color:#02000000;">no</span><span style="color:#07000000;">&quot;
    </span></pre>
    "###
    );
}

#[test]
fn coldark_cold() {
    insta::assert_snapshot!(
        sample_html(EmbeddedThemeName::ColdarkCold),
        @r###"
    <pre style="background-color:#e3eaf2;">
    <span style="color:#3c526d;"># There currently is no ternary operator like  true ? &quot;yes&quot; : &quot;no&quot;
    </span><span style="color:#3c526d;"># So the following is suggested
    </span><span style="color:#116b00;">&quot;no&quot; </span><span style="color:#a04900;">= if </span><span style="color:#755f00;">1 </span><span style="color:#a04900;">== </span><span style="color:#755f00;">0</span><span style="color:#111b27;">, </span><span style="color:#005a8e;">do</span><span style="color:#111b27;">: </span><span style="color:#116b00;">&quot;yes&quot;</span><span style="color:#111b27;">, </span><span style="color:#005a8e;">else</span><span style="color:#111b27;">: </span><span style="color:#116b00;">&quot;no&quot;
    </span></pre>
    "###
    );
}

#[test]
fn coldark_dark() {
    insta::assert_snapshot!(
        sample_html(EmbeddedThemeName::ColdarkDark),
        @r###"
    <pre style="background-color:#111b27;">
    <span style="color:#8da1b9;"># There currently is no ternary operator like  true ? &quot;yes&quot; : &quot;no&quot;
    </span><span style="color:#8da1b9;"># So the following is suggested
    </span><span style="color:#91d076;">&quot;no&quot; </span><span style="color:#e9ae7e;">= if </span><span style="color:#e6d37a;">1 </span><span style="color:#e9ae7e;">== </span><span style="color:#e6d37a;">0</span><span style="color:#e3eaf2;">, </span><span style="color:#6cb8e6;">do</span><span style="color:#e3eaf2;">: </span><span style="color:#91d076;">&quot;yes&quot;</span><span style="color:#e3eaf2;">, </span><span style="color:#6cb8e6;">else</span><span style="color:#e3eaf2;">: </span><span style="color:#91d076;">&quot;no&quot;
    </span></pre>
    "###
    );
}

#[test]
fn dark_neon() {
    insta::assert_snapshot!(
        sample_html(EmbeddedThemeName::DarkNeon),
        @r###"
    <pre style="background-color:#000000;">
    <span style="background-color:#212121;color:#7c7c7c;"># There currently is no ternary operator like  true ? &quot;yes&quot; : &quot;no&quot;
    </span><span style="background-color:#212121;color:#7c7c7c;"># So the following is suggested
    </span><span style="color:#ccff66;">&quot;no&quot; </span><span style="color:#aaaaaa;">= </span><span style="color:#66ccff;">if </span><span style="font-weight:bold;color:#ff73fd;">1 </span><span style="color:#aaaaaa;">== </span><span style="font-weight:bold;color:#ff73fd;">0</span><span style="color:#ffffff;">, </span><span style="color:#99cc99;">do: </span><span style="color:#ccff66;">&quot;yes&quot;</span><span style="color:#ffffff;">, </span><span style="color:#99cc99;">else: </span><span style="color:#ccff66;">&quot;no&quot;
    </span></pre>
    "###
    );
}

#[test]
fn dracula() {
    insta::assert_snapshot!(
        sample_html(EmbeddedThemeName::Dracula),
        @r###"
    <pre style="background-color:#282a36;">
    <span style="color:#6272a4;"># There currently is no ternary operator like  true ? &quot;yes&quot; : &quot;no&quot;
    </span><span style="color:#6272a4;"># So the following is suggested
    </span><span style="color:#f1fa8c;">&quot;no&quot; </span><span style="color:#ff79c6;">= if </span><span style="color:#bd93f9;">1 </span><span style="color:#ff79c6;">== </span><span style="color:#bd93f9;">0</span><span style="color:#f8f8f2;">, </span><span style="color:#bd93f9;">do: </span><span style="color:#f1fa8c;">&quot;yes&quot;</span><span style="color:#f8f8f2;">, </span><span style="color:#bd93f9;">else: </span><span style="color:#f1fa8c;">&quot;no&quot;
    </span></pre>
    "###
    );
}

#[test]
fn github() {
    insta::assert_snapshot!(
        sample_html(EmbeddedThemeName::Github),
        @r###"
    <pre style="background-color:#ffffff;">
    <span style="color:#969896;"># There currently is no ternary operator like  true ? &quot;yes&quot; : &quot;no&quot;
    </span><span style="color:#969896;"># So the following is suggested
    </span><span style="color:#183691;">&quot;no&quot; </span><span style="color:#a71d5d;">= if </span><span style="color:#0086b3;">1 </span><span style="color:#a71d5d;">== </span><span style="color:#0086b3;">0</span><span style="color:#333333;">, </span><span style="color:#0086b3;">do: </span><span style="color:#183691;">&quot;yes&quot;</span><span style="color:#333333;">, </span><span style="color:#0086b3;">else: </span><span style="color:#183691;">&quot;no&quot;
    </span></pre>
    "###
    );
}

#[test]
fn gruvbox_dark() {
    insta::assert_snapshot!(
        sample_html(EmbeddedThemeName::GruvboxDark),
        @r###"
    <pre style="background-color:#282828;">
    <span style="font-style:italic;color:#928374;"># There currently is no ternary operator like  true ? &quot;yes&quot; : &quot;no&quot;
    </span><span style="font-style:italic;color:#928374;"># So the following is suggested
    </span><span style="color:#fbf1c7;">&quot;</span><span style="color:#b8bb26;">no</span><span style="color:#fbf1c7;">&quot; </span><span style="color:#8ec07c;">= </span><span style="color:#fb4934;">if </span><span style="color:#d3869b;">1 </span><span style="color:#8ec07c;">== </span><span style="color:#d3869b;">0</span><span style="color:#fbf1c7;">, </span><span style="color:#d3869b;">do</span><span style="color:#fbf1c7;">: &quot;</span><span style="color:#b8bb26;">yes</span><span style="color:#fbf1c7;">&quot;, </span><span style="color:#d3869b;">else</span><span style="color:#fbf1c7;">: &quot;</span><span style="color:#b8bb26;">no</span><span style="color:#fbf1c7;">&quot;
    </span></pre>
    "###
    );
}

#[test]
fn gruvbox_light() {
    insta::assert_snapshot!(
        sample_html(EmbeddedThemeName::GruvboxLight),
        @r###"
    <pre style="background-color:#fbf1c7;">
    <span style="font-style:italic;color:#928374;"># There currently is no ternary operator like  true ? &quot;yes&quot; : &quot;no&quot;
    </span><span style="font-style:italic;color:#928374;"># So the following is suggested
    </span><span style="color:#282828;">&quot;</span><span style="color:#79740e;">no</span><span style="color:#282828;">&quot; </span><span style="color:#427b58;">= </span><span style="color:#9d0006;">if </span><span style="color:#8f3f71;">1 </span><span style="color:#427b58;">== </span><span style="color:#8f3f71;">0</span><span style="color:#282828;">, </span><span style="color:#8f3f71;">do</span><span style="color:#282828;">: &quot;</span><span style="color:#79740e;">yes</span><span style="color:#282828;">&quot;, </span><span style="color:#8f3f71;">else</span><span style="color:#282828;">: &quot;</span><span style="color:#79740e;">no</span><span style="color:#282828;">&quot;
    </span></pre>
    "###
    );
}

#[test]
fn inspired_github() {
    insta::assert_snapshot!(
        sample_html(EmbeddedThemeName::InspiredGithub),
        @r###"
    <pre style="background-color:#ffffff;">
    <span style="font-style:italic;color:#969896;"># There currently is no ternary operator like  true ? &quot;yes&quot; : &quot;no&quot;
    </span><span style="font-style:italic;color:#969896;"># So the following is suggested
    </span><span style="color:#183691;">&quot;no&quot; </span><span style="font-weight:bold;color:#a71d5d;">= if </span><span style="color:#0086b3;">1 </span><span style="font-weight:bold;color:#a71d5d;">== </span><span style="color:#0086b3;">0</span><span style="color:#323232;">, </span><span style="color:#0086b3;">do: </span><span style="color:#183691;">&quot;yes&quot;</span><span style="color:#323232;">, </span><span style="color:#0086b3;">else: </span><span style="color:#183691;">&quot;no&quot;
    </span></pre>
    "###
    );
}

#[test]
fn leet() {
    insta::assert_snapshot!(
        sample_html(EmbeddedThemeName::Leet),
        @r###"
    <pre style="background-color:#191919;">
    <span style="color:#6d6d6d;"># There currently is no ternary operator like  true ? &quot;yes&quot; : &quot;no&quot;
    </span><span style="color:#6d6d6d;"># So the following is suggested
    </span><span style="color:#ffffff;">&quot;</span><span style="color:#fbe3bf;">no</span><span style="color:#ffffff;">&quot; </span><span style="color:#ff5e5e;">= if </span><span style="color:#fdb082;">1 </span><span style="color:#ff5e5e;">== </span><span style="color:#fdb082;">0</span><span style="color:#f8f8f2;">, </span><span style="color:#fdb082;">do: </span><span style="color:#ffffff;">&quot;</span><span style="color:#fbe3bf;">yes</span><span style="color:#ffffff;">&quot;</span><span style="color:#f8f8f2;">, </span><span style="color:#fdb082;">else: </span><span style="color:#ffffff;">&quot;</span><span style="color:#fbe3bf;">no</span><span style="color:#ffffff;">&quot;
    </span></pre>
    "###
    );
}

#[test]
fn monokai_extended() {
    insta::assert_snapshot!(
        sample_html(EmbeddedThemeName::MonokaiExtended),
        @r###"
    <pre style="background-color:#222222;">
    <span style="color:#75715e;"># There currently is no ternary operator like  true ? &quot;yes&quot; : &quot;no&quot;
    </span><span style="color:#75715e;"># So the following is suggested
    </span><span style="color:#e6db74;">&quot;no&quot; </span><span style="color:#f92672;">= if </span><span style="color:#be84ff;">1 </span><span style="color:#f92672;">== </span><span style="color:#be84ff;">0</span><span style="color:#f8f8f2;">, </span><span style="color:#be84ff;">do: </span><span style="color:#e6db74;">&quot;yes&quot;</span><span style="color:#f8f8f2;">, </span><span style="color:#be84ff;">else: </span><span style="color:#e6db74;">&quot;no&quot;
    </span></pre>
    "###
    );
}

#[test]
fn monokai_extended_bright() {
    insta::assert_snapshot!(
        sample_html(EmbeddedThemeName::MonokaiExtendedBright),
        @r###"
    <pre style="background-color:#272822;">
    <span style="color:#75715e;"># There currently is no ternary operator like  true ? &quot;yes&quot; : &quot;no&quot;
    </span><span style="color:#75715e;"># So the following is suggested
    </span><span style="color:#e6db74;">&quot;no&quot; </span><span style="color:#f92672;">= if </span><span style="color:#ae81ff;">1 </span><span style="color:#f92672;">== </span><span style="color:#ae81ff;">0</span><span style="color:#f8f8f2;">, </span><span style="color:#ae81ff;">do: </span><span style="color:#e6db74;">&quot;yes&quot;</span><span style="color:#f8f8f2;">, </span><span style="color:#ae81ff;">else: </span><span style="color:#e6db74;">&quot;no&quot;
    </span></pre>
    "###
    );
}

#[test]
fn monokai_extended_light() {
    insta::assert_snapshot!(
        sample_html(EmbeddedThemeName::MonokaiExtendedLight),
        @r###"
    <pre style="background-color:#fafafa;">
    <span style="color:#75715e;"># There currently is no ternary operator like  true ? &quot;yes&quot; : &quot;no&quot;
    </span><span style="color:#75715e;"># So the following is suggested
    </span><span style="color:#998f2f;">&quot;no&quot; </span><span style="color:#f9005a;">= if </span><span style="color:#684d99;">1 </span><span style="color:#f9005a;">== </span><span style="color:#684d99;">0</span><span style="color:#49483e;">, </span><span style="color:#684d99;">do: </span><span style="color:#998f2f;">&quot;yes&quot;</span><span style="color:#49483e;">, </span><span style="color:#684d99;">else: </span><span style="color:#998f2f;">&quot;no&quot;
    </span></pre>
    "###
    );
}

#[test]
fn monokai_extended_origin() {
    insta::assert_snapshot!(
        sample_html(EmbeddedThemeName::MonokaiExtendedOrigin),
        @r###"
    <pre style="background-color:#272822;">
    <span style="color:#75715e;"># There currently is no ternary operator like  true ? &quot;yes&quot; : &quot;no&quot;
    </span><span style="color:#75715e;"># So the following is suggested
    </span><span style="color:#e6db74;">&quot;no&quot; </span><span style="color:#f92672;">= if </span><span style="color:#ae81ff;">1 </span><span style="color:#f92672;">== </span><span style="color:#ae81ff;">0</span><span style="color:#f8f8f2;">, </span><span style="color:#ae81ff;">do: </span><span style="color:#e6db74;">&quot;yes&quot;</span><span style="color:#f8f8f2;">, </span><span style="color:#ae81ff;">else: </span><span style="color:#e6db74;">&quot;no&quot;
    </span></pre>
    "###
    );
}

#[test]
fn nord() {
    insta::assert_snapshot!(
        sample_html(EmbeddedThemeName::Nord),
        @r###"
    <pre style="background-color:#2e3440;">
    <span style="color:#616e88;"># There currently is no ternary operator like  true ? &quot;yes&quot; : &quot;no&quot;
    </span><span style="color:#616e88;"># So the following is suggested
    </span><span style="color:#a3be8c;">&quot;no&quot; </span><span style="color:#81a1c1;">= if </span><span style="color:#b48ead;">1 </span><span style="color:#81a1c1;">== </span><span style="color:#b48ead;">0</span><span style="color:#eceff4;">, </span><span style="color:#d8dee9;">do: </span><span style="color:#a3be8c;">&quot;yes&quot;</span><span style="color:#eceff4;">, </span><span style="color:#d8dee9;">else: </span><span style="color:#a3be8c;">&quot;no&quot;
    </span></pre>
    "###
    );
}

#[test]
fn one_half_dark() {
    insta::assert_snapshot!(
        sample_html(EmbeddedThemeName::OneHalfDark),
        @r###"
    <pre style="background-color:#282c34;">
    <span style="color:#5c6370;"># There currently is no ternary operator like  true ? &quot;yes&quot; : &quot;no&quot;
    </span><span style="color:#5c6370;"># So the following is suggested
    </span><span style="color:#98c379;">&quot;no&quot; </span><span style="color:#c678dd;">= if </span><span style="color:#e5c07b;">1 </span><span style="color:#c678dd;">== </span><span style="color:#e5c07b;">0</span><span style="color:#dcdfe4;">, </span><span style="color:#e5c07b;">do: </span><span style="color:#98c379;">&quot;yes&quot;</span><span style="color:#dcdfe4;">, </span><span style="color:#e5c07b;">else: </span><span style="color:#98c379;">&quot;no&quot;
    </span></pre>
    "###
    );
}

#[test]
fn one_half_light() {
    insta::assert_snapshot!(
        sample_html(EmbeddedThemeName::OneHalfLight),
        @r###"
    <pre style="background-color:#fafafa;">
    <span style="color:#a0a1a7;"># There currently is no ternary operator like  true ? &quot;yes&quot; : &quot;no&quot;
    </span><span style="color:#a0a1a7;"># So the following is suggested
    </span><span style="color:#50a14f;">&quot;no&quot; </span><span style="color:#a626a4;">= if </span><span style="color:#c18401;">1 </span><span style="color:#a626a4;">== </span><span style="color:#c18401;">0</span><span style="color:#383a42;">, </span><span style="color:#c18401;">do: </span><span style="color:#50a14f;">&quot;yes&quot;</span><span style="color:#383a42;">, </span><span style="color:#c18401;">else: </span><span style="color:#50a14f;">&quot;no&quot;
    </span></pre>
    "###
    );
}

#[test]
fn solarized_dark() {
    insta::assert_snapshot!(
        sample_html(EmbeddedThemeName::SolarizedDark),
        @r###"
    <pre style="background-color:#002b36;">
    <span style="color:#586e75;"># There currently is no ternary operator like  true ? &quot;yes&quot; : &quot;no&quot;
    </span><span style="color:#586e75;"># So the following is suggested
    </span><span style="color:#839496;">&quot;</span><span style="color:#2aa198;">no</span><span style="color:#839496;">&quot; </span><span style="color:#657b83;">= </span><span style="color:#859900;">if </span><span style="color:#6c71c4;">1 </span><span style="color:#657b83;">== </span><span style="color:#6c71c4;">0</span><span style="color:#839496;">, </span><span style="color:#cb4b16;">do: </span><span style="color:#839496;">&quot;</span><span style="color:#2aa198;">yes</span><span style="color:#839496;">&quot;, </span><span style="color:#cb4b16;">else: </span><span style="color:#839496;">&quot;</span><span style="color:#2aa198;">no</span><span style="color:#839496;">&quot;
    </span></pre>
    "###
    );
}

#[test]
fn solarized_light() {
    insta::assert_snapshot!(
        sample_html(EmbeddedThemeName::SolarizedLight),
        @r###"
    <pre style="background-color:#fdf6e3;">
    <span style="color:#93a1a1;"># There currently is no ternary operator like  true ? &quot;yes&quot; : &quot;no&quot;
    </span><span style="color:#93a1a1;"># So the following is suggested
    </span><span style="color:#839496;">&quot;</span><span style="color:#2aa198;">no</span><span style="color:#839496;">&quot; </span><span style="color:#657b83;">= </span><span style="color:#859900;">if </span><span style="color:#6c71c4;">1 </span><span style="color:#657b83;">== </span><span style="color:#6c71c4;">0</span><span style="color:#657b83;">, </span><span style="color:#cb4b16;">do: </span><span style="color:#839496;">&quot;</span><span style="color:#2aa198;">yes</span><span style="color:#839496;">&quot;</span><span style="color:#657b83;">, </span><span style="color:#cb4b16;">else: </span><span style="color:#839496;">&quot;</span><span style="color:#2aa198;">no</span><span style="color:#839496;">&quot;
    </span></pre>
    "###
    );
}

#[test]
fn sublime_snazzy() {
    insta::assert_snapshot!(
        sample_html(EmbeddedThemeName::SublimeSnazzy),
        @r###"
    <pre style="background-color:#282a36;">
    <span style="color:#686868;"># There currently is no ternary operator like  true ? &quot;yes&quot; : &quot;no&quot;
    </span><span style="color:#686868;"># So the following is suggested
    </span><span style="color:#f3f99d;">&quot;no&quot; </span><span style="color:#ff5c57;">= if </span><span style="color:#f1f1f0;">1 </span><span style="color:#ff5c57;">== </span><span style="color:#f1f1f0;">0</span><span style="color:#f8f8f2;">, </span><span style="color:#5af78e;">do: </span><span style="color:#f3f99d;">&quot;yes&quot;</span><span style="color:#f8f8f2;">, </span><span style="color:#5af78e;">else: </span><span style="color:#f3f99d;">&quot;no&quot;
    </span></pre>
    "###
    );
}

#[test]
fn two_dark() {
    insta::assert_snapshot!(
        sample_html(EmbeddedThemeName::TwoDark),
        @r###"
    <pre style="background-color:#282c34;">
    <span style="font-style:italic;color:#5c6370;"># There currently is no ternary operator like  true ? &quot;yes&quot; : &quot;no&quot;
    </span><span style="font-style:italic;color:#5c6370;"># So the following is suggested
    </span><span style="color:#98c379;">&quot;no&quot; </span><span style="color:#abb2bf;">= </span><span style="color:#c678dd;">if </span><span style="color:#d19a66;">1 </span><span style="color:#abb2bf;">== </span><span style="color:#d19a66;">0</span><span style="color:#abb2bf;">, </span><span style="color:#d19a66;">do: </span><span style="color:#98c379;">&quot;yes&quot;</span><span style="color:#abb2bf;">, </span><span style="color:#d19a66;">else: </span><span style="color:#98c379;">&quot;no&quot;
    </span></pre>
    "###
    );
}

#[test]
fn visual_studio_dark_plus() {
    insta::assert_snapshot!(
        sample_html(EmbeddedThemeName::VisualStudioDarkPlus),
        @r###"
    <pre style="background-color:#1e1e1e;">
    <span style="color:#608b4e;"># There currently is no ternary operator like  true ? &quot;yes&quot; : &quot;no&quot;
    </span><span style="color:#608b4e;"># So the following is suggested
    </span><span style="color:#d69d85;">&quot;no&quot; </span><span style="color:#dcdcdc;">= </span><span style="color:#c586c0;">if </span><span style="color:#b5cea8;">1 </span><span style="color:#dcdcdc;">== </span><span style="color:#b5cea8;">0</span><span style="color:#dcdcdc;">, </span><span style="color:#b4cea8;">do: </span><span style="color:#d69d85;">&quot;yes&quot;</span><span style="color:#dcdcdc;">, </span><span style="color:#b4cea8;">else: </span><span style="color:#d69d85;">&quot;no&quot;
    </span></pre>
    "###
    );
}

#[test]
fn zenburn() {
    insta::assert_snapshot!(
        sample_html(EmbeddedThemeName::Zenburn),
        @r#"
    <pre style="background-color:#3f3f3f;">
    <span style="color:#a0cfa1;">#</span><span style="color:#87ae86;"> There currently is no ternary operator like  true ? &quot;yes&quot; : &quot;no&quot;
    </span><span style="color:#a0cfa1;">#</span><span style="color:#87ae86;"> So the following is suggested
    </span><span style="color:#d6d6d680;">&quot;</span><span style="color:#d68686;">no</span><span style="color:#d6d6d680;">&quot; </span><span style="color:#ececec;">= </span><span style="color:#fed6af;">if </span><span style="font-weight:bold;color:#87d6d5;">1 </span><span style="color:#ececec;">== </span><span style="font-weight:bold;color:#87d6d5;">0</span><span style="color:#dedede;">, </span><span style="font-weight:bold;color:#d58684;">do: </span><span style="color:#d6d6d680;">&quot;</span><span style="color:#d68686;">yes</span><span style="color:#d6d6d680;">&quot;</span><span style="color:#dedede;">, </span><span style="font-weight:bold;color:#d58684;">else: </span><span style="color:#d6d6d680;">&quot;</span><span style="color:#d68686;">no</span><span style="color:#d6d6d680;">&quot;
    </span></pre>
    "#
    );
}
