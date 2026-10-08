use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(version, about = "Inspect CMake command parser output")]
pub(crate) struct Args {
    /// Input file; use '-' or omit it to read stdin.
    pub file: Option<PathBuf>,
    /// Parse this text instead of reading a file or stdin.
    #[arg(long, conflicts_with = "file")]
    pub text: Option<String>,
    /// Enable ANSI colors (disabled by default).
    #[arg(long)]
    pub color: bool,
}
