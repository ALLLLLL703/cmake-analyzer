// AI-generated tests.

//! Exercise wrapper task retirement with a stalled writer and a panicking business handler.
//! Uses in-memory streams and Tokio tasks only; no files, sockets, or child processes.

use super::{fixture::Server, session::Session};
use lspally::server::Wrapper;
use std::{
    pin::Pin,
    sync::{Arc, Mutex},
    task::{Context, Poll},
    time::Duration,
};
use tokio::{
    io::{AsyncWrite, AsyncWriteExt},
    sync::Notify,
};

/// Writer fixture that signals a write attempt but never completes a write.
/// The EOF cleanup test gives this to [`Wrapper::run`] so the writer task blocks.
/// [`Notify`] lets the test close its client only after a response write begins.
/// Flush/shutdown succeed immediately; no bytes are stored and no external I/O occurs.
///
/// # Examples
/// Local asynchronous test context:
/// ```text
/// let started = Arc::new(Notify::new());
/// let writer = StalledWriter(started.clone());
/// ```
struct StalledWriter(
    /// Notification shared with the EOF test; every poll_write calls notify_one.
    /// Example: `writer.0.notified().await` waits for a write-attempt signal.
    Arc<Notify>,
);
impl AsyncWrite for StalledWriter {
    /// Signal a write attempt and remain pending forever; never consume or store bytes.
    /// Changes the shared notification only; no filesystem or subprocess access.
    ///
    /// # Arguments
    /// - `self` `[in,out]`: pinned writer; its notification receives notify_one, with no field replacement.
    /// - first `_` `[in,out]`: task context, ignored; no waker is registered.
    /// - second `_` `[in]`: input byte slice, ignored and never consumed.
    ///
    /// # Returns
    /// `Poll::Pending` always; never yields a byte count or I/O error.
    ///
    /// # Examples
    /// Local context with a prepared task Context:
    /// ```text
    /// let mut writer = StalledWriter(Arc::new(Notify::new()));
    /// let pending = Pin::new(&mut writer).poll_write(&mut cx, b"frame");
    /// ```
    fn poll_write(
        self: Pin<&mut Self>,
        _: &mut Context<'_>,
        _: &[u8],
    ) -> Poll<std::io::Result<usize>> {
        self.0.notify_one();
        Poll::Pending
    }
    /// Complete flushing immediately without changing state or performing I/O.
    ///
    /// # Arguments
    /// - `self` `[in]`: pinned writer, unchanged; no notification is sent.
    /// - `_` `[in,out]`: task context, unused and unmodified.
    ///
    /// # Returns
    /// `Poll::Ready(Ok(()))` always; no pending state or error.
    ///
    /// # Examples
    /// Local context with a prepared task Context:
    /// ```text
    /// let ready = Pin::new(&mut writer).poll_flush(&mut cx);
    /// ```
    fn poll_flush(self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<std::io::Result<()>> {
        Poll::Ready(Ok(()))
    }
    /// Complete shutdown immediately without changing state or performing I/O.
    ///
    /// # Arguments
    /// - `self` `[in]`: pinned writer, unchanged; no notification is sent.
    /// - `_` `[in,out]`: task context, unused and unmodified.
    ///
    /// # Returns
    /// `Poll::Ready(Ok(()))` always; no pending state or error.
    ///
    /// # Examples
    /// Local context with a prepared task Context:
    /// ```text
    /// let ready = Pin::new(&mut writer).poll_shutdown(&mut cx);
    /// ```
    fn poll_shutdown(self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<std::io::Result<()>> {
        Poll::Ready(Ok(()))
    }
}

/// Send initialize into a wrapper whose writer never completes, then close the client.
/// Creates duplex streams and a Tokio task; no filesystem or subprocess access.
///
/// # Arguments
/// None; the test constructs its own fixtures.
///
/// # Returns
/// `()` after all checks pass.
///
/// # Panics
/// If fixture setup or any documented assertion fails.
///
/// # Algorithm
/// Wait for the first write notification before EOF; require the wrapper task to join within three seconds, without requiring a particular run result.
///
/// # Flow
/// ```text
/// [initialize frame] --> [StalledWriter::poll_write]
///                                 |             |
///                                 v             v
///                        [notify test]     [stay pending]
///                                 |
///                                 v
///                          [drop client / EOF]
///                                 |
///                                 v
///                        [require joined task < 3s]
/// ```
///
/// # Examples
/// Local test-module context, outside any running Tokio runtime:
/// the macro-generated test wrapper runs its own runtime and returns `()`.
/// Prefer the test harness; ignored measurements require explicit selection.
/// ```text
/// eof_cancels_a_writer_blocked_during_write();
/// ```
#[tokio::test]
async fn eof_cancels_a_writer_blocked_during_write() {
    tokio::time::timeout(Duration::from_secs(3),async {
        let(mut client,input)=tokio::io::duplex(8192);let started=Arc::new(Notify::new());
        let server=Server{events:Arc::new(Mutex::new(Vec::new()))};
        let task=tokio::spawn(Wrapper::new(server).run(input,StalledWriter(started.clone())));
        let body=serde_json::json!({"jsonrpc":"2.0","id":1,"method":"initialize","params":{"processId":null,"capabilities":{"workspace":{},"general":{"positionEncodings":["utf-8"]}}}}).to_string();
        client.write_all(format!("Content-Length: {}\r\n\r\n{body}",body.len()).as_bytes()).await.unwrap();
        started.notified().await;drop(client);
        // Closure can interrupt the pending flush acknowledgement; either result must retire all tasks.
        let _=task.await.unwrap();
    }).await.expect("EOF must retire a blocked writer");
}

/// Initialize a session and invoke the fixture's custom/panic request.
/// Creates in-memory transport/tasks; the fixture intentionally panics in its business task. No files or subprocesses.
///
/// # Arguments
/// None; the test constructs its own fixtures.
///
/// # Returns
/// `()` after all checks pass.
///
/// # Panics
/// If fixture setup or any documented assertion fails.
///
/// # Algorithm
/// Require await_stop to return an I/O error while the client remains open.
///
/// # Flow
/// ```text
/// [Session::initialize]
///           |
///           v
/// [custom/panic] --> [business task panics]
///                             |
///                             v
///                   [await_stop without EOF]
///                             |
///                             v
///                      [require run error]
/// ```
///
/// # Examples
/// Local test-module context, outside any running Tokio runtime:
/// the macro-generated test wrapper runs its own runtime and returns `()`.
/// Prefer the test harness; ignored measurements require explicit selection.
/// ```text
/// business_task_panic_returns_an_error_and_retires_session();
/// ```
#[tokio::test]
async fn business_task_panic_returns_an_error_and_retires_session() {
    let mut session = Session::new();
    session.initialize().await;
    session
        .send(serde_json::json!({"jsonrpc":"2.0","id":2,"method":"custom/panic","params":{}}))
        .await;
    assert!(session.await_stop().await.is_err());
}
