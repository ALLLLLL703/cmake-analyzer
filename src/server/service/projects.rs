//! Assign files to independent Workspaces and synchronize editor overlays.
//! Explicit interpreter bindings win over discovered dependency membership, which
//! wins over the nearest containing project directory and then file-only fallback.
//! One file can belong to several projects. Flushing repeats after new dependencies
//! change ownership; no recursive directory scan or project execution is performed.
// AI-authored dependency-first Workspace ownership.
use super::{
    super::{model::*, roots},
    internal,
};
use crate::{
    parser::interpreter_commands::InterpreterServices, runtime_cmake::model::ExecutionMode,
    workspace::Workspace,
};
use lspally::server::model::*;
use std::{
    collections::{BTreeMap, BTreeSet},
    path::{Path, PathBuf},
    sync::Arc,
};

impl Service {
    /// Select an inferred configure or explicit script profile, then reuse/admit a Workspace.
    /// May load layered settings and native module defaults; does not execute project code.
    ///
    /// # Arguments
    /// - `self` `[in,out]`: Project/configuration state: may append a profile and override its runtime settings from a script mapping.
    /// - `entry` `[in]`: Consumed normalized entry file.
    /// - `standalone` `[in]`: True for file-only fallback, suppressing inference and mapping selection.
    /// - `binary` `[in]`: Optional explicit binary directory for Workspace identity.
    ///
    /// # Returns
    /// `Ok(index)` into the current projects vector, reused or newly appended; `Err` on settings/admission/path failure.
    ///
    /// # Errors
    /// Configuration loading, Workspace setup, or workspace-count limit errors; earlier configuration-cache updates may remain.
    ///
    /// # Algorithm
    /// Infer configure only for nonstandalone CMakeLists.txt. A matching script mapping overrides filename inference. Admit using full profile identity, then copy selecting script runtime settings if present.
    ///
    /// # Flow
    /// ```text
    ///  [Entry + standalone]
    ///            |
    ///     <Mapped script?>
    ///       /          \
    ///     yes          no
    ///      |            |
    ///  [Script mode] [Infer configure / none]
    ///      \            /
    ///       [Reuse or admit profile]
    ///                 |
    ///        [Apply script runtime settings]
    /// ```
    ///
    /// # Examples
    /// ```text
    /// let owner = service.add_project(entry, false, None).await?;
    /// ```
    pub(super) async fn add_project(
        &mut self,
        entry: PathBuf,
        standalone: bool,
        binary: Option<PathBuf>,
    ) -> ServerResult<usize> {
        let mut execution = (!standalone
            && entry
                .file_name()
                .is_some_and(|name| name == "CMakeLists.txt"))
        .then_some(ExecutionMode::Configure);
        let mut profile = None;
        if !standalone {
            let directory = entry
                .parent()
                .unwrap_or(&self.config.sources.cwd)
                .to_owned();
            let settings = self.mapping_settings(&directory, &entry).await?;
            if matches!(settings.interpreter_commands.get(&entry),Some(InterpreterServices::Script {file}) if file==&entry)
            {
                execution = Some(ExecutionMode::Script {
                    file: entry.clone(),
                });
                profile = Some(settings);
            }
        }
        let index = self
            .add_execution_project(entry, standalone, binary, execution)
            .await?;
        if let Some(settings) = profile {
            Arc::make_mut(&mut self.projects[index].settings).runtime = settings.runtime.clone();
        }
        Ok(index)
    }
    /// Load entry-directory settings and fall back to startup-directory interpreter mappings.
    /// Updates only the configuration cache; does not create projects or execute commands.
    ///
    /// # Arguments
    /// - `self` `[in,out]`: Configuration store whose directory settings cache may be populated; projects and open text are unchanged.
    /// - `directory` `[in]`: Directory whose layered configuration is consulted first.
    /// - `path` `[in]`: Normalized file key to look up in interpreter_commands.
    ///
    /// # Returns
    /// `Ok(settings)` from the entry directory if it maps path, otherwise startup settings if they map it, otherwise the original directory settings.
    ///
    /// # Errors
    /// Configuration/default discovery failures from either directory lookup.
    ///
    /// # Algorithm
    /// Prefer a mapping in the entry directory. Only if absent, load startup settings
    /// and use them when they map the same file; otherwise retain entry settings.
    ///
    /// # Flow
    /// ```text
    ///  [Entry-directory settings]
    ///              |
    ///       <Maps file?> -- yes --> [Return entry settings]
    ///              | no
    ///      [Startup settings]
    ///              |
    ///       <Maps file?> -- yes --> [Return startup settings]
    ///              | no
    ///      [Return entry settings]
    /// ```
    ///
    /// # Examples
    /// ```text
    /// let settings = service.mapping_settings(&directory, &path).await?;
    /// ```
    async fn mapping_settings(
        &mut self,
        directory: &Path,
        path: &Path,
    ) -> ServerResult<Arc<crate::config::Settings>> {
        let settings = self
            .config
            .for_directory(directory)
            .await
            .map_err(internal)?;
        if settings.interpreter_commands.contains_key(path) {
            return Ok(settings);
        }
        let cwd = self.config.sources.cwd.clone();
        let startup = self.config.for_directory(&cwd).await.map_err(internal)?;
        Ok(if startup.interpreter_commands.contains_key(path) {
            startup
        } else {
            settings
        })
    }
    /// Reuse an exact analysis profile or append a configured independent Workspace.
    /// Reads settings and may inspect module defaults; does not parse source or launch project CMake.
    ///
    /// # Arguments
    /// - `self` `[in,out]`: Project list appended only after successful setup; configuration caches may also be populated.
    /// - `entry` `[in]`: Consumed normalized analysis entry.
    /// - `standalone` `[in]`: Whether only local documents will be flushed.
    /// - `binary` `[in]`: Optional binary-directory identity.
    /// - `execution` `[in]`: Configure/script identity or None; part of profile reuse matching.
    ///
    /// # Returns
    /// `Ok(index)` of an existing matching profile or newly appended project; `Err` without appending if setup fails.
    ///
    /// # Errors
    /// Workspace-count admission limit, configuration loading or Workspace path/module setup failures.
    ///
    /// # Algorithm
    /// Compare entry, standalone, binary and execution identity before checking capacity. Load settings, configure script context/module indexing, then append an initially dirty project (except standalone) with its entry as the initial member.
    ///
    /// # Flow
    /// ```text
    ///  [Requested profile]
    ///           |
    ///      <Exact match?>
    ///       /          \
    ///     yes          no
    ///      |            |
    ///  [Old index] [Limit + settings]
    ///                   |
    ///             [Setup Workspace]
    ///                   |
    ///            [Append + new index]
    /// ```
    ///
    /// # Examples
    /// ```text
    /// let owner = service.add_execution_project(entry, false, None,
    ///     Some(ExecutionMode::Configure)).await?;
    /// ```
    async fn add_execution_project(
        &mut self,
        entry: PathBuf,
        standalone: bool,
        binary: Option<PathBuf>,
        execution: Option<ExecutionMode>,
    ) -> ServerResult<usize> {
        if let Some(index) = self.projects.iter().position(|project| {
            project.workspace.root().file == entry
                && project.standalone == standalone
                && project.workspace.root().binary_directory == binary
                && project.execution == execution
        }) {
            return Ok(index);
        }
        if self.projects.len() >= self.config.limits.max_workspaces {
            return Err(ServerError::new(
                -32000,
                format!(
                    "Workspace limit ({}) reached",
                    self.config.limits.max_workspaces
                ),
            ));
        }
        let directory = entry
            .parent()
            .unwrap_or(&self.config.sources.cwd)
            .to_owned();
        let settings = self
            .config
            .for_directory(&directory)
            .await
            .map_err(internal)?;
        let mut workspace = Workspace::new(&entry, binary).map_err(internal)?;
        if matches!(execution, Some(ExecutionMode::Script { .. })) {
            workspace
                .set_script_working_directory(&self.config.sources.cwd)
                .map_err(internal)?;
        }
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
        workspace
            .set_builtin_module_directory(if settings.modules.enable {
                settings.modules.builtin_directory.clone()
            } else {
                None
            })
            .map_err(internal)?;
        let index = self.projects.len();
        self.projects.push(Project {
            path_state: Default::default(),
            paths_dirty: true,
            workspace,
            settings,
            bindings: BTreeSet::new(),
            standalone,
            execution,
            dirty: !standalone,
            members: BTreeSet::from([entry]),
            overlays: BTreeMap::new(),
        });
        Ok(index)
    }
    /// Select current owners by explicit bindings, dependency membership, directory proximity, then file-only fallback.
    /// No I/O or state mutation occurs; indices are valid only while the projects vector is unchanged.
    ///
    /// # Arguments
    /// - `self` `[in]`: Current project bindings, membership and root directories read without mutation.
    /// - `path` `[in]`: Normalized absolute file identity.
    ///
    /// # Returns
    /// All explicitly bound owners if any; otherwise all nonstandalone dependency members; otherwise one nearest containing CMakeLists project; otherwise matching standalone owners, possibly empty.
    ///
    /// # Algorithm
    /// Stop at the first nonempty ownership class. Directory fallback chooses the longest root entry path and only applies to ordinary CMakeLists entries.
    ///
    /// # Flow
    /// ```text
    ///  [File path]
    ///       |
    ///  <Bound owners?> -- yes --> [All bindings]
    ///       | no
    ///  <Dependency owners?> -- yes --> [All importers]
    ///       | no
    ///  <Containing project?> -- yes --> [Nearest one]
    ///       | no
    ///  [Matching standalone / empty]
    /// ```
    ///
    /// # Examples
    /// ```text
    /// let owners = service.owners(&module_path);
    /// ```
    pub(in crate::server) fn owners(&self, path: &Path) -> Vec<usize> {
        let bound: Vec<_> = self
            .projects
            .iter()
            .enumerate()
            .filter(|(_, project)| project.bindings.contains(path))
            .map(|(index, _)| index)
            .collect();
        if !bound.is_empty() {
            return bound;
        }
        let dependencies: Vec<_> = self
            .projects
            .iter()
            .enumerate()
            .filter(|(_, p)| !p.standalone && p.members.contains(path))
            .map(|(i, _)| i)
            .collect();
        if !dependencies.is_empty() {
            return dependencies;
        }
        let nearest = self
            .projects
            .iter()
            .enumerate()
            .filter(|(_, p)| {
                !p.standalone
                    && p.workspace
                        .root()
                        .file
                        .file_name()
                        .is_some_and(|name| name == "CMakeLists.txt")
                    && p.workspace
                        .root()
                        .file
                        .parent()
                        .is_some_and(|root| path.starts_with(root))
            })
            .max_by_key(|(_, p)| p.workspace.root().file.components().count())
            .map(|(i, _)| i);
        if let Some(index) = nearest {
            return vec![index];
        }
        self.projects
            .iter()
            .enumerate()
            .filter(|(_, p)| p.standalone && p.workspace.root().file == path)
            .map(|(i, _)| i)
            .collect()
    }
    /// Resolve mappings/discovery and admit a project when no current owner suffices.
    /// May read filesystem metadata/settings and append profiles; never executes interpreter mappings.
    ///
    /// # Arguments
    /// - `self` `[in,out]`: Project/config caches and root/binding sets may grow; editor text is not changed.
    /// - `path` `[in]`: Normalized file whose owner must exist.
    ///
    /// # Returns
    /// `Ok(indices)` for the selected owners; mapped files receive their binding. `Err` on discovery, configuration or project admission failure.
    ///
    /// # Errors
    /// Metadata discovery, configuration/default loading, workspace-count limit or Workspace setup errors. Cache/admission changes before a later failure are not rolled back.
    ///
    /// # Algorithm
    /// Honor existing explicit bindings. Use an existing owner root or ancestor discovery as configuration context, check directory/startup interpreter mappings, then reuse existing owners or create a discovered/file-only project. Script mappings inherit the selecting runtime settings; newly discovered ordinary roots enter the root set.
    ///
    /// # Flow
    /// ```text
    ///  [Current owners]
    ///         |
    ///  <Explicit binding?> -- yes --> [Return owners]
    ///         | no
    ///  [Discover + mapping settings]
    ///         |
    ///     <Mapping?> -- yes --> [Admit profile + bind file]
    ///         | no
    ///  <Owners exist?> -- yes --> [Return owners]
    ///         | no
    ///  [Admit discovered / standalone]
    /// ```
    ///
    /// # Examples
    /// ```text
    /// let owners = service.ensure_owners(&path).await?;
    /// ```
    pub(super) async fn ensure_owners(&mut self, path: &Path) -> ServerResult<Vec<usize>> {
        let owners = self.owners(path);
        if owners
            .iter()
            .any(|owner| self.projects[*owner].bindings.contains(path))
        {
            return Ok(owners);
        }
        let discovered = if let Some(owner) = owners.first() {
            Some(self.projects[*owner].workspace.root().file.clone())
        } else {
            roots::find_project(path).await.map_err(internal)?
        };
        let directory = discovered
            .as_deref()
            .and_then(Path::parent)
            .or_else(|| path.parent())
            .unwrap_or(&self.config.sources.cwd)
            .to_owned();
        let settings = self.mapping_settings(&directory, path).await?;
        if let Some(services) = settings.interpreter_commands.get(path) {
            let execution = match services {
                InterpreterServices::Project { .. } => ExecutionMode::Configure,
                InterpreterServices::Script { file } => {
                    ExecutionMode::Script { file: file.clone() }
                }
            };
            let script = matches!(execution, ExecutionMode::Script { .. });
            let index = self
                .add_execution_project(
                    services.entry(),
                    false,
                    services.binary_directory(),
                    Some(execution),
                )
                .await?;
            if script {
                Arc::make_mut(&mut self.projects[index].settings).runtime =
                    settings.runtime.clone();
            }
            self.projects[index].bindings.insert(path.to_owned());
            return Ok(vec![index]);
        }
        if !owners.is_empty() {
            return Ok(owners);
        }
        let (root, standalone) = match discovered {
            Some(root) => (root, false),
            None => (path.to_owned(), true),
        };
        let index = self.add_project(root.clone(), standalone, None).await?;
        if !standalone {
            self.roots.insert(root);
        }
        Ok(vec![index])
    }
    /// Remove overlays from former owners and install newer editor versions in current owners.
    /// Changes Workspace caches and marks affected projects dirty; no disk writes or parsing occurs.
    ///
    /// # Arguments
    /// - `self` `[in,out]`: Projects whose overlay maps/Workspace caches are updated and dirty flags set; documents are read only.
    ///
    /// # Returns
    /// `Ok(())` after synchronizing all open documents, or `Err` after a Workspace path/cache failure.
    ///
    /// # Errors
    /// Workspace close_overlay/set_overlay errors; earlier removals/installs remain applied.
    ///
    /// # Algorithm
    /// Capture ownership before mutating projects. Remove each obsolete overlay, then install text where the recorded version differs from the authoritative open version.
    ///
    /// # Flow
    /// ```text
    ///  [Documents + ownership map]
    ///               |
    ///     [Each project: old overlays]
    ///               |
    ///        <Still owned/open?>
    ///         /             \
    ///        no             yes
    ///        |               |
    ///  [Close + dirty] [Compare versions]
    ///                        |
    ///                [Install changed + dirty]
    /// ```
    ///
    /// # Examples
    /// ```text
    /// service.sync_overlays()?;
    /// ```
    pub(super) fn sync_overlays(&mut self) -> ServerResult<()> {
        let ownership: BTreeMap<_, _> = self
            .documents
            .keys()
            .map(|path| (path.clone(), self.owners(path)))
            .collect();
        for (index, project) in self.projects.iter_mut().enumerate() {
            let removed: Vec<_> = project
                .overlays
                .keys()
                .filter(|path| {
                    !ownership
                        .get(*path)
                        .is_some_and(|owners| owners.contains(&index))
                })
                .cloned()
                .collect();
            for path in removed {
                project.overlays.remove(&path);
                project.workspace.close_overlay(&path).map_err(internal)?;
                project.dirty = true;
            }
        }
        for (path, document) in &self.documents {
            for &owner in &ownership[path] {
                let project = &mut self.projects[owner];
                if project.overlays.get(path) != Some(&document.version) {
                    project
                        .workspace
                        .set_overlay(path, document.text.clone())
                        .map_err(internal)?;
                    project.overlays.insert(path.clone(), document.version);
                    project.dirty = true;
                }
            }
        }
        Ok(())
    }
    /// Synchronize overlays and repeatedly refresh dirty projects until ownership settles, then refresh path warnings.
    /// May read/parse source and inspect input paths; no project execution or disk writes.
    ///
    /// # Arguments
    /// - `self` `[in,out]`: Projects, ownership, overlays, analysis/path caches and fallback project list refreshed in place.
    ///
    /// # Returns
    /// `Ok(())` after synchronization and path refresh; `Err` for overlay/owner admission failures. Per-Workspace analysis failures are logged and not returned.
    ///
    /// # Errors
    /// Overlay synchronization and ensure_owners errors; state can already contain successful updates from earlier passes.
    ///
    /// # Algorithm
    /// Apply overlays, clear each dirty flag before its joined flush, and rebuild membership from visits, dependency candidates and runtime documents when a graph exists. Discover owners for unbound opens and resynchronize overlays, repeating while dirty. Prune redundant/closed standalone caches, then refresh dirty path state. Logged flush errors do not restore dirty flags.
    ///
    /// # Flow
    /// ```text
    ///  [Sync overlays] <-------------------------+
    ///         |                                  |
    ///  <Any dirty projects?>                      |
    ///      /           \                         |
    ///    yes           no                         |
    ///     |             |                         |
    ///  [Flush dirty] [Prune standalone]            |
    ///     |             |                         |
    ///  [Update members] [Refresh path warnings]    |
    ///     |                                       |
    ///  [Ensure unbound owners] -------------------+
    /// ```
    ///
    /// # Examples
    /// ```text
    /// service.flush().await?;
    /// ```
    pub(super) async fn flush(&mut self) -> ServerResult<()> {
        self.sync_overlays()?;
        // A new project may discover an already-open external module. Apply its overlay and revisit once needed.
        while self.projects.iter().any(|project| project.dirty) {
            for project in &mut self.projects {
                if !project.dirty {
                    continue;
                }
                project.dirty = false;
                project.paths_dirty = true;
                let result = if project.standalone {
                    project.workspace.flush_documents().await
                } else {
                    project.workspace.flush().await
                };
                if let Err(error) = result {
                    eprintln!(
                        "Workspace {}: {error}",
                        project.workspace.root().file.display()
                    );
                }
                if let Some(graph) = project.workspace.root_graph() {
                    project.members = graph
                        .invocations
                        .iter()
                        .map(|visit| visit.file.clone())
                        .chain(
                            graph
                                .edges
                                .iter()
                                .flat_map(|edge| edge.candidates.iter().cloned()),
                        )
                        .collect();
                    if let Some(layer) = project.workspace.runtime() {
                        project.members.extend(layer.documents.keys().cloned());
                    }
                }
            }
            let unbound: Vec<_> = self
                .documents
                .keys()
                .filter(|path| self.owners(path).is_empty())
                .cloned()
                .collect();
            for path in unbound {
                self.ensure_owners(&path).await?;
            }
            self.sync_overlays()?;
        }
        // Standalone caches become redundant once a project owns their file.
        let redundant: BTreeSet<_> = self
            .projects
            .iter()
            .filter(|project| project.standalone)
            .filter(|project| {
                self.owners(&project.workspace.root().file)
                    .iter()
                    .any(|owner| !self.projects[*owner].standalone)
            })
            .map(|project| project.workspace.root().file.clone())
            .collect();
        self.projects.retain(|project| {
            !project.standalone
                || (self.documents.contains_key(&project.workspace.root().file)
                    && !redundant.contains(&project.workspace.root().file))
        });
        for owner in 0..self.projects.len() {
            if self.projects[owner].paths_dirty {
                self.refresh_paths(owner).await;
            }
        }
        Ok(())
    }
}
