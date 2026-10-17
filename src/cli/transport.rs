//! Own one stdio or Unix-socket LSP session and join its analyzer service at completion.
//! Unix startup creates one socket path and removes it after normal async completion;
//! forced process termination is not a cleanup guarantee.
//!
//! Contains AI-generated documentation additions.
// AI-authored transport startup. Each process serves one LSP client with multiple roots.
use crate::server::{CmakeAnalyzer, ServerOptions};
use lspally::server::{ExitStatus, Wrapper};
use std::{io, path::PathBuf};

/// Serve one client and await analyzer shutdown even when the wrapper returns an error.
/// Uses protocol stdin/stdout unless a socket is selected.
///
/// # Arguments
/// - `options` `[in]`: server roots/configuration moved into the analyzer.
/// - `socket` `[in]`: optional bind path; no pre-existing path is removed.
///
/// # Returns
/// `Ok(status)` is the wrapper exit status after service stop; `Err` is transport or stop failure (wrapper errors take priority).
///
/// # Errors
/// Binding, stream I/O or analyzer shutdown may fail; socket mode also reports cleanup failures.
///
/// # Algorithm
/// Route socket mode to its platform implementation. For stdio, retain an analyzer
/// clone while the wrapper owns the session; always await service.stop after the
/// wrapper resolves. Return wrapper errors before service-stop errors.
///
/// # Flow
/// ```text
///  [options + socket choice]
///              |
///         <socket set?>
///          /        \
///        yes        no
///         |          |
///  [unix session] [stdio wrapper]
///         |          |
///         |      [service stop]
///         \          /
///          [status / I/O error]
/// ```
///
/// # Examples
/// ```text
/// let status = run(options, None).await?;
/// ```
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
/// Bind a Unix socket, accept exactly one client, serve it, join the service and remove the bound path.
/// Does not unlink a pre-existing file or retry clients.
///
/// # Arguments
/// - `options` `[in]`: configuration moved into the accepted client service.
/// - `path` `[in]`: socket filesystem path, removed only after successful bind.
///
/// # Returns
/// `Ok(status)` after service and path cleanup; `Err` for bind/accept/session/stop/cleanup failure.
/// Session errors take priority over cleanup errors.
///
/// # Errors
/// OS socket, stream, service-stop and remove-file errors. Abrupt termination can leave the socket behind.
///
/// # Algorithm
/// Bind before entering cleanup-managed work. Accept once; split the stream and stop the analyzer after the wrapper. Drop listener, then remove the bound path.
///
/// # Flow
/// ```text
///  [bind path] --failure--> [return; never unlink]
///       |
///    [accept]
///       |
///  [wrapper session] --> [service stop]
///                              |
///  [return result] <--- [drop listener + unlink]
/// ```
///
/// # Examples
/// ```text
/// let status = unix(options, "/tmp/analyzer.sock".into()).await?;
/// ```
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
/// Reject Unix sockets on non-Unix platforms without I/O.
///
/// # Arguments
/// - `_options` `[in]`: unused server settings.
/// - `_path` `[in]`: unused requested socket path.
///
/// # Returns
/// Always `Err` with `Unsupported`; no session starts.
///
/// # Errors
/// Always reports that Unix-domain sockets require Unix.
///
/// # Examples
/// ```text
/// assert_eq!(unix(options, "socket".into()).await.unwrap_err().kind(), std::io::ErrorKind::Unsupported);
/// ```
async fn unix(_options: ServerOptions, _path: PathBuf) -> io::Result<ExitStatus> {
    Err(io::Error::new(
        io::ErrorKind::Unsupported,
        "Unix-domain sockets require a Unix platform",
    ))
}
