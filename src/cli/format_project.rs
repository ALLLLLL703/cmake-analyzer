//! Project-format command assembly: load settings, traverse static dependencies,
//! select canonical project-local files, render all changes, then check or replace.
//! No configure/project script is run; version discovery may invoke `cmake --version`.
//!
//! Contains AI-generated documentation additions.
mod model;
mod write;
use crate::{
    config::{ConfigSources, ConfigStore},
    format,
    server::roots,
    workspace::Workspace,
};
use lspally::server::ExitStatus;
use model::{Edit, Input};
use std::{
    collections::{BTreeMap, BTreeSet},
    io,
    path::{Path, PathBuf},
};

/// Format resolved project-local visits or report differences. Reads config/source/metadata;
/// check prints changed paths to stdout, both modes print a summary to stderr. Write mode
/// replaces files only after every selected file renders successfully.
///
/// # Arguments
/// - `project` `[in]`: project directory or CMakeLists.txt entry.
/// - `check` `[in]`: true prevents all replacements and reports differences.
/// - `cwd` `[in]`: startup base for config and interpreter paths.
/// - `explicit` `[in]`: required highest-priority TOML file when present.
///
/// # Returns
/// `Ok(Abnormal)` for check differences, `Ok(Graceful)` otherwise; `Err` aborts on graph, render or I/O failure.
///
/// # Errors
/// Invalid entry/settings, unreadable files/metadata, traversal diagnostics, invalid syntax, worker failure or replacement error.
/// Render errors precede all writes; replacement I/O errors can leave earlier files replaced.
///
/// # Algorithm
/// Canonicalize entry/build paths, configure a static Workspace and flush. Reject graph diagnostics.
/// Deduplicate eligible canonical visits, excluding external/build/generated paths. Render on a worker, then report or apply edits in path order.
///
/// # Flow
/// ```text
///  [entry + config] ---> [static graph]
///                            |
///                      <diagnostics?> --yes--> [abort; no writes]
///                            | no
///                     [filter + deduplicate]
///                            |
///                     [render every input] --error--> [abort; no writes]
///                            |
///                        <check?>
///                        /     \
///                 [print paths] [replace files]
///                        \     /
///                        [summary/status]
/// ```
///
/// # Examples
/// ```text
/// let status = run("demo".into(), true, std::env::current_dir()?, None).await?;
/// ```
pub(super) async fn run(
    project: PathBuf,
    check: bool,
    cwd: PathBuf,
    explicit: Option<PathBuf>,
) -> io::Result<ExitStatus> {
    let root = roots::explicit_root(&project).await?;
    let directory = root
        .parent()
        .ok_or_else(|| io::Error::other("Project entry has no directory"))?;
    let directory = tokio::fs::canonicalize(directory).await?;
    let mut sources = ConfigSources::new(cwd);
    sources.explicit_file = explicit;
    let mut config = ConfigStore::load(sources).await?;
    let settings = config.for_directory(&directory).await?;
    let binary = settings
        .runtime
        .build_directory
        .clone()
        .unwrap_or_else(|| directory.join(".cmake-analyzer/build"));
    let build = match tokio::fs::canonicalize(&binary).await {
        Ok(path) => path,
        Err(error) if error.kind() == io::ErrorKind::NotFound => binary.clone(),
        Err(error) => return Err(error),
    };
    let mut workspace = Workspace::new(&root, Some(binary))?;
    workspace.set_module_lookup_enabled(settings.modules.enable);
    workspace.set_external_indexing(
        settings.modules.external_include,
        settings.modules.external_find_package,
        if settings.modules.enable {
            settings.modules.package_search_paths.clone()
        } else {
            Vec::new()
        },
    );
    workspace.set_module_paths(if settings.modules.enable {
        settings.modules.search_paths.clone()
    } else {
        Vec::new()
    });
    workspace.set_builtin_module_directory(if settings.modules.enable {
        settings.modules.builtin_directory.clone()
    } else {
        None
    })?;
    workspace.flush().await?;
    let graph = workspace
        .root_graph()
        .ok_or_else(|| io::Error::other("Project dependency graph is unavailable"))?;
    if let Some(diagnostic) = graph.diagnostics.first() {
        return Err(io::Error::other(format!(
            "Cannot format an incomplete dependency graph: {}",
            diagnostic.message
        )));
    }
    let options = format::FormatOptions {
        indent_width: settings.format.indent_width,
        line_width: settings.format.line_width,
        insert_spaces: settings.format.insert_spaces,
    };
    let mut source_directories = BTreeSet::new();
    for source in graph
        .invocations
        .iter()
        .map(|visit| visit.directory.source.clone())
        .collect::<BTreeSet<_>>()
    {
        source_directories.insert(tokio::fs::canonicalize(source).await?);
    }
    let mut inputs = BTreeMap::new();
    for visit in &graph.invocations {
        let path = tokio::fs::canonicalize(&visit.file).await?;
        if !path.starts_with(&directory)
            || build != directory && path.starts_with(&build)
            || generated(&path, &directory, &source_directories).await?
        {
            continue;
        }
        inputs.entry(path.clone()).or_insert_with(|| Input {
            path,
            snapshot: visit.snapshot.clone(),
            options,
        });
    }
    let total = inputs.len();
    // Prepare every file before the first write: syntax errors never partially format a project.
    let edits = tokio::task::spawn_blocking(move || prepare(inputs.into_values()))
        .await
        .map_err(io::Error::other)??;
    for edit in &edits {
        if check {
            println!("{}", edit.path.display());
        } else {
            write::apply(edit).await?;
        }
    }
    eprintln!(
        "{} {} of {total} project files",
        if check { "Would format" } else { "Formatted" },
        edits.len()
    );
    Ok(if check && !edits.is_empty() {
        ExitStatus::Abnormal
    } else {
        ExitStatus::Graceful
    })
}
/// Render staged snapshots and collect only changed outputs in memory. No filesystem or process effects.
///
/// # Arguments
/// - `inputs` `[in]`: consumed sequence of eligible paths, snapshots and styles.
///
/// # Returns
/// `Ok(edits)` retains iteration order and excludes unchanged text; `Err` discards the staged vector.
///
/// # Errors
/// Any render rejection becomes an I/O error annotated with its path; no file has been written.
///
/// # Algorithm
/// Render each snapshot with its own style and cancellation disabled. Retain a full edit
/// only when bytes differ from saved source. Return the entire list only after every
/// input succeeds, so callers cannot write a partial batch after a syntax error.
///
/// # Flow
/// ```text
///  [next Input] <-------------------------+
///       |                                |
///  [render snapshot] --error--> [Err]     |
///       |                                |
///   <text differs?>                      |
///      /        \                        |
///    no         yes                      |
///     |          |                       |
///  [skip]   [append staged Edit]          |
///     \          /                       |
///      [continue] -----------------------+
///             | exhausted
///         [Ok(edits)]
/// ```
///
/// # Examples
/// ```text
/// let edits = prepare(std::iter::once(input))?;
/// ```
fn prepare(inputs: impl Iterator<Item = Input>) -> io::Result<Vec<Edit>> {
    let mut edits = Vec::new();
    for input in inputs {
        let after = format::render(
            &input.snapshot.text,
            &input.snapshot.lexed,
            &input.snapshot.semantics,
            input.options,
            || false,
        )
        .map_err(|error| io::Error::other(format!("{}: {error}", input.path.display())))?;
        if after != input.snapshot.text.as_ref() {
            edits.push(Edit {
                path: input.path,
                before: input.snapshot.text.clone(),
                after,
            });
        }
    }
    Ok(edits)
}
/// Check exclusion markers without writing or running processes. Known source directories
/// remain eligible even when they contain an in-source CMake cache.
///
/// # Arguments
/// - `file` `[in]`: canonical candidate file.
/// - `root` `[in]`: canonical project directory.
/// - `source_directories` `[in]`: canonical graph source directories exempt from cache-directory exclusion.
///
/// # Returns
/// `Ok(true)` for .git/target components or a non-source ancestor with a regular CMakeCache.txt;
/// `Ok(false)` otherwise, including missing marker files; `Err` on other metadata failures.
///
/// # Errors
/// Metadata access errors except NotFound. Does not inspect the root directory cache marker.
///
/// # Algorithm
/// Reject .git/target path components first. For candidate parent directories below
/// root, inspect CMakeCache.txt; regular markers exclude the file unless that parent
/// is a known source directory. Missing markers do not exclude anything.
///
/// # Flow
/// ```text
///  [candidate below root] ---> <.git/target component?> --yes--> [true]
///                                     | no
///                             [next ancestor] <----------------+
///                                     |                        |
///                             [metadata cache marker] --error--> [Err]
///                                     |                        |
///                         <regular marker outside sources?>    |
///                              /                \              |
///                            yes                no-------------+
///                             |                    | exhausted
///                           [true]               [false]
/// ```
///
/// # Examples
/// ```text
/// let skip = generated(&file, &root, &source_directories).await?;
/// ```
async fn generated(
    file: &Path,
    root: &Path,
    source_directories: &BTreeSet<PathBuf>,
) -> io::Result<bool> {
    if file.strip_prefix(root).ok().is_some_and(|relative| {
        relative
            .components()
            .any(|part| part.as_os_str() == ".git" || part.as_os_str() == "target")
    }) {
        return Ok(true);
    }
    for parent in file
        .parent()
        .into_iter()
        .flat_map(Path::ancestors)
        .take_while(|parent| parent.starts_with(root) && *parent != root)
    {
        match tokio::fs::metadata(parent.join("CMakeCache.txt")).await {
            Ok(meta) if meta.is_file() && !source_directories.contains(parent) => return Ok(true),
            Ok(_) => {}
            Err(error) if error.kind() == io::ErrorKind::NotFound => {}
            Err(error) => return Err(error),
        }
    }
    Ok(false)
}
