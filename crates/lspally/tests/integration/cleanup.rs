// AI-generated tests.
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

struct StalledWriter(Arc<Notify>);
impl AsyncWrite for StalledWriter {
    fn poll_write(
        self: Pin<&mut Self>,
        _: &mut Context<'_>,
        _: &[u8],
    ) -> Poll<std::io::Result<usize>> {
        self.0.notify_one();
        Poll::Pending
    }
    fn poll_flush(self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<std::io::Result<()>> {
        Poll::Ready(Ok(()))
    }
    fn poll_shutdown(self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<std::io::Result<()>> {
        Poll::Ready(Ok(()))
    }
}

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

#[tokio::test]
async fn business_task_panic_returns_an_error_and_retires_session() {
    let mut session = Session::new();
    session.initialize().await;
    session
        .send(serde_json::json!({"jsonrpc":"2.0","id":2,"method":"custom/panic","params":{}}))
        .await;
    assert!(session.await_stop().await.is_err());
}
