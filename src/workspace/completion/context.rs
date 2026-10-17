//! Capture visit contexts and restore local search hints before the completion command.
//! A joined blocking replay honors cancellation and leaves workspace graphs/snapshots unchanged.
// Contains AI-generated documentation additions.
use super::model::Context;
use crate::{
    parser::semanticer::{
        model::{CommandId, ValueBindings, ValueDirectories},
        values::flags,
    },
    workspace::{Workspace, model::DirectoryContext},
};
use std::path::{Path, PathBuf};

/// Build directory-builtin context for a completion source without filesystem access or mutation.
///
/// # Arguments
/// - `workspace` `[in]`: Root/script context and configured binary root.
/// - `file` `[in]`: List-file identity for list-directory/file builtins.
/// - `directory` `[in]`: Current visit source/binary directories.
///
/// # Returns
/// Owned ValueDirectories. source_root uses explicit script cwd when present; binary_root uses
/// configured root.binary_directory.
///
/// # Examples
/// ```text
/// let dirs = directories(workspace, path, &visit.directory);
/// ```
pub(super) fn directories(
    workspace: &Workspace,
    file: &Path,
    directory: &DirectoryContext,
) -> ValueDirectories {
    ValueDirectories {
        source_root: workspace
            .root
            .script_working_directory
            .clone()
            .unwrap_or_else(|| {
                workspace
                    .root
                    .file
                    .parent()
                    .unwrap_or(Path::new("/"))
                    .to_owned()
            }),
        source: directory.source.clone(),
        binary_root: workspace.root.binary_directory.clone(),
        binary: directory.binary.clone(),
        file: file.to_owned(),
    }
}
/// Capture all current visit contexts and restore local search hints before the active command.
/// Does not mutate workspace/snapshots; joins a cancellable blocking replay.
///
/// # Arguments
/// - `workspace` `[in]`: Workspace graph and current local snapshot.
/// - `path` `[in]`: Normalized completion file identity.
/// - `command` `[in]`: Active command to exclude with its suffix; None replays all local commands.
/// - `stop` `[in]`: Owned cancellation predicate called during command replay.
///
/// # Returns
/// Ok contexts, at least one fallback root context when no visit matches; Err on snapshot
/// normalization/cancellation/worker failure.
///
/// # Errors
/// Snapshot identity normalization, interrupted cancellation or blocking-worker join failure.
///
/// # Algorithm
/// Clone matching graph scope values and directories or create root defaults. Capture the current
/// snapshot, then replay nondeferred commands before the active command on a joined worker. This
/// restores local hints that a partial request may have invalidated in final graph state; it is not
/// general historical state reconstruction.
///
/// # Flow
/// ```text
/// [matching visits]
///         |
///  <any contexts?>
///      /       \
///  clone     root fallback
///      \       /
///  [current local snapshot]
///            |
///  [worker: replay before command]
///            |
///     [restored contexts]
/// ```
///
/// # Examples
/// ```text
/// let contexts = capture(workspace, path, Some(command.id), || false).await?;
/// ```
pub(super) async fn capture(
    workspace: &Workspace,
    path: &Path,
    command: Option<CommandId>,
    stop: impl Fn() -> bool + Send + 'static,
) -> std::io::Result<Vec<Context>> {
    let mut contexts = Vec::new();
    if let Some(graph) = workspace.root_graph() {
        for visit in graph.invocations.iter().filter(|visit| visit.file == path) {
            contexts.push(Context {
                directory: visit.directory.clone(),
                values: graph.scopes[visit.scope].values.clone(),
            });
        }
    }
    if contexts.is_empty() {
        contexts.push(Context {
            directory: DirectoryContext {
                source: workspace
                    .root
                    .script_working_directory
                    .clone()
                    .unwrap_or_else(|| {
                        workspace
                            .root
                            .file
                            .parent()
                            .unwrap_or(Path::new("/"))
                            .to_owned()
                    }),
                binary: workspace.root.binary_directory.clone(),
            },
            values: ValueBindings::default(),
        });
    }
    let directories = contexts
        .iter()
        .map(|context| directories(workspace, path, &context.directory))
        .collect::<Vec<_>>();
    let snapshot = workspace.snapshot(path)?;
    tokio::task::spawn_blocking(move || {
        // A partial package name can invalidate final graph values. Restore local hints before the cursor.
        if let Some(snapshot) = snapshot {
            for (context, dirs) in contexts.iter_mut().zip(directories) {
                for item in snapshot
                    .semantics
                    .commands
                    .iter()
                    .take_while(|item| command.is_none_or(|command| item.id != command))
                {
                    super::check(&stop)?;
                    let (conditional, deferred) = flags(&snapshot.semantics, item);
                    if !deferred {
                        context.values.apply(item, conditional, Some(&dirs));
                    }
                }
            }
        }
        Ok(contexts)
    })
    .await
    .map_err(std::io::Error::other)?
}
/// Split a known path-list value into nonempty path entries. No I/O or mutation.
///
/// # Arguments
/// - `values` `[in]`: Conservative binding set to query.
/// - `name` `[in]`: Case-sensitive variable name.
///
/// # Returns
/// Paths in semicolon-list order; empty for missing/unknown values.
///
/// # Examples
/// ```text
/// let roots = paths(values, "CMAKE_PREFIX_PATH");
/// ```
pub(super) fn paths(values: &ValueBindings, name: &str) -> Vec<PathBuf> {
    values
        .get(name, None)
        .map(|value| {
            value
                .text()
                .split(';')
                .filter(|path| !path.is_empty())
                .map(PathBuf::from)
                .collect()
        })
        .unwrap_or_default()
}
