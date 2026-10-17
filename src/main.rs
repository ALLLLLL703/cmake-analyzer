//! Executable assembly for the analyzer CLI. Clap selects a server transport or project
//! formatting; errors go to stderr. LSP stdout is reserved for protocol frames.
//!
//! Contains AI-generated documentation additions.
// AI-authored executable assembly; LSP transport keeps stdout protocol-only.
use clap::Parser;
use cmake_analyzer::cli::{self, Args};
use lspally::server::ExitStatus;

#[tokio::main]
/// Parse process arguments and await CLI completion, printing startup errors to stderr.
/// May serve a client or rewrite project files according to the selected command.
///
/// # Arguments
/// None.
///
/// # Returns
/// Success for graceful completion or disconnect; failure for abnormal status or I/O errors.
///
/// # Examples
/// ```text
/// // Call from outside an existing Tokio runtime; the entrypoint reads process args.
/// let status = main();
/// ```
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
