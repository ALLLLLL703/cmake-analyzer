// AI-authored graph refresh without replaying unchanged dependency effects.
use super::model::*;
use crate::parser::build_struct::model::BuildEffect;
use std::collections::BTreeSet;
use std::path::PathBuf;

impl Workspace {
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
