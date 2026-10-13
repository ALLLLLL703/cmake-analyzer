// AI-generated implementation.
use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(
    version,
    about = "Draw a CMake workspace dependency tree (no configure)"
)]
pub(crate) struct Args {
    #[arg(help = "Root file; '-' or omitted reads stdin with cwd/CMakeLists.txt as its path")]
    pub file: Option<PathBuf>,
    #[arg(
        long,
        conflicts_with = "file",
        help = "Use this root text at cwd/CMakeLists.txt"
    )]
    pub text: Option<String>,
    #[arg(long, help = "Enable owo-colors terminal colors")]
    pub color: bool,
    #[arg(long)]
    pub binary_directory: Option<PathBuf>,
    #[arg(long, help = "Extra include module directory; may be repeated")]
    pub module_path: Vec<PathBuf>,
    #[arg(long)]
    pub builtin_module_directory: Option<PathBuf>,
}
