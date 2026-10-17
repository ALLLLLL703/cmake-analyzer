//! Borrowed payload positions, malformed token rejection and formatter error propagation. A local
//! rejecting writer injects output failures without I/O.

// Contains AI-generated documentation edits.
use cmake_analyzer::{
    model::{Spanned, TextSpan},
    parser::lexer::model::{LexContent, Lexer, RawBuffer},
};
// AI-generated tests.

use std::fmt;

/// Scan quoted Chinese and CRLF-prefixed bracket crab payloads. Check narrowed columns/rows, scalar
/// iteration and pointer equality to prove payload text borrows the source.
///
/// Only local values and analysis buffers are created or changed; no file, network or process side
/// effects.
///
/// # Arguments
/// None. The test builds its own fixture and expected results.
///
/// # Returns
/// `()` after all checks pass; no data or output parameter is returned.
///
/// # Panics
/// If a stated invariant fails or an unwrapped fixture/setup operation fails.
///
/// # Examples
/// Local test-module call:
/// ```text
/// payload_spans_locate_unwrapped_unicode_and_initial_newlines();
/// ```
#[test]
fn payload_spans_locate_unwrapped_unicode_and_initial_newlines() {
    let source = "前 \"中\" [=[\r\n🦀]=]";
    let buffer = Lexer::new(RawBuffer::new(source)).parse();
    let quote = &buffer.lex[1];
    assert_eq!(quote.text(source), Some("中"));
    assert_eq!(quote.payload_span(source).unwrap().column, 3);
    let bracket = &buffer.lex[2];
    let payload = bracket.payload_span(source).unwrap();
    assert_eq!((payload.row, payload.column), (1, 0));
    assert_eq!(payload.text(source), Some("🦀"));
    assert_eq!(
        payload.iter_text(source).unwrap().collect::<Vec<_>>(),
        ['🦀']
    );
    assert_eq!(
        bracket.text(source).unwrap().as_ptr(),
        source[payload.start_byte..].as_ptr()
    );
}

/// Manually construct string/bracket tokens over missing or mismatched delimiters. Payload
/// extraction must return absence instead of guessing or panicking.
///
/// Only local values and analysis buffers are created or changed; no file, network or process side
/// effects.
///
/// # Arguments
/// None. The test builds its own fixture and expected results.
///
/// # Returns
/// `()` after all checks pass; no data or output parameter is returned.
///
/// # Panics
/// If a stated invariant fails.
///
/// # Examples
/// Local test-module call:
/// ```text
/// invalid_payload_tokens_do_not_panic_or_return_guessed_text();
/// ```
#[test]
fn invalid_payload_tokens_do_not_panic_or_return_guessed_text() {
    for (source, kind) in [
        ("abc", LexContent::StringLiteral),
        ("\"", LexContent::StringLiteral),
        ("[[", LexContent::BracketArgument),
        ("[=[x]]", LexContent::BracketArgument),
    ] {
        let token = Spanned {
            content: kind,
            span: TextSpan {
                end_byte: source.len(),
                ..TextSpan::default()
            },
        };
        assert!(token.text(source).is_none());
    }
}

/// Format plain and colored lexed output into a writer that always rejects text. Both formatting
/// calls must return errors instead of swallowing the writer failure.
///
/// Only local values and analysis buffers are created or changed; no file, network or process side
/// effects.
///
/// # Arguments
/// None. The test builds its own fixture and expected results.
///
/// # Returns
/// `()` after all checks pass; no data or output parameter is returned.
///
/// # Panics
/// If a stated invariant fails.
///
/// # Examples
/// Local test-module call:
/// ```text
/// display_propagates_formatter_errors();
/// ```
#[test]
fn display_propagates_formatter_errors() {
    /// Stateless failing output sink for the enclosing lexer display test.
    /// Its [`fmt::Write`] implementation rejects all bytes, proving display forwards errors.
    /// Construction and writes have no external side effects.
    ///
    /// # Examples
    /// Local context inside the enclosing test:
    /// ```text
    /// let mut sink = Reject;
    /// assert!(fmt::Write::write_str(&mut sink, "token").is_err());
    /// ```
    struct Reject;
    impl fmt::Write for Reject {
        /// Reject every formatting write without storing text or performing I/O. Supplies the failure
        /// injected by the enclosing display test.
        ///
        /// # Arguments
        /// - `self` `[in,out]`: Rejecting writer required mutably by fmt::Write; no state is changed.
        /// - `_` `[in]`: Borrowed text deliberately discarded, including empty strings.
        ///
        /// # Returns
        /// `Err(fmt::Error)` unconditionally; never `Ok(())` and never partially stores output.
        ///
        /// # Errors
        /// Always returns fmt::Error to simulate a failed output sink.
        ///
        /// # Examples
        /// Local test-module context:
        /// ```text
        /// assert!(fmt::Write::write_str(&mut Reject, "token").is_err());
        /// ```
        fn write_str(&mut self, _: &str) -> fmt::Result {
            Err(fmt::Error)
        }
    }
    let source = "foo(\"bar\")";
    let buffer = Lexer::new(RawBuffer::new(source)).parse();
    for colored in [false, true] {
        assert!(
            fmt::write(
                &mut Reject,
                format_args!("{}", buffer.display(colored, source))
            )
            .is_err()
        );
    }
}
