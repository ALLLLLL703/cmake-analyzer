// AI-generated tests.

use std::{
    io::Write,
    process::{Command, Stdio},
};

#[path = "../../cli/parser/nparser/output.rs"]
mod inspection;

#[test]
fn formatter_preserves_the_underlying_io_error() {
    struct Reject;
    impl Write for Reject {
        fn write(&mut self, _: &[u8]) -> std::io::Result<usize> {
            Err(std::io::Error::from(std::io::ErrorKind::PermissionDenied))
        }
        fn flush(&mut self) -> std::io::Result<()> {
            Ok(())
        }
    }
    let error = inspection::inspect("foo(a)", false, Reject).unwrap_err();
    assert_eq!(error.kind(), std::io::ErrorKind::PermissionDenied);
}

#[test]
fn closing_stdout_consumer_does_not_turn_large_inspection_into_an_error() {
    let mut child = Command::new(env!("CARGO_BIN_EXE_nparser"))
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    // Close the consumer before output begins; enough output forces buffered writes.
    drop(child.stdout.take().unwrap());
    let mut stdin = child.stdin.take().unwrap();
    stdin
        .write_all("foo(a)\n".repeat(20_000).as_bytes())
        .unwrap();
    drop(stdin);
    let output = child.wait_with_output().unwrap();
    assert!(
        output.status.success(),
        "{:?}: {}",
        output.status,
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(
        output.stderr.is_empty(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
}
