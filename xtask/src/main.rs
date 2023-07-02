use std::{
    env,
    path::{Path, PathBuf},
};

use clap::{Parser, Subcommand};
use xshell::{cmd, Shell};

mod gen;

#[derive(Parser, Debug)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Gen {
        #[arg(short, long)]
        yes_update_submodules: bool,
    },
}

// TODO: add context everywhere
fn main() -> anyhow::Result<()> {
    let env = env_logger::Env::default().filter_or("LOG", "xtask=debug");
    env_logger::init_from_env(env);

    let cli = Cli::parse();
    log::info!("CLI args: {cli:?}");

    let project_root = project_root();
    log::debug!("Detected project root at: {}", project_root.display());
    env::set_current_dir(project_root).unwrap();

    match cli.command {
        Commands::Gen {
            yes_update_submodules,
        } => {
            anyhow::ensure!(
                yes_update_submodules,
                "You must pass `--yes-update-submodules` to generate assets",
            );
            log::info!("Attempting to init/update submodules");
            let shell = Shell::new()?;
            cmd!(shell, "git submodule update --init --recursive").run()?;

            gen::gen()?;
        }
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
