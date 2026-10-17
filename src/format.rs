mod layout;
mod model;
mod trivia;
mod writer;
use crate::parser::{
    lexer::model::LexedBuffer,
    semanticer::model::{SemanticError, SemanticedBuffer},
};
pub use model::{Error, Options};

/// Format semantic command/block structure while copying argument and comment lexemes verbatim.
/// Syntactically invalid buffers are rejected; unknown command names do not prevent formatting.
pub fn render(
    source: &str,
    lexed: &LexedBuffer,
    semantics: &SemanticedBuffer,
    options: Options,
    stop: impl Fn() -> bool,
) -> Result<String, Error> {
    if options.indent_width == 0
        || options.line_width == 0
        || u8::try_from(options.indent_width).is_err()
    {
        return Err(Error::InvalidOptions);
    }
    if stop() {
        return Err(Error::Cancelled);
    }
    if !lexed.errors.is_empty()
        || semantics.commands.iter().any(|command| !command.closed)
        || semantics
            .diagnostics
            .iter()
            .any(|error| !matches!(error.content, SemanticError::CommandNotFound(_)))
    {
        return Err(Error::InvalidSyntax);
    }
    layout::render(source, lexed, semantics, options, &stop)
}
