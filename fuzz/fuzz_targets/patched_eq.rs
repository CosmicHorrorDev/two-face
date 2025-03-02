#![no_main]

use std::sync::OnceLock;

use libfuzzer_sys::fuzz_target;
use two_face::re_exports::syntect::parsing::{ParseState, SyntaxReference, SyntaxSet};
use two_face::{fuzz::fuzzer_syntaxes, syntax::extra_newlines};

// TODO: make fuzzing generic over the syntax, so that we don't have one target per syntax

fuzz_target!(|input: &str| {
    let Some((syn @ "md", md_text)) = input.split_once('\n') else {
        return;
    };
    let fuzzer_syn_set = fuzzer_syn_set();
    let mut fuzzer_syn_state = ParseState::new(fuzzer_syn_ref(syn));
    let patched_syn_set = patched_syn_set();
    let mut patched_syn_state = ParseState::new(patched_syn_ref(syn));
    for line in md_text.lines() {
        let line = format!("{line}\n");
        assert!(
            line_eq(
                &line,
                &mut fuzzer_syn_state,
                fuzzer_syn_set,
                &mut patched_syn_state,
                patched_syn_set,
            ),
            "Not equal for {line:?}"
        );
    }
});

// --- The syntax set stuff is very expensive to load, so cache it ---

fn fuzzer_syn_set() -> &'static SyntaxSet {
    static SET: OnceLock<SyntaxSet> = OnceLock::new();
    SET.get_or_init(fuzzer_syntaxes)
}

fn fuzzer_syn_ref(syn_ext: &str) -> &'static SyntaxReference {
    fuzzer_syn_set().find_syntax_by_extension(syn_ext).unwrap()
}

fn patched_syn_set() -> &'static SyntaxSet {
    static SET: OnceLock<SyntaxSet> = OnceLock::new();
    SET.get_or_init(extra_newlines)
}

fn patched_syn_ref(syn_ext: &str) -> &'static SyntaxReference {
    patched_syn_set().find_syntax_by_extension(syn_ext).unwrap()
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
