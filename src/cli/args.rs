//! Clap input model for [`super::run`]. Parsing only records choices; it does not open
//! a transport, read configuration, format files or launch CMake.
//!
//! Contains AI-generated documentation additions.
// AI-authored startup arguments.
use clap::{ArgGroup, Parser, Subcommand};
use std::path::PathBuf;

// Pin clap descriptions separately so Rustdoc does not change runtime help.
#[derive(Parser, Debug)]
#[command(
    name = "cmake-analyzer",
    version,
    about = "Incremental CMake language server",
    long_about = None
)]
#[command(group(ArgGroup::new("transport").args(["stdio","unix_socket"]).multiple(false)))]
/// Startup choices consumed by [`super::run`]. [`Command`] selects disk formatting;
/// without a command, stdio is the default. Construction has no I/O effects.
///
/// # Examples
/// ```
/// # use cmake_analyzer::cli::Args;
/// # use clap::Parser;
/// let args = Args::parse_from(["cmake-analyzer", "--stdio"]);
/// assert!(args.stdio);
/// ```
pub struct Args {
    #[command(subcommand)]
    /// Optional non-server operation. Example: `args.command = None`.
    pub command: Option<Command>,
    #[arg(long, help = "if you knew it, then you knew it", long_help = None)]
    /// Explicit stdio selection; mutually exclusive with unix_socket in clap. Example: `args.stdio = true`.
    pub stdio: bool,
    /// Listen for one client on this Unix-domain socket
    #[arg(
        long,
        alias = "unix-domain-socket",
        value_name = "PATH",
        help = "Listen for one client on this Unix-domain socket",
        long_help = None
    )]
    /// One-client Unix socket path; absence selects stdio. Example: `args.unix_socket = Some("/tmp/analyzer.sock".into())`.
    pub unix_socket: Option<PathBuf>,
    /// Explicit project directory or CMakeLists.txt;
    #[arg(
        long = "root",
        value_name = "PATH",
        help = "Explicit project directory or CMakeLists.txt;",
        long_help = None
    )]
    /// Repeatable project directories or CMakeLists.txt paths; startup normalizes each. Example: `args.roots.push("project".into())`.
    pub roots: Vec<PathBuf>,
    #[arg(
        long,
        global = true,
        value_name = "TOML",
        help = "the config path",
        long_help = None
    )]
    /// Highest-priority TOML source; relative paths use startup cwd. Example: `args.config = Some("style.toml".into())`.
    pub config: Option<PathBuf>,
}
#[derive(Subcommand, Debug)]
#[command(about = None, long_about = None)]
/// Non-server CLI operations dispatched by [`super::run`]. [`Args`] contains this choice.
/// Constructing a command does not touch files.
///
/// # Examples
/// ```
/// # use cmake_analyzer::cli::Command;
/// let command = Command::Format { project: ".".into(), check: true };
/// ```
pub enum Command {
    /// Format resolved project-local CMake dependencies without executing project code
    /// Format project-local dependency visits; no configure is run. Example: `Command::Format { project: ".".into(), check: true }`.
    #[command(
        about = "Format resolved project-local CMake dependencies without executing project code",
        long_about = None
    )]
    Format {
        #[arg(default_value = ".", value_name = "PROJECT", help = None, long_help = None)]
        /// Project directory or CMakeLists.txt; defaults to current directory. Example: `Command::Format { project: "demo".into(), check: false }`.
        project: PathBuf,
        /// Report files needing formatting without writing them
        #[arg(long, help = "Report files needing formatting without writing them", long_help = None)]
        /// Print changed paths without writing; differences yield abnormal status. Example: `Command::Format { project: ".".into(), check: true }`.
        check: bool,
    },
}
