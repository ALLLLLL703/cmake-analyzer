//! Discover sorted module/package names from shallow search roots and open module overlays.
//! One replaceable root/name cache per workspace avoids repeated scans; invalidation is explicit,
//! not a filesystem watcher. No package code is executed.
// Contains AI-generated documentation additions.
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

/// Read one directory into sorted file/directory candidates, following symlinks only to classify
/// directory targets. Does not recurse or mutate workspace state.
///
/// # Arguments
/// - `directory` `[in]`: Directory to enumerate; open failure is treated as no entries.
/// - `stop` `[in]`: Cancellation predicate checked between directory entries.
///
/// # Returns
/// Ok sorted (path, is_directory) pairs; Err on iteration failure or cancellation. Unknown file
/// types are skipped.
///
/// # Errors
/// Directory iteration errors or Interrupted cancellation; opening an unavailable directory instead
/// returns an empty list.
///
/// # Examples
/// ```text
/// let children = entries(Path::new("/usr/lib/cmake"), &|| false).await?;
/// ```
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
/// Borrow the package stem of a conventional config filename. No I/O or mutation.
///
/// # Arguments
/// - `path` `[in]`: Candidate path; only a UTF-8 final component is inspected.
///
/// # Returns
/// Some nonempty stem before Config.cmake or -config.cmake; None for other filenames/non-UTF-8
/// names.
///
/// # Examples
/// ```text
/// assert_eq!(config_name(Path::new("FooConfig.cmake")), Some("Foo"));
/// ```
fn config_name(path: &Path) -> Option<&str> {
    let file = path.file_name()?.to_str()?;
    file.strip_suffix("Config.cmake")
        .or_else(|| file.strip_suffix("-config.cmake"))
        .filter(|name| !name.is_empty())
}
/// Normalize and merge source-relative roots into a sorted set. Reads cwd only if the joined path
/// is relative; no filesystem probe.
///
/// # Arguments
/// - `roots` `[in,out]`: Appends normalized unique paths; preserves prior roots.
/// - `paths` `[in]`: Owned path entries to consume.
/// - `source` `[in]`: Source context against which relative paths are joined.
///
/// # Returns
/// () after insertion; unnormalizable paths are skipped.
///
/// # Examples
/// ```text
/// add_roots(&mut roots, vec!["cmake".into()], source);
/// ```
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
/// Assemble sorted unique module roots and package prefixes across all captured contexts. Reads
/// enabled environment hints and cached snapshots, not directories; no mutation.
///
/// # Arguments
/// - `workspace` `[in]`: Search configuration and known variable/reference names.
/// - `path` `[in]`: Requesting file identity used for module precedence and local variables.
/// - `contexts` `[in]`: Visit-qualified source directories and restored known hints.
///
/// # Returns
/// (module roots, package prefix roots), each normalized, sorted and unique.
///
/// # Algorithm
/// Reuse dependency module precedence then merge roots into sets. Prefixes combine configured
/// roots, known/environment CMAKE_PREFIX_PATH and known non-CMAKE *_DIR/*_ROOT variables named in
/// local or graph indexes. This discovers candidate names, not authoritative package selection.
///
/// # Flow
/// ```text
/// [visit contexts]         [configured/environment hints]
///         |                            |
///  [module roots]     [known prefix/package bindings]
///         |                            |
///  [normalize set]             [normalize set]
///         |                            |
///  [module vector]             [prefix vector]
/// ```
///
/// # Examples
/// ```text
/// let (modules, prefixes) = roots(workspace, path, &contexts);
/// ```
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
/// Discover config filename stems in fixed conventional prefix layouts. Reads shallow directories;
/// no recursive scan or package execution.
///
/// # Arguments
/// - `prefixes` `[in]`: Normalized package installation roots.
/// - `stop` `[in]`: Cancellation predicate checked before roots and directory entries.
///
/// # Returns
/// Ok sorted unique config package names; Err on cancellation or directory iteration failure.
///
/// # Errors
/// Interrupted cancellation or directory iteration errors; unavailable directories are skipped.
///
/// # Algorithm
/// Try root/cmake/CMake and package subdirectories under root, lib/cmake, lib64/cmake, share/cmake
/// and share. Add one lib architecture level. Scan each candidate directory once and retain only
/// conventional config file stems.
///
/// # Flow
/// ```text
/// [prefix loop]
///        |
///  [fixed package bases]       [lib architectures]
///        |                            |
///  [package/cmake/CMake]        [cmake packages]
///        +----------+-----------------+
///                   |
///           [deduplicate directories]
///                   |
///            [config file stems]
/// ```
///
/// # Examples
/// ```text
/// let names = package_names(&prefixes, &|| false).await?;
/// ```
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
/// Refresh root-keyed name caches and return requested labels. May scan directories; does not load
/// package code or change graph bindings.
///
/// # Arguments
/// - `workspace` `[in,out]`: Updates root lists and invalidates/replaces cached Arc name lists.
/// - `path` `[in]`: Requesting file identity.
/// - `contexts` `[in]`: Restored visit search hints.
/// - `request` `[in]`: Module/package category flags; targets are handled separately.
/// - `stop` `[in]`: Cancellation predicate checked during scans and before returning.
///
/// # Returns
/// Ok catalog with requested shared name arrays and no target sources; Err on
/// cancellation/iteration failure. Module cache is prepared even for package-only requests to
/// derive Find names.
///
/// # Errors
/// Interrupted cancellation or directory iteration errors; a cache may already have been
/// invalidated/replaced before error.
///
/// # Algorithm
/// Changed module roots invalidate both name caches; changed prefixes invalidate packages.
/// Enumerate bare .cmake modules plus open module overlays. Packages combine shallow config names
/// and nonempty Find-module stems. Reuse shared arrays until explicit workspace invalidation or
/// root-key changes; no watcher or independent name-count cap exists.
///
/// # Flow
/// ```text
/// [current root keys] ----> <changed?>
///                                /    \
///                          invalidate reuse
///                                |     |
///  [module files + overlays] --> [module Arc]
///                                |
///  [prefix configs] ------> [Find stems union]
///                                |
///                          [package Arc]
///                                |
///                       [requested catalog]
/// ```
///
/// # Examples
/// ```text
/// let catalog = collect(workspace, path, &contexts, &request, &|| false).await?;
/// ```
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
