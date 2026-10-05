use cmake_analyzer::parser::semanticer::Semanticer;
use std::{
    io::Write,
    path::Path,
    process::{Command, Output, Stdio},
};

fn command() -> Command {
    Command::new(env!("CARGO_BIN_EXE_semanticer"))
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
fn text_file_and_stdin_inputs_use_semantic_display() {
    let source = "set(x value)\nmessage(\"${x}\")";
    let expected = Semanticer::analyze(source).to_string();
    let output = command().args(["--text", source]).output().unwrap();
    assert!(output.status.success());
    assert_eq!(stdout(&output), expected);
    for args in [&[][..], &["-"][..]] {
        let output = piped(args, source.as_bytes());
        assert!(output.status.success());
        assert_eq!(stdout(&output), expected);
    }
    let fixture =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/semanticer/basic.cmake");
    let output = command().arg(fixture).output().unwrap();
    assert!(output.status.success(), "{}", stdout(&output));
    assert!(stdout(&output).contains("Blocks (2)"));
    assert!(stdout(&output).contains("Command/Function"));
    assert!(stdout(&output).contains("Target/Target"));
}

#[test]
fn errors_exit_one_but_warnings_and_unresolved_names_do_not() {
    for (source, diagnostic) in [
        ("if(ON)", "UnclosedBlock"),
        ("else()", "UnexpectedBranch"),
        ("function(F a b) endfunction() F(a)", "InvalidArity"),
        (
            "set(x value) [=[unfinished",
            "LexError(UnclosedBracketArgument)",
        ),
    ] {
        let output = command().args(["--text", source]).output().unwrap();
        assert_eq!(output.status.code(), Some(1));
        assert!(stdout(&output).contains(diagnostic));
        assert!(output.stderr.is_empty());
    }
    let output = command()
        .args([
            "--text",
            "add_library(x) add_library(x) message(\"${unknown}\") unknown_command()",
        ])
        .output()
        .unwrap();
    assert!(output.status.success());
    assert!(stdout(&output).contains("Warning: DuplicateTarget"));
}

#[test]
fn color_is_explicit_and_help_and_version_work() {
    let output = command().args(["--text", "set(x value)"]).output().unwrap();
    assert!(!stdout(&output).contains('\x1b'));
    let output = command()
        .args(["--text", "set(x value)", "--color"])
        .output()
        .unwrap();
    assert!(output.status.success());
    assert!(stdout(&output).contains("\x1b["));
    for arg in ["--help", "--version"] {
        assert!(command().arg(arg).output().unwrap().status.success());
    }
}

#[test]
fn input_and_argument_errors_exit_two_and_empty_input_succeeds() {
    let missing =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/semanticer/nonexistent.cmake");
    let output = command().arg(&missing).output().unwrap();
    assert_eq!(output.status.code(), Some(2));
    assert!(output.stdout.is_empty());
    assert!(!output.stderr.is_empty());
    let output = command()
        .args(["--text", "set(x value)"])
        .arg(missing)
        .output()
        .unwrap();
    assert_eq!(output.status.code(), Some(2));
    assert_eq!(piped(&[], &[0xff]).status.code(), Some(2));
    let output = command().args(["--text", ""]).output().unwrap();
    assert!(output.status.success());
    assert!(stdout(&output).contains("Scopes (1)"));
}

#[cfg(unix)]
#[test]
fn broken_output_pipe_is_successful() {
    let mut child = command()
        .args(["--text", "set(x value)"])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    drop(child.stdout.take());
    let output = child.wait_with_output().unwrap();
    assert!(output.status.success());
    assert!(output.stderr.is_empty());
}
