// AI-generated tests.

//! Check inspection writer failures and the nparser executable's broken-pipe handling.
//! One test spawns the built inspection binary with fixed source on stdin; no project is executed.

use std::{
    io::Write,
    process::{Command, Stdio},
};

#[path = "../../cli/parser/nparser/output.rs"]
mod inspection;

/// Inspect foo(a) through a local writer that rejects every write with PermissionDenied.
/// No external I/O; the rejecting sink never stores bytes.
///
/// # Arguments
/// None; the test constructs its own fixtures.
///
/// # Returns
/// `()` after all checks pass.
///
/// # Panics
/// If fixture setup or any documented assertion fails.
///
/// # Algorithm
/// Require the returned inspection error to preserve PermissionDenied; flush itself succeeds.
///
/// # Examples
/// Local test-module context; run through the test harness (ignored measurements require explicit selection).
/// ```text
/// formatter_preserves_the_underlying_io_error();
/// ```
#[test]
fn formatter_preserves_the_underlying_io_error() {
    /// Sink that injects PermissionDenied into [`inspection::inspect`] output writes.
    /// Used only by the formatter error-preservation test; stores no data and performs no I/O.
    /// Its [`Write`] implementation rejects writes but accepts flush.
    ///
    /// # Examples
    /// Local test context:
    /// ```text
    /// let error = inspection::inspect("foo(a)", false, Reject).unwrap_err();
    /// ```
    struct Reject;
    impl Write for Reject {
        /// Reject all bytes with PermissionDenied; no bytes are written and no external side effects occur.
        ///
        /// # Arguments
        /// - `self` `[in]`: zero-sized sink, unchanged.
        /// - `_` `[in]`: bytes deliberately ignored, including an empty slice.
        ///
        /// # Returns
        /// `Err(io::Error)` always; never returns a written-byte count.
        ///
        /// # Errors
        /// Always PermissionDenied, injected rather than obtained from the filesystem.
        ///
        /// # Examples
        /// Local test context:
        /// ```text
        /// let error = Reject.write(b"output").unwrap_err();
        /// ```
        fn write(&mut self, _: &[u8]) -> std::io::Result<usize> {
            Err(std::io::Error::from(std::io::ErrorKind::PermissionDenied))
        }
        /// Accept flushing without storing or writing anything; no side effects.
        ///
        /// # Arguments
        /// - `self` `[in]`: zero-sized sink, unchanged.
        ///
        /// # Returns
        /// `Ok(())` unconditionally; no error is produced.
        ///
        /// # Examples
        /// Local test context:
        /// ```text
        /// Reject.flush().unwrap();
        /// ```
        fn flush(&mut self) -> std::io::Result<()> {
            Ok(())
        }
    }
    let error = inspection::inspect("foo(a)", false, Reject).unwrap_err();
    assert_eq!(error.kind(), std::io::ErrorKind::PermissionDenied);
}

/// Spawn the built nparser binary, close its stdout consumer, and feed 20,000 fixed foo(a) lines.
/// Spawns/reaps only the built inspection binary and writes pipes; does not run CMake or project commands.
///
/// # Arguments
/// None; the test constructs its own fixtures.
///
/// # Returns
/// `()` after all checks pass.
///
/// # Panics
/// If fixture setup or any documented assertion fails.
///
/// # Algorithm
/// Close stdin and wait for the child; require successful exit and empty stderr.
///
/// # Flow
/// ```text
/// [spawn built nparser] --> [close stdout reader]
///           |
///           v
/// [stdin: 20000 foo(a) lines]
///           |
///           v
/// [close stdin / wait child] --> [require success]
///           |
///           v
/// [require empty stderr]
/// ```
///
/// # Examples
/// Local test-module context; run through the test harness (ignored measurements require explicit selection).
/// ```text
/// closing_stdout_consumer_does_not_turn_large_inspection_into_an_error();
/// ```
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
