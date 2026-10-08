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
            eprintln!("nparser: {error}");
            ExitCode::from(2)
        }
    }
}

async fn run(args: args::Args) -> std::io::Result<bool> {
    let colored = args.color;
    let source = io::read_source(args).await?;
    tokio::task::spawn_blocking(move || {
        let stdout = std::io::stdout();
        output::inspect(&source, colored, BufWriter::new(stdout.lock()))
    })
    .await
    .map_err(std::io::Error::other)?
}
