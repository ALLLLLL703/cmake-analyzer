// AI-generated executable and socket startup tests.
use clap::Parser;
use cmake_analyzer::cli::{self, Args};
use std::process::Stdio;

#[test]
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
    async fn send(output: &mut tokio::net::unix::OwnedWriteHalf, value: Value) {
        let body = serde_json::to_vec(&value).unwrap();
        output
            .write_all(format!("Content-Length: {}\r\n\r\n", body.len()).as_bytes())
            .await
            .unwrap();
        output.write_all(&body).await.unwrap();
    }
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
