//! Retain owner-local completion data without copying or changing analysis snapshots.
//! Directory ancestry limits variable candidates; capture performs no I/O.

use super::model::{Input, Source};
use crate::workspace::Workspace;
use std::{
    collections::{HashMap, HashSet},
    path::Path,
};

/// Retain the queried snapshot, graph bindings and unique full-indexed dependency sources.
/// No I/O or workspace mutation; shared analysis remains immutable.
///
/// # Arguments
/// - `workspace` `[in]`: One owner; all retained file IDs and bindings belong to it.
/// - `path` `[in]`: Queried file path looked up in the existing snapshot cache.
///
/// # Returns
/// `Some(input)` retains the cached snapshot and optional graph index; `None` means snapshot lookup failed or no snapshot exists.
///
/// # Algorithm
/// Collect scopes for visits of the queried file and walk their parents. Deduplicate full-indexed sources by file ID, OR-ing ancestor visibility across repeated visits. Functions-only visits are excluded from variable/target source candidates.
///
/// # Flow
/// ```text
///  [Queried file visits]        [Full-indexed visits]
///            |                          |
///            v                          v
///  [Walk scope ancestors] --> [Deduplicate by file ID]
///                                       |
///                                       v
///                            [OR directory visibility]
///                                       |
///                                       v
///                             [Retained Input + index]
/// ```
///
/// # Examples
/// ```text
/// let input = inputs::capture(&workspace, &path);
/// ```
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
        catalog: Default::default(),
        snapshot,
        index: graph.map(|graph| graph.references.clone()),
        sources: sources.into_values().collect(),
    })
}
