// AI-authored CLI assembly.
mod args;
mod format_project;
mod transport;
pub use crate::config::{PROJECT_CONFIG, USER_CONFIG};
use crate::server::{ServerOptions, roots};
pub use args::{Args, Command};
use lspally::server::ExitStatus;
use std::io;

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
