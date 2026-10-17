//! Clap choices for the static dependency graph inspector. Construction records input and display
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
    about = "Draw a CMake workspace dependency tree (no configure)",
    long_about = None
)]
/// Input and display choices for the static dependency graph executable. The sibling
/// [`super::io`] consumes the input; parsing options alone performs no source I/O.
///
/// # Examples
/// ```text
/// use clap::Parser;
/// let args = Args::parse_from(["dependency-graph", "--text", "set(A 1)"]);
/// ```
pub(crate) struct Args {
    #[arg(help = "Root file; '-' or omitted reads stdin with cwd/CMakeLists.txt as its path", long_help = None)]
    /// Optional UTF-8 input file; omission reads stdin. A literal `-` also reads stdin. Example: `args.file = Some("CMakeLists.txt".into())`.
    pub file: Option<PathBuf>,
    #[arg(
        long,
        conflicts_with = "file",
        help = "Use this root text at cwd/CMakeLists.txt",
        long_help = None
    )]
    /// Inline source taking precedence over I/O; clap rejects combining it with file. Example: `args.text = Some("set(A 1)".into())`.
    pub text: Option<String>,
    #[arg(long, help = "Enable owo-colors terminal colors", long_help = None)]
    /// Explicit colored display; false by default, without terminal auto-detection. Example: `args.color = true`.
    pub color: bool,
    #[arg(long, help = None, long_help = None)]
    /// Optional binary context for directory builtins; no configure/build is run. Example: `args.binary_directory = Some("build".into())`.
    pub binary_directory: Option<PathBuf>,
    #[arg(long, help = "Extra include module directory; may be repeated", long_help = None)]
    /// Additional include search paths, resolved against cwd by the loader. Example: `args.module_path.push("cmake".into())`.
    pub module_path: Vec<PathBuf>,
    #[arg(long, help = None, long_help = None)]
    /// Optional builtin module root passed to the Workspace. Example: `args.builtin_module_directory = Some("/opt/cmake/Modules".into())`.
    pub builtin_module_directory: Option<PathBuf>,
}
