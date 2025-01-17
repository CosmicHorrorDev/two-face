use std::{ffi::OsStr, fs, path::Path};

use self::themes::LazyThemeSet;

use syntect::{
    highlighting::ThemeSet,
    parsing::{SyntaxSet, SyntaxSetBuilder},
};
use tempfile::TempDir;
use xshell::{cmd, Shell};

use self::acknowledgements::{Acknowledgements, License, NORM_LICENSE_STEMS};

mod acknowledgements;
mod themes;
mod utils;

const NORM_RELEVANT_EXTS: &[&str] = &["patch", "sublime-syntax", "tmtheme"];

#[derive(Clone, Copy, PartialEq, Eq)]
enum RegexImpl {
    Onig,
    Fancy,
}

impl RegexImpl {
    /// Detects `syntect`'s currently active regex implementation
    ///
    /// # Panics
    ///
    /// Panics when zero or multiple implementations are active
    fn detect() -> Self {
        let onig = cfg!(feature = "syntect-onig");
        let fancy = cfg!(feature = "syntect-fancy");
        match (onig, fancy) {
            (true, false) => Self::Onig,
            (false, true) => Self::Fancy,
            (false, false) => panic!("No regex impl feature detected"),
            (true, true) => panic!("Both onig and fancy impls detected"),
        }
    }

    fn newlines_asset_name(self) -> &'static str {
        match self {
            RegexImpl::Fancy => "syntaxes-fancy-newlines.bin",
            RegexImpl::Onig => "syntaxes-onig-newlines.bin",
        }
    }

    fn no_newlines_asset_name(self) -> &'static str {
        match self {
            RegexImpl::Fancy => "syntaxes-fancy-no-newlines.bin",
            RegexImpl::Onig => "syntaxes-onig-no-newlines.bin",
        }
    }
}

struct AssetsDir {
    tempdir: TempDir,
    regex_impl: RegexImpl,
}

impl AssetsDir {
    /// Move desired assets to a tempdir and apply patches
    fn new(input: &Path) -> anyhow::Result<Self> {
        let regex_impl = RegexImpl::detect();

        log::info!("Setting up assets dir");
        let tempdir = tempfile::Builder::new()
            .prefix("two-face-assets-")
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

        let apply_patches_from = |patch_shell: &Shell, from| -> anyhow::Result<()> {
            for patch in utils::walk_files(from)? {
                let patch_contents = fs::read(&patch)?;
                let output = cmd!(patch_shell, "patch --strip=0")
                    .stdin(&patch_contents)
                    .quiet()
                    .read()?;
                log::debug!("Patch output:\n{output}");
            }

            Ok(())
        };
        let patch_shell = Shell::new()?;
        patch_shell.change_dir(temp_path);
        // Apply bat's patches
        let bat_patches = temp_path.join("patches");
        apply_patches_from(&patch_shell, &bat_patches)?;
        // And now our own
        apply_patches_from(&patch_shell, Path::new("patches"))?;

        Ok(Self {
            tempdir,
            regex_impl,
        })
    }

    fn path(&self) -> &Path {
        self.tempdir.path()
    }

    fn load_syntax_set(&self, newlines: utils::IncludeNewlines) -> anyhow::Result<SyntaxSet> {
        log::debug!("Loading syntax set");
        let syn_dir = self.path().join("syntaxes");
        let mut builder = SyntaxSetBuilder::new();
        builder.add_plain_text_syntax();
        for file in utils::walk_files(&syn_dir)? {
            if file.extension().and_then(OsStr::to_str) == Some("sublime-syntax") {
                match utils::load_syntax_file(&file, newlines) {
                    Ok(syntax) => builder.add(syntax),
                    Err(err) => log::warn!("Failed loading syntax from file. Skipping...\n{err}"),
                }
            }
        }
        let syn_set = builder.build();
        Ok(syn_set)
    }

    fn load_theme_set(&self) -> anyhow::Result<LazyThemeSet> {
        log::debug!("Loading theme set");
        let theme_dir = self.tempdir.path().join("themes");
        anyhow::ensure!(theme_dir.is_dir(), "Can't find themes dir at {theme_dir:?}",);
        let mut theme_set = ThemeSet::load_from_folder(&theme_dir)?;

        let mut full_set = ThemeSet::load_defaults();
        full_set.themes.append(&mut theme_set.themes);

        let lazy_theme_set = LazyThemeSet::from(&full_set);
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
    let output_dir = assets_dir.path().join("out");
    fs::create_dir_all(&output_dir)?;

    log::info!("Generating dumps for syntaxes with newlines");
    let syn_set_newlines = assets_dir.load_syntax_set(utils::IncludeNewlines::Yes)?;
    let syn_name = assets_dir.regex_impl.newlines_asset_name();
    syntect::dumps::dump_to_uncompressed_file(&syn_set_newlines, output_dir.join(syn_name))?;
    log::info!("Again now with no newlines");
    let syn_set_no_newlines = assets_dir.load_syntax_set(utils::IncludeNewlines::No)?;
    let syn_name = assets_dir.regex_impl.no_newlines_asset_name();
    // Syntax set has each syntax internally compressed, so no point re-compressing everything
    syntect::dumps::dump_to_uncompressed_file(&syn_set_no_newlines, output_dir.join(syn_name))?;

    let generated_dir = Path::new("generated");
    fs::create_dir_all(generated_dir)?;

    // Onig contains all syntaxes that are present with fancy, and more, so generate any one-off
    // embedded assets when it's the active implementation
    if assets_dir.regex_impl == RegexImpl::Onig {
        log::info!("Generating dumps for all other assets");
        let theme_set = assets_dir.load_theme_set()?;
        let acks = assets_dir.load_acknowledgements()?;
        let theme_name = "themes.bin";
        let ack_name = "acknowledgements_full.md";
        let ack_full_bin_name = "acknowledgements_full.bin";
        syntect::dumps::dump_to_uncompressed_file(&theme_set, output_dir.join(theme_name))?;
        fs::write(output_dir.join(ack_name), acks.to_md())?;

        // The static markdown file will have _all_ the acknowledgements while the embedded data
        // will only keep ones that require acknowledgement
        let filter_needs_ack = |licenses: Vec<License>| {
            licenses
                .into_iter()
                .filter(|license| license.needs_acknowledgement())
                .collect()
        };
        let needs_ack = Acknowledgements {
            for_themes: filter_needs_ack(acks.for_themes),
            for_syntaxes: filter_needs_ack(acks.for_syntaxes),
        };
        syntect::dumps::dump_to_file(&needs_ack, output_dir.join(ack_full_bin_name))?;

        let assets_dir = Path::new("assets");
        log::info!(
            "Copying from `{}` to `{}`",
            assets_dir.display(),
            generated_dir.display()
        );
        for file in utils::walk_files(assets_dir)? {
            fs::copy(&file, generated_dir.join(file.file_name().unwrap()))?;
        }
    }

    log::info!(
        "Copying from `{}` to `{}`",
        output_dir.display(),
        generated_dir.display()
    );
    for file in utils::walk_files(&output_dir)? {
        fs::copy(&file, generated_dir.join(file.file_name().unwrap()))?;
    }

    Ok(())
}
