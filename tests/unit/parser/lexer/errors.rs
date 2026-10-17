//! Lexical error ranges and display: end-of-file, strings and parentheses keep exact source
//! positions. Only local scanner/buffer state changes.

// Contains AI-generated documentation edits.
// AI-generated tests.

use cmake_analyzer::{
    model::TextSpan,
    parser::lexer::model::{LexError, LexResult, LexedBuffer, Lexer, RawBuffer},
};

/// Exhaust a lexer on Chinese and crab text, then call peek, advance and identifier advance. Each
/// must report end-of-file at a zero-width final-byte span with the final scalar row/column.
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
/// scanner_eof_errors_have_zero_width_spans();
/// ```
#[test]
fn scanner_eof_errors_have_zero_width_spans() {
    let source = "前\n🦀";
    let mut lexer = Lexer::new(RawBuffer::new(source));
    assert!(lexer.parse().errors.is_empty());
    let errors = [
        lexer.peek().unwrap_err(),
        lexer.advance().unwrap_err(),
        lexer.advance_identifier().unwrap_err(),
    ];
    for error in errors {
        assert!(matches!(error.content, LexError::ReachTheEof));
        assert_eq!(
            (
                error.span.start_byte,
                error.span.end_byte,
                error.span.row,
                error.span.column
            ),
            (source.len(), source.len(), 1, 1)
        );
    }
}

/// Scan an unfinished multiline quoted string after Chinese text. Require exactly one
/// unclosed-string error covering the opening quote through end-of-file at row 1, column 0.
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
/// string_error_spans_include_the_opener_through_eof();
/// ```
#[test]
fn string_error_spans_include_the_opener_through_eof() {
    let source = "前\n\"中\nunfinished";
    let result = Lexer::new(RawBuffer::new(source)).parse();
    let [error] = result.errors.as_slice() else {
        panic!("{:?}", result.errors)
    };
    assert!(matches!(error.content, LexError::UnclosedStringLiteral));
    assert_eq!(
        (
            error.span.start_byte,
            error.span.end_byte,
            error.span.row,
            error.span.column
        ),
        (4, source.len(), 1, 0)
    );
}

/// Consume an opening quote manually and call the string scanner with an explicit `LexResult<()>`
/// type. Require its error to retain the same opener-through-end range.
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
/// direct_string_scanner_returns_the_same_spanned_error_type();
/// ```
#[test]
fn direct_string_scanner_returns_the_same_spanned_error_type() {
    let mut lexer = Lexer::new(RawBuffer::new("\"unfinished"));
    lexer.advance().unwrap();
    let result: LexResult<()> = lexer.advance_string_literal();
    let error = result.unwrap_err();
    assert!(matches!(error.content, LexError::UnclosedStringLiteral));
    assert_eq!(
        (error.span.start_byte, error.span.end_byte),
        (0, "\"unfinished".len())
    );
}

/// Scan an unmatched closing parenthesis after Chinese text and check its one-byte range and
/// display label. Also insert an end-of-file error into a fresh lexed buffer and require its label
/// to render.
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
/// parentheses_and_display_use_the_external_span();
/// ```
#[test]
fn parentheses_and_display_use_the_external_span() {
    let result = Lexer::new(RawBuffer::new("前\n)")).parse();
    let [error] = result.errors.as_slice() else {
        panic!("{:?}", result.errors)
    };
    assert!(matches!(error.content, LexError::UnclosedParentheses));
    assert_eq!(
        (
            error.span.start_byte,
            error.span.end_byte,
            error.span.row,
            error.span.column
        ),
        (4, 5, 1, 0)
    );
    assert!(
        result
            .display(false, "前\n)")
            .to_string()
            .contains("UnclosedParentheses at TextSpan")
    );
    let mut buffer = LexedBuffer::default();
    buffer.add_error(LexError::ReachTheEof, TextSpan::default());
    assert!(
        buffer
            .display(false, "")
            .to_string()
            .contains("ReachTheEof at TextSpan")
    );
}
