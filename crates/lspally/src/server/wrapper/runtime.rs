// AI-generated implementation.
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
    /// Business calls run in arrival order. The reader and writer remain independent.
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
    pub async fn run_stdio(self) -> io::Result<ExitStatus> {
        self.run(tokio::io::stdin(), tokio::io::stdout()).await
    }
}

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
