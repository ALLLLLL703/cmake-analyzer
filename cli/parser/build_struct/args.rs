//! Clap choices for the build declaration inspector. Construction records input and display
//! options only; the sibling I/O and main modules own reads and output.
//!
//! Contains AI-generated documentation additions.
// AI-generated implementation.
use clap::Parser;
use std::path::PathBuf;

// Pin clap descriptions separately so Rustdoc does not change runtime help.
#[derive(Parser, Debug)]
#[command(
    version,
    about = "Inspect CMake build declarations in one file (no dependency loading)",
    long_about = None
)]
/// Input and display choices for the build declaration executable. The sibling
/// [`super::io`] consumes the input; parsing options alone performs no source I/O.
///
/// # Examples
/// ```text
/// use clap::Parser;
/// let args = Args::parse_from(["build-struct", "--text", "set(A 1)"]);
/// ```
pub(crate) struct Args {
    #[arg(help = "Input file; use '-' or omit it to read stdin", long_help = None)]
    /// Optional UTF-8 input file; omission reads stdin. A literal `-` also reads stdin. Example: `args.file = Some("CMakeLists.txt".into())`.
    pub file: Option<PathBuf>,
    #[arg(
        long,
        conflicts_with = "file",
        help = "Analyze this text instead of a file or stdin",
        long_help = None
    )]
    /// Inline source taking precedence over I/O; clap rejects combining it with file. Example: `args.text = Some("set(A 1)".into())`.
    pub text: Option<String>,
    #[arg(long, help = "Enable rich colors (disabled by default)", long_help = None)]
    /// Explicit colored display; false by default, without terminal auto-detection. Example: `args.color = true`.
    pub color: bool,
}
