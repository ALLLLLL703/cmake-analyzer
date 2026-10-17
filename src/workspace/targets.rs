use super::model::RootGraph;
use crate::parser::build_struct::{
    model::{DependencyKind, TargetDeclaration},
    target_references::{TargetCollector, TargetDirectoryId},
};

pub(super) fn analyze(graph: &RootGraph) -> Vec<TargetDeclaration> {
    let mut collector = TargetCollector::default();
    let mut subdirectories = vec![false; graph.invocations.len()];
    for edge in &graph.edges {
        if let Some(child) = edge.invocation {
            subdirectories[child] = edge.kind == DependencyKind::Subdirectory;
        }
    }
    let mut directories: Vec<TargetDirectoryId> = Vec::with_capacity(graph.invocations.len());
    for (position, visit) in graph.invocations.iter().enumerate() {
        let parent = visit.parent.map(|parent| directories[parent]);
        // Includes retain the invoking directory, regardless of the included file's path.
        let directory = match (parent, subdirectories[position]) {
            (Some(parent), false) => parent,
            (parent, _) => collector.directory(parent),
        };
        directories.push(directory);
        if visit.index != super::model::ExternalIndex::Full {
            continue;
        }
        collector.file(
            &visit.snapshot.build,
            &visit.snapshot.semantics,
            directory,
            Some(&graph.references),
        );
    }
    collector.finish()
}
