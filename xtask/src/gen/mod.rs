use std::{ffi::OsStr, fs, path::Path};

use self::themes::LazyThemeSet;

use syntect::{
    highlighting::ThemeSet,
    parsing::{SyntaxSet, SyntaxSetBuilder},
};
use tempfile::TempDir;
use xshell::{cmd, Shell};

use self::acknowledgements::{Acknowledgements, NORM_LICENSE_STEMS};

mod acknowledgements;
mod themes;
mod utils;

const NORM_RELEVANT_EXTS: &[&str] = &["patch", "sublime-syntax", "tmtheme"];

struct AssetsDir {
    tempdir: TempDir,
}

impl AssetsDir {
    /// Move desired assets to a tempdir and apply patches
    fn new(input: &Path) -> anyhow::Result<Self> {
        log::info!("Setting up assets dir");
        let tempdir = tempfile::Builder::new()
            .prefix("syntect-assets-")
            .tempdir()?;
        let temp_path = tempdir.path();

        // Copy over relevant files
        for file in utils::walk_files(input)? {
            let is_ext_we_want = file
                .extension()
                .and_then(OsStr::to_str)
                .map(|ext| NORM_RELEVANT_EXTS.contains(&&*ext.to_ascii_lowercase()))
                .unwrap_or(false);
            let is_license = file
                .file_stem()
                .and_then(OsStr::to_str)
                .map(|stem| NORM_LICENSE_STEMS.contains(&&*stem.to_ascii_lowercase()))
                .unwrap_or(false);
            if is_ext_we_want || is_license {
                let out_file = temp_path.join(file.strip_prefix(input)?);
                let mut out_dir = out_file.clone();
                out_dir.pop();

                fs::create_dir_all(&out_dir)?;
                fs::copy(file, &out_file)?;
            }
        }

        // Apply all patches
        let patch_shell = Shell::new()?;
        patch_shell.change_dir(temp_path);
        for patch in utils::walk_files(&temp_path.join("patches"))? {
            let patch_contents = fs::read(&patch)?;
            let output = cmd!(patch_shell, "patch --strip=0")
                .stdin(&patch_contents)
                .quiet()
                .read()?;
            log::debug!("Patch output:\n{output}");
        }

        Ok(Self { tempdir })
    }

    fn path(&self) -> &Path {
        self.tempdir.path()
    }

    fn load_syntax_set(&self) -> anyhow::Result<SyntaxSet> {
        log::debug!("Loading syntax set");
        let syn_dir = self.tempdir.path().join("syntaxes");
        let mut builder = SyntaxSetBuilder::new();
        builder.add_plain_text_syntax();
        for file in utils::walk_files(&syn_dir)? {
            if file.extension().and_then(OsStr::to_str) == Some("sublime-syntax") {
                let syntax = utils::load_syntax_file(&file)?;
                builder.add(syntax);
            }
        }
        let syn_set = builder.build();
        Ok(syn_set)
    }

    fn load_theme_set(&self) -> anyhow::Result<LazyThemeSet> {
        log::debug!("Loading theme set");
        let theme_dir = self.tempdir.path().join("themes");
        anyhow::ensure!(theme_dir.is_dir(), "Can't find themes dir at {theme_dir:?}",);
        let theme_set = ThemeSet::load_from_folder(&theme_dir)?;
        let lazy_theme_set = LazyThemeSet::from(&theme_set);
        Ok(lazy_theme_set)
    }

    fn load_acknowledgements(&self) -> anyhow::Result<Acknowledgements> {
        log::debug!("Loading acknowledgements");
        let acknowledgements = Acknowledgements::load_from_assets(self.tempdir.path())?;
        Ok(acknowledgements)
    }
}

pub fn gen() -> anyhow::Result<()> {
    let assets_dir = AssetsDir::new(Path::new("bat/assets"))?;

    log::info!("Loading assets");
    let syn_set = assets_dir.load_syntax_set()?;
    let theme_set = assets_dir.load_theme_set()?;
    let acks = assets_dir.load_acknowledgements()?;

    log::info!("Dumping into different formats");
    let output_dir = assets_dir.path().join("out");
    let syn_name = "syntaxes.bin";
    let theme_name = "themes.bin";
    let ack_name = "acknowledgements_full.md";
    let ack_syn_name = "acknowledgements_syntaxes.bin";
    let ack_theme_name = "acknowledgements_theme.bin";
    let ack_full_bin_name = "acknowledgements_full.bin";
    fs::create_dir_all(&output_dir)?;
    syntect::dumps::dump_to_uncompressed_file(&syn_set, output_dir.join(syn_name))?;
    syntect::dumps::dump_to_file(&theme_set, output_dir.join(theme_name))?;
    fs::write(output_dir.join(ack_name), acks.to_md())?;
    // The static markdown file will have _all_ the acknowledgements while the embedded data will
    // only keep ones that require acknowledgement
    // TODO: Maybe allow for providing all with a feature flag?
    let needs_ack = Acknowledgements {
        for_themes: acks
            .for_themes
            .into_iter()
            .filter(|ack| ack.needs_acknowledgement())
            .collect(),
        for_syntaxes: acks
            .for_syntaxes
            .into_iter()
            .filter(|ack| ack.needs_acknowledgement())
            .collect(),
    };
    syntect::dumps::dump_to_file(&needs_ack.for_syntaxes, output_dir.join(ack_syn_name))?;
    syntect::dumps::dump_to_file(&needs_ack.for_themes, output_dir.join(ack_theme_name))?;
    syntect::dumps::dump_to_file(&needs_ack, output_dir.join(ack_full_bin_name))?;

    log::info!("Copying output data");
    let generated_dir = Path::new("generated");
    fs::create_dir_all(&generated_dir)?;
    for file in utils::walk_files(&output_dir)? {
        fs::copy(&file, generated_dir.join(file.file_name().unwrap()))?;
    }

    Ok(())
}
