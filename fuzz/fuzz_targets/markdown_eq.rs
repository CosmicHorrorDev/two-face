#![no_main]

use std::sync::OnceLock;

use libfuzzer_sys::fuzz_target;
use two_face::re_exports::syntect::parsing::{ParseState, SyntaxReference, SyntaxSet};
use two_face::{fuzz::fuzzer_syntaxes, syntax::extra_newlines};

// --- The syntax set stuff is very expensive to load, so cache it ---

fn fuzzer_set() -> &'static SyntaxSet {
    static SET: OnceLock<SyntaxSet> = OnceLock::new();
    SET.get_or_init(fuzzer_syntaxes)
}

fn fuzzer_markdown() -> &'static SyntaxReference {
    fuzzer_set().find_syntax_by_extension("md").unwrap()
}

fn patched_set() -> &'static SyntaxSet {
    static SET: OnceLock<SyntaxSet> = OnceLock::new();
    SET.get_or_init(extra_newlines)
}

fn patched_markdown() -> &'static SyntaxReference {
    patched_set().find_syntax_by_extension("md").unwrap()
}

// --- ^^ ---

fn line_eq(
    line: &str,
    state1: &mut ParseState,
    set1: &SyntaxSet,
    state2: &mut ParseState,
    set2: &SyntaxSet,
) -> bool {
    match (state1.parse_line(line, set1), state2.parse_line(line, set2)) {
        (Ok(v1), Ok(v2)) => v1 == v2,
        // TODO: parse error doesn't impl `PartialEq`, upstream change?
        (Err(_), Err(_)) => true,
        _ => false,
    }
}

fuzz_target!(|md_text: &str| {
    let fuzzer_set = fuzzer_set();
    let mut fuzzer_md_state = ParseState::new(fuzzer_markdown());
    let patched_set = patched_set();
    let mut patched_md_state = ParseState::new(patched_markdown());
    for line in md_text.lines() {
        let line = format!("{line}\n");
        assert!(
            line_eq(
                &line,
                &mut fuzzer_md_state,
                fuzzer_set,
                &mut patched_md_state,
                patched_set
            ),
            "Not equal for {line:?}"
        );
    }
});
