// AI-authored editor document lifecycle.
use super::{
    super::{model::*, roots, text, uri},
    internal,
};
use lspally::server::model::*;

impl Service {
    pub(super) async fn apply(&mut self, command: ServiceCommand) -> ServerResult<()> {
        match command {
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
            ServiceCommand::Symbols { .. } | ServiceCommand::ReloadConfig { .. } => {
                return Err(ServerError::new(-32603, "Query reached edit handler"));
            }
        }
        self.sync_overlays()
    }
}
