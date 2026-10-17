//! Bracket-argument delimiter and payload regressions, including Unicode, newline trimming, long
//! equals runs and unterminated input. Only local lexer state changes.

// Contains AI-generated documentation edits.
// Contains AI-generated tests or test edits.

use cmake_analyzer::parser::lexer::model::{LexContent, LexError, Lexer, RawBuffer};

/// Scan empty, mismatched-delimiter and literal-special-character bracket arguments. Require one
/// bracket token, unwrapped payload, full delimiter-inclusive span and a cursor at end-of-file.
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
/// bracket_arguments_are_unwrapped_but_keep_their_token_kind_and_full_span();
/// ```
#[test]
fn bracket_arguments_are_unwrapped_but_keep_their_token_kind_and_full_span() {
    for (source, expected) in [
        ("[[]]", ""),
        ("[[hello]]", "hello"),
        ("[=[a ]==] b]=]", "a ]==] b"),
        ("[==[a ]=] b]==]", "a ]=] b"),
        (r#"[[${x}\n "()#;" ]]"#, r#"${x}\n "()#;" "#),
        (r#"[[x\]]"#, r#"x\"#),
    ] {
        let mut lexer = Lexer::new(RawBuffer::new(source));
        let result = lexer.parse();
        assert!(result.errors.is_empty(), "{source}: {:?}", result.errors);
        assert_eq!(result.lex.len(), 1);
        assert_eq!(result.lex[0].content, LexContent::BracketArgument);
        assert_eq!(result.lex[0].text(source), Some(expected));
        let span = result.lex[0].span;
        assert_eq!(
            (span.start_byte, span.end_byte, span.row, span.column),
            (0, source.len(), 0, 0)
        );
        assert_eq!(lexer.buffer.span_to_text(span), Some(source));
        assert_eq!(lexer.buffer.cursor.offset, source.len());
    }
}

/// Compare bracket payloads beginning with LF, CRLF, doubled newlines, spaces or a lone carriage
/// return. Only one immediate LF/CRLF is omitted, including the empty-payload case.
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
/// exactly_one_initial_lf_or_crlf_is_removed();
/// ```
#[test]
fn exactly_one_initial_lf_or_crlf_is_removed() {
    for (source, expected) in [
        ("[[\nhello]]", "hello"),
        ("[[\r\nhello]]", "hello"),
        ("[[\n\nhello]]", "\nhello"),
        ("[[\r\n\r\nhello]]", "\r\nhello"),
        ("[[ \nhello]]", " \nhello"),
        ("[[\rhello]]", "\rhello"),
        ("[[\n]]", ""),
    ] {
        let result = Lexer::new(RawBuffer::new(source)).parse();
        assert!(result.errors.is_empty());
        assert_eq!(result.lex[0].content, LexContent::BracketArgument);
        assert_eq!(result.lex[0].text(source), Some(expected));
    }
}

/// Scan a bracket payload containing Chinese text, parentheses, quotes and an expansion. Check its
/// full range and start column, the next command's row and exactly two real pairs of parentheses.
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
/// multiline_brackets_do_not_affect_parentheses_and_preserve_later_positions();
/// ```
#[test]
fn multiline_brackets_do_not_affect_parentheses_and_preserve_later_positions() {
    let source = "  message([=[\r\n中 ( \"${x}\"\n]=])\r\nnext()";
    let result = Lexer::new(RawBuffer::new(source)).parse();
    assert!(result.errors.is_empty());
    let bracket = &result.lex[2];
    assert_eq!(bracket.content, LexContent::BracketArgument);
    assert_eq!(bracket.text(source), Some("中 ( \"${x}\"\n"));
    assert_eq!(
        (bracket.span.start_byte, bracket.span.end_byte),
        (source.find("[=[").unwrap(), source.find("]=]").unwrap() + 3)
    );
    assert_eq!((bracket.span.row, bracket.span.column), (0, 10));
    let next = result
        .lex
        .iter()
        .find(|token| token.content == LexContent::Identifier && token.text(source) == Some("next"))
        .unwrap();
    assert_eq!((next.span.row, next.span.column), (3, 0));
    for kind in [LexContent::LeftParentheses, LexContent::RightParentheses] {
        assert_eq!(
            result
                .lex
                .iter()
                .filter(|token| token.content == kind)
                .count(),
            2
        );
    }
}

/// Scan two closing sequences where a failed delimiter candidate overlaps the eventual match.
/// Require the preceding unmatched bracket/equal bytes to remain in the payload.
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
/// closing_delimiters_can_overlap_a_mismatched_candidate();
/// ```
#[test]
fn closing_delimiters_can_overlap_a_mismatched_candidate() {
    for (source, expected) in [("[=[x]==]=]", "x]=="), ("[=[x]]=]", "x]")] {
        let result = Lexer::new(RawBuffer::new(source)).parse();
        assert!(result.errors.is_empty());
        assert_eq!(result.lex[0].text(source), Some(expected));
    }
}

/// Build bracket delimiters with zero through 63 equals signs and a 4096-sign case around
/// Unicode/special-character text. Every scan must retain one bracket payload without errors.
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
/// delimiters_support_arbitrary_equals_counts();
/// ```
#[test]
fn delimiters_support_arbitrary_equals_counts() {
    for count in (0..64).chain([4096]) {
        let equals = "=".repeat(count);
        let source = format!("[{equals}[文本 ) ${{x}} \\n]{equals}]");
        let result = Lexer::new(RawBuffer::new(&source)).parse();
        assert!(result.errors.is_empty());
        assert_eq!(result.lex[0].content, LexContent::BracketArgument);
        assert_eq!(result.lex[0].text(&source), Some("文本 ) ${x} \\n"));
    }
}

/// Scan incomplete openers, ordinary bracket text and brackets after an identifier prefix. Require
/// ordinary identifier tokens with unchanged text rather than bracket-argument interpretation.
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
/// invalid_openers_and_embedded_brackets_remain_unquoted();
/// ```
#[test]
fn invalid_openers_and_embedded_brackets_remain_unquoted() {
    for source in ["[", "[=", "[==foo", "[text]", "prefix[[text]]"] {
        let result = Lexer::new(RawBuffer::new(source)).parse();
        assert!(result.errors.is_empty());
        assert_eq!(result.lex[0].content, LexContent::Identifier);
        assert_eq!(result.lex[0].text(source), Some(source));
    }
}

/// Scan `[[a [[b]] tail`; the first closing pair must end the argument, leaving `a [[b` as payload
/// and `tail` as a separate identifier.
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
/// bracket_arguments_do_not_nest();
/// ```
#[test]
fn bracket_arguments_do_not_nest() {
    let source = "[[a [[b]] tail";
    let result = Lexer::new(RawBuffer::new(source)).parse();
    assert!(result.errors.is_empty());
    assert_eq!(result.lex[0].content, LexContent::BracketArgument);
    assert_eq!(result.lex[0].text(source), Some("a [[b"));
    assert_eq!(result.lex[1].content, LexContent::Identifier);
    assert_eq!(result.lex[1].text(source), Some("tail"));
}

/// Prefix unterminated or mismatched bracket arguments with Chinese text. Require the preceding
/// identifier only, one unclosed-bracket error spanning opener through end-of-file and an exhausted
/// cursor.
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
/// unterminated_brackets_report_the_opener_and_consume_to_eof();
/// ```
#[test]
fn unterminated_brackets_report_the_opener_and_consume_to_eof() {
    for source in ["前 [==[\n内容", "前 [==[x]=]", "前 [[", "前 [[\n"] {
        let mut lexer = Lexer::new(RawBuffer::new(source));
        let result = lexer.parse();
        assert_eq!(result.lex.len(), 1);
        let [error] = result.errors.as_slice() else {
            panic!("unexpected errors: {:?}", result.errors);
        };
        assert!(matches!(error.content, LexError::UnclosedBracketArgument));
        let span = error.span;
        assert_eq!(
            (span.start_byte, span.end_byte, span.row, span.column),
            (4, source.len(), 0, 2)
        );
        assert_eq!(lexer.buffer.cursor.offset, source.len());
    }
}

/// Render a complete bracket token with and without color, then render an unfinished bracket input.
/// Check unwrapped plain text, ANSI presence and the unclosed-bracket error label.
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
/// bracket_tokens_and_errors_can_be_displayed();
/// ```
#[test]
fn bracket_tokens_and_errors_can_be_displayed() {
    let source = "[[hello]]";
    let result = Lexer::new(RawBuffer::new(source)).parse();
    assert_eq!(
        result.display(false, source).to_string(),
        "BracketArgument: hello\n"
    );
    assert!(result.display(true, source).to_string().contains("\x1b["));
    let source = "[=[unfinished";
    assert!(
        Lexer::new(RawBuffer::new(source))
            .parse()
            .display(false, source)
            .to_string()
            .contains("UnclosedBracketArgument")
    );
}
