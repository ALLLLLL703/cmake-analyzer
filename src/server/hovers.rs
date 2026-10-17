//! Resolve hover sites and combine static values, recorded observations and documentation.
//! Source inspection never configures a project or runs its script. Native help may start startup-PATH
//! CMake with help-only flags, using global inspection budgets rather than project execution settings.
//! The worker uses the smallest owning project's view budget and joins source-site lookup before help
//! I/O.

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
    /// Queue an ordered hover query and wait for its reply or cancellation; never directly execute
    /// project code.
    ///
    /// # Arguments
    /// - `self` `[in]`: analyzer queue handle; service processing may fill the local-help cache.
    /// - `context` `[in]`: request cancellation cloned into the worker query.
    /// - `params` `[in]`: document URI and negotiated cursor position.
    ///
    /// # Returns
    /// Ok(Some(hover)) when content exists; Ok(None) when no supported target/content exists.
    ///
    /// # Errors
    /// Queue/service shutdown, cancellation or a worker lookup/coordinate error.
    ///
    /// # Examples
    /// Local server context:
    /// ```text
    /// let hover = analyzer.hover_variable(context, params).await?;
    /// ```
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
    /// Locate a hover site on a joined worker and combine source content with optional local help; never
    /// configure or run a project script.
    /// Native help inspection may read PATH metadata and launch help-only CMake children; source
    /// comments and values remain available when help is absent.
    ///
    /// # Arguments
    /// - `self` `[in,out]`: service; may initialize/fill local_help, but does not modify source, static
    ///   graph or runtime layer.
    /// - `uri` `[in]`: document to resolve against its owners.
    /// - `position` `[in]`: cursor in negotiated text coordinates.
    /// - `cancel` `[in]`: request cancellation checked after lookup and help; passed into help inspection.
    ///
    /// # Returns
    /// Ok(Some(hover)) with nonempty byte-limited content; Ok(None) if no owner snapshot, site or
    /// documentation/value content exists.
    ///
    /// # Errors
    /// Invalid local URI/coordinates, lookup-worker failure or cancellation. A known user-command
    /// documentation result returns before the final cancellation check.
    ///
    /// # Algorithm
    /// Select the first owner snapshot and smallest owner max_view_bytes. Join blocking name lookup,
    /// then prefer user-command docs or collect variable values. Fill remaining space with canonical
    /// local help and render Markdown/plaintext. Known user commands suppress native fallback.
    ///
    /// # Flow
    /// ```text
    ///  [Owner snapshot + minimum view budget]
    ///                   |
    ///          [Joined site lookup] --none--> [None]
    ///                   |
    ///          [Command user docs?] --yes--> [Render and return]
    ///                   | no
    ///          [Variable values / empty]
    ///                   |
    ///           [Room for native help?] --no--+
    ///                   | yes               |
    ///           [Help-only cached lookup]    |
    ///                   +-------------------+
    ///                   v
    ///           [Cancel check + render]
    /// ```
    ///
    /// # Examples
    /// Local server context (requires the initialized service and its client):
    /// ```text
    /// let hover = service.variable_hover(uri, Position { line: 0, character: 4 }, cancel).await?;
    /// ```
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
