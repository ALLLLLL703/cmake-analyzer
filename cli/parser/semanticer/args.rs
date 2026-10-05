use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(
    version,
    about = "Inspect CMake control blocks and static semantic facts"
)]
pub struct Args {
    /// Input file; omit or use '-' to read stdin
    pub file: Option<PathBuf>,
    /// Analyze source text directly
    #[arg(long, conflicts_with = "file")]
    pub text: Option<String>,
    /// Enable ANSI colors, including when output is redirected
    #[arg(long)]
    pub color: bool,
}
