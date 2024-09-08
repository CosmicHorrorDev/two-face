use std::{
    env, fs,
    path::{Path, PathBuf},
};

use clap::{Parser, Subcommand};
use xshell::{cmd, Shell};

mod gen;
mod test_meta;
mod utils;

#[derive(Parser, Debug)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Gen {
        /// Update submodules while running (this is required)
        #[arg(short, long)]
        yes_update_submodules: bool,
        /// Should only be set by `cargo xtask gen` running. Never set manually
        ///
        /// Only generate the syntax dumps for fancy-regex
        #[arg(long)]
        only_fancy_syntaxes: bool,
        /// Should only be set by `cargo xtask gen` running. Never set manually
        ///
        /// Weird hack because `cargo xtask gen` calls back into itself to run. This is done, so
        /// that we can have a single "run" that sets different features for `syntect`
        #[arg(long)]
        calling_self: bool,
    },
    /// Update the `syntect-meta.toml` file that's used for tests
    TestMeta,
}

// TODO: add context everywhere
fn main() -> anyhow::Result<()> {
    let env = env_logger::Env::default().filter_or("LOG", "xtask=info");
    env_logger::init_from_env(env);

    let cli = Cli::parse();
    log::info!("CLI args: {cli:?}");

    let project_root = project_root();
    log::debug!("Detected project root at: {}", project_root.display());
    env::set_current_dir(project_root).unwrap();

    match cli.command {
        Commands::Gen {
            yes_update_submodules,
            only_fancy_syntaxes,
            calling_self,
        } => {
            if calling_self {
                gen::gen(only_fancy_syntaxes)?;
            } else {
                anyhow::ensure!(
                    yes_update_submodules,
                    "You must pass `--yes-update-submodules` to generate assets",
                );
                log::info!("Attempting to init/update submodules");
                let shell = Shell::new()?;
                cmd!(shell, "git submodule update --init --recursive").run()?;

                // We only want to keep newly generated artifacts
                let _ = fs::remove_dir_all("generated");

                // Call back into this xtask to generate new assets with multiple syntect features
                let xtask_args = ["run", "--quiet", "--release", "--package", "xtask"];
                let fancy_feature = ["--no-default-features", "--features", "syntect-fancy"];
                let args = ["gen", "--calling-self"];
                cmd!(shell, "cargo {xtask_args...} -- {args...}").run()?;
                cmd!(
                    shell,
                    "cargo {xtask_args...} {fancy_feature...} -- {args...} --only-fancy-syntaxes"
                )
                .run()?;
            }
        }
        Commands::TestMeta => test_meta::update_test_metadata(),
    }

    Ok(())
}

// From: https://github.com/rust-lang/rust-analyzer/tree/master/xtask
fn project_root() -> PathBuf {
    Path::new(
        &env::var("CARGO_MANIFEST_DIR").unwrap_or_else(|_| env!("CARGO_MANIFEST_DIR").to_owned()),
    )
    .ancestors()
    .nth(1)
    .unwrap()
    .to_path_buf()
}
