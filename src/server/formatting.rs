//! Queue formatting barriers and render client-applied whole-document edits.
//! Only open text matching its analyzed snapshot is eligible. Shared owners must
//! agree on effective style; any owner disabling save formatting suppresses pre-save
//! edits. Rendering does not change overlays or write files.
// Contains AI-generated documentation edits.
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
    /// Queue a formatting barrier and await client-applied edits or cancellation.
    /// Does not format or modify files in the adapter.
    ///
    /// # Arguments
    /// - `self` `[in]`: Adapter addressing the shared service; no direct project mutation.
    /// - `context` `[in]`: Consumed request context supplying cancellation.
    /// - `uri` `[in]`: Consumed document URI.
    /// - `options` `[in]`: Optional consumed client indentation settings; None uses configuration.
    /// - `save` `[in]`: True for willSaveWaitUntil, false for manual formatting.
    ///
    /// # Returns
    /// `Ok(Some(edits))` (possibly empty) from the actor; response type also permits None. `Err` on cancellation, transport or formatting failure.
    ///
    /// # Errors
    /// Cancellation, service/reply-channel failure, conflicting owner styles, invalid URI or actor worker/render/range errors.
    ///
    /// # Examples
    /// ```text
    /// let edits = server.query_formatting(context, uri, None, true).await?;
    /// ```
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
    /// Render at most one whole-document replacement from a matching open snapshot.
    /// Does not apply the edit, update editor versions or write disk; rendering runs on a joined worker.
    ///
    /// # Arguments
    /// - `self` `[in]`: Open text, owner settings and cached snapshots read without mutation.
    /// - `document` `[in]`: Consumed URI; unopened files return no edits.
    /// - `client` `[in]`: Optional indentation-width/spaces overrides; configured line width always remains in effect.
    /// - `save` `[in]`: Whether any owner disabling on_save should suppress edits.
    /// - `cancel` `[in]`: Consumed cooperative cancellation signal.
    ///
    /// # Returns
    /// `Ok(Some(vec![edit]))` for changed valid text, or `Ok(Some(empty))` for unopened/disabled/missing/stale/invalid-syntax/unchanged text. Never Ok(None). `Err` on style conflict, cancellation or conversion/render failure.
    ///
    /// # Errors
    /// Invalid URI, inconsistent effective styles across owners, snapshot lookup, cancelled rendering, non-syntax renderer errors, join failure or byte-range conversion. Early no-edit exits do not check cancellation.
    ///
    /// # Algorithm
    /// Inspect all owners for save permission and equal effective options, retain a snapshot and require its text equals the authoritative buffer. Render with cancellation; invalid syntax yields no edits. If changed, convert the complete original span and return one replacement.
    ///
    /// # Flow
    /// ```text
    ///  [Open document]
    ///          |
    ///  [Owner permissions + styles]
    ///          |
    ///  <Matching analyzed text?> -- no --> [No edits]
    ///          | yes
    ///  [Blocking cancellable render]
    ///          |
    ///  <Valid + changed?> -- no --> [No edits]
    ///          | yes
    ///  [Whole-range replacement for client]
    /// ```
    ///
    /// # Examples
    /// ```text
    /// let edits = service.format_document(uri, None, true, cancel).await?;
    /// ```
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
