//! Blocking command syntax inspection and buffered caller-supplied output.
//! No source files are rewritten; output errors propagate to the executable.
//!
//! Contains AI-generated documentation additions.
use cmake_analyzer::parser::{
    lexer::model::{Lexer, RawBuffer},
    nparser::model::{CommandState, NParsedNode, NParser},
};
use std::io::{self, Write};

/// Lex and parse commands, write their borrowed display and flush. No filesystem reads
/// or process launch; only the supplied writer is affected.
///
/// # Arguments
/// - `source` `[in]`: complete UTF-8 text, borrowed for this inspection.
/// - `colored` `[in]`: enable explicit terminal color escapes.
/// - `output` `[in,out]`: consumed writer handle; formatted bytes are appended at its current position and it is flushed. Existing content is not cleared.
///
/// # Returns
/// `Ok(true)` for error nodes or unclosed commands; `Ok(false)` otherwise; `Err` for write/flush failure.
///
/// # Errors
/// Writer formatting/flush errors; partial output may already have been written.
///
/// # Algorithm
/// Lex and parse, record whether nodes contain errors/unclosed commands, then write
/// the borrowed display and flush before returning the recorded flag.
///
/// # Flow
/// ```text
///  [input snapshot/text] ---> [display writer]
///            |                      |
///     [diagnostic scan]         [flush output]
///            |                      |
///            +-------> [bool or I/O error]
/// ```
///
/// # Examples
/// ```text
/// let errors = inspect("set(A 1)", false, Vec::new())?;
/// ```
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
