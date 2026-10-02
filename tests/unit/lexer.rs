use cmake_analyzer::{
    model::trait1::IBuffer,
    parser::lexer::{LexBuffer, LexErrorKind, LexKind, Lexer, PreBuffer},
};

fn assert_lossless(buffer: &LexBuffer<'_>, start: usize) {
    let mut end = start;
    for part in &buffer.parts {
        assert_eq!(part.span.start_byte, end);
        assert!(part.span.end_byte > end);
        assert!(buffer.part_text(part).is_some());
        end = part.span.end_byte;
    }
    assert_eq!(end, buffer.text.len());
}

fn assert_parts(source: &str, expected: &[(LexKind, &str)]) {
    let buffer = Lexer::lex(source);
    assert!(buffer.errors.is_empty(), "{:?}", buffer.errors);
    let actual: Vec<_> = buffer
        .parts
        .iter()
        .map(|part| (part.kind, buffer.part_text(part).unwrap()))
        .collect();
    assert_eq!(actual, expected);
    assert_lossless(&buffer, 0);
    assert_eq!(buffer.text.as_ptr(), source.as_ptr());
}

#[test]
fn basic_call_and_trivia() {
    use LexKind::*;
    assert_parts(
        "MESSAGE (hello a;b) # tail\r\n",
        &[
            (Identifier, "MESSAGE"),
            (Space, " "),
            (ParenLeft, "("),
            (Identifier, "hello"),
            (Space, " "),
            (ArgumentUnquoted, "a;b"),
            (ParenRight, ")"),
            (Space, " "),
            (CommentLine, "# tail"),
            (Newline, "\r\n"),
        ],
    );
}

#[test]
fn quoted_content_is_raw_not_evaluated() {
    for source in [
        "\"\"",
        "\"${x};#()\"",
        "\"first\nsecond\"",
        "\"one\\\ncontinued\"",
        "\"one\\\r\ncontinued\"",
        "\"escaped\\\"quote\\\\\"",
    ] {
        assert_parts(source, &[(LexKind::ArgumentQuoted, source)]);
    }
}

#[test]
fn bracket_delimiters_and_comments() {
    for source in ["[[]]", "[=[\n${x};\\n]]]=]", "[==[ ]=] ]]]==]"] {
        assert_parts(source, &[(LexKind::ArgumentBracket, source)]);
    }
    for source in ["#[[]]", "#[==[\r\n#()${x}]===] ]==]"] {
        assert_parts(source, &[(LexKind::CommentBracket, source)]);
    }
    assert_parts(
        "# [[not bracket]]",
        &[(LexKind::CommentLine, "# [[not bracket]]")],
    );
}

#[test]
fn mismatched_bracket_closers_do_not_terminate() {
    let source = "[==[a ]=] b ]===] c ]==]";
    assert_parts(source, &[(LexKind::ArgumentBracket, source)]);
}

#[test]
fn large_bracket_delimiter() {
    let equals = "=".repeat(4096);
    let source = format!("[{equals}[ ]=] ]{equals}]");
    assert_parts(&source, &[(LexKind::ArgumentBracket, &source)]);
}

#[test]
fn legacy_and_escaped_arguments() {
    for source in [
        "-Da=\"b c\"",
        "-Da=$(v)",
        "a\" \"b\"c\"d",
        "$(MAKE_VAR)",
        "a\\ b",
        "a\\;b",
        "a\\#b",
        "a\\(b\\)",
        "[not_a_bracket",
        "a[[embedded]]",
        "${outer_${inner}_suffix}",
        "$<$<CONFIG:Debug>:DEBUG>",
        "a\\中",
    ] {
        assert_parts(source, &[(LexKind::ArgumentUnquoted, source)]);
    }
}

#[test]
fn make_reference_is_not_a_general_parenthesis_escape() {
    use LexKind::*;
    assert_parts(
        "$(a-b)",
        &[
            (ArgumentUnquoted, "$"),
            (ParenLeft, "("),
            (ArgumentUnquoted, "a-b"),
            (ParenRight, ")"),
        ],
    );
}

#[test]
fn nested_parentheses_remain_tokens() {
    use LexKind::*;
    assert_parts(
        "if(A AND (B OR C))",
        &[
            (Identifier, "if"),
            (ParenLeft, "("),
            (Identifier, "A"),
            (Space, " "),
            (Identifier, "AND"),
            (Space, " "),
            (ParenLeft, "("),
            (Identifier, "B"),
            (Space, " "),
            (Identifier, "OR"),
            (Space, " "),
            (Identifier, "C"),
            (ParenRight, ")"),
            (ParenRight, ")"),
        ],
    );
}

#[test]
fn adjacent_quotes_are_not_merged() {
    assert_parts(
        "\"a\"\"b\"",
        &[
            (LexKind::ArgumentQuoted, "\"a\""),
            (LexKind::ArgumentQuoted, "\"b\""),
        ],
    );
}

#[test]
fn incomplete_input_retains_tokens_and_errors() {
    for (source, kind, error) in [
        (
            "\"missing",
            LexKind::ArgumentQuoted,
            LexErrorKind::UnterminatedQuotedArgument,
        ),
        (
            "\"trailing\\",
            LexKind::ArgumentQuoted,
            LexErrorKind::UnterminatedQuotedArgument,
        ),
        (
            "[=[missing]]",
            LexKind::ArgumentBracket,
            LexErrorKind::UnterminatedBracketArgument,
        ),
        (
            "#[==[missing",
            LexKind::CommentBracket,
            LexErrorKind::UnterminatedBracketComment,
        ),
    ] {
        let buffer = Lexer::lex(source);
        assert_eq!(buffer.parts.len(), 1);
        assert_eq!(buffer.parts[0].kind, kind);
        assert_eq!(buffer.errors.len(), 1);
        assert_eq!(buffer.errors[0].kind, error);
        assert_eq!(buffer.errors[0].span.start_byte, 0);
        assert_eq!(buffer.errors[0].span.end_byte, source.len());
        assert_lossless(&buffer, 0);
    }
}

#[test]
fn invalid_character_recovers_without_losing_text() {
    let buffer = Lexer::lex("a\0b\\\nc");
    assert_eq!(buffer.errors.len(), 2);
    assert!(
        buffer
            .errors
            .iter()
            .all(|error| error.kind == LexErrorKind::UnexpectedCharacter)
    );
    assert_eq!(buffer.parts.last().unwrap().kind, LexKind::Identifier);
    assert_lossless(&buffer, 0);
}

#[test]
fn unicode_bom_and_crlf_positions() {
    let source = "\u{feff}中 文\r\nmessage(\"中文\")\n";
    let buffer = Lexer::lex(source);
    assert!(buffer.errors.is_empty());
    assert_eq!(buffer.parts[0].kind, LexKind::Bom);
    let first = &buffer.parts[1].span;
    assert_eq!((first.row, first.column, first.start_byte), (0, 0, 3));
    let word = buffer
        .parts
        .iter()
        .find(|part| buffer.part_text(part) == Some("message"))
        .unwrap();
    assert_eq!((word.span.row, word.span.column), (1, 0));
    for part in &buffer.parts {
        let position = PreBuffer {
            text: source,
            current_byte: part.span.start_byte,
        }
        .current_byte_to_span();
        assert_eq!(
            (position.row, position.column),
            (part.span.row, part.span.column)
        );
    }
    assert_lossless(&buffer, 0);
}

#[test]
fn scanning_from_a_valid_cursor() {
    let source = "中\r\nmessage(x)";
    let start = "中\r\n".len();
    let buffer = Lexer::parse_buffer(PreBuffer {
        text: source,
        current_byte: start,
    });
    assert!(buffer.errors.is_empty());
    assert_eq!(
        (buffer.parts[0].span.row, buffer.parts[0].span.column),
        (1, 0)
    );
    assert_lossless(&buffer, start);
}

#[test]
fn invalid_cursor_is_reported_not_panicked() {
    for offset in [1, 2, 4, usize::MAX] {
        let buffer = Lexer::parse_buffer(PreBuffer {
            text: "中",
            current_byte: offset,
        });
        assert!(buffer.parts.is_empty());
        assert_eq!(buffer.errors[0].kind, LexErrorKind::InvalidOffset);
    }
}

#[test]
fn empty_input_and_eof_cursor() {
    assert_parts("", &[]);
    let buffer = Lexer::parse_buffer(PreBuffer {
        text: "x",
        current_byte: 1,
    });
    assert!(buffer.parts.is_empty());
    assert!(buffer.errors.is_empty());
}

#[test]
fn short_arbitrary_inputs_always_progress_and_keep_utf8_boundaries() {
    const ALPHABET: [&str; 12] = [
        "a", "中", "[", "]", "=", "#", "\"", "\\", "\n", "\r", "$", "\0",
    ];
    for a in ALPHABET {
        for b in ALPHABET {
            for c in ALPHABET {
                let source = format!("{a}{b}{c}");
                let buffer = Lexer::lex(&source);
                assert_lossless(&buffer, 0);
            }
        }
    }
}
