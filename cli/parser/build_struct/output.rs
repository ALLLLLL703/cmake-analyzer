//! Blocking build declaration inspection and buffered caller-supplied output.
//! No source files are rewritten; output errors propagate to the executable.
//!
//! Contains AI-generated documentation additions.
// AI-generated implementation.
use cmake_analyzer::parser::{
    build_struct::model::{AnalysisKey, BuildStructAnalyzer},
    lexer::model::{Lexer, RawBuffer},
    nparser::model::NParser,
    semanticer::model::{FileId, Semanticer},
};
use std::{
    collections::HashMap,
    io::{self, Write},
};

/// Analyze one source through lexical, command, semantic and build-fact stages, then display
/// facts with semantic diagnostics/warnings. Version detection may query CMake; no dependencies
/// are loaded and no project code is executed.
///
/// # Arguments
/// - `source` `[in]`: complete UTF-8 text, borrowed for this inspection.
/// - `colored` `[in]`: enable explicit terminal color escapes.
/// - `runtime` `[in]`: live Tokio handle used from the blocking worker for semantic version discovery.
/// - `output` `[in,out]`: consumed writer handle; formatted bytes are appended at its current position and it is flushed. Existing content is not cleared.
///
/// # Returns
/// `Ok(true)` for semantic or build diagnostics, `Ok(false)` otherwise; warnings alone do not fail. `Err` for output failure.
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
///                              [build facts + diagnostic display]
///                                      |
///                              [writer + flush] --error--> [Err]
///                                      |
///                            [diagnostic failure flag]
/// ```
///
/// # Examples
/// ```text
/// // Call on a blocking worker with a live runtime:
/// let errors = inspect("cmake_minimum_required(VERSION 3.20)", false, runtime, Vec::new())?;
/// ```
pub(crate) fn inspect(
    source: &str,
    colored: bool,
    runtime: tokio::runtime::Handle,
    mut output: impl Write,
) -> io::Result<bool> {
    let parsed = NParser::new(Lexer::new(RawBuffer::new(source)).parse()).parse(source);
    let mut analyzer = Semanticer::new(parsed, HashMap::new());
    // This runs on a worker; version detection still uses the live Tokio runtime.
    let semantics = runtime.block_on(analyzer.parse(source, FileId::default()));
    let build = BuildStructAnalyzer::new(
        &semantics,
        AnalysisKey {
            file: semantics.id.id,
            revision: 0,
        },
    )
    .analyze();
    write!(
        output,
        "{}",
        build
            .display(colored)
            .with_semantic_diagnostics(&semantics.diagnostics)
            .with_semantic_warnings(&semantics.warnings)
    )?;
    output.flush()?;
    Ok(!semantics.diagnostics.is_empty() || !build.diagnostics.is_empty())
}
