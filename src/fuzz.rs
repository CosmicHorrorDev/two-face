use syntect::{dumps, parsing::SyntaxSet};

pub fn fuzzer_syntaxes() -> SyntaxSet {
    let bytes = include_bytes!("../generated/fuzzer-syntaxes.bin");
    dumps::from_uncompressed_data(bytes).unwrap()
}
