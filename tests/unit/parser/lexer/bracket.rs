use cmake_analyzer::parser::lexer::model::{LexContent, LexError, Lexer, RawBuffer};

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
        assert!(
            matches!(&result.lex[0].content, LexContent::BracketArgument(text) if text == expected)
        );
        let span = result.lex[0].span;
        assert_eq!(
            (span.start_byte, span.end_byte, span.row, span.column),
            (0, source.len(), 0, 0)
        );
        assert_eq!(lexer.buffer.span_to_text(span).as_deref(), Some(source));
        assert_eq!(lexer.buffer.cursor.offset, source.len());
    }
}

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
        let mut lexer = Lexer::new(RawBuffer::new(source));
        let result = lexer.parse();
        assert!(result.errors.is_empty());
        assert!(
            matches!(&result.lex[0].content, LexContent::BracketArgument(text) if text == expected)
        );
    }
}

#[test]
fn multiline_brackets_do_not_affect_parentheses_and_preserve_later_positions() {
    let source = "  message([=[\r\n中 ( \"${x}\"\n]=])\r\nnext()";
    let mut lexer = Lexer::new(RawBuffer::new(source));
    let result = lexer.parse();
    assert!(result.errors.is_empty());
    let bracket = &result.lex[2];
    assert!(
        matches!(&bracket.content, LexContent::BracketArgument(text) if text == "中 ( \"${x}\"\n")
    );
    assert_eq!(
        (bracket.span.start_byte, bracket.span.end_byte),
        (source.find("[=[").unwrap(), source.find("]=]").unwrap() + 3)
    );
    assert_eq!((bracket.span.row, bracket.span.column), (0, 10));
    let next = result
        .lex
        .iter()
        .find(|token| matches!(&token.content, LexContent::Identifier(text) if text == "next"))
        .unwrap();
    assert_eq!((next.span.row, next.span.column), (3, 0));
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
fn closing_delimiters_can_overlap_a_mismatched_candidate() {
    for (source, expected) in [("[=[x]==]=]", "x]=="), ("[=[x]]=]", "x]")] {
        let mut lexer = Lexer::new(RawBuffer::new(source));
        let result = lexer.parse();
        assert!(result.errors.is_empty());
        assert!(
            matches!(&result.lex[0].content, LexContent::BracketArgument(text) if text == expected)
        );
    }
}

#[test]
fn delimiters_support_arbitrary_equals_counts() {
    for count in (0..64).chain([4096]) {
        let equals = "=".repeat(count);
        let source = format!("[{equals}[文本 ) ${{x}} \\n]{equals}]");
        let mut lexer = Lexer::new(RawBuffer::new(&source));
        let result = lexer.parse();
        assert!(result.errors.is_empty());
        assert!(
            matches!(&result.lex[0].content, LexContent::BracketArgument(text) if text == "文本 ) ${x} \\n")
        );
    }
}

#[test]
fn invalid_openers_and_embedded_brackets_remain_unquoted() {
    for source in ["[", "[=", "[==foo", "[text]", "prefix[[text]]"] {
        let mut lexer = Lexer::new(RawBuffer::new(source));
        let result = lexer.parse();
        assert!(result.errors.is_empty());
        assert!(matches!(&result.lex[0].content, LexContent::Identifier(text) if text == source));
    }
}

#[test]
fn bracket_arguments_do_not_nest() {
    let mut lexer = Lexer::new(RawBuffer::new("[[a [[b]] tail"));
    let result = lexer.parse();
    assert!(result.errors.is_empty());
    assert!(matches!(&result.lex[0].content, LexContent::BracketArgument(text) if text == "a [[b"));
    assert!(matches!(&result.lex[1].content, LexContent::Identifier(text) if text == "tail"));
}

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

#[test]
fn bracket_tokens_and_errors_can_be_displayed() {
    let mut lexer = Lexer::new(RawBuffer::new("[[hello]]"));
    let result = lexer.parse();
    assert_eq!(
        result.display(false).to_string(),
        "BracketArgument: hello\n"
    );
    assert!(result.display(true).to_string().contains("\x1b["));
    let mut lexer = Lexer::new(RawBuffer::new("[=[unfinished"));
    assert!(
        lexer
            .parse()
            .display(false)
            .to_string()
            .contains("UnclosedBracketArgument")
    );
}
