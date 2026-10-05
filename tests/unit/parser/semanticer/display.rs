use cmake_analyzer::parser::semanticer::Semanticer;
use std::fmt;

#[test]
fn display_shows_commands_blocks_scopes_symbols_references_and_diagnostics() {
    let buffer =
        Semanticer::analyze("function(F p)\nmessage(\"${p}\")\nendfunction()\nF(value)\nelse()");
    let plain = buffer.to_string();
    for label in [
        "Command: function",
        "Blocks (1)",
        "Scopes (2)",
        "Symbols (2)",
        "References (",
        "Command/Function",
        "candidates=[",
        "UnexpectedBranch",
        "TextSpan",
    ] {
        assert!(plain.contains(label), "missing {label}: {plain}");
    }
    assert!(!plain.contains('\x1b'));
    assert_eq!(plain, buffer.display(false).to_string());
    assert!(buffer.display(true).to_string().contains("\x1b["));
}

#[test]
fn source_control_characters_in_symbol_names_are_escaped() {
    let buffer = Semanticer::analyze("set([[line\n\x1b[31m]] value)");
    let plain = buffer.to_string();
    assert!(!plain.contains('\x1b'));
    assert!(plain.contains("line\\n"));
}

#[test]
fn formatting_failures_propagate_for_plain_and_colored_output() {
    struct Reject;
    impl fmt::Write for Reject {
        fn write_str(&mut self, _: &str) -> fmt::Result {
            Err(fmt::Error)
        }
    }
    let buffer = Semanticer::analyze("set(x value)");
    for colored in [false, true] {
        assert!(fmt::write(&mut Reject, format_args!("{}", buffer.display(colored))).is_err());
    }
}
