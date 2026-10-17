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
    let options = format::Options {
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
