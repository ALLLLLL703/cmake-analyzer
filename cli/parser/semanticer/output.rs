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
