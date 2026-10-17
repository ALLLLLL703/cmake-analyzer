//! Blocking semantic inspection and buffered caller-supplied output.
//! No source files are rewritten; output errors propagate to the executable.
//!
//! Contains AI-generated documentation additions.
// AI-generated implementation.
use cmake_analyzer::parser::{
    lexer::model::{Lexer, RawBuffer},
    nparser::model::NParser,
    semanticer::model::{FileId, Semanticer},
};
use std::{
    collections::HashMap,
    io::{self, Write},
};

/// Lex, parse and analyze one source, write its semantic display and flush. Semantic version
/// detection may run `cmake --version`; no project code is executed or dependency files loaded.
///
/// # Arguments
/// - `source` `[in]`: complete UTF-8 text, borrowed for this inspection.
/// - `colored` `[in]`: enable explicit terminal color escapes.
/// - `all_variables` `[in]`: include unchanged builtin variable records only in display.
/// - `runtime` `[in]`: live Tokio handle used from the blocking worker for semantic version discovery.
/// - `output` `[in,out]`: consumed writer handle; formatted bytes are appended at its current position and it is flushed. Existing content is not cleared.
///
/// # Returns
/// `Ok(true)` if semantic diagnostics exist, `Ok(false)` otherwise (warnings alone are clean); `Err` for output failure.
///
/// # Errors
/// Writer formatting/flush errors; partial output may already have been written.
///
/// # Panics
/// The Tokio handle must be used on a blocking worker, not inside an async runtime task; `block_on` panics in that context.
///
/// # Algorithm
/// Analyze/render first, flush output, then compute the failure flag from retained diagnostics.
///
/// # Flow
/// ```text
/// [blocking worker: source] ---> [lexer + command parser]
///                                      |
///                                [semantic analysis]
///                                  /         \
///                        [declared version] [runtime handle: version probe]
///                                  \         /
///                              [semantic display + all_variables]
///                                      |
///                              [writer + flush] --error--> [Err]
///                                      |
///                            [diagnostic failure flag]
/// ```
///
/// # Examples
/// ```text
/// // Call on a blocking worker with a live runtime:
/// let errors = inspect("cmake_minimum_required(VERSION 3.20)", false, false, runtime, Vec::new())?;
/// ```
pub(crate) fn inspect(
    source: &str,
    colored: bool,
    all_variables: bool,
    runtime: tokio::runtime::Handle,
    mut output: impl Write,
) -> io::Result<bool> {
    let parsed = NParser::new(Lexer::new(RawBuffer::new(source)).parse()).parse(source);
    let mut analyzer = Semanticer::new(parsed, HashMap::new());
    let buffer = runtime.block_on(analyzer.parse(source, FileId::default()));
    write!(
        output,
        "{}",
        buffer.display(colored).with_all_variables(all_variables)
    )?;
    output.flush()?;
    Ok(!buffer.diagnostics.is_empty())
}
