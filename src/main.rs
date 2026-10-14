// AI-authored executable assembly; stdout belongs exclusively to LSP.
use clap::Parser;
use cmake_analyzer::cli::{self, Args};
use lspally::server::ExitStatus;

#[tokio::main]
async fn main() -> std::process::ExitCode {
    match cli::run(Args::parse()).await {
        Ok(ExitStatus::Graceful | ExitStatus::Disconnected) => std::process::ExitCode::SUCCESS,
        Ok(ExitStatus::Abnormal) => std::process::ExitCode::FAILURE,
        Err(error) => {
            eprintln!("cmake-analyzer: {error}");
            std::process::ExitCode::FAILURE
        }
    }
}
