//! Capture each owner's index with its own file-ID map before joined worker conversion.
//! Dependency-path definition lookup may read metadata; symbol lookup retains immutable snapshots.

use super::{
    dependencies, locations,
    model::{Input, Query},
};
use crate::{
    server::{model::Service, uri},
    workspace::model::DocumentSnapshot,
};
use lspally::server::model::*;
use std::{path::Path, sync::Arc};
impl Service {
    /// Capture owner-qualified bindings and join navigation conversion after the service barrier.
    /// Dependency definition lookup may read metadata/search paths; symbol conversion is read-only.
    ///
    /// # Arguments
    /// - `self` `[in]`: Service with flushed workspaces, open-document URIs and negotiated encoding.
    /// - `document` `[in]`: Local queried document URI.
    /// - `position` `[in]`: Cursor in the service's negotiated client coordinates.
    /// - `query` `[in]`: Definition or references and declaration policy.
    /// - `cancel` `[in]`: Cooperative stop signal carried into dependency and symbol lookup.
    ///
    /// # Returns
    /// `Ok(locations)` is a deduplicated set of usable source/file-start locations, possibly empty.
    ///
    /// # Errors
    /// `Err` reports invalid URI/position/ranges, cancellation or joined-worker failure.
    ///
    /// # Algorithm
    /// Intercept only definition requests on native dependency operands. Otherwise retain each owner's snapshots, file-ID map, reference and target indexes. Use graph indexes only if the queried file participates. Preserve open client URIs, encode closed file URIs, then join locations::query without borrowing mutable workspace state.
    ///
    /// # Flow
    /// ```text
    ///  [Ordered Service]
    ///         |
    ///  <dependency definition?> --handled--> [File-start links]
    ///         | not handled
    ///         v
    ///  [For each owner: snapshot + index + files]
    ///         |                         |
    ///         v                         v
    ///  [Owner A Input]             [Owner B Input]
    ///         +------------+------------+
    ///                      v
    ///           [Joined locations worker]
    ///                      |
    ///                      v
    ///               [URI/range results]
    /// ```
    ///
    /// # Examples
    /// ```text
    /// let locations = service.navigate(uri, position, Query::Definition, cancel).await?;
    /// ```
    pub(in crate::server) async fn navigate(
        &self,
        document: Uri,
        position: Position,
        query: Query,
        cancel: Cancellation,
    ) -> ServerResult<Vec<Location>> {
        let path = uri::to_path(&document)?;
        if matches!(query, Query::Definition) {
            if let Some(locations) = dependencies::query(self, &path, &position, &cancel).await? {
                return Ok(locations);
            }
        }
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
            let mut targets = None;
            if let Some(graph) = workspace.root_graph() {
                for visit in &graph.invocations {
                    files
                        .entry(visit.snapshot.semantics.id)
                        .or_insert_with(|| source(&visit.file, &visit.snapshot));
                }
                if files.contains_key(&snapshot.semantics.id) {
                    index = Some(graph.references.clone());
                    targets = Some(graph.targets.clone());
                }
            }
            files.insert(snapshot.semantics.id, source(&path, &snapshot));
            inputs.push(Input {
                snapshot,
                index,
                targets,
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
