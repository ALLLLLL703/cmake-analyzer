//! Reconstruct target directory ancestry from full-indexed visits and bind owner-qualified names.
//! Includes/packages share directories, subdirectories nest them; no snapshot mutation or I/O.
// Contains AI-generated documentation additions.
use super::model::RootGraph;
use crate::parser::build_struct::{
    model::{DependencyKind, TargetDeclaration},
    target_references::{TargetCollector, TargetDirectoryId},
};

/// Build owner-qualified target bindings using CMake directory ancestry, without changing snapshots
/// or doing I/O.
///
/// # Arguments
/// - `graph` `[in]`: Graph visits, dependency edges and completed function/reference index.
///
/// # Returns
/// Collected declarations from full-indexed visits only, with directory-visible literal references.
///
/// # Algorithm
/// Mark subdirectory edges. Process visits in caller-before-child order, sharing the parent target
/// directory for includes/packages and allocating nested directories for subdirectories/root.
/// Gather only full-indexed files with owner-qualified shadow information, then resolve all names.
///
/// # Flow
/// ```text
/// [graph edges] ----> [subdirectory flags]
///                            |
///  [ordered visits] --> <include/package?>
///                        /          \
///                   share parent   new directory
///                        \          /
///                       <full index?>
///                        /          \
///                    collect        skip
///                        |
///                  [finish binding]
/// ```
///
/// # Examples
/// ```text
/// let targets = analyze(graph);
/// ```
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
