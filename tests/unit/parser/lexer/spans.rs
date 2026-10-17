//! Token and cursor coordinates use byte ranges and Unicode-scalar columns. In-memory tests also
//! cover borrowed slicing and colored display.

// Contains AI-generated documentation edits.
// Contains AI-generated tests or test edits.

use cmake_analyzer::{
    model::TextSpan,
    parser::lexer::model::{LexContent, Lexer, RawBuffer},
};

/// Scan mixed tabs, Chinese text, an escaped quote/crab/multiline string and CRLF. Compare every
/// token's byte endpoints, scalar row/column and raw source text, then check the string payload
/// omits quotes.
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
/// tokens_keep_exact_ranges_and_start_positions();
/// ```
#[test]
fn tokens_keep_exact_ranges_and_start_positions() {
    let source = " \tfoo(中 \"a\\\"🦀\n尾\")\r\nbar";
    let mut lexer = Lexer::new(RawBuffer::new(source));
    let result = lexer.parse();
    assert!(result.errors.is_empty());
    let expected = [
        (2, 5, 0, 2, "foo"),
        (5, 6, 0, 5, "("),
        (6, 9, 0, 6, "中"),
        (10, 23, 0, 8, "\"a\\\"🦀\n尾\""),
        (23, 24, 1, 2, ")"),
        (26, 29, 2, 0, "bar"),
    ];
    assert_eq!(result.lex.len(), expected.len());
    for (token, (start, end, row, column, raw)) in result.lex.iter().zip(expected) {
        let span = token.span;
        assert_eq!(
            (span.start_byte, span.end_byte, span.row, span.column),
            (start, end, row, column)
        );
        assert_eq!(lexer.buffer.span_to_text(span).as_deref(), Some(raw));
    }
    assert_eq!(result.lex[3].content, LexContent::StringLiteral);
    assert_eq!(result.lex[3].text(source), Some("a\\\"🦀\n尾"));
}

/// Scan two quotes and require an empty string payload with the full 0..2 token range at row/column
/// zero.
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
/// empty_strings_have_a_two_byte_token_range();
/// ```
#[test]
fn empty_strings_have_a_two_byte_token_range() {
    let mut lexer = Lexer::new(RawBuffer::new("\"\""));
    let result = lexer.parse();
    assert!(result.errors.is_empty());
    assert_eq!(result.lex[0].content, LexContent::StringLiteral);
    assert_eq!(result.lex[0].text("\"\""), Some(""));
    let span = result.lex[0].span;
    assert_eq!(
        (span.start_byte, span.end_byte, span.row, span.column),
        (0, 2, 0, 0)
    );
}

/// Advance over Chinese text and a newline before parsing the remaining indented identifier.
/// Require absolute byte 6..9 and row 1, column 2 rather than suffix-relative coordinates.
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
/// parsing_from_an_existing_cursor_keeps_absolute_positions();
/// ```
#[test]
fn parsing_from_an_existing_cursor_keeps_absolute_positions() {
    let mut lexer = Lexer::new(RawBuffer::new("前\n  abc"));
    assert_eq!(lexer.advance().unwrap(), '前');
    assert_eq!(lexer.advance().unwrap(), '\n');
    let result = lexer.parse();
    assert!(result.errors.is_empty());
    assert_eq!(result.lex.len(), 1);
    let span = result.lex[0].span;
    assert_eq!(
        (span.start_byte, span.end_byte, span.row, span.column),
        (6, 9, 1, 2)
    );
    assert_eq!(result.lex[0].content, LexContent::Identifier);
    assert_eq!(result.lex[0].text("前\n  abc"), Some("abc"));
}

/// Manually assign five valid byte offsets in `中\nabc` and require each zero-width cursor span to
/// use that offset's row and scalar column.
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
/// cursor_spans_locate_the_cursor_instead_of_the_end_of_the_document();
/// ```
#[test]
fn cursor_spans_locate_the_cursor_instead_of_the_end_of_the_document() {
    let mut buffer = RawBuffer::new("中\nabc");
    for (offset, row, column) in [(0, 0, 0), (3, 0, 1), (4, 1, 0), (6, 1, 2), (7, 1, 3)] {
        buffer.cursor.offset = offset;
        let span = buffer.current_byte_to_span();
        assert_eq!(
            (span.start_byte, span.end_byte, span.row, span.column),
            (offset, offset, row, column)
        );
    }
}

/// Read valid Unicode/ASCII/empty-end spans and reject split-scalar, out-of-bounds and reversed
/// byte ranges against one raw buffer.
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
/// span_text_uses_half_open_utf8_ranges_including_empty_eof();
/// ```
#[test]
fn span_text_uses_half_open_utf8_ranges_including_empty_eof() {
    let buffer = RawBuffer::new("中x");
    for (start, end, expected) in [
        (0, 3, Some("中")),
        (3, 4, Some("x")),
        (4, 4, Some("")),
        (1, 3, None),
        (0, 2, None),
        (0, 5, None),
        (4, 3, None),
    ] {
        let span = TextSpan {
            start_byte: start,
            end_byte: end,
            ..TextSpan::default()
        };
        assert_eq!(buffer.span_to_text(span).as_deref(), expected);
    }
}

/// Render a call with a quoted argument as exact plain token lines and require ANSI escape bytes
/// when color is enabled.
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
/// display_still_formats_token_content_with_optional_color();
/// ```
#[test]
fn display_still_formats_token_content_with_optional_color() {
    let mut lexer = Lexer::new(RawBuffer::new("foo(\"bar\")"));
    let result = lexer.parse();
    assert_eq!(
        result.display(false, "foo(\"bar\")").to_string(),
        "Identifier: foo\n(\nStringLiteral: bar\n)\n"
    );
    assert!(
        result
            .display(true, "foo(\"bar\")")
            .to_string()
            .contains("\x1b[")
    );
}
