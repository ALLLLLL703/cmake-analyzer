//! Refresh cached input-path warnings through asynchronous metadata reads; never create files or run
//! CMake.
//! Known outputs and separate build directories are exempt. Graph-owned dependency failures are reported
//! elsewhere. Checks are per refresh, not a filesystem watcher or a general argument validator.

// AI-authored cached input-path warnings; output/search/expression paths are not existence requirements.
mod candidates;
pub(in crate::server) mod model;
use super::model::Service;
use crate::parser::semanticer::model::PathKind;
use model::*;
use std::{
    collections::{HashMap, HashSet},
    io,
    path::PathBuf,
};
impl Service {
    /// Replace one project's cached path warnings after asynchronous filesystem metadata checks; never
    /// write files or run CMake.
    /// Disabled diagnostics clears the cache without disk access. Checks follow metadata symlinks but do
    /// not canonicalize exemption keys. Results persist until the service marks paths dirty and
    /// refreshes again; there is no watcher.
    ///
    /// # Arguments
    /// - `self` `[in,out]`: overwrites project.path_state and sets project.paths_dirty=false; reads
    ///   filesystem metadata but leaves source text and graph unchanged.
    /// - `owner` `[in]`: valid index of the project to refresh.
    ///
    /// # Returns
    /// (); metadata errors become warnings rather than returned errors. All warnings and exemptions
    /// replace the prior cache together.
    ///
    /// # Panics
    /// Panics if owner is not a live project index; callers use indices from the current project vector.
    ///
    /// # Algorithm
    /// Collect candidates, record output exemptions and separate configure binary directories, then skip
    /// outputs, graph-owned dependency operands and any candidate with a generated choice. Cache each
    /// metadata read for this refresh. Any valid alternative succeeds; suppress optional all-not-found
    /// failures, otherwise deduplicate by source file/start/message and store warnings. File/Module
    /// require files, Directory requires directories; other input kinds accept either.
    ///
    /// # Flow
    /// ```text
    ///  [Diagnostics enabled?] --no--> [Clear cache + dirty flag]
    ///              | yes
    ///  [Collect candidates + generated/build exemptions]
    ///              |
    ///  [Next input] <--------------------------------+
    ///              |                                 |
    ///  [Output / graph dependency / generated?] --yes--+
    ///              | no                              |
    ///  [Cached or async metadata for choices]         |
    ///              |                                 |
    ///  [Any valid / optional all missing?] --yes------+
    ///              | no                              |
    ///  [Append deduplicated warning] ----------------+
    ///              | end
    ///  [Replace state; clear dirty flag]
    /// ```
    ///
    /// # Examples
    /// Local server context (requires the initialized service and its client):
    /// ```text
    /// service.refresh_paths(owner).await;
    /// ```
    pub(in crate::server) async fn refresh_paths(&mut self, owner: usize) {
        if !self.projects[owner].settings.diagnostics.enable {
            self.projects[owner].path_state = PathState::default();
            self.projects[owner].paths_dirty = false;
            return;
        }
        let inputs = candidates::collect(&self.projects[owner]);
        let generated: HashSet<_> = inputs
            .iter()
            .filter(|input| input.kind == PathKind::Output)
            .flat_map(|input| input.choices.iter().cloned())
            .collect();
        let project = &self.projects[owner];
        let mut build_directories = Vec::new();
        if matches!(
            project.execution,
            Some(crate::runtime_cmake::model::ExecutionMode::Configure)
        ) {
            if let Some(base) = project.workspace.root().file.parent() {
                if let Ok(path) = crate::parser::interpreter_commands::resolve_path(
                    base,
                    &project
                        .settings
                        .runtime
                        .build_directory
                        .clone()
                        .unwrap_or_else(|| base.join(".cmake-analyzer/build")),
                ) {
                    if path != base {
                        build_directories.push(path);
                    }
                }
            }
            if let Some(graph) = project.workspace.root_graph() {
                for visit in &graph.invocations {
                    if let Some(binary) = &visit.directory.binary {
                        if binary != &visit.directory.source {
                            build_directories.push(binary.clone());
                        }
                    }
                }
            }
        }
        let mut state = PathState {
            warnings: Vec::new(),
            generated,
            build_directories,
        };
        let mut metadata: HashMap<PathBuf, Result<(bool, bool), (io::ErrorKind, String)>> =
            HashMap::new();
        let mut emitted = HashSet::new();
        let graph = self.projects[owner].workspace.root_graph().is_some();
        for input in inputs {
            if input.kind == PathKind::Output || input.dependency && graph {
                continue;
            }
            if input.choices.iter().any(|path| state.is_generated(path)) {
                continue;
            }
            let mut failure = None;
            let mut valid = false;
            let mut missing_only = true;
            for path in &input.choices {
                if !metadata.contains_key(path) {
                    let result = match tokio::fs::metadata(path).await {
                        Ok(metadata) => Ok((metadata.is_file(), metadata.is_dir())),
                        Err(error) if error.kind() == io::ErrorKind::NotFound => Err((
                            io::ErrorKind::NotFound,
                            format!("Input path does not exist: {path:?}"),
                        )),
                        Err(error) => Err((
                            error.kind(),
                            format!("Cannot inspect input path {path:?}: {error}"),
                        )),
                    };
                    metadata.insert(path.clone(), result);
                }
                match &metadata[path] {
                    Ok((file, directory)) => {
                        let expected = match input.kind {
                            PathKind::File | PathKind::Module => *file,
                            PathKind::Directory => *directory,
                            _ => true,
                        };
                        if expected {
                            valid = true;
                            break;
                        }
                        missing_only = false;
                        failure = Some(format!(
                            "Input path is not a {}: {path:?}",
                            if input.kind == PathKind::Directory {
                                "directory"
                            } else {
                                "file"
                            }
                        ));
                    }
                    Err((kind, message)) => {
                        missing_only &= *kind == io::ErrorKind::NotFound;
                        failure = Some(message.clone());
                    }
                }
            }
            if !valid && !(input.optional && missing_only) {
                let message = failure.unwrap_or_else(|| "Input module could not be found".into());
                if emitted.insert((input.file.clone(), input.span.start_byte, message.clone())) {
                    state.warnings.push(PathWarning {
                        file: input.file,
                        span: input.span,
                        message,
                    });
                }
            }
        }
        self.projects[owner].path_state = state;
        self.projects[owner].paths_dirty = false;
    }
}
