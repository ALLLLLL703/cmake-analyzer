use super::model::{Input, Source};
use crate::workspace::Workspace;
use std::{
    collections::{HashMap, HashSet},
    path::Path,
};

pub(super) fn capture(workspace: &Workspace, path: &Path) -> Option<Input> {
    let snapshot = workspace.snapshot(path).ok().flatten()?;
    let mut sources: HashMap<_, Source> = HashMap::new();
    let graph = workspace.root_graph();
    if let Some(graph) = graph {
        let mut scopes = HashSet::new();
        for visit in graph
            .invocations
            .iter()
            .filter(|visit| visit.snapshot.semantics.id == snapshot.semantics.id)
        {
            let mut current = Some(visit.scope);
            while let Some(scope) = current {
                if !scopes.insert(scope) {
                    break;
                }
                current = graph.scopes.get(scope).and_then(|scope| scope.parent);
            }
        }
        for visit in graph
            .invocations
            .iter()
            .filter(|visit| visit.index == crate::workspace::model::ExternalIndex::Full)
        {
            let visible = scopes.contains(&visit.scope);
            sources
                .entry(visit.snapshot.semantics.id)
                .and_modify(|source| source.variables_visible |= visible)
                .or_insert_with(|| Source {
                    snapshot: visit.snapshot.clone(),
                    variables_visible: visible,
                });
        }
    }
    Some(Input {
        snapshot,
        index: graph.map(|graph| graph.references.clone()),
        sources: sources.into_values().collect(),
    })
}
