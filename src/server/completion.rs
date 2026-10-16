mod arguments;
mod candidates;
mod context;
pub mod model;
mod query;
mod render;

use super::model::{CmakeAnalyzer, ServiceCommand};
use lspally::server::model::*;
use tokio::sync::oneshot;

impl CmakeAnalyzer {
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

#[cfg(test)]
#[path = "../../tests/unit/server/completion.rs"]
mod tests;
