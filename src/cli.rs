// AI-authored CLI assembly.
mod args;
mod config;
mod transport;
use crate::server::{ServerOptions, roots};
pub use args::Args;
pub use config::{PROJECT_CONFIG, USER_CONFIG};
use lspally::server::ExitStatus;
use std::io;

pub async fn run(args: Args) -> io::Result<ExitStatus> {
    if args.config.is_some() {
        return Err(io::Error::new(
            io::ErrorKind::Unsupported,
            "TOML configuration loading is not implemented yet",
        ));
    }
    let start_directory = std::env::current_dir()?;
    let mut options = ServerOptions::new(start_directory);
    for root in args.roots {
        options.roots.push(roots::explicit_root(&root).await?);
    }
    transport::run(options, args.unix_socket).await
}
