use std::{
    fs,
    path::{Path, PathBuf},
};

use anyhow::Context;
use syntect::parsing::SyntaxDefinition;
use walkdir::WalkDir;

#[derive(Clone, Copy, PartialEq)]
pub enum IncludeNewlines {
    Yes,
    No,
}

// Helper function copied from syntect internals
pub fn load_syntax_file(p: &Path, newlines: IncludeNewlines) -> anyhow::Result<SyntaxDefinition> {
    let s = fs::read_to_string(p)?;

    let include_newlines = newlines == IncludeNewlines::Yes;
    SyntaxDefinition::load_from_str(&s, include_newlines, p.file_stem().and_then(|x| x.to_str()))
        .with_context(|| format!("Failed loading syntax from file: {}", p.display()))
}

pub fn walk_files(base_dir: &Path) -> anyhow::Result<Vec<PathBuf>> {
    let mut files = Vec::new();

    for entry in WalkDir::new(base_dir).follow_links(true) {
        let entry = entry?;

        if entry.file_type().is_file() {
            files.push(entry.into_path());
        }
    }

    // Sorted to keep ordering in generated assets stable
    // TODO: need to normalize to handle filesystems with different case sensitivity
    files.sort();

    Ok(files)
}
