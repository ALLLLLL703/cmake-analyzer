// AI-authored CLI assembly.
mod args;
mod transport;
pub use crate::config::{PROJECT_CONFIG, USER_CONFIG};
use crate::server::{ServerOptions, roots};
pub use args::Args;
use lspally::server::ExitStatus;
use std::io;

pub async fn run(args: Args) -> io::Result<ExitStatus> {
    let start_directory = std::env::current_dir()?;
    let mut options = ServerOptions::new(start_directory);
    options.config.explicit_file = args.config;
    crate::config::ConfigStore::load(options.config.clone()).await?;
    for root in args.roots {
        options.roots.push(roots::explicit_root(&root).await?);
    }
    transport::run(options, args.unix_socket).await
}
