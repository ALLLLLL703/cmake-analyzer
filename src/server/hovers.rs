// AI-authored hover dispatch; individual features live in hovers/.
mod builtins;
mod commands;
mod documentation;
mod model;
mod render;
mod targets;
mod variables;
use super::model::*;
pub(in crate::server) use builtins::model::LocalHelp;
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
            cancel: context.cancellation.clone(),
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
        &mut self,
        uri: Uri,
        position: Position,
        cancel: Cancellation,
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
        if cancel.is_cancelled() {
            return Err(ServerError::cancelled());
        }
        let (mut content, kind) = match target.kind {
            model::TargetKind::Variable => (
                variables::content(self, &path, &target, limit),
                builtins::Kind::Variable,
            ),
            model::TargetKind::BuiltinVariable => (String::new(), builtins::Kind::Variable),
            model::TargetKind::Command => {
                if let Some(content) = commands::content(self, &path, &target, limit) {
                    return Ok(Some(render::hover(
                        content,
                        target.range,
                        limit,
                        self.hover_markdown,
                    )));
                }
                (String::new(), builtins::Kind::Command)
            }
        };
        if content.len() < limit {
            if let Some(document) = self
                .local_help
                .documentation(kind, &target.name, &cancel)
                .await?
            {
                render::local_help(&mut content, &document, limit, self.hover_markdown);
            }
        }
        if cancel.is_cancelled() {
            return Err(ServerError::cancelled());
        }
        Ok((!content.is_empty())
            .then(|| render::hover(content, target.range, limit, self.hover_markdown)))
    }
}
