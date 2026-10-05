use std::fmt;

use cmake_analyzer::{
    model::TextSpan,
    parser::{
        lexer::model::{LexError, Lexer, RawBuffer},
        nparser::model::{NParsedBuffer, NParsedError, NParser},
    },
};

#[test]
fn plain_display_groups_commands_and_preserves_argument_kinds() {
    let source = "if(A AND (B OR C))\nmessage(\"hello\" [=[raw]=])\nempty()";
    let buffer = NParser::new(Lexer::new(RawBuffer::new(source)).parse()).parse();
    let expected = "Command: if\n  Unquoted: \"A\"\n  Unquoted: \"AND\"\n  (\n  Unquoted: \"B\"\n  Unquoted: \"OR\"\n  Unquoted: \"C\"\n  )\nCommand: message\n  Quoted: \"hello\"\n  Bracked: \"raw\"\nCommand: empty\n";
    assert_eq!(buffer.display(false).to_string(), expected);
    assert_eq!(buffer.to_string(), expected);
    assert!(buffer.display(true).to_string().contains("\x1b["));
    assert!(NParsedBuffer::default().to_string().is_empty());
}

#[test]
fn errors_include_labels_and_positions() {
    let mut buffer = NParsedBuffer::default();
    let span = TextSpan {
        row: 2,
        column: 3,
        start_byte: 10,
        end_byte: 11,
    };
    let errors = [
        (NParsedError::ReachTheEof, "ReachTheEof"),
        (NParsedError::ExpectedCommandName, "ExpectedCommandName"),
        (NParsedError::InvalidCommandName, "InvalidCommandName"),
        (
            NParsedError::ExpectedLeftParenthesis,
            "ExpectedLeftParenthesis",
        ),
        (NParsedError::UnclosedArguments(span), "UnclosedArguments"),
        (
            NParsedError::LexError(LexError::ReachTheEof),
            "LexError(ReachTheEof)",
        ),
        (
            NParsedError::LexError(LexError::UnclosedParentheses(span)),
            "LexError(UnclosedParentheses)",
        ),
        (
            NParsedError::LexError(LexError::UnclosedStringLiteral(span)),
            "LexError(UnclosedStringLiteral)",
        ),
        (
            NParsedError::LexError(LexError::UnclosedBracketArgument(span)),
            "LexError(UnclosedBracketArgument)",
        ),
    ];
    let mut expected = String::new();
    for (error, label) in errors {
        buffer.add_error(error, span);
        expected.push_str(&format!("{label} at {span:?}\n"));
    }
    assert_eq!(buffer.to_string(), expected);
    assert!(buffer.display(true).to_string().contains("\x1b["));
}

#[test]
fn source_control_characters_are_escaped_instead_of_executed() {
    let source = "message([[line\n\x1b[31m]])";
    let buffer = NParser::new(Lexer::new(RawBuffer::new(source)).parse()).parse();
    let plain = buffer.display(false).to_string();
    assert!(!plain.contains('\x1b'));
    assert!(plain.contains("line\\n"));
    assert_eq!(plain.lines().count(), 2);
}

#[test]
fn formatter_errors_are_propagated() {
    struct Reject;
    impl fmt::Write for Reject {
        fn write_str(&mut self, _: &str) -> fmt::Result {
            Err(fmt::Error)
        }
    }
    let buffer = NParser::new(Lexer::new(RawBuffer::new("message(x)")).parse()).parse();
    for colored in [false, true] {
        assert!(fmt::write(&mut Reject, format_args!("{}", buffer.display(colored))).is_err());
    }
}
