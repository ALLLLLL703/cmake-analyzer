// AI-authored startup arguments.
use clap::{ArgGroup, Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(
    name = "cmake-analyzer",
    version,
    about = "Incremental CMake language server"
)]
#[command(group(ArgGroup::new("transport").args(["stdio","unix_socket"]).multiple(false)))]
pub struct Args {
    #[command(subcommand)]
    pub command: Option<Command>,
    /// if you knew it, then you knew it
    #[arg(long)]
    pub stdio: bool,
    /// Listen for one client on this Unix-domain socket
    #[arg(long, alias = "unix-domain-socket", value_name = "PATH")]
    pub unix_socket: Option<PathBuf>,
    /// Explicit project directory or CMakeLists.txt;
    #[arg(long = "root", value_name = "PATH")]
    pub roots: Vec<PathBuf>,
    /// the config path
    #[arg(long, global = true, value_name = "TOML")]
    pub config: Option<PathBuf>,
}
#[derive(Subcommand, Debug)]
pub enum Command {
    /// Format resolved project-local CMake dependencies without executing project code
    Format {
        #[arg(default_value = ".", value_name = "PROJECT")]
        project: PathBuf,
        /// Report files needing formatting without writing them
        #[arg(long)]
        check: bool,
    },
}
