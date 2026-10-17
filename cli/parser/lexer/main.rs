//! Executable assembly for the token inspector. Async input is separated from
//! blocking analysis/display and buffered stdout writing.
//!
//! Contains AI-generated documentation additions.
use std::io::Write;
pub mod args;
pub mod io;

use clap::{Arg, Parser};
use cmake_analyzer::parser::lexer::model::{Lexer, RawBuffer};

use crate::io::read_source;

#[tokio::main(flavor = "current_thread")]
/// Read CLI source asynchronously, lex and display it on a joined blocking worker, and flush stdout.
/// Does not launch CMake or write source files; lexical diagnostics are displayed without selecting a failure exit.
///
/// # Arguments
/// None.
///
/// # Returns
/// `Ok(())` after flushed display, even with lexer diagnostics; `Err` propagates input/output or worker failures to Rust main.
///
/// # Errors
/// Source reading/UTF-8, stdout write/flush or worker join failure.
///
/// # Algorithm
/// Separate async reads from CPU work and stdout locking. Move source/color into the worker and await both join and I/O results.
///
/// # Flow
/// ```text
///  [args] ---> [async read]
///                  |
///  [await] <--- [worker lexes]
///     |            |
///     |       [buffered stdout]
///     |            |
///  [result] <--- [flush]
/// ```
///
/// # Examples
/// ```text
/// // Call from outside an existing Tokio runtime; the entrypoint reads process args.
/// let status = main();
/// ```
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
