use super::{
    model::{CmakeAnalyzer, Service, ServiceCommand},
    service::internal,
    text::TextIndex,
    uri,
};
use crate::format::{self, Options};
use lspally::server::model::*;
use tokio::sync::oneshot;

impl CmakeAnalyzer {
    pub(super) async fn query_formatting(
        &self,
        context: Context,
        uri: Uri,
        options: Option<FormattingOptions>,
        save: bool,
    ) -> ServerResult<Option<Vec<TextEdit>>> {
        let (reply, response) = oneshot::channel();
        tokio::select! {
            _=context.cancellation.cancelled()=>Err(ServerError::cancelled()),
            result=async {self.send(ServiceCommand::Format{uri,options,save,cancel:context.cancellation.clone(),reply}).await?;response.await.map_err(|_|ServerError::new(-32603,"Workspace service stopped"))?}=>result,
        }
    }
}
impl Service {
    pub(in crate::server) async fn format_document(
        &self,
        document: Uri,
        client: Option<FormattingOptions>,
        save: bool,
        cancel: Cancellation,
    ) -> ServerResult<Option<Vec<TextEdit>>> {
        let path = uri::to_path(&document)?;
        let Some(open) = self.documents.get(&path) else {
            return Ok(Some(Vec::new()));
        };
        let mut selected = None;
        let mut snapshot = None;
        for owner in self.owners(&path) {
            let project = &self.projects[owner];
            let settings = &project.settings.format;
            if save && !settings.on_save {
                return Ok(Some(Vec::new()));
            }
            let options = Options {
                indent_width: client
                    .as_ref()
                    .map_or(settings.indent_width, |options| options.tab_size as usize),
                line_width: settings.line_width,
                insert_spaces: client
                    .as_ref()
                    .map_or(settings.insert_spaces, |options| options.insert_spaces),
            };
            if selected.is_some_and(|selected| selected != options) {
                return Err(ServerError::invalid(
                    "Conflicting formatting settings for shared document",
                ));
            }
            selected = Some(options);
            snapshot = project.workspace.snapshot(&path).map_err(internal)?;
        }
        let (Some(options), Some(snapshot)) = (selected, snapshot) else {
            return Ok(Some(Vec::new()));
        };
        if snapshot.text != open.text {
            return Ok(Some(Vec::new()));
        }
        let encoding = self.encoding;
        tokio::task::spawn_blocking(move || {
            let formatted = match format::render(
                &snapshot.text,
                &snapshot.lexed,
                &snapshot.semantics,
                options,
                || cancel.is_cancelled(),
            ) {
                Ok(text) => text,
                Err(format::Error::InvalidSyntax) => return Ok(Some(Vec::new())),
                Err(format::Error::Cancelled) => return Err(ServerError::cancelled()),
                Err(error) => return Err(ServerError::invalid(error.to_string())),
            };
            if cancel.is_cancelled() {
                return Err(ServerError::cancelled());
            }
            if formatted == snapshot.text.as_ref() {
                return Ok(Some(Vec::new()));
            }
            let range = TextIndex::new(&snapshot.text, encoding).range(0..snapshot.text.len())?;
            Ok(Some(vec![TextEdit {
                range,
                new_text: formatted,
            }]))
        })
        .await
        .map_err(internal)?
    }
}
