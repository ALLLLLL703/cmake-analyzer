//! Clap choices for the command syntax inspector. Construction records input and display
//! options only; the sibling I/O and main modules own reads and output.
//!
//! Contains AI-generated documentation additions.
use clap::Parser;
use std::path::PathBuf;

// Pin clap descriptions separately so Rustdoc does not change runtime help.
#[derive(Parser, Debug)]
#[command(version, about = "Inspect CMake command parser output", long_about = None)]
/// Input and display choices for the command syntax executable. The sibling
/// [`super::io`] consumes the input; parsing options alone performs no source I/O.
///
/// # Examples
/// ```text
/// use clap::Parser;
/// let args = Args::parse_from(["nparser", "--text", "set(A 1)"]);
/// ```
pub(crate) struct Args {
    /// Input file; use '-' or omit it to read stdin.
    /// Optional UTF-8 input file; omission reads stdin. A literal `-` also reads stdin. Example: `args.file = Some("CMakeLists.txt".into())`.
    #[arg(help = "Input file; use '-' or omit it to read stdin", long_help = None)]
    pub file: Option<PathBuf>,
    /// Parse this text instead of reading a file or stdin.
    #[arg(long, conflicts_with = "file", help = "Parse this text instead of reading a file or stdin", long_help = None)]
    /// Inline source taking precedence over I/O; clap rejects combining it with file. Example: `args.text = Some("set(A 1)".into())`.
    pub text: Option<String>,
    /// Enable ANSI colors (disabled by default).
    #[arg(long, help = "Enable ANSI colors (disabled by default)", long_help = None)]
    /// Explicit colored display; false by default, without terminal auto-detection. Example: `args.color = true`.
    pub color: bool,
}
