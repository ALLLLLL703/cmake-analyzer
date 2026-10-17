//! Reuse dependency visits only when scope layout and traversal-relevant effects are unchanged.
//! Refresh replaces snapshot/source references without replaying dependencies; no I/O occurs.
// Contains AI-generated documentation additions.
// AI-authored graph refresh without replaying unchanged dependency effects.
use super::model::*;
use crate::parser::build_struct::model::BuildEffect;
use std::collections::BTreeSet;
use std::path::PathBuf;

impl Workspace {
    /// Refresh visit snapshots and dependency origins only when traversal inputs are unchanged.
    /// Mutates graph references on success, not source snapshots; no I/O.
    ///
    /// # Arguments
    /// - `self` `[in,out]` : Workspace; replaces visit snapshots/analysis keys and edge origins,
    ///   then overwrites reused_invocations.
    /// - `dirty` `[in]`: Drained pending paths used to validate changed root inputs.
    ///
    /// # Returns
    /// True after safe visit reuse; false without refreshing when invalidated, diagnostic-bearing,
    /// missing or traversal-relevant inputs changed.
    ///
    /// # Algorithm
    /// Reject invalidated or failed graphs. Check changed dependencies against scope/fact
    /// signatures; any visited-text change rejects reuse if a dependency path is nonliteral.
    /// Replace retained snapshots and find fresh dependency origins by command ID. Reference
    /// indexes are rebuilt separately.
    ///
    /// # Flow
    /// ```text
    /// [saved graph] ----> <invalidated/diagnostics?>
    ///                           /         \
    ///                         reject     [compare dirty inputs]
    ///                                       |
    ///                                <same signatures?>
    ///                                 /             \
    ///                              reject      [replace visits/edges]
    ///                                                |
    ///                                         [report reuse]
    /// ```
    ///
    /// # Examples
    /// ```text
    /// let reused = workspace.refresh_graph_snapshots(&dirty);
    /// ```
    pub(super) fn refresh_graph_snapshots(&mut self, dirty: &BTreeSet<PathBuf>) -> bool {
        if self.graph_invalidated {
            return false;
        }
        let Some(graph) = self.graph.as_mut() else {
            return false;
        };
        // Failed resolutions and traversal diagnostics need another dependency walk.
        if !graph.diagnostics.is_empty() {
            return false;
        }
        let value_dependent=graph.invocations.iter().any(|visit|visit.snapshot.build.facts.iter().any(|fact|matches!(&fact.effect,BuildEffect::Dependency(dependency) if !dependency.path.literal)));
        for path in dirty {
            if !self.dependencies.contains(path) && *path != self.root.file {
                continue;
            }
            let Some(old) = graph.invocations.iter().find(|visit| visit.file == *path) else {
                return false;
            };
            let Some(new) = self.documents.get(path).and_then(|d| d.snapshot.as_ref()) else {
                return false;
            };
            if value_dependent && old.snapshot.text != new.text {
                return false;
            }
            if !same_graph_inputs(&old.snapshot, new) {
                return false;
            }
        }
        let mut reused = 0;
        for visit in &mut graph.invocations {
            if let Some(snapshot) = self
                .documents
                .get(&visit.file)
                .and_then(|d| d.snapshot.as_ref())
            {
                visit.analysis = snapshot.build.analysis;
                visit.snapshot = snapshot.clone();
                reused += 1;
            }
        }
        // The signature includes fact order/command IDs. Only revision and source coordinates changed.
        for edge in &mut graph.edges {
            if let Some(fact) = graph.invocations[edge.caller]
                .snapshot
                .build
                .facts
                .iter()
                .find(|fact| {
                    fact.source.command == edge.source.command
                        && matches!(fact.effect, BuildEffect::Dependency(_))
                })
            {
                edge.source = fact.source.clone();
            }
        }
        self.reused_invocations = reused;
        true
    }
}
/// Compare scope structure and traversal-relevant fact order/context, ignoring source coordinates
/// and unrelated effect payloads. No side effects.
///
/// # Arguments
/// - `old` `[in]`: Previous retained snapshot.
/// - `new` `[in]`: Candidate snapshot for visit reuse.
///
/// # Returns
/// True when scope IDs/parents/kinds and ordered context-sensitive facts match.
///
/// # Algorithm
/// Require identical semantic scope count and ID/kind/block/parent layout. Then compare ordered
/// build-fact counts, command/scope/control context and the subset of payloads affecting
/// traversal/inheritance. Coordinates and unrelated build metadata are intentionally excluded.
///
/// # Flow
/// ```text
/// [old/new scope layout]
///             |
///       <identical?> -- no --> [false]
///             | yes              ^
///  [ordered fact count/context]  |
///             |                  |
///      <same context effects?> -- no
///             | yes
///           [true]
/// ```
///
/// # Examples
/// ```text
/// let compatible = same_graph_inputs(&old_snapshot, &new_snapshot);
/// ```
fn same_graph_inputs(old: &DocumentSnapshot, new: &DocumentSnapshot) -> bool {
    let (a, b) = (&old.semantics.scopes, &new.semantics.scopes);
    if a.len() != b.len()
        || !a.iter().zip(b).all(|(a, b)| {
            a.id == b.id && a.kind == b.kind && a.block == b.block && a.parent == b.parent
        })
    {
        return false;
    }
    let (a, b) = (&old.build.facts, &new.build.facts);
    a.len() == b.len()
        && a.iter().zip(b).all(|(a, b)| {
            a.source.command == b.source.command
                && a.source.scope == b.source.scope
                && a.source.conditional == b.source.conditional
                && a.source.deferred == b.source.deferred
                && same_context_effect(&a.effect, &b.effect)
        })
}
/// Compare only effects that change traversal or inherited write/project context. No side effects.
///
/// # Arguments
/// - `a` `[in]`: Old build effect.
/// - `b` `[in]`: New build effect.
///
/// # Returns
/// True for equal project/dependency/locality-and-write payloads, or for any pair of other effects;
/// false when only one side is context-sensitive.
///
/// # Examples
/// ```text
/// let unchanged = same_context_effect(&old.effect, &new.effect);
/// ```
fn same_context_effect(a: &BuildEffect, b: &BuildEffect) -> bool {
    match (a, b) {
        (BuildEffect::Project(a), BuildEffect::Project(b)) => a == b,
        (BuildEffect::Dependency(a), BuildEffect::Dependency(b)) => a == b,
        (
            BuildEffect::VariableWrites {
                names: a,
                local: la,
            },
            BuildEffect::VariableWrites {
                names: b,
                local: lb,
            },
        ) => a == b && la == lb,
        (
            BuildEffect::Project(_)
            | BuildEffect::Dependency(_)
            | BuildEffect::VariableWrites { .. },
            _,
        ) => false,
        (
            _,
            BuildEffect::Project(_)
            | BuildEffect::Dependency(_)
            | BuildEffect::VariableWrites { .. },
        ) => false,
        // Other facts cannot change traversal or inherited write state.
        _ => true,
    }
}
