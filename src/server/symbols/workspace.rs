//! Collect workspace-wide symbols from already cached analysis buffers.
//! Paths deduplicate repeated visits and owners; open buffers keep their client URI.
//! External functions-only visits expose only top-level function/macro definitions
//! unless that file is also fully indexed or opened. Client-side filtering/ranking is
//! intentional: this service returns all collected names, without a directory scan.
// Contains AI-generated documentation edits.
use crate::server::{
    model::{CmakeAnalyzer, Service, ServiceCommand, SymbolInput},
    service::internal,
    uri,
};
use lspally::server::model::*;
use std::collections::BTreeMap;
use tokio::sync::oneshot;

impl CmakeAnalyzer {
    /// Queue an ordered workspace-symbol barrier and await its reply or request cancellation.
    /// No direct snapshot reads or project mutation occurs in this adapter.
    ///
    /// # Arguments
    /// - `self` `[in]`: Adapter addressing the shared service; no direct project mutation.
    /// - `context` `[in]`: Consumed request context supplying cancellation.
    ///
    /// # Returns
    /// `Ok(Some(Flat(...)))` from the actor, possibly empty; actor result type also permits None. `Err` on cancellation, stopped service or conversion failure.
    ///
    /// # Errors
    /// Cancellation, missing/stopped service, reply-channel failure or actor snapshot/worker/range errors.
    ///
    /// # Examples
    /// ```text
    /// let symbols = server.query_workspace_symbols(context).await?;
    /// ```
    pub(in crate::server) async fn query_workspace_symbols(
        &self,
        context: Context,
    ) -> ServerResult<Option<WorkspaceSymbolResponse>> {
        let (reply, response) = oneshot::channel();
        tokio::select! {
            _=context.cancellation.cancelled()=>Err(ServerError::cancelled()),
            result=async {self.send(ServiceCommand::WorkspaceSymbols{cancel:context.cancellation.clone(),reply}).await?;response.await.map_err(|_|ServerError::new(-32603,"Workspace service stopped"))?}=>result,
        }
    }
}
impl Service {
    /// Collect unique cached paths from enabled projects and convert all names to flat workspace symbols.
    /// No filesystem scan or project execution; reads cached snapshots and joins a cancellable blocking conversion.
    ///
    /// # Arguments
    /// - `self` `[in]`: Cached snapshots and feature settings read without mutation; no buffers are loaded from disk.
    /// - `cancel` `[in]`: Consumed cooperative cancellation signal.
    ///
    /// # Returns
    /// `Ok(Some(WorkspaceSymbolResponse::Flat(symbols)))`, possibly empty; never Ok(None). `Err` on cancellation, snapshot lookup, join or conversion failure.
    ///
    /// # Errors
    /// Cooperative cancellation between projects/buffers and after conversion, Workspace snapshot lookup errors, failed blocking task or invalid symbol ranges.
    ///
    /// # Algorithm
    /// Deduplicate graph visits by path, preserving first visit snapshots. Full visits and runtime documents mark full visibility; root/open-overlay cached snapshots replace earlier choices and force full visibility. Preserve open URIs or encode closed paths, convert restricted files with functions_only, concatenate every flat response without query filtering.
    ///
    /// # Flow
    /// ```text
    ///  [Enabled projects]
    ///          |
    ///  [Graph visits] ---+
    ///          |         |
    ///  [Runtime docs]    +----> [Path snapshots + full flags]
    ///          |         |                    |
    ///  [Root/overlays] --+                    v
    ///                               [URI + restriction inputs]
    ///                                         |
    ///                               [Cancellable flat conversion]
    ///                                         |
    ///                               [All names, client filters]
    /// ```
    ///
    /// # Examples
    /// ```text
    /// let symbols = service.workspace_symbols(cancel).await?;
    /// ```
    pub(in crate::server) async fn workspace_symbols(
        &self,
        cancel: Cancellation,
    ) -> ServerResult<Option<WorkspaceSymbolResponse>> {
        let mut snapshots = BTreeMap::new();
        let mut full = std::collections::BTreeSet::new();
        for project in &self.projects {
            if cancel.is_cancelled() {
                return Err(ServerError::cancelled());
            }
            if !project.settings.document_symbols.enable {
                continue;
            }
            let workspace = &project.workspace;
            if let Some(graph) = workspace.root_graph() {
                for visit in &graph.invocations {
                    if visit.index == crate::workspace::model::ExternalIndex::Full {
                        full.insert(visit.file.clone());
                    }
                    snapshots
                        .entry(visit.file.clone())
                        .or_insert_with(|| visit.snapshot.clone());
                }
            }
            if let Some(layer) = workspace.runtime() {
                for (path, snapshot) in &layer.documents {
                    full.insert(path.clone());
                    snapshots
                        .entry(path.clone())
                        .or_insert_with(|| snapshot.clone());
                }
            }
            for path in std::iter::once(&workspace.root().file).chain(project.overlays.keys()) {
                if let Some(snapshot) = workspace.snapshot(path).map_err(internal)? {
                    full.insert(path.clone());
                    snapshots.insert(path.clone(), snapshot);
                }
            }
        }
        let inputs = snapshots
            .into_iter()
            .map(|(path, snapshot)| SymbolInput {
                functions_only: !full.contains(&path),
                uri: self.documents.get(&path).map_or_else(
                    || uri::from_path(&path),
                    |document| Uri(document.uri.0.clone()),
                ),
                snapshot,
                encoding: self.encoding,
                hierarchical: false,
            })
            .collect::<Vec<_>>();
        tokio::task::spawn_blocking(move || {
            let mut result = Vec::new();
            // Return every buffer's symbols; the client owns query filtering/ranking.
            for input in inputs {
                if cancel.is_cancelled() {
                    return Err(ServerError::cancelled());
                }
                if let Some(DocumentSymbolResponse::Flat(symbols)) = super::document_symbols(input)?
                {
                    result.extend(symbols);
                }
            }
            if cancel.is_cancelled() {
                return Err(ServerError::cancelled());
            }
            Ok(Some(WorkspaceSymbolResponse::Flat(result)))
        })
        .await
        .map_err(internal)?
    }
}
