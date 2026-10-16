use super::{
    locations,
    model::{Input, Query},
};
use crate::{
    server::{model::Service, uri},
    workspace::model::DocumentSnapshot,
};
use lspally::server::model::*;
use std::{path::Path, sync::Arc};
impl Service {
    pub(in crate::server) async fn navigate(
        &self,
        document: Uri,
        position: Position,
        query: Query,
        cancel: Cancellation,
    ) -> ServerResult<Vec<Location>> {
        let path = uri::to_path(&document)?;
        let mut inputs = Vec::new();
        for owner in self.owners(&path) {
            let workspace = &self.projects[owner].workspace;
            let Some(snapshot) = workspace.snapshot(&path).ok().flatten() else {
                continue;
            };
            let source = |file: &Path, snapshot: &Arc<DocumentSnapshot>| {
                let uri = self
                    .documents
                    .get(file)
                    .map_or_else(|| uri::from_path(file).0, |document| document.uri.0.clone());
                (uri, snapshot.clone())
            };
            let mut files = std::collections::HashMap::new();
            let mut index = None;
            if let Some(graph) = workspace.root_graph() {
                for visit in &graph.invocations {
                    files
                        .entry(visit.snapshot.semantics.id)
                        .or_insert_with(|| source(&visit.file, &visit.snapshot));
                }
                if files.contains_key(&snapshot.semantics.id) {
                    index = Some(graph.references.clone());
                }
            }
            files.insert(snapshot.semantics.id, source(&path, &snapshot));
            inputs.push(Input {
                snapshot,
                index,
                files,
            });
        }
        let encoding = self.encoding;
        tokio::task::spawn_blocking(move || {
            locations::query(inputs, position, query, encoding, &cancel)
        })
        .await
        .map_err(crate::server::service::internal)?
    }
}
