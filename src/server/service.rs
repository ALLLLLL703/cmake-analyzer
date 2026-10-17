//! Start, address and stop the bounded analysis service.
//! The mutex protects only lifecycle state; it is released before queue or join waits.
//! Notification sends acknowledge queue admission, not analysis completion. Queries
//! carry a one-use reply channel and act as ordering barriers in [`runtime`].
// AI-authored bounded Workspace service.
mod automatic;
mod configuration;
mod configure;
mod documents;
mod projects;
mod reports;
mod runtime;
use super::model::*;
use lspally::server::model::*;
use tokio::sync::{mpsc, oneshot};

/// Maximum queued actor commands; senders wait for space rather than dropping edits.
/// Example: `mpsc::channel(SERVICE_QUEUE_CAPACITY)` bounds the startup queue.
static SERVICE_QUEUE_CAPACITY: usize = 64;
/// Maximum notifications applied before one analysis flush, even when more are queued.
/// A query ends the batch early. Example: `0..EDIT_BATCH_SIZE` bounds one edit pass.
static EDIT_BATCH_SIZE: usize = 64;

impl CmakeAnalyzer {
    /// Clone the bounded service sender under a short-held lifecycle lock. No queue send or I/O occurs.
    ///
    /// # Arguments
    /// - `self` `[in]`: Adapter whose lifecycle handle addresses the shared service; not mutated directly.
    ///
    /// # Returns
    /// The active sender on success; an error if lifecycle state is unavailable.
    ///
    /// # Errors
    /// Returns an internal error for a poisoned mutex, or not-initialized when no service handle is installed.
    ///
    /// # Examples
    /// ```text
    /// let sender = server.sender()?;
    /// sender.send(ServiceCommand::RuntimeRefresh).await?;
    /// ```
    pub(super) fn sender(&self) -> ServerResult<mpsc::Sender<ServiceCommand>> {
        self.state
            .lock()
            .map_err(|_| ServerError::new(-32603, "Server state poisoned"))?
            .service
            .as_ref()
            .map(|service| service.sender.clone())
            .ok_or_else(|| ServerError::new(-32002, "Workspace service is not running"))
    }
    /// Wait for bounded queue capacity and enqueue one command. Does not wait for its analysis or reply.
    ///
    /// # Arguments
    /// - `self` `[in]`: Adapter whose lifecycle handle addresses the shared service; not mutated directly.
    /// - `command` `[in]`: Owned notification or query transferred to the service on success.
    ///
    /// # Returns
    /// `Ok(())` once enqueued; `Err` if no live sender or the receiver stopped.
    ///
    /// # Errors
    /// Propagates sender lookup failures; a closed service queue becomes an internal error.
    ///
    /// # Examples
    /// ```text
    /// server.send(ServiceCommand::Open(document)).await?;
    /// ```
    pub(super) async fn send(&self, command: ServiceCommand) -> ServerResult<()> {
        self.sender()?
            .send(command)
            .await
            .map_err(|_| ServerError::new(-32603, "Workspace service stopped"))
    }
    /// Enqueue an ordered document-symbol query and await its one-use reply. Does not publish diagnostics itself.
    ///
    /// # Arguments
    /// - `self` `[in]`: Adapter whose lifecycle handle addresses the shared service; not mutated directly.
    /// - `context` `[in]`: Request context consumed for cancellation and/or client session data.
    /// - `uri` `[in]`: Consumed local document URI; open documents retain their client spelling.
    ///
    /// # Returns
    /// `Ok(Some(response))` with nested/flat symbols, `Ok(None)` when disabled, or the query/transport error.
    ///
    /// # Errors
    /// Cancellation, unavailable/stopped service, or ownership/snapshot/range conversion errors from the actor. Dropping the receiver lets an unstarted query be skipped.
    ///
    /// # Examples
    /// ```text
    /// let symbols = server.query_symbols(context, uri).await?;
    /// ```
    pub(super) async fn query_symbols(
        &self,
        context: Context,
        uri: Uri,
    ) -> ServerResult<Option<DocumentSymbolResponse>> {
        let (reply, response) = oneshot::channel();
        tokio::select! {
            _=context.cancellation.cancelled()=>Err(ServerError::cancelled()),
            result=async {self.send(ServiceCommand::Symbols{uri,reply}).await?;response.await.map_err(|_|ServerError::new(-32603,"Workspace service stopped"))?}=>result,
        }
    }
    /// Remove the service handle, cancel its work and join it. Safe to call when already stopped.
    /// Does not hold the lifecycle mutex while waiting for the actor or background jobs.
    ///
    /// # Arguments
    /// - `self` `[in,out]`: Shared adapter state: the active service handle is taken and its stop signal is set; options remain unchanged.
    ///
    /// # Returns
    /// `Ok(())` when there was no service or it joined successfully; `Err` on lock/join failure.
    ///
    /// # Errors
    /// Poisoned lifecycle mutex or a failed/panicked actor task. The taken handle is not restored.
    ///
    /// # Algorithm
    /// Take the handle exactly once, signal cancellation, then await the actor; actor exit drains runtime jobs.
    ///
    /// # Flow
    /// ```text
    ///  [Lifecycle mutex]
    ///         |
    ///    <Handle present?>
    ///     /          \
    ///    no          yes
    ///    |            |
    ///  [Ok]      [Cancel + join]
    ///                 |
    ///            [Ok / join error]
    /// ```
    ///
    /// # Examples
    /// ```no_run
    /// use cmake_analyzer::server::{CmakeAnalyzer, ServerOptions};
    /// # async fn shutdown() -> Result<(), Box<dyn std::error::Error>> {
    /// let server = CmakeAnalyzer::new(ServerOptions::new("/work/demo".into()));
    /// // In a running session, call this after Wrapper::run returns, including EOF.
    /// server.stop().await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn stop(&self) -> ServerResult<()> {
        let service = self
            .state
            .lock()
            .map_err(|_| ServerError::new(-32603, "Server state poisoned"))?
            .service
            .take();
        if let Some(mut service) = service {
            service.stop.cancel();
            (&mut service.task)
                .await
                .map_err(|e| ServerError::new(-32603, e.to_string()))?;
        }
        Ok(())
    }
    /// Move prepared roots/configuration into a spawned actor and install its bounded sender.
    /// No project analysis is performed synchronously while the lifecycle lock is held.
    ///
    /// # Arguments
    /// - `self` `[in,out]`: Shared lifecycle state: roots/config are taken and the service handle is installed.
    /// - `context` `[in]`: Request context consumed for cancellation and/or client session data.
    ///
    /// # Returns
    /// `Ok(())` after task creation; `Err` if state cannot be used or startup was already performed.
    ///
    /// # Errors
    /// Poisoned mutex, already-running service, or missing initialized configuration. Roots are taken before the missing-config check.
    ///
    /// # Panics
    /// Tokio spawning requires a running Tokio runtime.
    ///
    /// # Algorithm
    /// Lock lifecycle state, reject a second start, create the bounded channel and cancellation signal, move initialization inputs, then spawn runtime::run.
    ///
    /// # Flow
    /// ```text
    ///  [Initialized state]
    ///          |
    ///     <Already started?>
    ///       /         \
    ///     yes         no
    ///      |           |
    ///   [Error]   [Take roots/config]
    ///                  |
    ///           [Spawn + install handle]
    /// ```
    ///
    /// # Examples
    /// ```text
    /// server.start(context)?; // Called by initialized, after initialize_server.
    /// ```
    pub(super) fn start(&self, context: Context) -> ServerResult<()> {
        let mut state = self
            .state
            .lock()
            .map_err(|_| ServerError::new(-32603, "Server state poisoned"))?;
        if state.service.is_some() {
            return Err(ServerError::invalid("Workspace service already started"));
        }
        let (sender, receiver) = mpsc::channel(SERVICE_QUEUE_CAPACITY);
        let stop = Cancellation::default();
        let roots = std::mem::take(&mut state.roots);
        let hierarchical = state.hierarchical;
        let config = state
            .config
            .take()
            .ok_or_else(|| ServerError::new(-32002, "Server is not initialized"))?;
        let task = tokio::spawn(runtime::run(
            roots,
            config,
            context,
            hierarchical,
            state.hover_markdown,
            receiver,
            stop.clone(),
        ));
        state.service = Some(ServiceHandle { sender, stop, task });
        Ok(())
    }
}
/// Convert a displayable failure to the protocol internal-error code. No I/O or state mutation.
///
/// # Arguments
/// - `error` `[in]`: Consumed failure or message formatted into the returned error.
///
/// # Returns
/// A ServerError with code -32603 and the formatted message.
///
/// # Examples
/// ```text
/// let error = internal("analysis worker stopped");
/// ```
pub(super) fn internal(error: impl std::fmt::Display) -> ServerError {
    ServerError::new(-32603, error.to_string())
}
