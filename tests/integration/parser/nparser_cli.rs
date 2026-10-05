use std::{
    io::Write,
    path::Path,
    process::{Command, Output, Stdio},
};

use cmake_analyzer::parser::{
    lexer::model::{Lexer, RawBuffer},
    nparser::model::NParser,
};

fn command() -> Command {
    Command::new(env!("CARGO_BIN_EXE_nparser"))
}

fn stdout(output: &Output) -> &str {
    std::str::from_utf8(&output.stdout).unwrap()
}

fn piped(args: &[&str], source: &[u8]) -> Output {
    let mut child = command()
        .args(args)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    child.stdin.take().unwrap().write_all(source).unwrap();
    child.wait_with_output().unwrap()
}

#[test]
fn text_file_and_stdin_inputs_use_the_nparser_display() {
    let source = "message(hello \"world\" [=[raw]=])";
    let expected = NParser::new(Lexer::new(RawBuffer::new(source)).parse())
        .parse()
        .to_string();
    let output = command().args(["--text", source]).output().unwrap();
    assert!(output.status.success());
    assert_eq!(stdout(&output), expected);
    assert!(!stdout(&output).contains('\x1b'));
    for args in [&[][..], &["-"][..]] {
        let output = piped(args, source.as_bytes());
        assert!(output.status.success());
        assert_eq!(stdout(&output), expected);
    }
    let fixture = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/nparser/basic.cmake");
    let output = command().arg(fixture).output().unwrap();
    assert!(output.status.success());
    assert!(stdout(&output).contains("Command: if"));
    assert!(stdout(&output).contains("Command: message"));
    assert!(stdout(&output).contains("Command: endif"));
}

#[test]
fn color_is_opt_in() {
    let output = command()
        .args(["--text", "message(x)", "--color"])
        .output()
        .unwrap();
    assert!(output.status.success());
    assert!(stdout(&output).contains("\x1b["));
}

#[test]
fn syntax_and_lexical_errors_exit_one_with_partial_results() {
    for (source, diagnostic) in [
        ("good()\nmissing", "ReachTheEof"),
        ("good()\n9bad()", "InvalidCommandName"),
        ("good()\nmessage(x", "UnclosedArguments"),
        ("good()\n[=[unfinished", "LexError(UnclosedBracketArgument)"),
    ] {
        let output = command().args(["--text", source]).output().unwrap();
        assert_eq!(output.status.code(), Some(1));
        assert!(stdout(&output).contains("Command: good"));
        assert!(stdout(&output).contains(diagnostic));
        assert!(output.stderr.is_empty());
    }
}

#[test]
fn io_and_argument_errors_exit_two() {
    let nonexistent =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/nparser/nonexistent.cmake");
    let output = command().arg(&nonexistent).output().unwrap();
    assert_eq!(output.status.code(), Some(2));
    assert!(output.stdout.is_empty());
    assert!(!output.stderr.is_empty());
    let output = command()
        .args(["--text", "message(x)"])
        .arg(nonexistent)
        .output()
        .unwrap();
    assert_eq!(output.status.code(), Some(2));
    let output = piped(&[], &[0xff]);
    assert_eq!(output.status.code(), Some(2));
}

#[test]
fn help_version_and_empty_input_work() {
    let output = command().arg("--help").output().unwrap();
    assert!(output.status.success());
    assert!(stdout(&output).contains("Usage: nparser"));
    assert!(stdout(&output).contains("--text"));
    let output = command().arg("--version").output().unwrap();
    assert!(output.status.success());
    assert!(stdout(&output).contains(env!("CARGO_PKG_VERSION")));
    let output = command().args(["--text", ""]).output().unwrap();
    assert!(output.status.success());
    assert!(output.stdout.is_empty());
}

#[cfg(unix)]
#[test]
fn broken_pipe_does_not_report_failure() {
    let mut child = command()
        .args(["--text", "message(x)"])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    drop(child.stdout.take());
    let output = child.wait_with_output().unwrap();
    assert!(output.status.success());
    assert!(output.stderr.is_empty());
}
