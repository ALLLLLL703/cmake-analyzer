// AI-authored transport startup. Each process serves one LSP client with multiple roots.
use crate::server::{CmakeAnalyzer, ServerOptions};
use lspally::server::{ExitStatus, Wrapper};
use std::{io, path::PathBuf};

pub(super) async fn run(options: ServerOptions, socket: Option<PathBuf>) -> io::Result<ExitStatus> {
    if let Some(path) = socket {
        return unix(options, path).await;
    }
    let server = CmakeAnalyzer::new(options);
    let result = Wrapper::new(server.clone()).run_stdio().await;
    let stopped = server.stop().await.map_err(io::Error::other);
    result.and_then(|status| stopped.map(|()| status))
}
#[cfg(unix)]
async fn unix(options: ServerOptions, path: PathBuf) -> io::Result<ExitStatus> {
    let listener = tokio::net::UnixListener::bind(&path)?;
    // Never unlink a pre-existing socket/file. Only remove the path after our successful bind.
    let result = async {
        let (stream, _) = listener.accept().await?;
        let (input, output) = stream.into_split();
        let server = CmakeAnalyzer::new(options);
        let result = Wrapper::new(server.clone()).run(input, output).await;
        let stopped = server.stop().await.map_err(io::Error::other);
        result.and_then(|status| stopped.map(|()| status))
    }
    .await;
    drop(listener);
    let cleaned = tokio::fs::remove_file(path).await;
    result.and_then(|status| cleaned.map(|()| status))
}
#[cfg(not(unix))]
async fn unix(_options: ServerOptions, _path: PathBuf) -> io::Result<ExitStatus> {
    Err(io::Error::new(
        io::ErrorKind::Unsupported,
        "Unix-domain sockets require a Unix platform",
    ))
}
