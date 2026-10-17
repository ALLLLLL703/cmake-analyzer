// AI-generated implementation.
//! Bounded session assembly, reader-side control and ordered business execution.
//!
//! Replies bypass the business queue, preventing deadlock when a handler calls the
//! client. Cancellation/trace controls take effect in the reader before their hooks.
//! Normal exit and EOF cancel the session and join the worker and writer; cancellation
//! drops handler futures rather than preempting application-spawned blocking work.
use super::{dispatch, lifecycle::*, transport};
use crate::{rpc, server::model::*};
use std::{
    collections::{HashMap, HashSet},
    io,
    sync::{
        Arc,
        atomic::{AtomicBool, AtomicI32, AtomicU8, Ordering},
    },
};
use tokio::{
    io::{AsyncRead, AsyncWrite, BufReader},
    sync::{Mutex, mpsc},
};

impl<S: ILanguageServer> Wrapper<S> {
    /// Serve one connection with independent reader/writer tasks and one ordered business worker.
    /// Consumes streams and performs asynchronous I/O; queues and client waits are bounded.
    /// Application-owned document state and spawned tasks remain the server implementation's concern.
    ///
    /// # Arguments
    /// - `self` `[in]`: wrapper consumed; its server is shared with the worker and limits configure this
    ///   session. Input/output queue and pending-request counts are clamped to at least one.
    /// - `input` `[in,out]`: owned read stream advanced by framing; dropped when the session ends.
    /// - `output` `[in,out]`: owned write stream receiving framed JSON only; cancellation may interrupt
    ///   output rather than drain it. Logs go to stderr, not this stream.
    ///
    /// # Returns
    /// `Ok(Graceful)` for a valid exit while the shutdown-request flag is set, `Ok(Abnormal)` for
    /// exit without that flag, or `Ok(Disconnected)` for EOF/session closure. The flag is set at
    /// shutdown admission, so graceful status is not proof the shutdown hook finished. All normal
    /// return paths join worker/writer; non-disconnected exit also awaits the server's exit hook
    /// with a closed client context. `Err` reports transport, worker/join or exit-hook failure.
    ///
    /// # Errors
    /// Propagates framing/read, queue saturation, write/flush and task failures. Worker and writer
    /// are both joined even if the worker panics; their errors take precedence over the reader result.
    /// An early worker stop is an error. Closure retires outgoing waits and cancels incoming calls;
    /// queued work/output need not complete. Dropping this entire future is not the joined shutdown path.
    ///
    /// # Panics
    /// Panics if polled without a Tokio runtime, because it spawns tasks, or if either queue
    /// capacity exceeds Tokio's `Semaphore::MAX_PERMITS`. An exit-hook panic is not caught.
    ///
    /// # Algorithm
    /// Create session state and bounded queues, spawn writer/worker, and race the reader against
    /// worker completion. Close session, drop input sender, join both tasks, then dispatch the exit
    /// hook only for protocol exit. Incoming queries/notifications run in arrival order; responses
    /// bypass that queue so a handler can await an outgoing client call without blocking its reply.
    ///
    /// # Flow
    /// ```text
    ///      [input stream] --> [reader] ---- replies ---> [outgoing waits]
    ///                            |
    ///                     [bounded input queue]
    ///                            v
    ///                        [worker] ---- client call ---+
    ///                            |                        |
    ///                            +----------+-------------+
    ///                                       v
    ///                               [bounded output queue]
    ///                                       |
    ///                                   [writer] --> [output stream]
    ///                                       |
    ///               [reader exits / worker stops]
    ///                            |
    ///                     [close + join both]
    ///                            |
    ///                   <protocol exit / EOF?>
    ///                      /             \
    ///                  [exit hook]    [no hook]
    ///                      \             /
    ///                        [status/error]
    /// ```
    /// # Examples
    /// Supply an implementation and a connected stream on a Tokio runtime; no document state is created here.
    /// ```no_run
    /// use lspally::server::{ILanguageServer, Wrapper};
    /// # async fn serve<S: ILanguageServer>(server: S, stream: tokio::net::TcpStream) -> std::io::Result<()> {
    /// let (input, output) = stream.into_split();
    /// let status = Wrapper::new(server).run(input, output).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn run<R, W>(self, input: R, output: W) -> io::Result<ExitStatus>
    where
        R: AsyncRead + Unpin + Send + 'static,
        W: AsyncWrite + Unpin + Send + 'static,
    {
        let (write_tx, write_rx) = mpsc::channel(self.limits.output_queue.max(1));
        let session = Arc::new(Session {
            writer: write_tx,
            phase: AtomicU8::new(NEW),
            shutdown_requested: AtomicBool::new(false),
            trace: AtomicU8::new(0),
            encoding: AtomicU8::new(1),
            next_id: AtomicI32::new(0),
            outgoing: std::sync::Mutex::new(HashMap::new()),
            incoming: Mutex::new(HashMap::new()),
            supported: Mutex::new(HashSet::new()),
            registerable: Mutex::new(HashSet::new()),
            initialization_progress: Mutex::new(None),
            offered_encodings: Mutex::new(Vec::new()),
            closed: Cancellation::default(),
            pending_limit: self.limits.pending_requests.max(1),
        });
        let (queue_tx, queue_rx) = mpsc::channel(self.limits.input_queue.max(1));
        let writer = tokio::spawn(transport::writer(output, write_rx, session.closed.clone()));
        let mut worker = tokio::spawn(work(self.server.clone(), session.clone(), queue_rx));
        let mut input = BufReader::new(input);
        let (result, finished_worker) = tokio::select! {
            result = read(&mut input, &self.limits, &queue_tx, &session) => (result, None),
            result = &mut worker => (Err(io::Error::other("Server worker stopped")), Some(result)),
        };
        session.close().await;
        drop(queue_tx);
        let worker_result = match finished_worker {
            Some(result) => result,
            None => worker.await,
        };
        // Join both tasks even when a business handler panics.
        let writer_result = writer.await;
        worker_result.map_err(io::Error::other)??;
        writer_result.map_err(io::Error::other)??;
        let status = result?;
        if status != ExitStatus::Disconnected {
            let client = ClientHandle { session };
            self.server
                .exit(client.context(Cancellation::default()), ())
                .await
                .map_err(io::Error::other)?;
        }
        Ok(status)
    }
    /// Serve one LSP connection on process stdin/stdout through [`Self::run`].
    /// Reads stdin and reserves stdout for framed protocol bytes; notification failures use stderr.
    ///
    /// # Arguments
    /// - `self` `[in]`: wrapper consumed to configure and run the session.
    ///
    /// # Returns
    /// `Ok(ExitStatus)` with the same exit/EOF meanings and joined cleanup as [`Self::run`];
    /// `Err` for transport, worker or exit-hook failures.
    ///
    /// # Errors
    /// Propagates all errors from [`Self::run`], including stdin/stdout failures and queue saturation.
    ///
    /// # Panics
    /// Inherits [`Self::run`]'s runtime and oversized-queue panics. Exit-hook panics propagate.
    ///
    /// # Examples
    /// Supply a business implementation and connect the host's standard streams to an LSP client.
    /// ```no_run
    /// use lspally::server::{ILanguageServer, Wrapper};
    /// # async fn serve<S: ILanguageServer>(server: S) -> std::io::Result<()> {
    /// let status = Wrapper::new(server).run_stdio().await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn run_stdio(self) -> io::Result<ExitStatus> {
        self.run(tokio::io::stdin(), tokio::io::stdout()).await
    }
}

/// Read frames and apply protocol controls without awaiting business handlers or output flushes.
/// Consumes input, routes replies directly, tracks incoming IDs and appends admitted work to the queue.
///
/// # Arguments
/// - `input` `[in,out]`: stream advanced through frames; no rewind on invalid/truncated input.
/// - `limits` `[in]`: header/body byte limits passed to the framing reader.
/// - `queue` `[in,out]`: shared sender receives admitted requests/notifications via nonblocking append.
/// - `session` `[in,out]`: shared lifecycle/trace/cancellation state and pending maps updated per message;
///   reader-generated error packets are appended to its output queue without waiting for flush.
///
/// # Returns
/// `Ok(Disconnected)` for clean EOF or session closure; valid exit returns `Graceful` when the
/// shutdown-request flag is set, otherwise `Abnormal`. `Err` for framing/I/O or queue failures.
/// Does not call the exit business hook or join tasks; the outer run method owns cleanup.
///
/// # Errors
/// Propagates framing/read failures and full/closed output queues. A full request queue sends
/// -32000 and rolls back admission state; a closed request queue fails the connection. A full/closed
/// notification queue fails the connection rather than silently losing document edits. Invalid JSON
/// (-32700), invalid envelopes/duplicate IDs (-32600) and phase errors enqueue replies then continue
/// if output capacity permits; invalid notifications/control parameters may be ignored.
///
/// # Algorithm
/// Decode one frame and classify it. Replies bypass business work. Requests reserve lifecycle
/// admission and a cancellation token before a nonblocking enqueue. Notifications gate initialization,
/// set trace/cancel tokens immediately, then enqueue hooks. Valid exit returns immediately, without
/// waiting for admitted shutdown work; malformed exit parameters are ignored.
///
/// # Flow
/// ```text
///              [frame + JSON decode] <--------------------------+
///                        |                                     |
///                   <message kind?>                            |
///               /            |             \                   |
///          [response]     [request]     [notification]           |
///               |            |             |                   |
///        [deliver waiter] [phase + ID] <exit?>                  |
///               |            |         /     \                  |
///               |        [try queue] [return] [control + gate]  |
///               |          /   \                    |           |
///               |     [error] [queued]         [try queue]      |
///               |          \   /                /      \        |
///               +-----------+------------ [queued]  [Err stop] |
///                           +----------------------------------+
/// ```
/// # Examples
/// Private runtime context: `queue` is the sender paired with the business worker.
/// ```text
/// let status = read(&mut tokio::io::BufReader::new(tokio::io::empty()),
///     &Limits::default(), &queue, &session).await?;
/// assert_eq!(status, ExitStatus::Disconnected);
/// ```
async fn read<R: AsyncRead + Unpin>(
    input: &mut BufReader<R>,
    limits: &Limits,
    queue: &mpsc::Sender<Inbound>,
    session: &Arc<Session>,
) -> io::Result<ExitStatus> {
    loop {
        let body = tokio::select! {
            _=session.closed.cancelled()=>return Ok(ExitStatus::Disconnected),
            frame=transport::frame(input,limits)=>match frame? {Some(body)=>body,None=>return Ok(ExitStatus::Disconnected)},
        };
        let value = match serde_json::from_slice::<serde_json::Value>(&body) {
            Ok(value) => value,
            Err(error) => {
                send_error(session, None, -32700, error.to_string())?;
                continue;
            }
        };
        let id = value
            .get("id")
            .and_then(|v| serde_json::from_value(v.clone()).ok());
        let message = match serde_json::from_value::<rpc::Message>(value) {
            Ok(message) => message,
            Err(error) => {
                send_error(session, id, -32600, error.to_string())?;
                continue;
            }
        };
        match message {
            rpc::Message::Response(response) => session.response(response),
            rpc::Message::Request(request) => {
                if let Some(error) = request_phase(session, &request.method) {
                    send_error(session, Some(request.id), error.code, error.message)?;
                    continue;
                }
                let cancellation = Cancellation::default();
                let id = request.id.clone();
                {
                    let mut incoming = session.incoming.lock().await;
                    if incoming.contains_key(&id) {
                        drop(incoming);
                        send_error(
                            session,
                            Some(id),
                            -32600,
                            "Duplicate active request ID".into(),
                        )?;
                        continue;
                    }
                    incoming.insert(id.clone(), cancellation.clone());
                }
                let method = request.method.clone();
                if let Err(error) = queue.try_send(Inbound::Request(request, cancellation)) {
                    session.incoming.lock().await.remove(&id);
                    rollback_phase(session, &method);
                    match error {
                        mpsc::error::TrySendError::Full(_) => send_error(
                            session,
                            Some(id),
                            -32000,
                            "Server input queue is full".into(),
                        )?,
                        mpsc::error::TrySendError::Closed(_) => {
                            return Err(io::Error::other("Server worker stopped"));
                        }
                    }
                }
            }
            rpc::Message::Notification(mut notification) => {
                if notification.method == "exit" {
                    let params = notification.take_params::<lsp_types::notification::Exit>();
                    if params.is_err() {
                        continue;
                    }
                    return Ok(if session.shutdown_requested.load(Ordering::Acquire) {
                        ExitStatus::Graceful
                    } else {
                        ExitStatus::Abnormal
                    });
                }
                let phase = session.phase.load(Ordering::Acquire);
                if notification.method == "initialized" {
                    if phase != INITIALIZED
                        || notification
                            .take_params::<lsp_types::notification::Initialized>()
                            .is_err()
                    {
                        continue;
                    }
                    notification.params = Some(rpc::Params::Left(serde_json::Map::new()));
                    session.phase.store(RUNNING, Ordering::Release);
                } else if phase != RUNNING || session.shutdown_requested.load(Ordering::Acquire) {
                    continue;
                }
                if notification.method == "$/setTrace" {
                    let value = notification.params.as_ref().and_then(|p| match p {
                        rpc::Params::Left(v) => v.get("value").and_then(|v| v.as_str()),
                        _ => None,
                    });
                    if let Some(value) = value {
                        match value {
                            "off" => session.trace.store(0, Ordering::Release),
                            "messages" => session.trace.store(1, Ordering::Release),
                            "verbose" => session.trace.store(2, Ordering::Release),
                            _ => continue,
                        }
                    }
                }
                if notification.method == "$/cancelRequest" {
                    let parsed = notification
                        .params
                        .as_ref()
                        .and_then(|p| match p {
                            rpc::Params::Left(v) => v.get("id"),
                            _ => None,
                        })
                        .and_then(|v| serde_json::from_value::<rpc::RequestId>(v.clone()).ok());
                    if let Some(id) = parsed {
                        if let Some(token) = session.incoming.lock().await.get(&id) {
                            token.cancel();
                        }
                    }
                }
                // Blocking here would prevent receipt of client replies. Never drop document changes silently.
                queue
                    .try_send(Inbound::Notification(notification))
                    .map_err(|_| io::Error::other("Server notification queue is full or closed"))?;
            }
        }
    }
}
/// Validate request admission and reserve initialize/shutdown state; no I/O.
/// A successful admission is not successful handler completion.
///
/// # Arguments
/// - `session` `[in,out]`: reads phase; initialize overwrites it with INITIALIZING, and shutdown
///   sets its requested flag. Other methods leave state unchanged.
/// - `method` `[in]`: exact request method string, including custom methods.
///
/// # Returns
/// `None` when admitted; `Some(error)` for repeated initialize, pre-running calls or shutdown-time
/// calls. Rejections do not mutate state.
///
/// # Algorithm
/// Initialize requires NEW. All other methods require at least RUNNING and no shutdown flag;
/// shutdown then sets its flag so later requests are rejected before the worker reaches it.
///
/// # Flow
/// ```text
///                   <initialize?>
///                   /           \
///                 yes           no
///                  |             |
///               <NEW?>   <running + no shutdown?>
///               /    \         /           \
///            [set] [error]   [allow]       [error]
///              |               |
///              |        [flag if shutdown]
///              \               /
///                  [None]
/// ```
/// # Examples
/// ```text
/// assert!(request_phase(&session, "initialize").is_none()); // NEW session
/// assert!(request_phase(&session, "initialize").is_some()); // now INITIALIZING
/// ```
fn request_phase(session: &Session, method: &str) -> Option<ServerError> {
    let phase = session.phase.load(Ordering::Acquire);
    if method == "initialize" {
        if phase != NEW {
            return Some(ServerError::new(-32600, "Initialize may only run once"));
        }
        session.phase.store(INITIALIZING, Ordering::Release);
    } else if phase < RUNNING {
        return Some(ServerError::new(-32002, "Server not initialized"));
    } else if session.shutdown_requested.load(Ordering::Acquire) {
        return Some(ServerError::new(-32600, "Server is shutting down"));
    } else if method == "shutdown" {
        session.shutdown_requested.store(true, Ordering::Release);
    }
    None
}
/// Undo failed initialize/shutdown admission after queue rejection or handler failure; no I/O.
/// Other method names do nothing.
///
/// # Arguments
/// - `session` `[in,out]`: initialize overwrites phase with NEW; shutdown clears its flag and
///   overwrites phase with RUNNING. Other session fields remain unchanged.
/// - `method` `[in]`: method whose failed admission/execution must be rolled back.
///
/// # Returns
/// `()` after the method-specific state update.
///
/// # Examples
/// ```text
/// rollback_phase(&session, "shutdown");
/// assert!(!session.shutdown_requested.load(Ordering::Acquire));
/// ```
fn rollback_phase(session: &Session, method: &str) {
    match method {
        "initialize" => session.phase.store(NEW, Ordering::Release),
        "shutdown" => {
            session.shutdown_requested.store(false, Ordering::Release);
            session.phase.store(RUNNING, Ordering::Release);
        }
        _ => {}
    }
}
/// Run admitted business calls serially and send one flushed reply per completed request.
/// Mutates lifecycle/correlation state and writes notification failures to stderr.
///
/// # Arguments
/// - `server` `[in]`: shared business implementation; its hooks may perform application side effects.
/// - `session` `[in,out]`: shared incoming map loses completed IDs, phases advance/roll back, successful
///   shutdown retires outgoing waits, and replies append to its writer queue.
/// - `queue` `[in,out]`: owned receiver consumed in arrival order until closure or sender exhaustion.
///
/// # Returns
/// `Ok(())` when session closure or queue exhaustion stops work; `Err` if sending a reply fails.
/// Handler errors become protocol error replies for requests, or stderr reports for notifications.
///
/// # Errors
/// A failed flush/closed writer becomes an I/O error. Request cancellation becomes -32800 and
/// rolls back lifecycle admission where applicable; session closure instead drops the active
/// handler future and returns without a reply or per-request map removal (outer cleanup clears it).
///
/// # Panics
/// Business-handler panics propagate to the spawned task; the outer runner joins and reports them.
///
/// # Algorithm
/// Race each dequeue with closure. For requests, race dispatch with closure and its cancellation
/// (closure/cancellation are prioritized), retire its incoming ID, adjust lifecycle, then await
/// reply flush. Notifications race closure only and get fresh untracked cancellation tokens.
/// Successful shutdown sets SHUT_DOWN and retires outgoing waits; failed shutdown returns RUNNING.
///
/// # Flow
/// ```text
///                 [dequeue] <-------------------------------+
///                     |                                     |
///                <request?>                                 |
///                /        \                                 |
///              yes        no                                |
///               |          |                                |
///      [dispatch / cancel] [notification hook]               |
///               |          |                                |
///        [remove ID]  [log error if any] --------------------+
///               |                                           |
///      [phase / outgoing cleanup]                           |
///               |                                           |
///         [send + flush] -- success ------------------------+
///               |
///            [Err stop]
///    Closure during dequeue/dispatch: drop the active wait and return Ok.
///    Closure during send/flush: the send error follows the Err stop path.
/// ```
/// # Examples
/// Private runtime context; closing the queue ends the worker without invoking a handler.
/// ```text
/// let (sender, queue) = tokio::sync::mpsc::channel(1);
/// drop(sender);
/// work(server.clone(), session.clone(), queue).await?;
/// ```
async fn work<S: ILanguageServer>(
    server: Arc<S>,
    session: Arc<Session>,
    mut queue: mpsc::Receiver<Inbound>,
) -> io::Result<()> {
    let client = ClientHandle {
        session: session.clone(),
    };
    loop {
        let message = tokio::select! {
            _=session.closed.cancelled()=>return Ok(()),
            received=queue.recv()=>match received{Some(message)=>message,None=>return Ok(())},
        };
        match message {
            Inbound::Request(request, cancellation) => {
                let id = request.id.clone();
                let method = request.method.clone();
                if method == "shutdown" {
                    session.phase.store(SHUTTING_DOWN, Ordering::Release);
                }
                let result = tokio::select! {
                    biased;
                    _=session.closed.cancelled()=>return Ok(()),
                    _=cancellation.cancelled()=>Err(ServerError::cancelled()),
                    result=dispatch::request(server.as_ref(),client.context(cancellation.clone()),request)=>result,
                };
                session.incoming.lock().await.remove(&id);
                let success = result.is_ok();
                let response = match result {
                    Ok(response) => response,
                    Err(error) => rpc::Response::error(
                        Some(id),
                        rpc::ResponseError::new(error.code, error.message),
                    ),
                };
                if method == "initialize" && success {
                    session.phase.store(INITIALIZED, Ordering::Release);
                }
                if method == "shutdown" && success {
                    session.phase.store(SHUT_DOWN, Ordering::Release);
                    session.retire_outgoing();
                }
                if !success {
                    rollback_phase(&session, &method);
                }
                session
                    .send(rpc::Message::Response(response))
                    .await
                    .map_err(io::Error::other)?;
            }
            Inbound::Notification(notification) => {
                let result = tokio::select! {
                    _=session.closed.cancelled()=>return Ok(()),
                    result=dispatch::notification(server.as_ref(),client.context(Cancellation::default()),notification)=>result,
                };
                if let Err(error) = result {
                    // Notifications have no response. Report handler failures on stderr, never stdout.
                    eprintln!("lspally notification: {error}");
                }
            }
        }
    }
}
/// Append a reader-generated error reply without waiting for socket flush; no direct stream I/O.
/// This keeps the reader available for replies while a business handler awaits a client call.
///
/// # Arguments
/// - `session` `[in,out]`: shared writer queue receives one appended packet if capacity exists.
/// - `id` `[in]`: originating key, or `None` for an unidentifiable/malformed request.
/// - `code` `[in]`: JSON-RPC/LSP failure code.
/// - `message` `[in]`: owned error description moved into the response.
///
/// # Returns
/// `Ok(())` when queued, not necessarily sent/flushed; `Err` if the queue is full or closed.
///
/// # Errors
/// Returns an I/O error on nonblocking queue failure. The acknowledgement receiver is intentionally
/// dropped, so later write failures are observed by the writer/run path, not by this call.
///
/// # Examples
/// ```text
/// send_error(&session, None, -32700, "Invalid JSON".into())?;
/// ```
fn send_error(
    session: &Session,
    id: Option<rpc::RequestId>,
    code: i32,
    message: String,
) -> io::Result<()> {
    // Reader-generated replies must not wait for a socket flush while a handler awaits a client reply.
    let (done, _received) = tokio::sync::oneshot::channel();
    let message = rpc::Message::Response(rpc::Response::error(
        id,
        rpc::ResponseError::new(code, message),
    ));
    session
        .writer
        .try_send(WritePacket { message, done })
        .map_err(|_| io::Error::other("Server output queue is full or closed"))
}
