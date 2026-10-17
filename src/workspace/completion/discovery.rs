use super::{
    check, context,
    model::{Catalog, Context, Request},
};
use crate::workspace::{Workspace, documents::absolute};
use std::{
    collections::BTreeSet,
    io,
    path::{Path, PathBuf},
    sync::Arc,
};

async fn entries(directory: &Path, stop: &impl Fn() -> bool) -> io::Result<Vec<(PathBuf, bool)>> {
    let mut result = Vec::new();
    let Ok(mut entries) = tokio::fs::read_dir(directory).await else {
        return Ok(result);
    };
    while let Some(entry) = entries.next_entry().await? {
        check(stop)?;
        let Ok(kind) = entry.file_type().await else {
            continue;
        };
        let directory = if kind.is_symlink() {
            tokio::fs::metadata(entry.path())
                .await
                .is_ok_and(|meta| meta.is_dir())
        } else {
            kind.is_dir()
        };
        if directory || kind.is_file() || kind.is_symlink() {
            result.push((entry.path(), directory));
        }
    }
    result.sort();
    Ok(result)
}
fn config_name(path: &Path) -> Option<&str> {
    let file = path.file_name()?.to_str()?;
    file.strip_suffix("Config.cmake")
        .or_else(|| file.strip_suffix("-config.cmake"))
        .filter(|name| !name.is_empty())
}
fn add_roots(
    roots: &mut BTreeSet<PathBuf>,
    paths: impl IntoIterator<Item = PathBuf>,
    source: &Path,
) {
    roots.extend(
        paths
            .into_iter()
            .filter_map(|path| absolute(&source.join(path)).ok()),
    );
}
fn roots(workspace: &Workspace, path: &Path, contexts: &[Context]) -> (Vec<PathBuf>, Vec<PathBuf>) {
    let mut modules = BTreeSet::new();
    let mut prefixes = BTreeSet::new();
    for context in contexts {
        add_roots(
            &mut modules,
            workspace.module_search_paths(path, &context.directory, Some(&context.values)),
            &context.directory.source,
        );
        add_roots(
            &mut prefixes,
            workspace.package_search_paths.iter().cloned(),
            &context.directory.source,
        );
        add_roots(
            &mut prefixes,
            context::paths(&context.values, "CMAKE_PREFIX_PATH"),
            &context.directory.source,
        );
        if let Some(paths) = std::env::var_os("CMAKE_PREFIX_PATH") {
            add_roots(
                &mut prefixes,
                std::env::split_paths(&paths),
                &context.directory.source,
            );
        }
        if let Ok(Some(snapshot)) = workspace.snapshot(path) {
            for name in snapshot
                .semantics
                .variables
                .iter()
                .map(|variable| variable.name.as_str())
                .chain(workspace.root_graph().into_iter().flat_map(|graph| {
                    graph
                        .references
                        .variables
                        .iter()
                        .map(|variable| variable.name.as_str())
                }))
            {
                if !name.starts_with("CMAKE_")
                    && (name.ends_with("_DIR") || name.ends_with("_ROOT"))
                {
                    add_roots(
                        &mut prefixes,
                        context::paths(&context.values, name),
                        &context.directory.source,
                    );
                }
            }
        }
    }
    (
        modules.into_iter().collect(),
        prefixes.into_iter().collect(),
    )
}
async fn package_names(
    prefixes: &[PathBuf],
    stop: &impl Fn() -> bool,
) -> io::Result<BTreeSet<String>> {
    let mut names = BTreeSet::new();
    let mut scanned = BTreeSet::new();
    for root in prefixes {
        check(stop)?;
        let mut directories = vec![root.clone(), root.join("cmake"), root.join("CMake")];
        for base in [
            root.clone(),
            root.join("lib/cmake"),
            root.join("lib64/cmake"),
            root.join("share/cmake"),
            root.join("share"),
        ] {
            for (path, is_directory) in entries(&base, stop).await? {
                if is_directory {
                    directories.extend([path.clone(), path.join("cmake"), path.join("CMake")]);
                }
            }
        }
        for (architecture, is_directory) in entries(&root.join("lib"), stop).await? {
            if is_directory {
                for (path, is_directory) in entries(&architecture.join("cmake"), stop).await? {
                    if is_directory {
                        directories.push(path);
                    }
                }
            }
        }
        for directory in directories {
            if !scanned.insert(directory.clone()) {
                continue;
            }
            for (path, is_directory) in entries(&directory, stop).await? {
                if !is_directory {
                    if let Some(name) = config_name(&path) {
                        names.insert(name.to_owned());
                    }
                }
            }
        }
    }
    Ok(names)
}
pub(super) async fn collect(
    workspace: &mut Workspace,
    path: &Path,
    contexts: &[Context],
    request: &Request,
    stop: &impl Fn() -> bool,
) -> io::Result<Catalog> {
    let (module_roots, prefix_roots) = roots(workspace, path, contexts);
    if workspace.name_cache.module_roots != module_roots {
        workspace.name_cache.module_roots = module_roots;
        workspace.name_cache.modules = None;
        workspace.name_cache.packages = None;
    }
    if workspace.name_cache.prefix_roots != prefix_roots {
        workspace.name_cache.prefix_roots = prefix_roots;
        workspace.name_cache.packages = None;
    }
    if workspace.name_cache.modules.is_none() {
        let mut names = BTreeSet::new();
        for directory in &workspace.name_cache.module_roots {
            for (path, is_directory) in entries(directory, stop).await? {
                if !is_directory {
                    if let Some(name) = path
                        .file_name()
                        .and_then(|name| name.to_str())
                        .and_then(|name| name.strip_suffix(".cmake"))
                    {
                        names.insert(name.to_owned());
                    }
                }
            }
            for (path, document) in &workspace.documents {
                if document.overlay.is_some() && path.parent() == Some(directory.as_path()) {
                    if let Some(name) = path
                        .file_name()
                        .and_then(|name| name.to_str())
                        .and_then(|name| name.strip_suffix(".cmake"))
                    {
                        names.insert(name.to_owned());
                    }
                }
            }
        }
        workspace.name_cache.modules = Some(Arc::from(names.into_iter().collect::<Vec<_>>()));
    }
    if request.packages && workspace.name_cache.packages.is_none() {
        let mut names = package_names(&workspace.name_cache.prefix_roots, stop).await?;
        if let Some(modules) = &workspace.name_cache.modules {
            names.extend(
                modules
                    .iter()
                    .filter_map(|name| name.strip_prefix("Find"))
                    .filter(|name| !name.is_empty())
                    .map(str::to_owned),
            );
        }
        workspace.name_cache.packages = Some(Arc::from(names.into_iter().collect::<Vec<_>>()));
    }
    check(stop)?;
    Ok(Catalog {
        modules: if request.modules {
            workspace.name_cache.modules.clone().unwrap_or_default()
        } else {
            Arc::default()
        },
        packages: if request.packages {
            workspace.name_cache.packages.clone().unwrap_or_default()
        } else {
            Arc::default()
        },
        targets: Vec::new(),
    })
}
