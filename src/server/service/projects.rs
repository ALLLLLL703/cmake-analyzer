// AI-authored dependency-first Workspace ownership.
use super::{
    super::{model::*, roots},
    internal,
};
use crate::workspace::Workspace;
use lspally::server::model::*;
use std::{
    collections::{BTreeMap, BTreeSet},
    path::{Path, PathBuf},
};

impl Service {
    pub(super) async fn add_project(
        &mut self,
        entry: PathBuf,
        standalone: bool,
        binary: Option<PathBuf>,
    ) -> ServerResult<usize> {
        if let Some(index) = self.projects.iter().position(|project| {
            project.workspace.root().file == entry
                && project.standalone == standalone
                && project.workspace.root().binary_directory == binary
        }) {
            return Ok(index);
        }
        if self.projects.len() >= 32 {
            return Err(ServerError::new(-32000, "Workspace limit (32) reached"));
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
            workspace,
            settings,
            bindings: BTreeSet::new(),
            standalone,
            dirty: !standalone,
            members: BTreeSet::from([entry]),
            overlays: BTreeMap::new(),
        });
        Ok(index)
    }
    pub(super) fn owners(&self, path: &Path) -> Vec<usize> {
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
        let settings = self
            .config
            .for_directory(&directory)
            .await
            .map_err(internal)?;
        if let Some(services) = settings.interpreter_commands.get(path) {
            let index = self
                .add_project(services.entry(), false, services.binary_directory())
                .await?;
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
    pub(super) async fn flush(&mut self) -> ServerResult<()> {
        self.sync_overlays()?;
        // A new project may discover an already-open external module. Apply its overlay and revisit once needed.
        while self.projects.iter().any(|project| project.dirty) {
            for project in &mut self.projects {
                if !project.dirty {
                    continue;
                }
                project.dirty = false;
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
        Ok(())
    }
}
