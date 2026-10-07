use std::io::Write;
pub mod args;
pub mod io;

use clap::{Arg, Parser};
use cmake_analyzer::parser::lexer::model::{Lexer, RawBuffer};

use crate::io::read_source;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = args::Args::parse();
    let colored = args.color;

    let source = read_source(args).await?;
    tokio::task::spawn_blocking(move || -> std::io::Result<()> {
        let mut lexer = Lexer::new(RawBuffer::new(&source));
        let buffer = lexer.parse();
        let stdout = std::io::stdout();
        let mut output = std::io::BufWriter::new(stdout.lock());
        write!(output, "{}", buffer.display(colored, &source))?;

        output.flush()?;

        Ok(())
    })
    .await??;

    Ok(())
}
