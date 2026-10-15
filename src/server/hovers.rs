// AI-authored hover dispatch; individual features live in hovers/.
mod builtins;
mod commands;
mod documentation;
pub mod include;
mod model;
mod render;
mod targets;
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
        let path = super::uri::to_path(&uri)?;
        let owners = self.owners(&path);
        let Some(snapshot) = owners.iter().find_map(|owner| {
            self.projects[*owner]
                .workspace
                .snapshot(&path)
                .ok()
                .flatten()
        }) else {
            return Ok(None);
        };
        let limit = owners
            .iter()
            .map(|owner| self.projects[*owner].settings.runtime.max_view_bytes)
            .min()
            .unwrap_or_else(|| crate::config::Runtime::default().max_view_bytes);
        let encoding = self.encoding;
        let target =
            tokio::task::spawn_blocking(move || targets::locate(&snapshot, &position, encoding))
                .await
                .map_err(super::service::internal)??;
        let Some(target) = target else {
            return Ok(None);
        };
        Ok(match target.kind {
            model::TargetKind::Variable => Some(variables::hover(self, &path, target, limit)),
            model::TargetKind::Command => commands::hover(self, &path, target, limit),
        })
    }
}
