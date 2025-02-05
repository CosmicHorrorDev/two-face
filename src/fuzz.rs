use std::fs;

use syntect::{dumps, parsing::SyntaxSet};

pub fn fuzzer_syntaxes() -> SyntaxSet {
    let bytes = fs::read("generated/fuzzer-syntaxes.bin").unwrap();
    dumps::from_uncompressed_data(&bytes).unwrap()
}
