use cmake_analyzer::{
    model::Spanned,
    parser::lexer::model::{LexContent, LexError, Lexer, RawBuffer},
};

#[test]
fn identifiers_keep_their_first_character() {
    let source = "message(hello world) next(foo)";
    let mut lexer = Lexer::new(RawBuffer::new(source));
    let result = lexer.parse();

    assert!(result.errors.is_empty());
    let identifiers: Vec<&str> = result
        .lex
        .iter()
        .filter_map(|token| match &token.content {
            LexContent::Identifier(text) => Some(text.as_str()),
            _ => None,
        })
        .collect();
    assert_eq!(identifiers, ["message", "hello", "world", "next", "foo"]);
    assert_eq!(lexer.buffer.cursor.offset, source.len());
    assert_eq!(
        result
            .lex
            .iter()
            .filter(|token| matches!(&token.content, LexContent::LeftParentheses))
            .count(),
        2
    );
    assert_eq!(
        result
            .lex
            .iter()
            .filter(|token| matches!(&token.content, LexContent::RightParentheses))
            .count(),
        2
    );
}

#[test]
fn unicode_identifiers_keep_their_first_character() {
    let source = "  中文(🦀值 参数)\n末尾";
    let mut lexer = Lexer::new(RawBuffer::new(source));
    let result = lexer.parse();
    assert!(result.errors.is_empty());
    let identifiers: Vec<&str> = result
        .lex
        .iter()
        .filter_map(|token| match &token.content {
            LexContent::Identifier(text) => Some(text.as_str()),
            _ => None,
        })
        .collect();
    assert_eq!(identifiers, ["中文", "🦀值", "参数", "末尾"]);
    assert_eq!(lexer.buffer.cursor.offset, source.len());
}

#[test]
fn quoted_strings_consume_both_quotes() {
    let source = r#"message("hello" "中\"文" "") next(value)"#;
    let mut lexer = Lexer::new(RawBuffer::new(source));
    let result = lexer.parse();
    assert!(result.errors.is_empty());
    let strings: Vec<&str> = result
        .lex
        .iter()
        .filter_map(|token| match &token.content {
            LexContent::StringLiteral(text) => Some(text.as_str()),
            _ => None,
        })
        .collect();
    assert_eq!(strings, ["hello", r#"中\"文"#, ""]);
    assert!(
        result
            .lex
            .iter()
            .any(|token| matches!(&token.content, LexContent::Identifier(text) if text == "next"))
    );
    assert_eq!(lexer.buffer.cursor.offset, source.len());
}

#[test]
fn normal_eof_is_not_an_error() {
    for source in ["", " \t\r\n", "x", "()", "\"text\"", "foo() \n"] {
        let mut lexer = Lexer::new(RawBuffer::new(source));
        let result = lexer.parse();
        assert!(result.errors.is_empty(), "{source:?}: {:?}", result.errors);
        assert_eq!(lexer.buffer.cursor.offset, source.len());
        assert!(lexer.parse().lex.is_empty());
    }
}

#[test]
fn unterminated_strings_are_reported() {
    let mut lexer = Lexer::new(RawBuffer::new("\"unfinished"));
    let result = lexer.parse();
    assert!(matches!(
        result.errors.as_slice(),
        [Spanned {
            content: LexError::UnclosedStringLiteral,
            ..
        }]
    ));
}

#[test]
fn parentheses_errors_are_still_reported() {
    for source in ["foo(", ") foo()"] {
        let mut lexer = Lexer::new(RawBuffer::new(source));
        let result = lexer.parse();
        assert!(
            result
                .errors
                .iter()
                .any(|error| matches!(error.content, LexError::UnclosedParentheses))
        );
    }
}
