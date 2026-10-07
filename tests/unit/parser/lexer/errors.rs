use cmake_analyzer::{
    model::TextSpan,
    parser::lexer::model::{LexError, LexResult, LexedBuffer, Lexer, RawBuffer},
};

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
