use cmake_analyzer::{
    model::Spanned,
    parser::{
        lexer::model::{LexContent, Lexer, RawBuffer},
        nparser::model::{
            NParsedArgument, NParsedBuffer, NParsedCommand, NParsedError, NParsedNode, NParser,
        },
    },
};

#[test]
fn command_and_argument_models_refer_to_source_without_owning_strings() {
    let source = "call(\"中\" [=[\r\nraw]=])";
    let lexed = Lexer::new(RawBuffer::new(source)).parse();
    let mut command = NParsedCommand::new(lexed.lex[0].clone());
    command.add_argument(NParsedArgument::Quoted, lexed.lex[2].span);
    command.add_argument(NParsedArgument::Bracked, lexed.lex[3].span);
    assert_eq!(command.name.content, LexContent::Identifier);
    assert_eq!(command.name.text(source), Some("call"));
    assert_eq!(command.args[0].text(source), Some("中"));
    assert_eq!(command.args[1].text(source), Some("raw"));
    assert_eq!(command.args[0].span.text(source), Some("\"中\""));
    assert_eq!(
        command.args[0].text(source).unwrap().as_ptr(),
        source[6..].as_ptr()
    );
    let argument = Spanned {
        content: NParsedArgument::Unquoted,
        span: command.name.span,
    };
    assert_eq!(argument.text(source), Some("call"));
    let mut parsed = NParsedBuffer::default();
    parsed.add_node(NParsedNode::Command(command), lexed.lex[0].span);
    assert_eq!(parsed.nodes.len(), 1);
}

#[test]
fn unexpected_string_error_keeps_its_text_via_the_outer_span() {
    let source = "\"unexpected\"";
    let lexed = Lexer::new(RawBuffer::new(source)).parse();
    let node = Spanned {
        content: NParsedNode::Error(NParsedError::UnArgStringLiteral),
        span: lexed.lex[0].span,
    };
    assert_eq!(node.span.text(source), Some(source));
}

#[test]
fn inherited_lexical_errors_keep_their_existing_spans() {
    let parser = NParser::new(Lexer::new(RawBuffer::new("[=[unfinished")).parse());
    let mut result = NParsedBuffer::default();
    parser.inherit_error(&mut result);
    assert_eq!(result.nodes.len(), 1);
    assert_eq!(result.nodes[0].span, parser.buffer.errors[0].span);
    assert!(matches!(
        result.nodes[0].content,
        NParsedNode::Error(NParsedError::LexError(_))
    ));
}
