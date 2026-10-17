// AI-generated executable and socket startup tests.
//! CLI option validation, silent stdio EOF and Unix-socket framing/cleanup.
//! Tests may launch the analyzer executable or a socket task; temporary paths are removed.
use clap::Parser;
use cmake_analyzer::cli::{self, Args};
use std::process::Stdio;

#[test]
/// Check conflicting transports fail and repeated roots/config paths parse.
/// No filesystem, process or network side effects.
///
/// # Arguments
/// None.
///
/// # Returns
/// `()` if every assertion passes; no output parameter.
///
/// # Panics
/// Panics on failed assertions or unexpected parsing failure.
///
/// # Algorithm
/// Parse --stdio with --unix-socket and require an error; parse two --root arguments with
/// --config and require two roots plus a configuration path. No files or processes are changed.
///
/// # Flow
/// ```text
///        [Build argument arrays]
///                |
///        [Parse clap options]
///                |
///        +-------+--------+
///        v                v
/// [Check conflict error]    [Check roots + config]
///        |                |
///        +-------+--------+
///                v
///        [Return after assertions]
/// ```
///
/// # Examples
/// ```text
/// transport_flags_and_toml_path_are_parsed();
/// ```
fn transport_flags_and_toml_path_are_parsed() {
    assert!(Args::try_parse_from(["cmake-analyzer", "--stdio", "--unix-socket", "x"]).is_err());
    let args = Args::try_parse_from([
        "cmake-analyzer",
        "--root",
        "a",
        "--root",
        "b",
        "--config",
        "test.toml",
    ])
    .unwrap();
    assert_eq!(args.roots.len(), 2);
    assert!(args.config.is_some());
}
#[tokio::test]
/// Check CLI startup reports NotFound for a missing explicit TOML file.
/// Creates and removes a temporary directory; startup fails before a session is established.
///
/// # Arguments
/// None.
///
/// # Returns
/// `()` if every assertion passes; no output parameter.
///
/// # Panics
/// Panics on any failed assertion or unwrapped fixture/transport/process failure.
/// Cleanup is explicit on the success path; a panic may leave a fixture tree.
///
/// # Algorithm
/// Create an empty fixture directory, parse its nonexistent --config path, call cli::run and
/// require an I/O NotFound error; remove the fixture.
///
/// # Flow
/// ```text
/// [Temporary sources + settings]      [Expected: NotFound error]
///              |                              |
///              v                              |
/// [cli::run(--config missing)]                 |
///              |                              |
///              +-------------+----------------+
///                            v
///                  [Compare assertions]
///                            |
///                            v
///             [Remove temporary directory]
/// ```
///
/// # Examples
/// ```text
/// missing_explicit_configuration_fails(); // #[tokio::test] supplies the runtime
/// ```
async fn missing_explicit_configuration_fails() {
    let directory = super::session::Project::new().await;
    let path = directory.0.join("missing.toml");
    let args =
        Args::try_parse_from(["cmake-analyzer", "--config", path.to_str().unwrap()]).unwrap();
    assert_eq!(
        cli::run(args).await.unwrap_err().kind(),
        std::io::ErrorKind::NotFound
    );
    directory.remove().await;
}
#[tokio::test]
/// Check immediate stdio EOF exits successfully without protocol-stdout noise.
/// Spawns the built analyzer and captures its output; creates no explicit fixture directory.
///
/// # Arguments
/// None.
///
/// # Returns
/// `()` if every assertion passes; no output parameter.
///
/// # Panics
/// Panics on any failed assertion or unwrapped fixture/transport/process failure.
///
/// # Algorithm
/// Launch the built analyzer with --stdio and null stdin. Capture its output and require
/// successful status and empty stdout; stderr is included only in assertion failures.
///
/// # Flow
/// ```text
/// [Built analyzer + null stdin]      [Expected: success + empty stdout]
///              |                              |
///              v                              |
/// [Run CLI / capture output]                 |
///              |                              |
///              +-------------+----------------+
///                            v
///                  [Compare assertions]
///                            |
///                            v
///             [Process/task finished]
/// ```
///
/// # Examples
/// ```text
/// stdio_eof_emits_no_banner(); // #[tokio::test] supplies the runtime
/// ```
async fn stdio_eof_emits_no_banner() {
    let output = tokio::process::Command::new(env!("CARGO_BIN_EXE_cmake-analyzer"))
        .arg("--stdio")
        .stdin(Stdio::null())
        .output()
        .await
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(output.stdout.is_empty());
}
#[cfg(unix)]
#[tokio::test]
/// Check one Unix socket session exchanges framed lifecycle messages and removes its socket.
/// Creates temporary files and an analyzer session; successful completion stops the session and
/// removes fixtures.
///
/// # Arguments
/// None.
///
/// # Returns
/// `()` if every assertion passes; no output parameter.
///
/// # Panics
/// Panics on any failed assertion or unwrapped fixture/transport/process failure.
/// Cleanup is explicit on the success path; a panic may leave a fixture tree.
///
/// # Algorithm
/// Start cli::run on a temporary socket and retry connection only for NotFound, bounded by five
/// seconds. Send initialize/initialized/shutdown/exit; check initialization and shutdown ID,
/// then require Graceful within five seconds and no socket path.
///
/// # Flow
/// ```text
/// [Temporary sources + settings]      [Expected: framed replies + removed socket]
///              |                              |
///              v                              |
/// [Socket lifecycle messages]                 |
///              |                              |
///              +-------------+----------------+
///                            v
///                  [Compare assertions]
///                            |
///                            v
///             [Join task + remove tree]
/// ```
///
/// # Examples
/// ```text
/// unix_socket_runs_framed_lsp_and_cleans_its_path(); // #[tokio::test] supplies the runtime
/// ```
async fn unix_socket_runs_framed_lsp_and_cleans_its_path() {
    use serde_json::{Value, json};
    use tokio::io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader};
    let project = super::session::Project::new().await;
    let path = project.0.join("server.sock");
    let args =
        Args::try_parse_from(["cmake-analyzer", "--unix-socket", path.to_str().unwrap()]).unwrap();
    let task = tokio::spawn(cli::run(args));
    let stream = tokio::time::timeout(std::time::Duration::from_secs(5), async {
        loop {
            match tokio::net::UnixStream::connect(&path).await {
                Ok(stream) => break stream,
                Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
                    tokio::time::sleep(std::time::Duration::from_millis(10)).await
                }
                Err(error) => panic!("{error}"),
            }
        }
    })
    .await
    .unwrap();
    let (input, mut output) = stream.into_split();
    let mut input = BufReader::new(input);
    /// Append one JSON Content-Length frame to the Unix socket; no explicit flush.
    ///
    /// # Arguments
    /// - `output` `[in,out]`: socket write half receives appended header/body bytes.
    /// - `value` `[in]`: consumed JSON message.
    ///
    /// # Returns
    /// `()` after writes complete; no response is read.
    ///
    /// # Panics
    /// Panics on serialization or socket write failure.
    ///
    /// # Examples
    /// ```text
    /// send(&mut output, json!({"jsonrpc":"2.0","method":"exit"})).await;
    /// ```
    async fn send(output: &mut tokio::net::unix::OwnedWriteHalf, value: Value) {
        let body = serde_json::to_vec(&value).unwrap();
        output
            .write_all(format!("Content-Length: {}\r\n\r\n", body.len()).as_bytes())
            .await
            .unwrap();
        output.write_all(&body).await.unwrap();
    }
    /// Consume one server frame from the Unix socket.
    /// Assumes one Content-Length header and one separator line; the separator is not
    /// validated.
    ///
    /// # Arguments
    /// - `input` `[in,out]`: buffered socket reader is advanced through the header and
    ///   exact body length.
    ///
    /// # Returns
    /// The parsed JSON body; no pending queue or timeout is supplied here.
    ///
    /// # Panics
    /// Panics on I/O, unexpected header, invalid byte count, truncated body or malformed
    /// JSON.
    ///
    /// # Algorithm
    /// Parse the first Content-Length line, consume the next separator line without
    /// checking it, read exactly the declared body length and deserialize JSON.
    /// Unlike the shared session reader, this helper imposes no per-frame deadline.
    ///
    /// # Flow
    /// ```text
    /// [Read header] -------> [Invalid length: panic]
    ///       |
    ///       v
    /// [Consume separator]
    ///       |
    ///       v
    /// [Read exact body] ----> [I/O failure: panic]
    ///       |
    ///       v
    /// [Decode + return] ----> [Invalid JSON: panic]
    /// ```
    ///
    /// # Examples
    /// ```text
    /// let reply = receive(&mut input).await;
    /// ```
    async fn receive(input: &mut BufReader<tokio::net::unix::OwnedReadHalf>) -> Value {
        let mut header = String::new();
        input.read_line(&mut header).await.unwrap();
        let len = header
            .trim()
            .strip_prefix("Content-Length: ")
            .unwrap()
            .parse::<usize>()
            .unwrap();
        header.clear();
        input.read_line(&mut header).await.unwrap();
        let mut body = vec![0; len];
        input.read_exact(&mut body).await.unwrap();
        serde_json::from_slice(&body).unwrap()
    }
    send(&mut output,json!({"jsonrpc":"2.0","id":1,"method":"initialize","params":{"capabilities":{},"rootUri":super::session::uri(&project.0)}})).await;
    assert!(receive(&mut input).await.get("error").is_none());
    send(
        &mut output,
        json!({"jsonrpc":"2.0","method":"initialized","params":{}}),
    )
    .await;
    send(
        &mut output,
        json!({"jsonrpc":"2.0","id":2,"method":"shutdown"}),
    )
    .await;
    assert_eq!(receive(&mut input).await["id"], 2);
    send(&mut output, json!({"jsonrpc":"2.0","method":"exit"})).await;
    assert_eq!(
        tokio::time::timeout(std::time::Duration::from_secs(5), task)
            .await
            .unwrap()
            .unwrap()
            .unwrap(),
        lspally::server::ExitStatus::Graceful
    );
    assert!(!path.exists());
    project.remove().await;
}
