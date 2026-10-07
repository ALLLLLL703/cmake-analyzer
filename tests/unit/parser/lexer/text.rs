use cmake_analyzer::{
    model::{Spanned, TextSpan},
    parser::lexer::model::{LexContent, Lexer, RawBuffer},
};
use std::fmt;

#[test]
fn payload_spans_locate_unwrapped_unicode_and_initial_newlines() {
    let source = "前 \"中\" [=[\r\n🦀]=]";
    let buffer = Lexer::new(RawBuffer::new(source)).parse();
    let quote = &buffer.lex[1];
    assert_eq!(quote.text(source), Some("中"));
    assert_eq!(quote.payload_span(source).unwrap().column, 3);
    let bracket = &buffer.lex[2];
    let payload = bracket.payload_span(source).unwrap();
    assert_eq!((payload.row, payload.column), (1, 0));
    assert_eq!(payload.text(source), Some("🦀"));
    assert_eq!(
        payload.iter_text(source).unwrap().collect::<Vec<_>>(),
        ['🦀']
    );
    assert_eq!(
        bracket.text(source).unwrap().as_ptr(),
        source[payload.start_byte..].as_ptr()
    );
}

#[test]
fn invalid_payload_tokens_do_not_panic_or_return_guessed_text() {
    for (source, kind) in [
        ("abc", LexContent::StringLiteral),
        ("\"", LexContent::StringLiteral),
        ("[[", LexContent::BracketArgument),
        ("[=[x]]", LexContent::BracketArgument),
    ] {
        let token = Spanned {
            content: kind,
            span: TextSpan {
                end_byte: source.len(),
                ..TextSpan::default()
            },
        };
        assert!(token.text(source).is_none());
    }
}

#[test]
fn display_propagates_formatter_errors() {
    struct Reject;
    impl fmt::Write for Reject {
        fn write_str(&mut self, _: &str) -> fmt::Result {
            Err(fmt::Error)
        }
    }
    let source = "foo(\"bar\")";
    let buffer = Lexer::new(RawBuffer::new(source)).parse();
    for colored in [false, true] {
        assert!(
            fmt::write(
                &mut Reject,
                format_args!("{}", buffer.display(colored, source))
            )
            .is_err()
        );
    }
}
