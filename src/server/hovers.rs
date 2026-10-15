// AI-authored hover dispatch; individual features live in hovers/.
mod variables;
use super::model::*;
use lspally::server::model::*;
use tokio::sync::oneshot;
impl CmakeAnalyzer {
    pub(in crate::server) async fn hover_variable(
        &self,
        context: Context,
        params: HoverParams,
    ) -> ServerResult<Option<Hover>> {
        let position = params.text_document_position_params;
        let (reply, response) = oneshot::channel();
        self.send(ServiceCommand::Hover {
            uri: position.text_document.uri,
            position: position.position,
            reply,
        })
        .await?;
        tokio::select! {_=context.cancellation.cancelled()=>Err(ServerError::cancelled()),result=response=>result.map_err(|_|ServerError::new(-32603,"Workspace service stopped"))?}
    }
}
impl Service {
    pub(in crate::server) async fn variable_hover(
        &self,
        uri: Uri,
        position: Position,
    ) -> ServerResult<Option<Hover>> {
        variables::hover(self, uri, position).await
    }
}
