// AI-generated test helpers.
use cmake_analyzer::server::{CmakeAnalyzer, ServerOptions};
use lspally::server::{ExitStatus, Wrapper};
use serde_json::{Value, json};
use std::{
    collections::VecDeque,
    path::{Path, PathBuf},
    sync::atomic::{AtomicU64, Ordering},
};
use tokio::io::{
    AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader, DuplexStream, ReadHalf, WriteHalf,
};

static NEXT: AtomicU64 = AtomicU64::new(0);
pub const HEADER: &str = "cmake_minimum_required(VERSION 3.31)\n";
pub struct Project(pub PathBuf);
impl Project {
    pub async fn new() -> Self {
        let path = std::env::temp_dir().join(format!(
            "cmake-lsp-{}-{}",
            std::process::id(),
            NEXT.fetch_add(1, Ordering::Relaxed)
        ));
        tokio::fs::create_dir_all(&path).await.unwrap();
        Self(path)
    }
    pub async fn write(&self, name: &str, text: &str) -> PathBuf {
        let path = self.0.join(name);
        if let Some(parent) = path.parent() {
            tokio::fs::create_dir_all(parent).await.unwrap();
        }
        tokio::fs::write(&path, format!("{HEADER}{text}"))
            .await
            .unwrap();
        path
    }
    pub async fn remove(self) {
        tokio::fs::remove_dir_all(self.0).await.unwrap();
    }
}
pub fn uri(path: &Path) -> String {
    format!("file://{}", path.display())
}
pub struct Session {
    pub input: WriteHalf<DuplexStream>,
    pub output: BufReader<ReadHalf<DuplexStream>>,
    pub task: tokio::task::JoinHandle<ExitStatus>,
    pub pending: VecDeque<Value>,
}
impl Session {
    pub async fn start(start: &Path, folders: Vec<Value>, nested: bool) -> Self {
        let mut options = ServerOptions::new(start.to_owned());
        options.config.user_file = None;
        Self::start_options(options, folders, nested).await
    }
    pub async fn start_options(options: ServerOptions, folders: Vec<Value>, nested: bool) -> Self {
        Self::start_hover_format(options, folders, nested, false).await
    }
    pub async fn start_hover_format(
        options: ServerOptions,
        folders: Vec<Value>,
        nested: bool,
        markdown: bool,
    ) -> Self {
        let server = CmakeAnalyzer::new(options);
        let (client, stream) = tokio::io::duplex(65536);
        let (reader, writer) = tokio::io::split(stream);
        let task = tokio::spawn(async move {
            let status = Wrapper::new(server.clone())
                .run(reader, writer)
                .await
                .unwrap();
            server.stop().await.unwrap();
            status
        });
        let (reader, writer) = tokio::io::split(client);
        let mut session = Self {
            input: writer,
            output: BufReader::new(reader),
            task,
            pending: VecDeque::new(),
        };
        session.send(json!({"jsonrpc":"2.0","id":1,"method":"initialize","params":{"workspaceFolders":folders,"capabilities":{"textDocument":{"hover":{"contentFormat":if markdown {json!(["markdown","plaintext"])}else{json!(["plaintext"])}},"documentSymbol":{"hierarchicalDocumentSymbolSupport":nested}}}}})).await;
        let initialized = session.response(1).await;
        assert!(initialized.get("error").is_none(), "{initialized}");
        assert_eq!(
            initialized["result"]["capabilities"]["definitionProvider"],
            true
        );
        assert_eq!(
            initialized["result"]["capabilities"]["referencesProvider"],
            true
        );
        assert_eq!(
            initialized["result"]["capabilities"]["positionEncoding"],
            "utf-16"
        );
        session
            .send(json!({"jsonrpc":"2.0","method":"initialized","params":{}}))
            .await;
        session
    }
    pub async fn send(&mut self, value: Value) {
        let body = serde_json::to_vec(&value).unwrap();
        self.input
            .write_all(format!("Content-Length: {}\r\n\r\n", body.len()).as_bytes())
            .await
            .unwrap();
        self.input.write_all(&body).await.unwrap();
        self.input.flush().await.unwrap();
    }
    async fn read(&mut self) -> Value {
        tokio::time::timeout(std::time::Duration::from_secs(10), async {
            let mut line = String::new();
            assert!(self.output.read_line(&mut line).await.unwrap() > 0);
            let length: usize = line
                .trim()
                .strip_prefix("Content-Length: ")
                .unwrap()
                .parse()
                .unwrap();
            line.clear();
            self.output.read_line(&mut line).await.unwrap();
            assert_eq!(line, "\r\n");
            let mut body = vec![0; length];
            self.output.read_exact(&mut body).await.unwrap();
            serde_json::from_slice(&body).unwrap()
        })
        .await
        .unwrap()
    }
    pub async fn wait(&mut self, predicate: impl Fn(&Value) -> bool) -> Value {
        if let Some(index) = self.pending.iter().position(&predicate) {
            return self.pending.remove(index).unwrap();
        }
        loop {
            let value = self.read().await;
            if predicate(&value) {
                return value;
            }
            self.pending.push_back(value);
        }
    }
    pub async fn response(&mut self, id: i32) -> Value {
        self.wait(|value| value["id"] == id).await
    }
    pub async fn diagnostics(&mut self, path: &Path, version: i32) -> Value {
        let uri = uri(path);
        self.wait(|value| {
            value["method"] == "textDocument/publishDiagnostics"
                && value["params"]["uri"] == uri
                && value["params"]["version"] == version
        })
        .await
    }
    pub async fn open(&mut self, path: &Path, text: &str) {
        self.send(json!({"jsonrpc":"2.0","method":"textDocument/didOpen","params":{"textDocument":{"uri":uri(path),"languageId":"cmake","version":1,"text":format!("{HEADER}{text}")}}})).await;
    }
    pub async fn symbols(&mut self, path: &Path, id: i32) -> Value {
        self.send(json!({"jsonrpc":"2.0","id":id,"method":"textDocument/documentSymbol","params":{"textDocument":{"uri":uri(path)}}})).await;
        self.response(id).await
    }
    pub async fn finish(mut self) {
        self.send(json!({"jsonrpc":"2.0","id":999,"method":"shutdown","params":null}))
            .await;
        assert!(self.response(999).await.get("error").is_none());
        self.send(json!({"jsonrpc":"2.0","method":"exit"})).await;
        assert_eq!(self.task.await.unwrap(), ExitStatus::Graceful);
    }
}
