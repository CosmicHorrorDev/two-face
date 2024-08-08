#[derive(Clone, Copy, Debug)]
pub enum GeneratedFiles {
    AckFull,
    SynOnigNewlines,
    SynFancyNewlines,
    SynOnigNoNewlines,
    SynFancyNoNewlines,
    Themes,
}

impl GeneratedFiles {
    pub fn rel_path(self) -> &'static str {
        match self {
            Self::AckFull => "generated/acknowledgements_full.bin",
            Self::SynOnigNewlines => "generated/syntaxes-onig-newlines.bin",
            Self::SynFancyNewlines => "generated/syntaxes-fancy-newlines.bin",
            Self::SynOnigNoNewlines => "generated/syntaxes-onig-no-newlines.bin",
            Self::SynFancyNoNewlines => "generated/syntaxes-fancy-no-newlines.bin",
            Self::Themes => "generated/themes.bin",
        }
    }
}
