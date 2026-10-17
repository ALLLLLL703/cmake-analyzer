use crate::server::{
    model::{CmakeAnalyzer, Service, ServiceCommand, SymbolInput},
    service::internal,
    uri,
};
use lspally::server::model::*;
use std::collections::BTreeMap;
use tokio::sync::oneshot;

impl CmakeAnalyzer {
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
