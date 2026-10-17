//! Clap choices for the token inspector. Construction records input and display
//! options only; the sibling I/O and main modules own reads and output.
//!
//! Contains AI-generated documentation additions.
use std::path::PathBuf;

use clap::Parser;
// Pin clap descriptions separately so Rustdoc does not change runtime help.
#[derive(Parser, Debug)]
#[command(version = "0.1.0", about = "Inspect cmake lexer output", long_about = None)]
/// Input and display choices for the token executable. The sibling
/// [`super::io`] consumes the input; parsing options alone performs no source I/O.
///
/// # Examples
/// ```text
/// use clap::Parser;
/// let args = Args::parse_from(["lexer", "--text", "set(A 1)"]);
/// ```
pub struct Args {
    /// Optional UTF-8 input file; omission reads stdin. A literal `-` is a file name here, not a stdin alias. Example: `args.file = Some("CMakeLists.txt".into())`.
    #[arg(help = None, long_help = None)]
    pub file: Option<PathBuf>,

    #[arg(long, conflicts_with = "file", help = None, long_help = None)]
    /// Inline source taking precedence over I/O; clap rejects combining it with file. Example: `args.text = Some("set(A 1)".into())`.
    pub text: Option<String>,

    #[arg(long, help = None, long_help = None)]
    /// Explicit colored display; false by default, without terminal auto-detection. Example: `args.color = true`.
    pub color: bool,
}
