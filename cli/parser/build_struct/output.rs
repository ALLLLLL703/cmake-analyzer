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
    )?;
    output.flush()?;
    Ok(!semantics.diagnostics.is_empty() || !build.diagnostics.is_empty())
}
