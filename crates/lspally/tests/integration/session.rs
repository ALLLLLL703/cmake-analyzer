// AI-generated test helper.

//! Drive the test server over paired in-memory streams through [`Session`].
//! Creates Tokio tasks and protocol messages, but no files or child processes.

use super::fixture::Server;
use lspally::server::{ExitStatus, Wrapper};
use serde_json::{Value, json};
use std::{
    sync::{Arc, Mutex},
    time::Duration,
};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt, BufReader, DuplexStream, ReadHalf, WriteHalf},
    task::JoinHandle,
};

/// In-memory client endpoint and join handle for one fixture server.
/// Runtime and cleanup tests call [`Self::initialize`], [`Self::query`], and
/// [`Self::finish`] to drive [`Server`] through [`Wrapper`] without sockets or child processes.
/// The stream halves carry framed JSON; the event mutex exposes business ordering.
/// Dropping this value drops its stream halves, but the join handle itself does not await the task.
///
/// # Flow
/// ```text
/// [Session::send] --> [output] --> [Wrapper / Server]
///                                       |       |
///                                       v       v
/// [Session::receive] <--- [input]    [events mutex]
///                                       |
///                                       v
///                                [task join result]
/// ```
///
/// # Examples
/// Local asynchronous test context:
/// ```text
/// let mut session = Session::new();
/// session.initialize().await;
/// session.shutdown().await;
/// let status = session.finish().await.unwrap();
/// ```
pub(super) struct Session {
    /// Buffered client reader for server frames; reads consume bytes. Example: `session.input.read_u8().await` reads one byte.
    input: BufReader<ReadHalf<DuplexStream>>,
    /// Client writer for framed requests/notifications. Example: `session.output.write_all(b"bad").await` injects raw bytes.
    output: WriteHalf<DuplexStream>,
    /// Shared fixture event log; handlers append while tests read it. Example: `session.events.lock().unwrap().len()` reads the recorded count.
    pub events: Arc<Mutex<Vec<String>>>,
    /// Running wrapper task joined by finish/await_stop. Example: `session.task.is_finished()` checks whether it has stopped.
    task: JoinHandle<std::io::Result<ExitStatus>>,
}
impl Session {
    /// Create a 65,536-byte duplex transport and spawn a wrapper with an empty shared event log.
    /// Requires an active Tokio runtime; no files, sockets, or subprocesses are created.
    ///
    /// # Arguments
    /// None.
    ///
    /// # Returns
    /// A client session retaining both stream halves, the shared log, and the wrapper join handle.
    ///
    /// # Panics
    /// If called outside a Tokio runtime when spawning the task.
    ///
    /// # Algorithm
    /// Split both duplex endpoints, share a mutex-backed log, and spawn Wrapper::run.
    ///
    /// # Flow
    /// ```text
    /// [duplex pair] --> [split client halves]
    ///        |                  |
    ///        v                  v
    /// [server halves]       [Session]
    ///        |                  ^
    ///        v                  |
    /// [spawn Wrapper::run] ------+
    /// ```
    ///
    /// # Examples
    /// Local asynchronous test context:
    /// ```text
    /// let session = Session::new();
    /// ```
    pub fn new() -> Self {
        let (client, server) = tokio::io::duplex(65536);
        let (input, output) = tokio::io::split(client);
        let (reader, writer) = tokio::io::split(server);
        let events = Arc::new(Mutex::new(Vec::new()));
        let wrapper = Wrapper::new(Server {
            events: events.clone(),
        });
        Self {
            input: BufReader::new(input),
            output,
            events,
            task: tokio::spawn(wrapper.run(reader, writer)),
        }
    }
    /// Serialize one JSON value and append its frame through [`Self::raw`].
    /// Writes the in-memory stream only; does not validate the envelope.
    ///
    /// # Arguments
    /// - `self` `[in,out]`: session whose output receives an appended frame.
    /// - `value` `[in]`: owned JSON envelope serialized compactly.
    ///
    /// # Returns
    /// `()` after the entire frame is written.
    ///
    /// # Panics
    /// If writing the stream fails.
    ///
    /// # Examples
    /// Local asynchronous test context:
    /// ```text
    /// session.send(json!({"jsonrpc":"2.0","method":"exit"})).await;
    /// ```
    pub async fn send(&mut self, value: Value) {
        self.raw(&value.to_string()).await;
    }
    /// Append a Content-Length header and arbitrary UTF-8 body to the client stream.
    /// Allows malformed JSON for error tests; no filesystem or subprocess I/O.
    ///
    /// # Arguments
    /// - `self` `[in,out]`: session output to which the frame is appended.
    /// - `body` `[in]`: text whose byte count supplies Content-Length.
    ///
    /// # Returns
    /// `()` after all frame bytes are written; no separate flush is issued.
    ///
    /// # Panics
    /// If the stream write fails.
    ///
    /// # Examples
    /// Local asynchronous test context:
    /// ```text
    /// session.raw("{").await; // malformed JSON, valid framing
    /// ```
    pub async fn raw(&mut self, body: &str) {
        self.output
            .write_all(format!("Content-Length: {}\r\n\r\n{body}", body.len()).as_bytes())
            .await
            .unwrap();
    }
    /// Read and decode the next server frame within three seconds.
    /// Consumes in-memory bytes using the fixture's single Content-Length header layout.
    ///
    /// # Arguments
    /// - `self` `[in,out]`: buffered input advanced past one frame; output is unchanged.
    ///
    /// # Returns
    /// Decoded JSON body; response ID and envelope kind are not checked.
    ///
    /// # Panics
    /// On timeout, read/EOF failure, invalid UTF-8 header, wrong prefix, invalid length, or invalid JSON. Consumed input is not restored.
    ///
    /// # Algorithm
    /// Read through CRLF-CRLF, parse length, read exactly the body, and decode JSON. One timeout bounds all reads.
    ///
    /// # Flow
    /// ```text
    /// [header bytes] <---+     [3-second deadline]
    ///        |           |               |
    ///        v           |               v
    ///  < complete? > -no-+        [panic on timeout]
    ///        | yes
    ///        v
    /// [exact body read] ----> [JSON value]
    /// ```
    ///
    /// # Examples
    /// Local asynchronous test context:
    /// ```text
    /// let response = session.receive().await;
    /// ```
    pub async fn receive(&mut self) -> Value {
        tokio::time::timeout(Duration::from_secs(3), async {
            let mut header = Vec::new();
            while !header.ends_with(b"\r\n\r\n") {
                header.push(self.input.read_u8().await.unwrap());
            }
            let header = String::from_utf8(header).unwrap();
            let length: usize = header
                .strip_prefix("Content-Length: ")
                .unwrap()
                .trim()
                .parse()
                .unwrap();
            let mut body = vec![0; length];
            self.input.read_exact(&mut body).await.unwrap();
            serde_json::from_slice(&body).unwrap()
        })
        .await
        .expect("expected an LSP frame")
    }
    /// Complete UTF-8 initialization and answer the fixture's configuration callback.
    /// Exchanges protocol frames only; no filesystem or subprocess access.
    ///
    /// # Arguments
    /// - `self` `[in,out]`: streams advanced through initialize, initialized, and the configuration reply.
    ///
    /// # Returns
    /// `()` after configured is sent; its business hook resumes asynchronously.
    ///
    /// # Panics
    /// On transport/decode timeout or failure, non-UTF-8 selected encoding, or unexpected callback method.
    ///
    /// # Algorithm
    /// Advertise configuration and UTF-8/UTF-16, verify UTF-8 selection, notify initialized, then answer workspace/configuration with `["configured"]`.
    ///
    /// # Flow
    /// ```text
    /// [initialize ID 1] --> [verify UTF-8]
    ///                             |
    ///                             v
    /// [reply configured] <-- [initialized / callback]
    ///        |
    ///        v
    /// [business hook resumes]
    /// ```
    ///
    /// # Examples
    /// Local asynchronous test context:
    /// ```text
    /// let mut session = Session::new();
    /// session.initialize().await;
    /// ```
    pub async fn initialize(&mut self) {
        self.send(json!({"jsonrpc":"2.0","id":1,"method":"initialize","params":{"processId":null,"capabilities":{"workspace":{"configuration":true},"general":{"positionEncodings":["utf-8","utf-16"]}}}})).await;
        let result = self.receive().await;
        assert_eq!(
            result["result"]["capabilities"]["positionEncoding"],
            "utf-8"
        );
        self.send(json!({"jsonrpc":"2.0","method":"initialized","params":{}}))
            .await;
        let request = self.receive().await;
        assert_eq!(request["method"], "workspace/configuration");
        self.send(json!({"jsonrpc":"2.0","id":request["id"],"result":["configured"]}))
            .await;
    }
    /// Send one request and require the next frame's ID to match.
    /// Does not skip notifications; exchanges in-memory protocol bytes only.
    ///
    /// # Arguments
    /// - `self` `[in,out]`: streams advanced by one request and one received frame.
    /// - `id` `[in]`: numeric identifier required in the next response.
    /// - `method` `[in]`: method string copied into the request.
    /// - `params` `[in]`: owned JSON payload moved into the envelope.
    ///
    /// # Returns
    /// Complete JSON response, retaining result or error without unwrapping either.
    ///
    /// # Panics
    /// On transport/decode failure or timeout, or an unexpected next-frame ID.
    ///
    /// # Examples
    /// Local asynchronous test context:
    /// ```text
    /// let response = session.query(2, "custom/echo", json!({"n": 1})).await;
    /// ```
    pub async fn query(&mut self, id: i32, method: &str, params: Value) -> Value {
        self.send(json!({"jsonrpc":"2.0","id":id,"method":method,"params":params}))
            .await;
        let response = self.receive().await;
        assert_eq!(response["id"], id);
        response
    }
    /// Request shutdown with ID 999, require a null result, and notify exit.
    /// Uses in-memory protocol I/O but does not join the server task.
    ///
    /// # Arguments
    /// - `self` `[in,out]`: streams advanced through shutdown response and exit notification.
    ///
    /// # Returns
    /// `()` after sending exit; finish or await_stop observes termination.
    ///
    /// # Panics
    /// On transport failure/timeout, mismatched ID, absent result, or non-null result.
    ///
    /// # Examples
    /// Local asynchronous test context:
    /// ```text
    /// session.shutdown().await;
    /// let status = session.finish().await.unwrap();
    /// ```
    pub async fn shutdown(&mut self) {
        let response = self.query(999, "shutdown", Value::Null).await;
        assert!(response.get("result").unwrap().is_null());
        self.send(json!({"jsonrpc":"2.0","method":"exit"})).await;
    }
    /// Join the wrapper within three seconds without first closing the client streams.
    /// Consumes the session; no filesystem or subprocess access.
    ///
    /// # Arguments
    /// - `self` `[in,out]`: consumed session; stream halves remain owned during the wait, then drop.
    ///
    /// # Returns
    /// `Ok(ExitStatus)` or `Err(io::Error)` directly from Wrapper::run.
    ///
    /// # Errors
    /// Forwards wrapper run errors unchanged; no recovery or retry is attempted.
    ///
    /// # Panics
    /// On timeout or failed task join; these failures are not returned errors.
    ///
    /// # Examples
    /// Local asynchronous test context:
    /// ```text
    /// session.send(json!({"jsonrpc":"2.0","method":"exit"})).await;
    /// let status = session.await_stop().await;
    /// ```
    pub async fn await_stop(mut self) -> std::io::Result<ExitStatus> {
        tokio::time::timeout(Duration::from_secs(3), &mut self.task)
            .await
            .expect("server task must stop without client EOF")
            .expect("server task joined")
    }
    /// Drop both client stream halves to signal EOF, then join the wrapper within three seconds.
    /// Uses in-memory transport/tasks only; no files or subprocesses.
    ///
    /// # Arguments
    /// - `self` `[in,out]`: consumed session; input/output are dropped before awaiting the task.
    ///
    /// # Returns
    /// `Ok(ExitStatus)` or `Err(io::Error)` directly from Wrapper::run.
    ///
    /// # Errors
    /// Forwards wrapper run errors unchanged; no recovery or retry is attempted.
    ///
    /// # Panics
    /// On timeout or failed task join; these failures are not returned errors.
    ///
    /// # Examples
    /// Local asynchronous test context:
    /// ```text
    /// let session = Session::new();
    /// let status = session.finish().await;
    /// ```
    pub async fn finish(self) -> std::io::Result<ExitStatus> {
        let Self {
            input,
            output,
            task,
            ..
        } = self;
        drop(input);
        drop(output);
        tokio::time::timeout(Duration::from_secs(3), task)
            .await
            .expect("server task must stop")
            .expect("server task joined")
    }
}
