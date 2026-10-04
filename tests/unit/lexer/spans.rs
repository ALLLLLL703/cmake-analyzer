use cmake_analyzer::{
    model::TextSpan,
    parser::lexer::model::{LexContent, Lexer, RawBuffer},
};

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
    assert!(
        matches!(&result.lex[3].content, LexContent::StringLiteral(text) if text == "a\\\"🦀\n尾")
    );
}

#[test]
fn empty_strings_have_a_two_byte_token_range() {
    let mut lexer = Lexer::new(RawBuffer::new("\"\""));
    let result = lexer.parse();
    assert!(result.errors.is_empty());
    assert!(matches!(&result.lex[0].content, LexContent::StringLiteral(text) if text.is_empty()));
    let span = result.lex[0].span;
    assert_eq!(
        (span.start_byte, span.end_byte, span.row, span.column),
        (0, 2, 0, 0)
    );
}

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
    assert!(matches!(&result.lex[0].content, LexContent::Identifier(text) if text == "abc"));
}

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

#[test]
fn display_still_formats_token_content_with_optional_color() {
    let mut lexer = Lexer::new(RawBuffer::new("foo(\"bar\")"));
    let result = lexer.parse();
    assert_eq!(
        result.display(false).to_string(),
        "Identifier: foo\n(\nStringLiteral: bar\n)\n"
    );
    assert!(result.display(true).to_string().contains("\x1b["));
}
