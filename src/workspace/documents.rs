// AI-generated implementation.
// human review over
use super::model::{Document, DocumentSnapshot, Root, RootGraph, Workspace, WorkspaceLimits};
use std::{
    io,
    path::{Component, Path, PathBuf},
    sync::Arc,
};

// Clean '.' and '..' without reading disk, so new editor files also work.
// Different symlink paths remain different file IDs.
pub(super) fn absolute(path: &Path) -> io::Result<PathBuf> {
    let path = if path.is_absolute() {
        path.to_owned()
    } else {
        std::env::current_dir()?.join(path)
    };
    let mut result = PathBuf::new();
    for part in path.components() {
        match part {
            Component::ParentDir => {
                result.pop();
            }
            Component::CurDir => {}
            part => result.push(part.as_os_str()),
        }
    }
    Ok(result)
}
impl Workspace {
    pub(super) fn document(&mut self, path: &Path) -> io::Result<&mut Document> {
        if !self.documents.contains_key(path) {
            let id = self.next_file;
            self.next_file = self
                .next_file
                .checked_add(1)
                .ok_or_else(|| io::Error::other("File ID space exhausted"))?;
            self.documents.insert(
                path.to_owned(),
                Document {
                    id,
                    revision: 0,
                    overlay: None,
                    snapshot: None,
                },
            );
        }
        self.documents
            .get_mut(path)
            .ok_or_else(|| io::Error::other("Missing document"))
    }
    /// Use editor text instead of disk text; parse it on the next flush().
    pub fn set_overlay(
        &mut self,
        path: impl AsRef<Path>,
        text: impl Into<Arc<str>>,
    ) -> io::Result<()> {
        let path = absolute(path.as_ref())?;
        let text = text.into();
        let document = self.document(&path)?;
        if document.overlay.as_ref().is_some_and(|old| **old == *text) {
            return Ok(());
        }
        document.revision = document
            .revision
            .checked_add(1)
            .ok_or_else(|| io::Error::other("Revision space exhausted"))?;
        document.overlay = Some(text);
        self.dirty.insert(path);
        Ok(())
    }
    /// Go back to disk text. Closing the editor does not remove the file from its project.
    pub fn close_overlay(&mut self, path: impl AsRef<Path>) -> io::Result<()> {
        let path = absolute(path.as_ref())?;
        if let Some(document) = self.documents.get_mut(&path) {
            if document.overlay.take().is_some() {
                document.revision = document
                    .revision
                    .checked_add(1)
                    .ok_or_else(|| io::Error::other("Revision space exhausted"))?;
                self.dirty.insert(path);
            }
        }
        Ok(())
    }
    pub fn invalidate_disk(&mut self, path: impl AsRef<Path>) -> io::Result<()> {
        self.dirty.insert(absolute(path.as_ref())?);
        Ok(())
    }
    /// Create one project's workspace. The root file need not exist yet.
    pub fn new(file: impl AsRef<Path>, binary: Option<PathBuf>) -> io::Result<Self> {
        Self::with_limits(file, binary, WorkspaceLimits::default())
    }
    pub fn with_limits(
        file: impl AsRef<Path>,
        binary: Option<PathBuf>,
        limits: WorkspaceLimits,
    ) -> io::Result<Self> {
        let root = Root {
            file: absolute(file.as_ref())?,
            binary_directory: binary.as_deref().map(absolute).transpose()?,
        };
        Ok(Self::from_root(root, limits))
    }
    pub fn snapshot(&self, path: impl AsRef<Path>) -> io::Result<Option<Arc<DocumentSnapshot>>> {
        Ok(self
            .documents
            .get(&absolute(path.as_ref())?)
            .and_then(|doc| {
                doc.snapshot
                    .as_ref()
                    .filter(|snapshot| snapshot.build.analysis.revision == doc.revision)
                    .cloned()
            }))
    }
    /// Return None until pending file or module-search changes have been processed.
    pub fn root_graph(&self) -> Option<&RootGraph> {
        self.graph.as_ref().filter(|graph| {
            !self.dirty.contains(&self.root.file)
                && graph.invocations.iter().all(|invocation| {
                    !self.dirty.contains(&invocation.file)
                        && self
                            .documents
                            .get(&invocation.file)
                            .is_some_and(|document| {
                                document.revision == invocation.analysis.revision
                            })
                })
                && graph.edges.iter().all(|edge| {
                    edge.candidates
                        .iter()
                        .all(|path| !self.dirty.contains(path))
                })
        })
    }
    pub fn set_module_paths(&mut self, paths: Vec<PathBuf>) {
        self.module_paths = paths;
        self.refresh_root();
    }
    pub fn set_builtin_module_directory(&mut self, path: Option<PathBuf>) -> io::Result<()> {
        self.builtin_module_directory = path.as_deref().map(absolute).transpose()?;
        self.refresh_root();
        Ok(())
    }
    pub(super) fn release_unused_snapshots(&mut self) {
        // Drop unused parse data, but keep file IDs. Readers may still hold the old results.
        for (path, document) in &mut self.documents {
            if document.overlay.is_none()
                && !self.dirty.contains(path)
                && !self.dependencies.contains(path)
            {
                document.snapshot = None;
            }
        }
    }
    pub fn root(&self) -> &Root {
        &self.root
    }
}
