use cmake_analyzer::parser::{
    lexer::model::{Lexer, RawBuffer},
    nparser::model::{CommandState, NParsedNode, NParser},
};
use std::io::{self, Write};

pub(crate) fn inspect(source: &str, colored: bool, mut output: impl Write) -> io::Result<bool> {
    let lexed = Lexer::new(RawBuffer::new(source)).parse();
    let parsed = NParser::new(lexed).parse(source);
    let has_errors = parsed.nodes.iter().any(|node| match &node.content {
        NParsedNode::Error(_) => true,
        NParsedNode::Command(command) => matches!(command.closed, CommandState::Unclosed { .. }),
    });
    write!(output, "{}", parsed.display(colored, source))?;
    output.flush()?;
    Ok(has_errors)
}
