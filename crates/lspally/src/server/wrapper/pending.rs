// AI-generated implementation.
use crate::{
    rpc::{Message, RequestId, Response},
    server::model::*,
};
use std::sync::{Arc, atomic::Ordering};
use tokio::sync::oneshot;

impl Session {
    pub(super) async fn send(&self, message: Message) -> ServerResult<()> {
        if self.closed.is_cancelled() {
            return Err(disconnected());
        }
        let (done, received) = oneshot::channel();
        tokio::select! {
            _=self.closed.cancelled()=>return Err(disconnected()),
            sent=self.writer.send(WritePacket{message,done})=>sent.map_err(|_|disconnected())?,
        }
        tokio::select! {
            _=self.closed.cancelled()=>Err(disconnected()),
            result=received=>result.map_err(|_|disconnected())?.map_err(|e|ServerError::new(-32603,e)),
        }
    }
    pub(super) fn response(&self, response: Response) {
        if let Some(id) = &response.id {
            if let Ok(mut pending) = self.outgoing.lock() {
                if let Some(sender) = pending.remove(id) {
                    let _ = sender.send(response);
                }
            } else {
                self.closed.cancel();
            }
        }
    }
    pub(super) fn retire_outgoing(&self) {
        if let Ok(mut pending) = self.outgoing.lock() {
            pending.clear();
        }
    }
    pub(super) async fn close(&self) {
        self.closed.cancel();
        self.retire_outgoing();
        let mut incoming = self.incoming.lock().await;
        for cancellation in incoming.values() {
            cancellation.cancel();
        }
        incoming.clear();
    }
}
impl ClientHandle {
    pub async fn closed(&self) {
        self.session.closed.cancelled().await;
    }
    pub(super) fn context(&self, cancellation: Cancellation) -> Context {
        let encoding = match self.session.encoding.load(Ordering::Acquire) {
            0 => CoordinateEncoding::Utf8,
            2 => CoordinateEncoding::Utf32,
            _ => CoordinateEncoding::Utf16,
        };
        Context {
            client: self.clone(),
            cancellation,
            encoding,
        }
    }
    pub(super) fn allocate_id(&self) -> ServerResult<RequestId> {
        self.session
            .next_id
            .fetch_update(Ordering::AcqRel, Ordering::Acquire, |id| id.checked_add(1))
            .map(RequestId::Number)
            .map_err(|_| ServerError::new(-32603, "Client request ID space exhausted"))
    }
}
pub(super) fn disconnected() -> ServerError {
    ServerError::new(-32603, "LSP connection closed")
}

/// Drop removes abandoned outgoing requests even when their awaiting future is cancelled.
pub(super) struct PendingGuard {
    pub session: Arc<Session>,
    pub id: Option<RequestId>,
}
impl PendingGuard {
    pub(super) fn finish(mut self) {
        if let Some(id) = self.id.take() {
            if let Ok(mut pending) = self.session.outgoing.lock() {
                pending.remove(&id);
            }
        }
    }
}
impl Drop for PendingGuard {
    fn drop(&mut self) {
        if let Some(id) = self.id.take() {
            // This lock only protects ID bookkeeping and is never held across an await.
            if let Ok(mut pending) = self.session.outgoing.lock() {
                pending.remove(&id);
            }
        }
    }
}
