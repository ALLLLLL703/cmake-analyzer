//! Executable assembly for the build declaration inspector. Async input is separated from
//! blocking analysis/display and buffered stdout writing.
//!
//! Contains AI-generated documentation additions.
// AI-generated implementation.
mod args;
mod io;
mod output;

use clap::Parser;
use std::{
    io::{BufWriter, ErrorKind},
    process::ExitCode,
};

#[tokio::main(flavor = "current_thread")]
/// Parse command-line options and map inspection results to process status.
/// Prints non-broken-pipe failures to stderr; the worker writes the inspection to stdout.
///
/// # Arguments
/// None.
///
/// # Returns
/// Exit 0 for no diagnostics or broken pipe, 1 for analysis diagnostics, 2 for other I/O/worker failures. Clap handles argument errors separately.
///
/// # Examples
/// ```text
/// // Call from outside an existing Tokio runtime; the entrypoint reads process args.
/// let status = main();
/// ```
async fn main() -> ExitCode {
    match run(args::Args::parse()).await {
        Ok(false) => ExitCode::SUCCESS,
        Ok(true) => ExitCode::from(1),
        Err(error) if error.kind() == ErrorKind::BrokenPipe => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("build-struct: {error}");
            ExitCode::from(2)
        }
    }
}
/// Read source asynchronously, then join a blocking worker that renders to locked buffered stdout. Semantic analysis may query `cmake --version`, but never executes project commands.
///
/// # Arguments
/// - `args` `[in]`: consumed source/display options.
///
/// # Returns
/// `Ok(true)` means analysis diagnostics, `Ok(false)` means clean display, and `Err` means I/O or worker failure.
///
/// # Errors
/// Source read/UTF-8, stdout write/flush, or blocking-worker join failure.
///
/// # Algorithm
/// Read source before moving owned inputs into spawn_blocking; the worker owns stdout lock until flushed. Await its result before returning.
///
/// # Flow
/// ```text
///  [CLI options] ---> [async input]
///                          |
///                     [owned input]
///                          |
///  [executor waits] <---- [blocking worker]
///         |                |
///         |          [analysis + stdout flush]
///         v                |
///      [result] <----------+
/// ```
///
/// # Examples
/// ```text
/// let errors = run(args::Args::parse_from(["build-struct", "--text", "set(A 1)"])).await?;
/// ```
async fn run(args: args::Args) -> std::io::Result<bool> {
    let colored = args.color;
    let source = io::read_source(args).await?;
    let runtime = tokio::runtime::Handle::current();
    // Input is async; parsing and buffered terminal output run off the executor.
    tokio::task::spawn_blocking(move || {
        let stdout = std::io::stdout();
        output::inspect(&source, colored, runtime, BufWriter::new(stdout.lock()))
    })
    .await
    .map_err(std::io::Error::other)?
}
