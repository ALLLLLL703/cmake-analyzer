// AI-generated test helpers.
//! Temporary project lifecycle and a framed in-memory analyzer LSP client.
//! [`Project::write`] and [`Session::open`] both prepend [`HEADER`]; feature helpers
//! choose whether to offset lines. [`Session::finish`] joins server shutdown, while
//! [`Project::remove`] separately deletes fixtures. Neither type cleans up through Drop.
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

/// Monotonic suffix for temporary directory names; relaxed ordering only allocates identity.
static NEXT: AtomicU64 = AtomicU64::new(0);
/// Minimum-version line prepended by fixture writes and opens; body lines start at protocol
/// line 1.
pub const HEADER: &str = "cmake_minimum_required(VERSION 3.31)\n";
/// Explicitly managed temporary fixture tree used by all server scenarios.
/// [`Self::new`] creates it, [`Self::write`] writes header-prefixed sources, and
/// [`Self::remove`] deletes it. No Drop implementation removes abandoned fixtures.
///
/// # Flow
/// ```text
/// [new: create directory]       [write: create parents]
///           |                           |
///           v                           v
///      [Project path] <---------- [header + body on disk]
///           |
///           v
/// [remove: delete tree] -------> [sources + run outputs gone]
/// ```
/// # Examples
/// ```text
/// let p = Project::new().await;
/// let root = p.write("CMakeLists.txt", "set(V yes)\n").await;
/// p.remove().await;
/// ```
pub struct Project(/** Directory path; e.g. `p.0.join("x")`; no I/O. */ pub PathBuf);
impl Project {
    /// Create a unique temporary fixture directory using the process ID and an atomic
    /// counter.
    /// Creates directories on disk; cleanup is explicit through [`Self::remove`].
    ///
    /// # Arguments
    /// None.
    ///
    /// # Returns
    /// A project owning the directory path; no files are written yet.
    ///
    /// # Panics
    /// Panics if creating the temporary directory fails.
    ///
    /// # Examples
    /// ```text
    /// let p = Project::new().await;
    /// p.remove().await;
    /// ```
    pub async fn new() -> Self {
        let path = std::env::temp_dir().join(format!(
            "cmake-lsp-{}-{}",
            std::process::id(),
            NEXT.fetch_add(1, Ordering::Relaxed)
        ));
        tokio::fs::create_dir_all(&path).await.unwrap();
        Self(path)
    }
    /// Create parent directories and overwrite a fixture file with [`HEADER`] plus supplied
    /// text.
    /// Writes disk only; it does not update any open document overlay.
    ///
    /// # Arguments
    /// - `self` `[in]`: project whose path anchors the file; not mutated.
    /// - `name` `[in]`: path joined to the project directory; callers supply
    ///   fixture-relative names.
    /// - `text` `[in]`: body appended after the standard minimum-version header.
    ///
    /// # Returns
    /// The joined file path after a successful write.
    ///
    /// # Panics
    /// Panics on parent-directory creation or file-write failure.
    ///
    /// # Examples
    /// ```text
    /// let root = p.write("CMakeLists.txt", "set(V yes)\n").await;
    /// ```
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
    /// Consume the fixture and recursively delete its directory, including any
    /// configure/script output.
    /// Call after stopping all sessions using it; there is no automatic Drop cleanup.
    ///
    /// # Arguments
    /// - `self` `[in]`: owned project path to remove; ownership is consumed, not returned.
    ///
    /// # Returns
    /// `()` after recursive deletion; no output parameter.
    ///
    /// # Panics
    /// Panics if recursive removal fails, including when the directory is already absent.
    ///
    /// # Examples
    /// ```text
    /// client.finish().await;
    /// p.remove().await;
    /// ```
    pub async fn remove(self) {
        tokio::fs::remove_dir_all(self.0).await.unwrap();
    }
}
/// Prefix a display path with file:// for controlled test inputs; no side effects.
/// This helper does not percent-encode or canonicalize paths; special-character tests
/// check server-produced encoded URIs separately.
///
/// # Arguments
/// - `path` `[in]`: local path, normally absolute and without URI-sensitive characters.
///
/// # Returns
/// An owned file URI string using the displayed path.
///
/// # Examples
/// ```text
/// let document_uri = uri(&p.0.join("CMakeLists.txt"));
/// ```
pub fn uri(path: &Path) -> String {
    format!("file://{}", path.display())
}
/// In-memory client and running analyzer task used by feature scenarios.
/// [`Self::send`] writes frames; [`Self::wait`] preserves unmatched output in a queue.
/// [`Self::finish`] performs shutdown/exit and joins the task; [`Project::remove`]
/// separately deletes files. Construction starts tasks; dropping this value is not graceful
/// cleanup.
///
/// # Flow
/// ```text
/// [test] --frames--> [input] ----> [Wrapper + analyzer task]
///    ^                                    |
///    |                                    v
///    +-- matched -- [wait] <---------- [output reader]
///                      |
///                      v
///               [pending nonmatches]
/// ```
/// # Examples
/// ```text
/// let mut client = Session::start(&p.0, vec![], true).await;
/// client.open(&root, "set(V yes)\n").await;
/// let report = client.diagnostics(&root, 1).await;
/// client.finish().await;
/// ```
pub struct Session {
    /// Client-to-server writer; send appends frames, EOF tests close it explicitly.
    /// Example: `session.input.shutdown().await.unwrap();` sends EOF.
    pub input: WriteHalf<DuplexStream>,
    /// Buffered server-to-client reader consumed by read/wait; no automatic message
    /// dispatch.
    /// Example: `let reader = &mut session.output;` borrows the stream for framed reads.
    pub output: BufReader<ReadHalf<DuplexStream>>,
    /// Wrapper/service task returning the transport exit status after analyzer stop.
    /// Example: `let status = session.task.await.unwrap();` joins it after EOF.
    pub task: tokio::task::JoinHandle<ExitStatus>,
    /// Unmatched JSON output kept in arrival order; wait removes only its matching entry.
    /// Example: `let queued = session.pending.len();` reads the buffered message count.
    pub pending: VecDeque<Value>,
}
impl Session {
    /// Start an initialized test client with user configuration discovery disabled.
    /// Creates an analyzer service/task; project configuration may still be loaded.
    ///
    /// # Arguments
    /// - `start` `[in]`: startup directory copied into ServerOptions.
    /// - `folders` `[in]`: consumed initialize workspace-folder entries.
    /// - `nested` `[in]`: hierarchical document-symbol capability.
    ///
    /// # Returns
    /// A live session using plaintext hover and no completion snippets; caller must finish
    /// it.
    ///
    /// # Panics
    /// Panics if initialization/framing fails or advertised capabilities violate the shared
    /// contract.
    ///
    /// # Algorithm
    /// Build default options, clear user_file, and delegate initialization to
    /// start_options.
    ///
    /// # Flow
    /// ```text
    /// [Client halves]                   [Spawn wrapper task]
    ///       |                                  |
    ///       +-- initialize ID 1 -------------> |
    ///       | <---------- capabilities --------+
    ///       v                                  |
    /// [Assert capabilities]                    |
    ///       |                                  |
    ///       +-- initialized notification ----> |
    ///       v                                  v
    /// [Return live session]              [Handle later frames]
    /// ```
    ///
    /// # Examples
    /// ```text
    /// let client = Session::start(&p.0, vec![], true).await;
    /// client.finish().await;
    /// ```
    pub async fn start(start: &Path, folders: Vec<Value>, nested: bool) -> Self {
        let mut options = ServerOptions::new(start.to_owned());
        options.config.user_file = None;
        Self::start_options(options, folders, nested).await
    }
    /// Initialize a session from explicit options with plaintext hover and no snippets.
    /// Starts a service/task and performs the shared capability assertions.
    ///
    /// # Arguments
    /// - `options` `[in]`: consumed startup options, including configuration sources and
    ///   root selection.
    /// - `folders` `[in]`: consumed JSON workspace-folder entries for initialize.
    /// - `nested` `[in]`: whether the client supports hierarchical document symbols.
    ///
    /// # Returns
    /// The live initialized session; no initialized response remains queued.
    ///
    /// # Panics
    /// Panics on shared initialization or transport assertions.
    ///
    /// # Examples
    /// ```text
    /// let client = Session::start_options(options, vec![], true).await;
    /// client.finish().await;
    /// ```
    pub async fn start_options(options: ServerOptions, folders: Vec<Value>, nested: bool) -> Self {
        Self::start_hover_format(options, folders, nested, false).await
    }
    /// Initialize a session selecting Markdown-first or plaintext-only hover preference.
    /// Starts a service/task; does not enable completion snippets.
    ///
    /// # Arguments
    /// - `options` `[in]`: consumed startup options, including configuration sources and
    ///   root selection.
    /// - `folders` `[in]`: consumed JSON workspace-folder entries for initialize.
    /// - `nested` `[in]`: whether the client supports hierarchical document symbols.
    /// - `markdown` `[in]`: true sends `[markdown, plaintext]`; false sends `[plaintext]`.
    ///
    /// # Returns
    /// The live initialized session using the requested hover preference.
    ///
    /// # Panics
    /// Panics on shared initialization or transport assertions.
    ///
    /// # Examples
    /// ```text
    /// let client = Session::start_hover_format(options, vec![], true, true).await;
    /// client.finish().await;
    /// ```
    pub async fn start_hover_format(
        options: ServerOptions,
        folders: Vec<Value>,
        nested: bool,
        markdown: bool,
    ) -> Self {
        Self::start_capabilities(options, folders, nested, markdown, false).await
    }
    /// Initialize a completion client with hierarchical symbols and plaintext hover.
    /// Starts a service/task and negotiates optional snippet support.
    ///
    /// # Arguments
    /// - `options` `[in]`: consumed startup options, including configuration sources and
    ///   root selection.
    /// - `folders` `[in]`: consumed JSON workspace-folder entries for initialize.
    /// - `snippets` `[in]`: completionItem.snippetSupport sent during initialize.
    ///
    /// # Returns
    /// A live initialized session; reserve_arg still depends on server configuration.
    ///
    /// # Panics
    /// Panics on shared initialization or transport assertions.
    ///
    /// # Examples
    /// ```text
    /// let client = Session::start_completion(options, vec![], true).await;
    /// client.finish().await;
    /// ```
    pub async fn start_completion(
        options: ServerOptions,
        folders: Vec<Value>,
        snippets: bool,
    ) -> Self {
        Self::start_capabilities(options, folders, true, false, snippets).await
    }
    /// Start an analyzer over a 65,536-byte duplex stream and assert its initialization
    /// contract.
    /// Spawns a wrapper task that stops the analyzer after the transport exits.
    ///
    /// # Arguments
    /// - `options` `[in]`: consumed startup options, including configuration sources and
    ///   root selection.
    /// - `folders` `[in]`: consumed JSON workspace-folder entries for initialize.
    /// - `nested` `[in]`: whether the client supports hierarchical document symbols.
    /// - `markdown` `[in]`: hover content-format preference; true offers Markdown first.
    /// - `snippets` `[in]`: client completion snippet support.
    ///
    /// # Returns
    /// The live session after initialize response consumption and initialized notification.
    ///
    /// # Panics
    /// Panics on I/O/JSON errors or initialize failure; requires no completion resolve,
    /// definition/references providers and utf-16 encoding. Task failures surface when
    /// joined.
    ///
    /// # Algorithm
    /// Create both duplex halves and spawn Wrapper::run followed by server.stop. Send
    /// request ID 1,
    /// consume its response, assert capabilities, then send initialized. Later messages
    /// stay in the stream.
    ///
    /// # Flow
    /// ```text
    /// [Client halves]                   [Spawn wrapper task]
    ///       |                                  |
    ///       +-- initialize ID 1 -------------> |
    ///       | <---------- capabilities --------+
    ///       v                                  |
    /// [Assert capabilities]                    |
    ///       |                                  |
    ///       +-- initialized notification ----> |
    ///       v                                  v
    /// [Return live session]              [Handle later frames]
    /// ```
    ///
    /// # Examples
    /// ```text
    /// let client = Session::start_capabilities(options, vec![], true, false, true).await;
    /// client.finish().await;
    /// ```
    async fn start_capabilities(
        options: ServerOptions,
        folders: Vec<Value>,
        nested: bool,
        markdown: bool,
        snippets: bool,
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
        session.send(json!({"jsonrpc":"2.0","id":1,"method":"initialize","params":{"workspaceFolders":folders,"capabilities":{"textDocument":{"completion":{"completionItem":{"snippetSupport":snippets}},"hover":{"contentFormat":if markdown {json!(["markdown","plaintext"])}else{json!(["plaintext"])}},"documentSymbol":{"hierarchicalDocumentSymbolSupport":nested}}}}})).await;
        let initialized = session.response(1).await;
        assert!(initialized.get("error").is_none(), "{initialized}");
        assert_eq!(
            initialized["result"]["capabilities"]["completionProvider"]["resolveProvider"],
            false
        );
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
    /// Serialize one JSON value into a Content-Length frame and flush the client stream.
    /// Appends bytes to server input; it does not wait for any response.
    ///
    /// # Arguments
    /// - `self` `[in,out]`: session input is appended/flushed; pending messages and output
    ///   are untouched.
    /// - `value` `[in]`: consumed JSON message to serialize.
    ///
    /// # Returns
    /// `()` once all bytes are written and flushed; no output parameter.
    ///
    /// # Panics
    /// Panics if serialization, writing or flushing fails.
    ///
    /// # Examples
    /// ```text
    /// client.send(json!({"jsonrpc":"2.0","method":"initialized","params":{}})).await;
    /// ```
    pub async fn send(&mut self, value: Value) {
        let body = serde_json::to_vec(&value).unwrap();
        self.input
            .write_all(format!("Content-Length: {}\r\n\r\n", body.len()).as_bytes())
            .await
            .unwrap();
        self.input.write_all(&body).await.unwrap();
        self.input.flush().await.unwrap();
    }
    /// Read exactly one framed server message with a ten-second deadline.
    /// Consumes stream bytes without inspecting or changing the pending-message queue.
    ///
    /// # Arguments
    /// - `self` `[in,out]`: output reader is advanced; input and pending queue are
    ///   untouched.
    ///
    /// # Returns
    /// The parsed JSON body, whether response, notification or request.
    ///
    /// # Panics
    /// Panics on timeout, EOF, malformed Content-Length/JSON, read failure or a missing
    /// CRLF separator.
    ///
    /// # Algorithm
    /// Read the first header line, parse its byte count, require a blank CRLF line, read
    /// exactly that
    /// many bytes and deserialize JSON. The timeout wraps the entire frame.
    ///
    /// # Flow
    /// ```text
    /// [Ten-second frame deadline]
    ///               |
    ///               v
    ///  [Header + byte count] ---------> [Bad/EOF: panic]
    ///               |
    ///               v
    ///  [Require CRLF separator] ------> [Mismatch: panic]
    ///               |
    ///               v
    ///  [Read exact body length] ------> [Short/error: panic]
    ///               |
    ///               v
    ///  [Parse JSON + return] ---------> [Bad JSON: panic]
    /// ```
    ///
    /// # Examples
    /// ```text
    /// let message = client.read().await;
    /// ```
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
    /// Consume the first message matching a predicate, retaining other messages for later
    /// waits.
    /// Searches pending messages before reading the stream; unrelated messages append to
    /// the queue.
    ///
    /// # Arguments
    /// - `self` `[in,out]`: removes a matching pending entry or advances output and appends
    ///   nonmatching messages.
    /// - `predicate` `[in]`: borrow-only message test; receives both queued and newly read
    ///   messages.
    ///
    /// # Returns
    /// The matching JSON value, removed from the queue/stream; no output parameter.
    ///
    /// # Panics
    /// Panics on a read/framing timeout/failure, or if the predicate panics. There is no
    /// overall wait deadline: each newly read frame has its own ten-second deadline.
    ///
    /// # Algorithm
    /// Remove the first queued match if present. Otherwise read one frame at a time; return
    /// a match
    /// and append each nonmatch to pending, preserving its order.
    ///
    /// # Flow
    /// ```text
    /// [Search pending] -- found --> [Remove + return]
    ///        |
    ///      absent
    ///        v
    ///  [Read one frame] <----------------+
    ///        |                          |
    ///        v                          |
    ///  <Predicate matches?> -- no --> [Append pending]
    ///        |
    ///       yes
    ///        v
    ///  [Return frame]
    /// ```
    ///
    /// # Examples
    /// ```text
    /// let notice = client.wait(|v| v["method"] == "window/showMessage").await;
    /// ```
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
    /// Wait for a message whose id equals the requested integer, buffering unrelated
    /// traffic.
    ///
    /// # Arguments
    /// - `self` `[in,out]`: advances output/removes a queued match and appends unrelated
    ///   messages via wait.
    /// - `id` `[in]`: numeric request ID to match; this helper does not check message
    ///   shape.
    ///
    /// # Returns
    /// The matching full JSON envelope, including result or error.
    ///
    /// # Panics
    /// Panics on transport/deadline failures from wait.
    ///
    /// # Examples
    /// ```text
    /// let response = client.response(10).await;
    /// ```
    pub async fn response(&mut self, id: i32) -> Value {
        self.wait(|value| value["id"] == id).await
    }
    /// Wait for a version-qualified textDocument/publishDiagnostics notification for a
    /// fixture path.
    ///
    /// # Arguments
    /// - `self` `[in,out]`: consumes the matching publication; unrelated messages remain
    ///   buffered.
    /// - `path` `[in]`: document path converted with uri; not read from disk.
    /// - `version` `[in]`: exact diagnostic version to match; unversioned close
    ///   publications do not match.
    ///
    /// # Returns
    /// The full matching notification, including params.diagnostics (possibly empty).
    ///
    /// # Panics
    /// Panics on transport/deadline failures from wait.
    ///
    /// # Examples
    /// ```text
    /// let report = client.diagnostics(&root, 1).await;
    /// ```
    pub async fn diagnostics(&mut self, path: &Path, version: i32) -> Value {
        let uri = uri(path);
        self.wait(|value| {
            value["method"] == "textDocument/publishDiagnostics"
                && value["params"]["uri"] == uri
                && value["params"]["version"] == version
        })
        .await
    }
    /// Send didOpen with CMake language, version 1 and [`HEADER`] prepended to text.
    /// Changes the server overlay, not the fixture file; does not wait for analysis.
    ///
    /// # Arguments
    /// - `self` `[in,out]`: appends a frame to input; does not drain output.
    /// - `path` `[in]`: document identity; it need not exist on disk.
    /// - `text` `[in]`: header-free document body copied into the notification.
    ///
    /// # Returns
    /// `()` after sending; use diagnostics or a request as an analysis barrier.
    ///
    /// # Panics
    /// Panics if sending the frame fails.
    ///
    /// # Examples
    /// ```text
    /// client.open(&root, "set(V yes)\n").await;
    /// ```
    pub async fn open(&mut self, path: &Path, text: &str) {
        self.send(json!({"jsonrpc":"2.0","method":"textDocument/didOpen","params":{"textDocument":{"uri":uri(path),"languageId":"cmake","version":1,"text":format!("{HEADER}{text}")}}})).await;
    }
    /// Request textDocument/documentSymbol and consume the response with the supplied ID.
    ///
    /// # Arguments
    /// - `self` `[in,out]`: writes a request and consumes its reply, buffering unrelated
    ///   messages.
    /// - `path` `[in]`: document URI path; no coordinate offset is involved.
    /// - `id` `[in]`: request ID, distinct from other outstanding requests.
    ///
    /// # Returns
    /// The full response envelope; result may be nested/flat symbols or null, or error is
    /// present.
    ///
    /// # Panics
    /// Panics on transport/deadline failures; does not assert request success.
    ///
    /// # Examples
    /// ```text
    /// let reply = client.symbols(&root, 20).await;
    /// ```
    pub async fn symbols(&mut self, path: &Path, id: i32) -> Value {
        self.send(json!({"jsonrpc":"2.0","id":id,"method":"textDocument/documentSymbol","params":{"textDocument":{"uri":uri(path)}}})).await;
        self.response(id).await
    }
    /// Gracefully shut down and join the analyzer task; does not delete fixture files.
    /// Consumes the session and asserts shutdown success plus a Graceful exit status.
    ///
    /// # Arguments
    /// - `self` `[in,out]`: consumed session sends lifecycle frames, drains the shutdown
    ///   reply and joins its task.
    ///
    /// # Returns
    /// `()` once the wrapper task has stopped and called server.stop.
    ///
    /// # Panics
    /// Panics on transport failures, shutdown error, task failure or non-Graceful status.
    /// No separate overall join deadline is imposed here; cancellation tests add an outer
    /// timeout.
    ///
    /// # Algorithm
    /// Send shutdown with reserved ID 999 and null params, consume its successful response,
    /// send exit,
    /// then await the task and compare its ExitStatus. Files remain for Project::remove.
    ///
    /// # Flow
    /// ```text
    /// [Client session]                  [Wrapper + analyzer]
    ///        |                                  |
    ///        +-- shutdown ID 999 -------------> |
    ///        | <--------- successful reply -----+
    ///        +-- exit ------------------------> |
    ///        |                                  v
    ///        |                           [run ends; stop service]
    ///        |                                  |
    ///        +-- await task <-------------------+
    ///        v
    /// [Assert Graceful; files remain]
    /// ```
    ///
    /// # Examples
    /// ```text
    /// client.finish().await;
    /// p.remove().await;
    /// ```
    pub async fn finish(mut self) {
        self.send(json!({"jsonrpc":"2.0","id":999,"method":"shutdown","params":null}))
            .await;
        assert!(self.response(999).await.get("error").is_none());
        self.send(json!({"jsonrpc":"2.0","method":"exit"})).await;
        assert_eq!(self.task.await.unwrap(), ExitStatus::Graceful);
    }
}
