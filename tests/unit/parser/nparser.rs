mod display;

use cmake_analyzer::{
    model::{Spanned, TextSpan},
    parser::{
        lexer::model::{LexContent, LexError, LexedBuffer, Lexer, RawBuffer},
        nparser::model::{NParsedArgument, NParsedError, NParser},
    },
};

fn parser(source: &str) -> NParser {
    NParser::new(Lexer::new(RawBuffer::new(source)).parse())
}

#[test]
fn parses_commands_and_preserves_argument_kinds_and_spans() {
    let source = "message(hello \"中\" [=[${x}\\n]=])\nnext()";
    let mut parser = parser(source);
    let spans: Vec<TextSpan> = parser.buffer.lex.iter().map(|token| token.span).collect();
    let result = parser.parse();
    assert!(result.error.is_empty());
    assert_eq!(result.commands.len(), 2);
    assert_eq!(result.commands[0].name.content, "message");
    assert_eq!(result.commands[1].name.content, "next");
    let args = &result.commands[0].args;
    assert!(matches!(&args[0].content, NParsedArgument::Unquoted(text) if text == "hello"));
    assert!(matches!(&args[1].content, NParsedArgument::Quoted(text) if text == "中"));
    assert!(matches!(&args[2].content, NParsedArgument::Bracked(text) if text == "${x}\\n"));
    for (arg, expected) in args.iter().zip(&spans[2..5]) {
        assert_eq!(
            (
                arg.span.start_byte,
                arg.span.end_byte,
                arg.span.row,
                arg.span.column
            ),
            (
                expected.start_byte,
                expected.end_byte,
                expected.row,
                expected.column
            )
        );
    }
    assert_eq!(result.commands[0].name.span.start_byte, 0);
    assert_eq!(result.commands[0].name.span.end_byte, 7);
    assert!(result.commands[1].args.is_empty());
    assert_eq!(parser.cursor.offset, parser.cursor.max_length);
    assert!(parser.parse().commands.is_empty());
}

#[test]
fn internal_parentheses_are_arguments_not_nested_commands() {
    let mut parser = parser("if(A AND (B OR C))\nmessage(other(x))");
    let result = parser.parse();
    assert!(result.error.is_empty());
    assert_eq!(result.commands.len(), 2);
    assert_eq!(result.commands[0].args.len(), 7);
    assert!(matches!(
        result.commands[0].args[2].content,
        NParsedArgument::LeftParen
    ));
    assert!(matches!(
        result.commands[0].args[6].content,
        NParsedArgument::RightParen
    ));
    assert_eq!(result.commands[1].args.len(), 4);
    assert!(
        matches!(&result.commands[1].args[0].content, NParsedArgument::Unquoted(text) if text == "other")
    );
}

#[test]
fn helpers_advance_exactly_the_tokens_they_own() {
    let mut parser = parser("call(value) next()");
    assert_eq!(parser.advance_a_function_name().unwrap().content, "call");
    assert_eq!(parser.cursor.offset, 1);
    assert!(matches!(
        parser.peek().unwrap().content,
        LexContent::LeftParentheses
    ));
    let args = parser.advance_a_function_args().unwrap();
    assert_eq!(args.len(), 1);
    assert!(matches!(&args[0].content, NParsedArgument::Unquoted(text) if text == "value"));
    assert_eq!(parser.cursor.offset, 4);
    assert!(
        matches!(&parser.peek().unwrap().content, LexContent::Identifier(text) if text == "next")
    );
}

#[test]
fn individual_argument_scanning_preserves_every_kind() {
    let mut parser = parser("value \"\" [[raw]] ()");
    assert!(
        matches!(parser.advance_a_arg().unwrap().content, NParsedArgument::Unquoted(text) if text == "value")
    );
    assert!(
        matches!(parser.advance_a_arg().unwrap().content, NParsedArgument::Quoted(text) if text.is_empty())
    );
    assert!(
        matches!(parser.advance_a_arg().unwrap().content, NParsedArgument::Bracked(text) if text == "raw")
    );
    assert!(matches!(
        parser.advance_a_arg().unwrap().content,
        NParsedArgument::LeftParen
    ));
    assert!(matches!(
        parser.advance_a_arg().unwrap().content,
        NParsedArgument::RightParen
    ));
    assert!(matches!(
        parser.advance_a_arg(),
        Err(NParsedError::ReachTheEof)
    ));
}

#[test]
fn token_strings_are_moved_without_copying_their_allocations() {
    let mut parser = parser("command(value)");
    let LexContent::Identifier(name) = &parser.buffer.lex[0].content else {
        panic!()
    };
    let name_pointer = name.as_ptr();
    let LexContent::Identifier(value) = &parser.buffer.lex[2].content else {
        panic!()
    };
    let value_pointer = value.as_ptr();
    let result = parser.parse();
    assert_eq!(result.commands[0].name.content.as_ptr(), name_pointer);
    let NParsedArgument::Unquoted(value) = &result.commands[0].args[0].content else {
        panic!()
    };
    assert_eq!(value.as_ptr(), value_pointer);
    assert!(
        matches!(&parser.buffer.lex[0].content, LexContent::Identifier(text) if text.is_empty())
    );
}

#[test]
fn failed_name_scans_do_not_consume_invalid_tokens() {
    for source in [
        "9bad()",
        "bad-name()",
        "中文()",
        "\"quoted\"",
        "[[bracket]]",
        "()",
    ] {
        let mut parser = parser(source);
        assert!(parser.advance_a_function_name().is_err());
        assert_eq!(parser.cursor.offset, 0);
    }
    for source in ["_name()", "Name_123()"] {
        let mut parser = parser(source);
        assert!(parser.advance_a_function_name().is_ok());
    }
}

#[test]
fn missing_opening_parenthesis_does_not_consume_the_next_command() {
    let mut parser = parser("missing\nmessage(ok)\n\"bad\"\nnext()");
    let result = parser.parse();
    let names: Vec<&str> = result
        .commands
        .iter()
        .map(|command| command.name.content.as_str())
        .collect();
    assert_eq!(names, ["message", "next"]);
    assert!(
        result
            .error
            .iter()
            .any(|error| matches!(error.content, NParsedError::ExpectedLeftParenthesis))
    );
    assert!(
        result
            .error
            .iter()
            .any(|error| matches!(error.content, NParsedError::ExpectedCommandName))
    );
    assert_eq!(parser.cursor.offset, parser.cursor.max_length);
}

#[test]
fn incomplete_arguments_report_the_opening_parenthesis() {
    let mut parser = parser("complete()\ncall(value (nested)");
    let result = parser.parse();
    assert_eq!(result.commands.len(), 1);
    assert_eq!(result.commands[0].name.content, "complete");
    let error = result
        .error
        .iter()
        .find(|error| matches!(error.content, NParsedError::UnclosedArguments(_)))
        .unwrap();
    assert_eq!(
        (
            error.span.start_byte,
            error.span.end_byte,
            error.span.row,
            error.span.column
        ),
        (15, 16, 1, 4)
    );
    assert!(result.error.iter().any(|error| matches!(
        error.content,
        NParsedError::LexError(LexError::UnclosedParentheses(_))
    )));
}

#[test]
fn lexical_errors_are_forwarded_even_without_any_tokens() {
    let mut parser = parser("[=[unfinished");
    let result = parser.parse();
    assert!(result.commands.is_empty());
    assert!(matches!(
        result.error[0].content,
        NParsedError::LexError(LexError::UnclosedBracketArgument(_))
    ));
    assert_eq!(result.error[0].span.start_byte, 0);
    assert_eq!(result.error[0].span.end_byte, "[=[unfinished".len());
    assert!(parser.buffer.errors.is_empty());
}

#[test]
fn eof_helpers_return_errors_without_panicking_or_advancing() {
    let mut parser = parser("");
    assert!(parser.parse().error.is_empty());
    assert!(matches!(parser.peek(), Err(NParsedError::ReachTheEof)));
    assert!(matches!(
        parser.advance_a_function_name(),
        Err(NParsedError::ReachTheEof)
    ));
    assert!(matches!(
        parser.advance_a_function_args(),
        Err(NParsedError::ReachTheEof)
    ));
    assert!(matches!(
        parser.advance_a_arg(),
        Err(NParsedError::ReachTheEof)
    ));
    assert_eq!(parser.cursor.offset, 0);
    let mut parser = NParser::new(LexedBuffer {
        lex: vec![Spanned {
            content: LexContent::RightParentheses,
            span: TextSpan::default(),
        }],
        errors: vec![],
    });
    assert_eq!(parser.parse().error.len(), 1);
    assert_eq!(parser.cursor.offset, 1);
}

#[test]
fn deep_parentheses_use_iterative_scanning() {
    let depth = 4096;
    let source = format!("if({}x{})", "(".repeat(depth), ")".repeat(depth));
    let result = parser(&source).parse();
    assert!(result.error.is_empty());
    assert_eq!(result.commands.len(), 1);
    assert_eq!(result.commands[0].args.len(), depth * 2 + 1);
}
