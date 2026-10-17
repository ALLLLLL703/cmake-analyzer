//! Apply ordered editor notifications to service state.
//! Editor `didChange` text is authoritative. Save invalidates disk caches and may
//! schedule an authorized runtime run, but never overwrites editor text or disk files.
//! Close releases overlays and queues an empty report only for previously published
//! files. Query commands belong to the runtime loop, not this edit handler.
// AI-authored editor document lifecycle.
use super::{
    super::{model::*, roots, text, uri},
    internal,
};
use lspally::server::model::*;

impl Service {
    /// Apply one ordered notification, invalidate affected runtime observations and synchronize overlays.
    /// Reads metadata/settings for admission, joins edit conversion, and may queue save jobs or close-clear reports; never writes editor files.
    ///
    /// # Arguments
    /// - `self` `[in,out]`: Open texts/versions, project caches, runtime markers, pending saves and diagnostic-clear state updated by the message.
    /// - `command` `[in]`: Consumed edit/folder/runtime-refresh message; query variants are rejected.
    ///
    /// # Returns
    /// `Ok(())` after final overlay synchronization; `Err` for invalid input or failed updates, with earlier invalidation/admission changes retained.
    ///
    /// # Errors
    /// Invalid/local URI failures, duplicate/over-limit open, missing/nonincreasing changed document, invalid edit ranges, blocking-worker failures, ownership/cache/discovery failures, or a query sent to this handler.
    ///
    /// # Algorithm
    /// Changes/saves/closes invalidate related runtime before validation; folders invalidate all runs and pending saves. Open admits owners/text; change computes all edits on a joined worker before replacing text/version; close removes overlays and publication keys; save ignores optional text, schedules authorized owners and invalidates disk; folders remove/add roots and reassign opens. Finally synchronize current overlays.
    ///
    /// # Flow
    /// ```text
    ///  [Notification]
    ///         |
    ///  [Invalidate runtime when affected]
    ///         |
    ///  <Kind?>
    ///    +----+-------------+------------+
    ///    |                  |            |
    ///  [Open/change]    [Close/save]   [Folders]
    ///    |                  |            |
    ///  [Text/version] [Release/queue] [Reassign roots]
    ///    +------------------+------------+
    ///                       v
    ///                 [Sync overlays]
    /// ```
    ///
    /// # Examples
    /// ```text
    /// service.apply(ServiceCommand::Change(params)).await?;
    /// ```
    pub(super) async fn apply(&mut self, command: ServiceCommand) -> ServerResult<()> {
        let affected = match &command {
            ServiceCommand::Change(params) => Some(&params.text_document.uri),
            ServiceCommand::Save(params) => Some(&params.text_document.uri),
            ServiceCommand::Close(uri) => Some(uri),
            _ => None,
        };
        if let Some(uri) = affected {
            let path = uri::to_path(uri)?;
            self.invalidate_runtime_file(&path);
        }
        if matches!(&command, ServiceCommand::Folders(_)) {
            self.invalidate_runtime();
            self.saved_runs.clear();
        }
        match command {
            ServiceCommand::RuntimeRefresh => {}
            ServiceCommand::Open(document) => {
                let path = uri::to_path(&document.uri)?;
                if self.documents.contains_key(&path) {
                    return Err(ServerError::invalid("Document already open"));
                }
                if self.documents.len() >= self.config.limits.max_open_documents {
                    return Err(ServerError::new(
                        -32000,
                        format!(
                            "Open document limit ({}) reached",
                            self.config.limits.max_open_documents
                        ),
                    ));
                }
                self.ensure_owners(&path).await?;
                self.documents.insert(
                    path,
                    OpenDocument {
                        uri: document.uri,
                        version: document.version,
                        text: document.text.into(),
                    },
                );
            }
            ServiceCommand::Change(params) => {
                let path = uri::to_path(&params.text_document.uri)?;
                let document = self
                    .documents
                    .get_mut(&path)
                    .ok_or_else(|| ServerError::invalid("Document is not open"))?;
                if params.text_document.version <= document.version {
                    return Err(ServerError::invalid("Document version did not increase"));
                }
                let source = document.text.clone();
                let encoding = self.encoding;
                let updated = tokio::task::spawn_blocking(move || {
                    text::apply_changes(&source, params.content_changes, encoding)
                })
                .await
                .map_err(internal)??;
                document.text = updated.into();
                document.version = params.text_document.version;
            }
            ServiceCommand::Close(uri) => {
                let path = uri::to_path(&uri)?;
                self.saved_runs.retain(|_, saved| saved.file != path);
                if let Some(document) = self.documents.remove(&path) {
                    for project in &mut self.projects {
                        if project.overlays.remove(&path).is_some() {
                            project.workspace.close_overlay(&path).map_err(internal)?;
                            project.dirty = true;
                        }
                    }
                    if self.published.remove(&path).is_some() {
                        self.clears.push(document.uri);
                    }
                }
            }
            ServiceCommand::Save(params) => {
                let path = uri::to_path(&params.text_document.uri)?;
                if self.documents.contains_key(&path) {
                    for owner in self.owners(&path) {
                        let project = &self.projects[owner];
                        if project.settings.runtime.enable
                            && project.settings.runtime.mode == crate::config::RuntimeMode::OnSave
                            && project.execution.is_some()
                        {
                            if let Some(mode) = &project.execution {
                                self.saved_runs.insert(
                                    project.workspace.root().file.clone(),
                                    SavedRun {
                                        file: path.clone(),
                                        mode: mode.clone(),
                                    },
                                );
                            }
                        }
                    }
                }
                // didChange is authoritative; clients were not asked to include save text.
                for owner in self.owners(&path) {
                    self.projects[owner]
                        .workspace
                        .invalidate_disk(&path)
                        .map_err(internal)?;
                    self.projects[owner].dirty = true;
                }
            }
            ServiceCommand::Folders(event) => {
                for folder in event.removed {
                    let path = uri::to_path(&folder.uri)?;
                    self.roots
                        .retain(|root| root.parent() != Some(path.as_path()));
                    self.projects.retain(|project| {
                        project.standalone
                            || project.workspace.root().file.parent() != Some(path.as_path())
                    });
                }
                for folder in event.added {
                    let path = uri::to_path(&folder.uri)?;
                    if let Some(root) = roots::find_project(&path).await.map_err(internal)? {
                        self.add_project(root.clone(), false, None).await?;
                        self.roots.insert(root);
                    }
                }
                let paths: Vec<_> = self.documents.keys().cloned().collect();
                for path in paths {
                    self.ensure_owners(&path).await?;
                }
            }
            ServiceCommand::Format { .. }
            | ServiceCommand::WorkspaceSymbols { .. }
            | ServiceCommand::Symbols { .. }
            | ServiceCommand::Hover { .. }
            | ServiceCommand::Navigate { .. }
            | ServiceCommand::Complete { .. }
            | ServiceCommand::ReloadConfig { .. }
            | ServiceCommand::Actions { .. }
            | ServiceCommand::RunAction { .. } => {
                return Err(ServerError::new(-32603, "Query reached edit handler"));
            }
        }
        self.sync_overlays()
    }
}
