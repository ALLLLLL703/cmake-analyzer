// AI-generated implementation.
use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(
    version,
    about = "Inspect CMake semantic symbols, blocks, scopes, writes, and diagnostics"
)]
pub(crate) struct Args {
    #[arg(help = "Input file; use '-' or omit it to read stdin")]
    pub file: Option<PathBuf>,
    #[arg(
        long,
        conflicts_with = "file",
        help = "Analyze this text instead of reading a file or stdin"
    )]
    pub text: Option<String>,
    #[arg(long, help = "Enable rich ANSI colors (disabled by default)")]
    pub color: bool,
    #[arg(long, help = "Include unchanged builtin variable records")]
    pub all_variables: bool,
}
