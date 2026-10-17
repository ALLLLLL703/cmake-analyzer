//! Supplement target-name completion from full-indexed package trees and resolvable includes.
//! Conditional nondeferred includes may contribute names, not graph/reference/value effects.
//! Workspace file/depth/attempt budgets and external policies still apply; no code is executed.
// Contains AI-generated documentation additions.
use super::{
    check, context,
    model::{TargetSource, Visit},
};
use crate::{
    parser::{
        build_struct::{dependencies, model::DependencyKind},
        semanticer::{model::ValueBindings, values::flags},
    },
    workspace::{Workspace, model::ExternalIndex},
};
use std::{
    collections::{HashMap, HashSet},
    io,
    path::Path,
};

/// Collect completion-only target sources from full-indexed package trees, including conditional
/// nondeferred includes. Reads/parses sources but installs no graph bindings or execution effects.
///
/// # Arguments
/// - `workspace` `[in,out]` : Loads immutable cached snapshots and replaces the supplemental
///   retention set on success.
/// - `path` `[in]`: Requesting file identity used to compute visible scope ancestry.
/// - `stop` `[in]`: Cancellation predicate checked per visit and command.
///
/// # Returns
/// Ok supplemental snapshots with visibility flags, possibly partial at traversal limits; empty if
/// no current graph/packages. Err on cancellation or non-skippable analysis failure.
///
/// # Errors
/// Interrupted cancellation or snapshot failures other than missing/permission-denied files.
/// Traversal limits truncate results without an error; caches loaded before failure may remain
/// until later release.
///
/// # Algorithm
/// Seed full-indexed package visits and descendants. Traverse includes in a stack, deduplicating
/// file/source/binary/visibility contexts while respecting attempts/depth/distinct-file budgets.
/// Replay local hints, skip deferred/user-shadowed commands, and follow only native resolvable
/// includes allowed by external policy. NotFound/PermissionDenied sources are skipped; retain the
/// latest source set and release unused caches.
///
/// # Flow
/// ```text
/// [current full package visits]
///                |
///  [request scope ancestry] --> [visibility]
///                |
///          [bounded pending stack]
///                |
///        <seen/depth/file limit?> -- yes --> [skip]
///                | no
///        [load immutable snapshot]
///                |
///  [nondeferred native includes] ---> [push allowed children]
///                |
///         [snapshot + visibility]
///                |
///        [replace retention set]
/// ```
///
/// # Examples
/// ```text
/// let sources = collect(workspace, path, &|| false).await?;
/// ```
pub(super) async fn collect(
    workspace: &mut Workspace,
    path: &Path,
    stop: &impl Fn() -> bool,
) -> io::Result<Vec<TargetSource>> {
    let Some(graph) = workspace.root_graph() else {
        return Ok(Vec::new());
    };
    let packages: HashSet<_> = graph
        .edges
        .iter()
        .filter(|edge| edge.kind == DependencyKind::Package)
        .filter_map(|edge| edge.invocation)
        .collect();
    if packages.is_empty() {
        workspace.completion_files.clear();
        workspace.release_unused_snapshots();
        return Ok(Vec::new());
    }
    let mut visible = HashSet::new();
    for visit in graph.invocations.iter().filter(|visit| visit.file == path) {
        let mut scope = Some(visit.scope);
        while let Some(id) = scope {
            if !visible.insert(id) {
                break;
            }
            scope = graph.scopes[id].parent;
        }
    }
    let mut pending = Vec::new();
    let mut package_tree = HashSet::new();
    let mut depths = HashMap::new();
    for (id, visit) in graph.invocations.iter().enumerate() {
        let depth = visit.parent.map_or(0, |parent| depths[&parent] + 1);
        depths.insert(id, depth);
        if visit.index == ExternalIndex::Full
            && (packages.contains(&id)
                || visit
                    .parent
                    .is_some_and(|parent| package_tree.contains(&parent)))
        {
            package_tree.insert(id);
            pending.push(Visit {
                file: visit.file.clone(),
                directory: visit.directory.clone(),
                visible: visible.contains(&visit.scope),
                depth,
            });
        }
    }
    let mut loaded: HashSet<_> = graph
        .invocations
        .iter()
        .map(|visit| visit.file.clone())
        .collect();
    let source = workspace
        .root
        .file
        .parent()
        .unwrap_or(Path::new("/"))
        .to_owned();
    let source = tokio::fs::canonicalize(&source).await.unwrap_or(source);
    let mut seen = HashSet::new();
    let mut attempts = 0;
    let mut result = Vec::new();
    let mut retained = HashSet::new();
    while let Some(visit) = pending.pop() {
        check(stop)?;
        if attempts >= workspace.limits.invocations {
            break;
        }
        attempts += 1;
        if visit.depth > workspace.limits.depth
            || !seen.insert((
                visit.file.clone(),
                visit.directory.source.clone(),
                visit.directory.binary.clone(),
                visit.visible,
            ))
        {
            continue;
        }
        if !loaded.contains(&visit.file) && loaded.len() >= workspace.limits.files {
            continue;
        }
        let snapshot = match workspace.load_snapshot(&visit.file).await {
            Ok(snapshot) => snapshot,
            Err(error)
                if matches!(
                    error.kind(),
                    io::ErrorKind::NotFound | io::ErrorKind::PermissionDenied
                ) =>
            {
                continue;
            }
            Err(error) => return Err(error),
        };
        loaded.insert(visit.file.clone());
        retained.insert(visit.file.clone());
        let dirs = context::directories(workspace, &visit.file, &visit.directory);
        let mut values = ValueBindings::default();
        for command in &snapshot.semantics.commands {
            check(stop)?;
            let (conditional, deferred) = flags(&snapshot.semantics, command);
            if deferred {
                continue;
            }
            values.apply(command, conditional, Some(&dirs));
            if !command.closed
                || !command.schema.as_ref().is_some_and(|schema| {
                    matches!(
                        schema.from,
                        crate::parser::semanticer::schema::CommandSchemaFrom::Builtin
                            | crate::parser::semanticer::schema::CommandSchemaFrom::KeyWord
                    )
                })
            {
                continue;
            }
            if snapshot
                .semantics
                .functions
                .iter()
                .filter(|function| function.definition.is_some())
                .flat_map(|function| &function.references)
                .any(|reference| {
                    reference.kind == crate::parser::semanticer::model::ReferenceKind::Call
                        && reference.command == command.id
                })
            {
                continue;
            }
            let Some(mut dependency) = dependencies::from_command(command)
                .filter(|dependency| dependency.kind == DependencyKind::Include)
            else {
                continue;
            };
            if !dependency.path.literal {
                if let Some(value) = command
                    .args
                    .first()
                    .and_then(|argument| values.expand(argument, Some(&dirs)))
                    .filter(|value| !value.text().contains(';'))
                {
                    dependency.path.text = value.text().to_owned();
                    dependency.path.literal = true;
                }
            }
            if let Some(file) = workspace
                .resolve_dependency_in(&dependency, &visit.file, &visit.directory, Some(&values))
                .await
                .0
            {
                if workspace.external_include != ExternalIndex::Full {
                    let physical = tokio::fs::canonicalize(&file)
                        .await
                        .unwrap_or_else(|_| file.clone());
                    if !physical.starts_with(&source) {
                        continue;
                    }
                }
                pending.push(Visit {
                    file,
                    directory: visit.directory.clone(),
                    visible: visit.visible,
                    depth: visit.depth + 1,
                });
            }
        }
        result.push(TargetSource {
            snapshot,
            visible: visit.visible,
        });
    }
    workspace.completion_files = retained;
    workspace.release_unused_snapshots();
    Ok(result)
}
