//! Assemble CLI settings, roots and the selected execution path.
//! [`run`] validates startup sources before serving LSP, or delegates project disk formatting.
//! This layer does not itself execute CMake project code; server runtime actions can do so later.
//!
//! Contains AI-generated documentation additions.
// AI-authored CLI assembly.
mod args;
mod format_project;
mod transport;
pub use crate::config::{PROJECT_CONFIG, USER_CONFIG};
use crate::server::{ServerOptions, roots};
pub use args::{Args, Command};
use lspally::server::ExitStatus;
use std::io;

/// Start one LSP session or format a project. Reads cwd and configuration, resolves roots,
/// and delegates transport ownership or filesystem writes; no project configure is started here.
///
/// # Arguments
/// - `args` `[in]`: parsed options, consumed; format rejects transport/root options.
///
/// # Returns
/// `Ok(status)` reports session termination or check-mode differences; `Err` reports startup or delegated I/O failure.
///
/// # Errors
/// Unavailable cwd, invalid format/transport combinations, invalid configuration/roots,
/// or formatter/transport failures. Earlier file replacements may persist on formatter I/O failure.
///
/// # Algorithm
/// Select format first. Otherwise load global configuration, normalize explicit roots, and hand ownership to the transport.
///
/// # Flow
/// ```text
///            [Parsed args]
///                  |
///            < format? >
///             /        \
///           yes        no
///            |          |
///    [project reads] [config + roots]
///            |          |
///    [check or write] [one LSP client]
///             \        /
///              [status]
/// ```
///
/// # Examples
/// ```no_run
/// # use cmake_analyzer::cli::{run, Args};
/// # use clap::Parser;
/// # async fn example() -> std::io::Result<()> {
/// // Requires a readable project; check mode does not rewrite files.
/// let status = run(Args::parse_from(["cmake-analyzer", "format", "/path/to/project", "--check"])).await?;
/// # Ok(())
/// # }
/// ```
pub async fn run(args: Args) -> io::Result<ExitStatus> {
    let start_directory = std::env::current_dir()?;
    if let Some(args::Command::Format { project, check }) = args.command {
        if args.stdio || args.unix_socket.is_some() || !args.roots.is_empty() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "format cannot be combined with LSP transport or --root options",
            ));
        }
        return format_project::run(project, check, start_directory, args.config).await;
    }
    let mut options = ServerOptions::new(start_directory);
    options.config.explicit_file = args.config;
    crate::config::ConfigStore::load(options.config.clone()).await?;
    for root in args.roots {
        options.roots.push(roots::explicit_root(&root).await?);
    }
    transport::run(options, args.unix_socket).await
}
