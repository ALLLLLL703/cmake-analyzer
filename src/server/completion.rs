//! Completion request entry and assembly of syntax, schema, discovery and rendering.
//! Requests cross the service edit barrier; no project is executed and edits are client-applied.

mod argument_context;
mod arguments;
mod candidates;
mod context;
mod inputs;
pub mod model;
mod names;
mod prepare;
mod query;
mod render;
mod roles;

use super::model::{CmakeAnalyzer, ServiceCommand};
use lspally::server::model::*;
use tokio::sync::oneshot;

impl CmakeAnalyzer {
    /// Queue completion with negotiated snippet support and await the service reply.
    /// Reads session state and sends a service message; does not edit text or run project code.
    ///
    /// # Arguments
    /// - `self` `[in]`: Analyzer handle; its short-lived state lock supplies client snippet capability.
    /// - `context` `[in]`: Request cancellation and client context; cancellation can end the wait.
    /// - `params` `[in]`: Document URI and negotiated-coordinate cursor; trigger metadata is not used.
    ///
    /// # Returns
    /// `Ok(Some(response))` carries completion items (possibly empty); any `Ok(None)` service response is forwarded.
    ///
    /// # Errors
    /// Cancelled requests, poisoned server state, failed service sends, or a stopped reply channel return `Err`.
    ///
    /// # Algorithm
    /// Read snippet support before awaiting. Send an ordered service command and race its response against cancellation.
    ///
    /// # Flow
    /// ```text
    ///  [Session state]          [Request cancellation]
    ///         |                          |
    ///         v                          v
    ///  [Queue Complete] --------> <reply or cancel?>
    ///                                   /       \
    ///                             [response]  [cancel error]
    /// ```
    ///
    /// # Examples
    /// ```text
    /// let response = analyzer.query_completion(context, params).await?;
    /// ```
    pub(super) async fn query_completion(
        &self,
        context: Context,
        params: CompletionParams,
    ) -> ServerResult<Option<CompletionResponse>> {
        let snippets = self
            .state
            .lock()
            .map_err(|_| ServerError::new(-32603, "Server state poisoned"))?
            .completion_snippets;
        let (reply, response) = oneshot::channel();
        let position = params.text_document_position;
        tokio::select! {
            _=context.cancellation.cancelled()=>Err(ServerError::cancelled()),
            result=async {
                self.send(ServiceCommand::Complete {uri:position.text_document.uri,position:position.position,snippets,cancel:context.cancellation.clone(),reply}).await?;
                response.await.map_err(|_|ServerError::new(-32603,"Workspace service stopped"))?
            }=>result,
        }
    }
}

// AI-generated test-module documentation.
/// Command-completion syntax and snippet regression suite; uses retained Workspace snapshots.
#[cfg(test)]
#[path = "../../tests/unit/server/completion.rs"]
mod tests;
