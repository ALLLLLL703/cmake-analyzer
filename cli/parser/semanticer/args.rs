//! Clap choices for the semantic inspector. Construction records input and display
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
    about = "Inspect CMake semantic symbols, blocks, scopes, writes, and diagnostics",
    long_about = None
)]
/// Input and display choices for the semantic executable. The sibling
/// [`super::io`] consumes the input; parsing options alone performs no source I/O.
///
/// # Examples
/// ```text
/// use clap::Parser;
/// let args = Args::parse_from(["semanticer", "--text", "set(A 1)"]);
/// ```
pub(crate) struct Args {
    #[arg(help = "Input file; use '-' or omit it to read stdin", long_help = None)]
    /// Optional UTF-8 input file; omission reads stdin. A literal `-` also reads stdin. Example: `args.file = Some("CMakeLists.txt".into())`.
    pub file: Option<PathBuf>,
    #[arg(
        long,
        conflicts_with = "file",
        help = "Analyze this text instead of reading a file or stdin",
        long_help = None
    )]
    /// Inline source taking precedence over I/O; clap rejects combining it with file. Example: `args.text = Some("set(A 1)".into())`.
    pub text: Option<String>,
    #[arg(long, help = "Enable rich ANSI colors (disabled by default)", long_help = None)]
    /// Explicit colored display; false by default, without terminal auto-detection. Example: `args.color = true`.
    pub color: bool,
    #[arg(long, help = "Include unchanged builtin variable records", long_help = None)]
    /// Include unchanged builtin variable records in the display; does not change analysis. Example: `args.all_variables = true`.
    pub all_variables: bool,
}
