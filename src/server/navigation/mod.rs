//! Definition/reference request assembly over source-order service barriers.
//! Symbol navigation follows bound references; dependency-path definitions have a separate resolution path.

mod definition;
mod dependencies;
mod locations;
pub(in crate::server) mod model;
mod query;
mod references;
mod targets;
use super::model::{CmakeAnalyzer, ServiceCommand};
use lspally::server::model::*;
use tokio::sync::oneshot;
impl CmakeAnalyzer {
    /// Queue an ordered navigation request and await its cancellable reply.
    /// Sends a service message; no direct analysis or text mutation.
    ///
    /// # Arguments
    /// - `self` `[in]`: Analyzer service handle.
    /// - `context` `[in]`: Request cancellation forwarded into service and worker.
    /// - `position` `[in]`: Document URI and cursor in negotiated coordinates.
    /// - `query` `[in]`: Definition or references, including declaration policy.
    ///
    /// # Returns
    /// `Ok(locations)` contains all deduplicated results; empty means no bound/usable location.
    ///
    /// # Errors
    /// `Err` forwards service errors, cancellation, failed sends or a stopped reply channel.
    ///
    /// # Examples
    /// ```text
    /// let locations = analyzer.navigate(context, position, Query::Definition).await?;
    /// ```
    async fn navigate(
        &self,
        context: Context,
        position: TextDocumentPositionParams,
        query: model::Query,
    ) -> ServerResult<Vec<Location>> {
        let (reply, response) = oneshot::channel();
        self.send(ServiceCommand::Navigate {
            uri: position.text_document.uri,
            position: position.position,
            query,
            cancel: context.cancellation.clone(),
            reply,
        })
        .await?;
        tokio::select! {_=context.cancellation.cancelled()=>Err(ServerError::cancelled()),result=response=>result.map_err(|_|ServerError::new(-32603,"Workspace service stopped"))?}
    }
}
