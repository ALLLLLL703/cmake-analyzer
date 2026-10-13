// AI-generated implementation.
use super::ClientHandle;
use std::{
    fmt,
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
};
use tokio::sync::Notify;

pub type ServerResult<T> = Result<T, ServerError>;
#[derive(Debug)]
pub struct ServerError {
    pub code: i32,
    pub message: String,
}
impl ServerError {
    pub fn new(code: i32, message: impl Into<String>) -> Self {
        Self {
            code,
            message: message.into(),
        }
    }
    pub fn unsupported() -> Self {
        Self::new(-32601, "Method not supported")
    }
    pub fn invalid(message: impl Into<String>) -> Self {
        Self::new(-32602, message)
    }
    pub fn cancelled() -> Self {
        Self::new(-32800, "Request cancelled")
    }
}
impl fmt::Display for ServerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.code, self.message)
    }
}
impl std::error::Error for ServerError {}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CoordinateEncoding {
    Utf8,
    Utf16,
    Utf32,
}
#[derive(Clone)]
pub struct Context {
    pub client: ClientHandle,
    pub cancellation: Cancellation,
    /// Positions in business models use this negotiated encoding, not byte offsets.
    pub encoding: CoordinateEncoding,
}
#[derive(Clone, Default)]
pub struct Cancellation(pub(crate) Arc<CancellationState>);
#[derive(Default)]
pub(crate) struct CancellationState {
    flag: AtomicBool,
    notify: Notify,
}
impl Cancellation {
    pub fn is_cancelled(&self) -> bool {
        self.0.flag.load(Ordering::Acquire)
    }
    pub fn cancel(&self) {
        self.0.flag.store(true, Ordering::Release);
        self.0.notify.notify_waiters();
    }
    pub async fn cancelled(&self) {
        let notified = self.0.notify.notified();
        tokio::pin!(notified);
        notified.as_mut().enable();
        if !self.is_cancelled() {
            notified.await;
        }
    }
}
