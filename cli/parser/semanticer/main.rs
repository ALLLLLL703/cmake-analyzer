mod args;
mod nio;

use clap::Parser;
use std::{io, process::ExitCode};

#[tokio::main(flavor = "current_thread")]
async fn main() -> ExitCode {
    let args = args::Args::parse();
    let colored = args.color;
    let result = async {
        let source = nio::read_source(args).await?;
        nio::inspect(source, colored).await
    }
    .await;
    match result {
        Ok(false) => ExitCode::SUCCESS,
        Ok(true) => ExitCode::from(1),
        Err(error) if error.kind() == io::ErrorKind::BrokenPipe => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("semanticer: {error}");
            ExitCode::from(2)
        }
    }
}
