use crate::args::Args;
use cmake_analyzer::parser::semanticer::Semanticer;
use std::{
    io::{self, Write},
    path::Path,
};
use tokio::io::AsyncReadExt;

pub async fn read_source(args: Args) -> io::Result<String> {
    if let Some(text) = args.text {
        return Ok(text);
    }
    if let Some(path) = args.file {
        if path != Path::new("-") {
            return tokio::fs::read_to_string(path).await;
        }
    }
    let mut source = String::new();
    tokio::io::stdin().read_to_string(&mut source).await?;
    Ok(source)
}

pub async fn inspect(source: String, colored: bool) -> io::Result<bool> {
    tokio::task::spawn_blocking(move || {
        let buffer = Semanticer::analyze(&source);
        let has_errors = buffer.has_errors();
        let stdout = io::stdout();
        let mut output = io::BufWriter::new(stdout.lock());
        write!(output, "{}", buffer.display(colored))?;
        output.flush()?;
        Ok(has_errors)
    })
    .await
    .map_err(io::Error::other)?
}
