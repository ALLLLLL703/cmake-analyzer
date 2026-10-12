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
async fn main() -> ExitCode {
    match run(args::Args::parse()).await {
        Ok(false) => ExitCode::SUCCESS,
        Ok(true) => ExitCode::from(1),
        Err(error) if error.kind() == ErrorKind::BrokenPipe => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("semanticer: {error}");
            ExitCode::from(2)
        }
    }
}
async fn run(args: args::Args) -> std::io::Result<bool> {
    let colored = args.color;
    let all_variables = args.all_variables;
    let source = io::read_source(args).await?;
    let runtime = tokio::runtime::Handle::current();
    tokio::task::spawn_blocking(move || {
        let stdout = std::io::stdout();
        output::inspect(
            &source,
            colored,
            all_variables,
            runtime,
            BufWriter::new(stdout.lock()),
        )
    })
    .await
    .map_err(std::io::Error::other)?
}
