// AI-generated implementation.
//! Handler context, protocol errors and cooperative cancellation; no file/network I/O.
use super::ClientHandle;
use std::{
    fmt,
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
};
use tokio::sync::Notify;

/// Handler success or a protocol error that the wrapper can return to the peer.
pub type ServerResult<T> = Result<T, ServerError>;
/// Error returned by business handlers and transport adapters; construction has no side effects.
/// [`ServerResult`] carries this to the wrapper, which keeps request IDs outside this type.
///
/// # Examples
/// ```
/// use lspally::server::model::ServerError;
/// let error = ServerError::invalid("missing document");
/// assert_eq!(error.code, -32602);
/// ```
#[derive(Debug)]
pub struct ServerError {
    /// JSON-RPC or LSP error code. Example: `ServerError::cancelled().code == -32800`.
    pub code: i32,
    /// Human-readable explanation, not an RPC envelope. Example: `ServerError::invalid("bad range").message`.
    pub message: String,
}
impl ServerError {
    /// Store an error code and owned message; no I/O or validation.
    ///
    /// # Arguments
    /// - `code` `[in]`: protocol error number to retain unchanged.
    /// - `message` `[in]`: explanation converted to owned text.
    /// # Returns
    /// An error containing the supplied code and text; no output parameters.
    /// # Examples
    /// ```
    /// use lspally::server::model::ServerError;
    /// assert_eq!(ServerError::new(-32603, "failed").code, -32603);
    /// ```
    pub fn new(code: i32, message: impl Into<String>) -> Self {
        Self {
            code,
            message: message.into(),
        }
    }
    /// Construct MethodNotFound for an unsupported handler; no side effects.
    ///
    /// # Arguments
    /// None.
    /// # Returns
    /// Error -32601 with the fixed message `Method not supported`.
    /// # Examples
    /// ```
    /// use lspally::server::model::ServerError;
    /// assert_eq!(ServerError::unsupported().code, -32601);
    /// ```
    pub fn unsupported() -> Self {
        Self::new(-32601, "Method not supported")
    }
    /// Construct InvalidParams with a caller-supplied explanation; no side effects.
    ///
    /// # Arguments
    /// - `message` `[in]`: explanation of the rejected input.
    /// # Returns
    /// Error -32602 containing the supplied explanation.
    /// # Examples
    /// ```
    /// use lspally::server::model::ServerError;
    /// assert_eq!(ServerError::invalid("missing URI").message, "missing URI");
    /// ```
    pub fn invalid(message: impl Into<String>) -> Self {
        Self::new(-32602, message)
    }
    /// Construct RequestCancelled; does not cancel any running task itself.
    ///
    /// # Arguments
    /// None.
    /// # Returns
    /// Error -32800 with the fixed message `Request cancelled`.
    /// # Examples
    /// ```
    /// use lspally::server::model::ServerError;
    /// assert_eq!(ServerError::cancelled().code, -32800);
    /// ```
    pub fn cancelled() -> Self {
        Self::new(-32800, "Request cancelled")
    }
}
impl fmt::Display for ServerError {
    /// Write `code: message` to the formatter; no transport output.
    ///
    /// # Arguments
    /// - `self` `[in]`: error to display.
    /// - `f` `[out]`: formatter receiving appended error text; existing output is not cleared.
    /// # Returns
    /// `Ok(())` after writing, or the formatter's error.
    /// # Errors
    /// Returns the error reported by the formatter's writer.
    /// # Examples
    /// ```
    /// use lspally::server::model::ServerError;
    /// assert_eq!(ServerError::cancelled().to_string(), "-32800: Request cancelled");
    /// ```
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.code, self.message)
    }
}
impl std::error::Error for ServerError {}
/// Unit used by [`super::Position::character`] in a negotiated session; no side effects.
/// [`Context::encoding`] tells handlers how to interpret client coordinates.
///
/// # Examples
/// ```
/// use lspally::server::model::CoordinateEncoding;
/// let encoding = CoordinateEncoding::Utf16;
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CoordinateEncoding {
    /// UTF-8 code units (bytes), not grapheme clusters. Example: `CoordinateEncoding::Utf8`.
    Utf8,
    /// UTF-16 code units; supplementary characters count twice. Example: `CoordinateEncoding::Utf16`.
    Utf16,
    /// UTF-32 code units (Unicode scalar values). Example: `CoordinateEncoding::Utf32`.
    Utf32,
}
/// Per-call services supplied by the wrapper to [`super::ILanguageServer`] handlers.
/// Cloning shares the client session and cancellation state; it does not start work.
/// [`Cancellation`] is cooperative: handlers must check or await it themselves.
///
/// # Examples
/// ```
/// use lspally::server::model::Context;
/// // A running wrapper supplies the context to a handler.
/// fn check(context: &Context) -> bool { context.cancellation.is_cancelled() }
/// ```
#[derive(Clone)]
pub struct Context {
    /// Outgoing peer handle. Example: `let client = context.client.clone();`.
    pub client: ClientHandle,
    /// This call's stop signal. Example: `context.cancellation.is_cancelled()`.
    pub cancellation: Cancellation,
    /// Positions use this negotiated unit, not unconditional byte offsets.
    /// Example: `matches!(context.encoding, CoordinateEncoding::Utf16)`.
    pub encoding: CoordinateEncoding,
}
/// Shared one-way cancellation signal used by [`Context`] and session shutdown.
/// Clones observe the same flag. Cancelling wakes waiters but does not abort tasks.
///
/// # Flow
/// ```text
/// [owner]                  [handler clone]
///    |                            |
/// cancel()                   cancelled()
///    |                            |
///    v                            v
/// [flag = true] -------> [registered waiter]
///    |                            |
///    +--- notify_waiters() --------+
///                                 v
///                         [handler resumes]
/// ```
/// # Examples
/// ```
/// use lspally::server::model::Cancellation;
/// let signal = Cancellation::default();
/// let observer = signal.clone();
/// signal.cancel();
/// assert!(observer.is_cancelled());
/// ```
#[derive(Clone, Default)]
pub struct Cancellation(
    /// Shared flag and wake-up queue. Example in this module: `let shared = signal.0.clone();`.
    pub(crate) Arc<CancellationState>,
);
/// Flag and waiter queue shared by [`Cancellation`] clones; construction has no I/O.
/// The flag makes cancellation persistent even when no waiter is registered.
///
/// # Examples
/// ```text
/// let state = CancellationState::default();
/// assert!(!state.flag.load(Ordering::Acquire));
/// ```
#[derive(Default)]
pub(crate) struct CancellationState {
    /// Persistent cancellation bit. Example: `state.flag.load(Ordering::Acquire)`.
    flag: AtomicBool,
    /// Wake-ups for registered async waiters. Example: `state.notify.notify_waiters()`.
    notify: Notify,
}
impl Cancellation {
    /// Read the shared cancellation flag; no mutation or I/O.
    ///
    /// # Arguments
    /// - `self` `[in]`: signal to inspect.
    /// # Returns
    /// `true` once any clone has been cancelled; otherwise `false`.
    /// # Examples
    /// ```
    /// use lspally::server::model::Cancellation;
    /// assert!(!Cancellation::default().is_cancelled());
    /// ```
    pub fn is_cancelled(&self) -> bool {
        self.0.flag.load(Ordering::Acquire)
    }
    /// Set the shared flag permanently and wake all current waiters; no I/O.
    /// Repeated calls are allowed; they never reset cancellation.
    ///
    /// # Arguments
    /// - `self` `[in,out]`: shared state whose flag is set and waiters are notified.
    /// # Returns
    /// `()` after notifying waiters; does not join or terminate their tasks.
    /// # Examples
    /// ```
    /// use lspally::server::model::Cancellation;
    /// let signal = Cancellation::default();
    /// signal.cancel();
    /// assert!(signal.is_cancelled());
    /// ```
    pub fn cancel(&self) {
        self.0.flag.store(true, Ordering::Release);
        self.0.notify.notify_waiters();
    }
    /// Wait until cancellation, registering a waiter but doing no I/O.
    /// Dropping this future does not cancel the shared signal.
    ///
    /// # Arguments
    /// - `self` `[in,out]`: shared signal whose waiter queue is temporarily extended.
    /// # Returns
    /// `()` immediately if already cancelled, or after a cancellation wake-up.
    /// # Algorithm
    /// Register before checking the persistent flag so a concurrent cancellation
    /// cannot be lost between the check and waiting.
    /// # Flow
    /// ```text
    ///       [register waiter]
    ///               |
    ///       <flag already set?>
    ///         /            \
    ///       yes             no
    ///        |              |
    ///        |       [await notification]
    ///        |              |
    ///        +------+-------+
    ///               v
    ///           [return ()]
    /// ```
    /// # Examples
    /// ```
    /// use lspally::server::model::Cancellation;
    /// # tokio::runtime::Builder::new_current_thread().build().unwrap().block_on(async {
    /// let signal = Cancellation::default();
    /// signal.cancel();
    /// signal.cancelled().await;
    /// # });
    /// ```
    pub async fn cancelled(&self) {
        let notified = self.0.notify.notified();
        tokio::pin!(notified);
        notified.as_mut().enable();
        if !self.is_cancelled() {
            notified.await;
        }
    }
}
