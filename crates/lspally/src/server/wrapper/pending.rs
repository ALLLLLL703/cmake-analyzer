// AI-generated implementation.
//! Flush acknowledgements, response correlation and cancellation cleanup.
//!
//! The reader removes outgoing IDs before delivering replies. A drop guard removes
//! abandoned waits. Short outgoing-map locks never span an asynchronous wait.
//! Session closure wakes client waits and cancels tracked incoming requests.
use crate::{
    rpc::{Message, RequestId, Response},
    server::model::*,
};
use std::sync::{Arc, atomic::Ordering};
use tokio::sync::oneshot;

impl Session {
    /// Queue one owned message and await its write/flush acknowledgement.
    /// Waits for bounded output capacity; closure can interrupt either wait. Does not await a reply.
    ///
    /// # Arguments
    /// - `self` `[in,out]`: shared session whose writer queue receives one appended packet;
    ///   the receiver reference is shared, not exclusively borrowed.
    /// - `message` `[in]`: envelope moved to the writer; not recoverable on error/cancellation.
    ///
    /// # Returns
    /// `Ok(())` after the writer flushes; `Err` on closure, dropped acknowledgement or writer failure.
    ///
    /// # Errors
    /// Closure/queue loss returns the disconnected error (-32603); writer failure becomes -32603
    /// with the writer's text. A failed/cancelled call may already have queued or partly written data.
    ///
    /// # Algorithm
    /// Race closure against queue admission, then against the one-use flush acknowledgement.
    /// No pending-map lock is held during either wait.
    ///
    /// # Flow
    /// ```text
    ///           [owned message]
    ///                  |
    ///         <closed or queue ready?>
    ///            /             \
    ///        [error]      [append packet]
    ///                          |
    ///                 <closed or ack?>
    ///                   /           \
    ///               [error]     [flush result]
    /// ```
    /// # Examples
    /// ```text
    /// session.send(Message::Response(Response::error(None,
    ///     crate::rpc::ResponseError::new(-32700, "Invalid JSON")))).await?;
    /// ```
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
    /// Deliver a reply to its outgoing waiter, removing the correlation entry first.
    /// Null/unknown IDs and dropped receivers are ignored; no stream I/O.
    ///
    /// # Arguments
    /// - `self` `[in,out]`: shared outgoing map loses a matching ID; poisoned lock cancels the session.
    /// - `response` `[in]`: owned reply moved to the one-use sender or discarded when unmatched.
    ///
    /// # Returns
    /// `()`, whether delivered or ignored.
    ///
    /// # Algorithm
    /// Ignore null IDs; otherwise lock only long enough to remove and deliver to a matching
    /// one-use sender. Lock poisoning closes the session instead of guessing correlation.
    ///
    /// # Flow
    /// ```text
    ///                 <reply ID?>
    ///                 /        \
    ///               null       key
    ///                |          |
    ///             [ignore]   <lock map?>
    ///                       /          \
    ///                   poisoned       held
    ///                      |            |
    ///                   [close]    <remove match?>
    ///                               /          \
    ///                           [deliver]    [ignore]
    /// ```
    /// # Examples
    /// ```text
    /// session.response(Response::success::<lsp_types::request::Shutdown>(RequestId::Number(0), ())?);
    /// ```
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
    /// Drop all outgoing waiters so awaiting client calls wake with disconnection; no I/O.
    /// Used after successful shutdown and during closure. A poisoned map is left untouched.
    ///
    /// # Arguments
    /// - `self` `[in,out]`: shared outgoing map is cleared, dropping senders rather than sending replies.
    ///
    /// # Returns
    /// `()`, including when the map lock is poisoned.
    ///
    /// # Examples
    /// ```text
    /// session.retire_outgoing();
    /// ```
    pub(super) fn retire_outgoing(&self) {
        if let Ok(mut pending) = self.outgoing.lock() {
            pending.clear();
        }
    }
    /// Mark closure, retire outgoing waits and cancel/clear tracked incoming requests; no stream I/O.
    /// Does not join tasks or drain output; [`crate::server::Wrapper::run`] owns task joining.
    ///
    /// # Arguments
    /// - `self` `[in,out]`: shared closed token is set; outgoing/incoming maps are cleared and
    ///   every incoming cancellation token is signalled. Waits for the incoming-map lock.
    ///
    /// # Returns
    /// `()` after incoming cleanup completes; repeated calls are harmless.
    ///
    /// # Algorithm
    /// Signal closure before map cleanup so blocked senders/waiters can wake immediately.
    ///
    /// # Flow
    /// ```text
    ///              [set closed token]
    ///                 /           \
    ///       [wake session waits] [clear outgoing]
    ///                                 |
    ///                          [lock incoming]
    ///                                 |
    ///                          [cancel each token]
    ///                                 |
    ///                          [clear incoming]
    /// ```
    /// # Examples
    /// ```text
    /// session.close().await;
    /// assert!(session.closed.is_cancelled());
    /// ```
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
    /// Wait until this client's session closes; no mutation or I/O by this wait.
    /// Returns immediately if closure was already signalled; does not imply task joining is complete.
    ///
    /// # Arguments
    /// - `self` `[in]`: client handle whose shared closure token is observed.
    ///
    /// # Returns
    /// `()` once closed; can wait indefinitely while the connection remains live.
    ///
    /// # Examples
    /// A live handle supplied by a business handler is required; run on a Tokio executor.
    /// ```no_run
    /// # async fn observe(client: lspally::server::ClientHandle) {
    /// client.closed().await;
    /// # }
    /// ```
    pub async fn closed(&self) {
        self.session.closed.cancelled().await;
    }
    /// Snapshot negotiated coordinates and attach cancellation to a business context; no I/O.
    /// Encoding tags 0/2 mean UTF-8/UTF-32; all other tags use UTF-16.
    ///
    /// # Arguments
    /// - `self` `[in]`: handle cloned into the returned context; session state is only read.
    /// - `cancellation` `[in]`: token moved into the context; clones share cancellation state.
    ///
    /// # Returns
    /// A new context retaining this client, the supplied token and the current encoding.
    ///
    /// # Examples
    /// ```text
    /// let context = client.context(Cancellation::default());
    /// assert!(!context.cancellation.is_cancelled());
    /// ```
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
    /// Reserve the next numeric outgoing key with an atomic checked increment; no I/O.
    /// IDs are not reused, even when a later request fails before sending.
    ///
    /// # Arguments
    /// - `self` `[in,out]`: shared counter advances by one on success; unchanged at exhaustion.
    ///
    /// # Returns
    /// `Ok(RequestId::Number(previous_counter))`, or `Err` once increment would overflow `i32`.
    ///
    /// # Errors
    /// Returns -32603 at counter exhaustion; does not wrap or reserve a pending-map entry.
    ///
    /// # Examples
    /// ```text
    /// let id = client.allocate_id()?;
    /// assert!(matches!(id, RequestId::Number(_)));
    /// ```
    pub(super) fn allocate_id(&self) -> ServerResult<RequestId> {
        self.session
            .next_id
            .fetch_update(Ordering::AcqRel, Ordering::Acquire, |id| id.checked_add(1))
            .map(RequestId::Number)
            .map_err(|_| ServerError::new(-32603, "Client request ID space exhausted"))
    }
}
/// Build the standard connection-closed error without changing state or performing I/O.
///
/// # Arguments
/// None.
///
/// # Returns
/// A -32603 server error with message `LSP connection closed`.
///
/// # Examples
/// ```text
/// assert_eq!(disconnected().code, -32603);
/// ```
pub(super) fn disconnected() -> ServerError {
    ServerError::new(-32603, "LSP connection closed")
}

/// Own cleanup responsibility for one pending outgoing ID.
/// Used by the client request helper; [`Self::finish`] and [`Drop::drop`] both remove the
/// entry without I/O. Cleanup is best-effort if the outgoing-map lock is poisoned.
///
/// # Ownership
/// ```text
///     [client request future]       [session outgoing map]
///              |                           |
///       [PendingGuard owns ID] ---- remove entry
///              |                           ^
///         <completion?>                    |
///          /         \                     |
///     [finish]    [future dropped]          |
///          \         |                     |
///           +------ drop -----------------+
/// ```
/// # Examples
/// ```text
/// let guard = PendingGuard { session: session.clone(), id: Some(id) };
/// // Await a reply, then retire any remaining map entry.
/// guard.finish();
/// ```
pub(super) struct PendingGuard {
    /// Session retaining the outgoing map; example: `let closed = guard.session.closed.is_cancelled();`.
    pub session: Arc<Session>,
    /// Cleanup key; `None` disarms cleanup. Example: `assert!(guard.id.is_some());`.
    pub id: Option<RequestId>,
}
impl PendingGuard {
    /// Consume the guard and remove its outgoing entry, disarming its later drop; no I/O.
    /// A reply normally removed the entry already; removal is idempotent.
    ///
    /// # Arguments
    /// - `self` `[in,out]`: consumed guard; its ID is taken and the shared pending entry is removed.
    ///   A poisoned lock skips map removal, but the guard remains disarmed.
    ///
    /// # Returns
    /// `()` after best-effort cleanup.
    ///
    /// # Examples
    /// ```text
    /// let guard = PendingGuard { session: session.clone(), id: Some(id) };
    /// guard.finish();
    /// ```
    pub(super) fn finish(mut self) {
        if let Some(id) = self.id.take() {
            if let Ok(mut pending) = self.session.outgoing.lock() {
                pending.remove(&id);
            }
        }
    }
}
impl Drop for PendingGuard {
    /// Remove an abandoned outgoing wait on scope exit or future cancellation; no I/O.
    /// A poisoned map lock skips removal. No lock spans an asynchronous wait.
    ///
    /// # Arguments
    /// - `self` `[in,out]`: takes the cleanup ID and removes its entry from the shared outgoing map.
    ///
    /// # Returns
    /// `()` after best-effort cleanup; an already-disarmed guard does nothing.
    ///
    /// # Examples
    /// ```text
    /// let guard = PendingGuard { session: session.clone(), id: Some(id) };
    /// std::mem::drop(guard); // invokes PendingGuard::drop and retires this wait
    /// ```
    fn drop(&mut self) {
        if let Some(id) = self.id.take() {
            // This lock only protects ID bookkeeping and is never held across an await.
            if let Ok(mut pending) = self.session.outgoing.lock() {
                pending.remove(&id);
            }
        }
    }
}
