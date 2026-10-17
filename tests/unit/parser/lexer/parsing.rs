//! Basic lexer tokenization and termination: ASCII/Unicode identifiers, quoted strings, normal
//! end-of-file and malformed input. No external side effects.

// Contains AI-generated documentation edits.
// Contains AI-generated tests or test edits.

use cmake_analyzer::{
    model::Spanned,
    parser::lexer::model::{LexContent, LexError, Lexer, RawBuffer},
};

/// Scan two ASCII calls and compare all command/argument names, cursor exhaustion and parenthesis
/// counts, guarding against dropping the first identifier character.
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
/// identifiers_keep_their_first_character();
/// ```
#[test]
fn identifiers_keep_their_first_character() {
    let source = "message(hello world) next(foo)";
    let mut lexer = Lexer::new(RawBuffer::new(source));
    let result = lexer.parse();

    assert!(result.errors.is_empty());
    let identifiers: Vec<&str> = result
        .lex
        .iter()
        .filter(|token| token.content == LexContent::Identifier)
        .filter_map(|token| token.text(source))
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

/// Scan Chinese commands and a crab-prefixed argument after whitespace; require every full
/// identifier and the final byte cursor.
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
/// unicode_identifiers_keep_their_first_character();
/// ```
#[test]
fn unicode_identifiers_keep_their_first_character() {
    let source = "  中文(🦀值 参数)\n末尾";
    let mut lexer = Lexer::new(RawBuffer::new(source));
    let result = lexer.parse();
    assert!(result.errors.is_empty());
    let identifiers: Vec<&str> = result
        .lex
        .iter()
        .filter(|token| token.content == LexContent::Identifier)
        .filter_map(|token| token.text(source))
        .collect();
    assert_eq!(identifiers, ["中文", "🦀值", "参数", "末尾"]);
    assert_eq!(lexer.buffer.cursor.offset, source.len());
}

/// Scan ordinary, escaped-Unicode and empty strings followed by another call. Require unwrapped
/// payloads, the following command token and an exhausted cursor.
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
/// quoted_strings_consume_both_quotes();
/// ```
#[test]
fn quoted_strings_consume_both_quotes() {
    let source = r#"message("hello" "中\"文" "") next(value)"#;
    let mut lexer = Lexer::new(RawBuffer::new(source));
    let result = lexer.parse();
    assert!(result.errors.is_empty());
    let strings: Vec<&str> = result
        .lex
        .iter()
        .filter(|token| token.content == LexContent::StringLiteral)
        .filter_map(|token| token.text(source))
        .collect();
    assert_eq!(strings, ["hello", r#"中\"文"#, ""]);
    assert!(
        result
            .lex
            .iter()
            .any(|token| token.content == LexContent::Identifier
                && token.text(source) == Some("next"))
    );
    assert_eq!(lexer.buffer.cursor.offset, source.len());
}

/// Scan empty, whitespace-only, identifier, parenthesis, string and trailing-whitespace fixtures.
/// Each must finish without errors; parsing the exhausted lexer again returns no tokens.
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
/// normal_eof_is_not_an_error();
/// ```
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

/// Scan a single unfinished quote and require exactly one spanned unclosed-string error.
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
/// unterminated_strings_are_reported();
/// ```
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

/// Scan an unfinished call and a leading unmatched closing parenthesis, requiring a parenthesis
/// error in each result.
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
/// parentheses_errors_are_still_reported();
/// ```
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
