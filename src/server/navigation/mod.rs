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
