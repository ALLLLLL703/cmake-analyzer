//! In-memory CMake formatting from matching source, lexical and semantic snapshots.
//! [`render`] never reads/writes files or starts processes; callers own snapshot creation
//! and decide whether to send editor edits or perform disk replacement.
//!
//! Contains AI-generated documentation additions.
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
/// Render new source from one matching source/analysis snapshot, preserving argument/comment
/// lexemes. Reject syntax/structural/version errors, but permit unknown command names.
/// No source mutation, filesystem access or process execution; stop callbacks are caller-owned.
///
/// # Arguments
/// - `source` `[in]`: original UTF-8 text matching all supplied ranges.
/// - `lexed` `[in]`: lexical tokens/comments for source, borrowed unchanged.
/// - `semantics` `[in]`: semantic command/block data for the same source, borrowed unchanged.
/// - `options` `[in]`: desired indentation/width policy; validated before rendering.
/// - `stop` `[in]`: synchronous cancellation predicate called at checkpoints; true rejects output.
///
/// # Returns
/// `Ok(text)` contains a complete replacement (possibly identical/empty), with one terminal newline
/// when nonempty; `Err` returns no partial output.
///
/// # Errors
/// InvalidOptions for zero widths or indent_width >255; Cancelled for stop requests;
/// InvalidSyntax for lexical errors, unclosed commands, semantic errors other than CommandNotFound,
/// or unsupported/inconsistent ranges encountered during layout.
///
/// # Panics
/// Inputs must describe the same source and have valid UTF-8 byte boundaries. Inconsistent
/// comment/command ranges may panic on slicing rather than produce InvalidSyntax. Callback panics propagate.
///
/// # Algorithm
/// Validate options and cancellation, reject unsupported analysis, then lay out commands,
/// comments and whitespace with block indentation and width-based vertical argument layout.
///
/// # Flow
/// ```text
///  [source + matching buffers]     [options/stop]
///               \                     /
///                   [validate inputs]
///                        /       \
///                     reject     accept
///                       |          |
///                     [Err]  [layout commands + trivia]
///                                  |
///                           [finish new String]
/// ```
///
/// # Examples
/// ```
/// # use cmake_analyzer::{format::{render, Options}, parser::{lexer::model::{Lexer, RawBuffer}, nparser::model::NParser, semanticer::model::{Semanticer, FileId}}};
/// # use std::collections::HashMap;
/// # tokio::runtime::Runtime::new().unwrap().block_on(async {
/// // Explicit version keeps this example offline and avoids CMake probing.
/// let source = "cmake_minimum_required(VERSION 3.20)\nset( A 1 )\n";
/// let lexed = Lexer::new(RawBuffer::new(source)).parse();
/// let parsed = NParser::new(lexed.clone()).parse(source);
/// let mut analyzer = Semanticer::new(parsed, HashMap::new());
/// let semantics = analyzer.parse(source, FileId::default()).await;
/// let text = render(source, &lexed, &semantics, Options::default(), || false).unwrap();
/// assert!(text.ends_with("set(A 1)\n"));
/// # });
/// ```
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
