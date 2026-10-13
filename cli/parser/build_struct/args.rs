// AI-generated implementation.
use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(
    version,
    about = "Inspect CMake build declarations in one file (no dependency loading)"
)]
pub(crate) struct Args {
    #[arg(help = "Input file; use '-' or omit it to read stdin")]
    pub file: Option<PathBuf>,
    #[arg(
        long,
        conflicts_with = "file",
        help = "Analyze this text instead of a file or stdin"
    )]
    pub text: Option<String>,
    #[arg(long, help = "Enable rich colors (disabled by default)")]
    pub color: bool,
}
