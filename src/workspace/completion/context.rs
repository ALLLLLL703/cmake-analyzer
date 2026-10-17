use super::model::Context;
use crate::{
    parser::semanticer::{
        model::{CommandId, ValueBindings, ValueDirectories},
        values::flags,
    },
    workspace::{Workspace, model::DirectoryContext},
};
use std::path::{Path, PathBuf};

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
