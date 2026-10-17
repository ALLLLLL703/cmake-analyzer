//! Normalize file identities, queue editor/disk changes, expose current results and release unused caches.
//! Paths are lexical absolute paths, not canonical symlink identities; setters never write files.
// Contains AI-generated documentation additions.
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
/// Normalize an absolute lexical path, joining relative paths to process cwd. Reads cwd only for
/// relative input; does not resolve symlinks or test existence.
///
/// # Arguments
/// - `path` `[in]`: File or directory spelling to normalize.
///
/// # Returns
/// Ok normalized path; Err when obtaining cwd fails.
///
/// # Errors
/// Returns the current-directory error for relative input if cwd cannot be read.
///
/// # Examples
/// ```text
/// let path = absolute(Path::new("/demo/./cmake/../CMakeLists.txt"))?;
/// ```
pub(crate) fn absolute(path: &Path) -> io::Result<PathBuf> {
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
    /// Return or create the stable per-file cache slot. No file reads occur.
    ///
    /// # Arguments
    /// - `self` `[in,out]`: Workspace; may insert a document and increment next_file.
    /// - `path` `[in]`: Already-normalized absolute identity; this helper does not normalize it.
    ///
    /// # Returns
    /// Ok mutable document slot; Err if the file ID space is exhausted or the inserted slot is
    /// unavailable.
    ///
    /// # Errors
    /// File ID overflow or an unavailable document entry; overflow leaves the new slot uncreated.
    ///
    /// # Examples
    /// ```text
    /// let document = workspace.document(&absolute_path)?;
    /// ```
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
                    analysis: None,
                },
            );
        }
        self.documents
            .get_mut(path)
            .ok_or_else(|| io::Error::other("Missing document"))
    }
    /// Use editor text instead of disk text; parse it on the next flush().
    /// Replace pending editor text, advance its revision and mark the path dirty. No disk writes or
    /// parsing occur. A changed snapshot text retires the runtime layer.
    ///
    /// # Arguments
    /// - `self` `[in,out]` : Workspace; overwrites the overlay, invalidates discovery for a new
    ///   file and queues the path.
    /// - `path` `[in]`: File identity, normalized lexically to absolute.
    /// - `text` `[in]`: Newest editor text; replaces, not appends to, prior text.
    ///
    /// # Returns
    /// Ok(()) when queued or identical to the existing overlay; Err on identity/revision failure.
    ///
    /// # Errors
    /// Current-directory lookup, file-ID exhaustion or revision overflow. No disk contents are
    /// changed.
    ///
    /// # Algorithm
    /// Normalize the identity and obtain its stable document slot. An identical existing overlay is
    /// a no-op. Otherwise check revision increment, compare against saved text, replace the
    /// overlay, clear runtime only when saved text differs, and coalesce the path into pending
    /// work.
    ///
    /// # Flow
    /// ```text
    /// [normalized path + text]
    ///             |
    ///  <same existing overlay?> -- yes --> [no-op]
    ///             | no
    ///  [checked revision increment]
    ///             |
    ///  [compare saved text] --> [replace overlay]
    ///             |                    |
    ///        <text changed?>          |
    ///           /       \             |
    ///  clear runtime    keep runtime   |
    ///           +-----------+----------+
    ///                       |
    ///                [mark path dirty]
    /// ```
    ///
    /// # Examples
    /// ```
    /// use cmake_analyzer::workspace::Workspace;
    /// let mut workspace = Workspace::new("/demo/CMakeLists.txt", None)?;
    /// workspace.set_overlay("/demo/CMakeLists.txt", "cmake_minimum_required(VERSION 3.31)\nproject(Editor)")?;
    /// # Ok::<(), std::io::Error>(())
    /// ```
    pub fn set_overlay(
        &mut self,
        path: impl AsRef<Path>,
        text: impl Into<Arc<str>>,
    ) -> io::Result<()> {
        let path = absolute(path.as_ref())?;
        let text = text.into();
        if !self.documents.contains_key(&path) {
            self.name_cache = Default::default();
        }
        let document = self.document(&path)?;
        if document.overlay.as_ref().is_some_and(|old| **old == *text) {
            return Ok(());
        }
        document.revision = document
            .revision
            .checked_add(1)
            .ok_or_else(|| io::Error::other("Revision space exhausted"))?;
        let changed = document
            .snapshot
            .as_ref()
            .is_none_or(|snapshot| snapshot.text != text);
        document.overlay = Some(text);
        if changed {
            self.clear_runtime();
        }
        self.dirty.insert(path);
        Ok(())
    }
    /// Go back to disk text. Closing the editor does not remove the file from its project.
    /// Remove an editor override and queue disk fallback without removing project membership.
    /// Clears discovery/runtime state even when no overlay exists; does not read/write disk.
    ///
    /// # Arguments
    /// - `self` `[in,out]` : Workspace; removes the overlay, increments revision when present and
    ///   marks the path dirty.
    /// - `path` `[in]`: File identity to normalize.
    ///
    /// # Returns
    /// Ok(()) even for an unknown file; Err on normalization or revision overflow.
    ///
    /// # Errors
    /// Current-directory failure or revision overflow. The overlay is already removed if
    /// incrementing its revision fails.
    ///
    /// # Algorithm
    /// Clear name/runtime caches and normalize the identity. For an existing overlay remove it,
    /// check and advance the revision, then queue the file for disk fallback; absent
    /// documents/overlays require no parse work.
    ///
    /// # Flow
    /// ```text
    /// [clear discovery/runtime]
    ///             |
    ///      [normalize identity]
    ///             |
    ///  <existing overlay?> -- no --> [done]
    ///             | yes
    ///        [remove overlay]
    ///             |
    ///      <revision increment?>
    ///         /           \
    ///       error        [mark dirty]
    ///         |              |
    ///  [overlay removed] [disk read deferred]
    /// ```
    ///
    /// # Examples
    /// ```
    /// use cmake_analyzer::workspace::Workspace;
    /// let mut workspace = Workspace::new("/demo/CMakeLists.txt", None)?;
    /// workspace.close_overlay("/demo/CMakeLists.txt")?;
    /// # Ok::<(), std::io::Error>(())
    /// ```
    pub fn close_overlay(&mut self, path: impl AsRef<Path>) -> io::Result<()> {
        self.name_cache = Default::default();
        self.clear_runtime();
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
    /// Queue a known disk change and clear discovery/runtime caches. Does not inspect the file or
    /// advance its revision immediately.
    ///
    /// # Arguments
    /// - `self` `[in,out]`: Workspace; inserts the normalized path into pending work.
    /// - `path` `[in]`: Changed or newly created file/search-candidate identity.
    ///
    /// # Returns
    /// Ok(()) after queuing; Err if relative-path normalization cannot read cwd.
    ///
    /// # Errors
    /// Current-directory lookup can fail; discovery/runtime caches are cleared before
    /// normalization.
    ///
    /// # Examples
    /// ```
    /// use cmake_analyzer::workspace::Workspace;
    /// let mut workspace = Workspace::new("/demo/CMakeLists.txt", None)?;
    /// workspace.invalidate_disk("/demo/new.cmake")?;
    /// # Ok::<(), std::io::Error>(())
    /// ```
    pub fn invalidate_disk(&mut self, path: impl AsRef<Path>) -> io::Result<()> {
        self.name_cache = Default::default();
        self.clear_runtime();
        self.dirty.insert(absolute(path.as_ref())?);
        Ok(())
    }
    /// Create one project's workspace. The root file need not exist yet.
    /// Create a deferred one-root workspace with default limits. The root need not exist; no files
    /// are read or created.
    ///
    /// # Arguments
    /// - `file` `[in]`: Root file, not a directory; normalized lexically to absolute.
    /// - `binary` `[in]`: Optional build directory, normalized but never created.
    ///
    /// # Returns
    /// Ok workspace with its root queued; Err if a relative path cannot be resolved against cwd.
    ///
    /// # Errors
    /// Current-directory lookup for a relative root/build directory can fail.
    ///
    /// # Examples
    /// ```
    /// use cmake_analyzer::workspace::Workspace;
    /// let mut workspace = Workspace::new("/demo/CMakeLists.txt", None)?;
    /// assert_eq!(workspace.root().file, std::path::Path::new("/demo/CMakeLists.txt"));
    /// # Ok::<(), std::io::Error>(())
    /// ```
    pub fn new(file: impl AsRef<Path>, binary: Option<PathBuf>) -> io::Result<Self> {
        Self::with_limits(file, binary, WorkspaceLimits::default())
    }
    /// Create deferred workspace state with caller-supplied budgets; no file I/O or traversal
    /// occurs.
    ///
    /// # Arguments
    /// - `file` `[in]`: Root file identity; existence is not required.
    /// - `binary` `[in]`: Optional build directory identity; not created.
    /// - `limits` `[in]`: Per-workspace budgets; workers is clamped to at least one.
    ///
    /// # Returns
    /// Ok initialized workspace; Err on relative-path normalization failure.
    ///
    /// # Errors
    /// Current-directory lookup for relative paths can fail.
    ///
    /// # Examples
    /// ```
    /// use cmake_analyzer::workspace::{Workspace, WorkspaceLimits};
    /// let limits = WorkspaceLimits { files: 32, ..Default::default() };
    /// let workspace = Workspace::with_limits("/demo/CMakeLists.txt", None, limits)?;
    /// # Ok::<(), std::io::Error>(())
    /// ```
    pub fn with_limits(
        file: impl AsRef<Path>,
        binary: Option<PathBuf>,
        limits: WorkspaceLimits,
    ) -> io::Result<Self> {
        let root = Root {
            script_working_directory: None,
            file: absolute(file.as_ref())?,
            binary_directory: binary.as_deref().map(absolute).transpose()?,
        };
        Ok(Self::from_root(root, limits))
    }
    /// Clone a shared saved result only when it matches the current document revision. No disk
    /// access or analysis occurs.
    ///
    /// # Arguments
    /// - `self` `[in]`: Workspace cache, unchanged.
    /// - `path` `[in]`: File identity to normalize and query.
    ///
    /// # Returns
    /// Ok(Some(snapshot)) for a current saved revision; Ok(None) for
    /// unknown/unparsed/revision-stale files; Err on normalization failure. Disk invalidation alone
    /// is processed by flush, not by this query.
    ///
    /// # Errors
    /// Current-directory lookup for a relative path can fail.
    ///
    /// # Examples
    /// ```
    /// use cmake_analyzer::workspace::Workspace;
    /// let mut workspace = Workspace::new("/demo/CMakeLists.txt", None)?;
    /// assert!(workspace.snapshot("/demo/CMakeLists.txt")?.is_none());
    /// # Ok::<(), std::io::Error>(())
    /// ```
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
    /// Borrow the saved graph unless a root, visited file or lookup candidate is dirty, or a visit
    /// revision is stale. No I/O or mutation occurs.
    ///
    /// # Arguments
    /// - `self` `[in]`: Workspace and pending input identities.
    ///
    /// # Returns
    /// Some graph passing dirty/revision checks, or None before construction/while those inputs are
    /// stale. This getter does not separately test graph_invalidated; configuration callers must
    /// schedule and flush root refreshes.
    ///
    /// # Algorithm
    /// Reject a missing graph, a pending root, a dirty/stale visit or a dirty search candidate. The
    /// saved graph is borrowed only after those checks; graph_invalidated is a traversal-reuse flag
    /// rather than a separate getter freshness check.
    ///
    /// # Flow
    /// ```text
    /// [saved graph]
    ///        |
    ///  <root/visits current?> -- no --> [None]
    ///        | yes                       ^
    ///  <search candidates clean?> -- no --+
    ///        | yes
    ///  [Some borrowed graph]
    /// ```
    ///
    /// # Examples
    /// ```
    /// use cmake_analyzer::workspace::Workspace;
    /// let mut workspace = Workspace::new("/demo/CMakeLists.txt", None)?;
    /// assert!(workspace.root_graph().is_none());
    /// # Ok::<(), std::io::Error>(())
    /// ```
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
    /// Set the bare-module/package discovery gate and force traversal on the next requested
    /// rebuild. Clears name discovery; does not itself mark the root dirty or perform I/O.
    ///
    /// # Arguments
    /// - `self` `[in,out]`: Workspace; overwrites lookup flag and marks graph invalidated.
    /// - `enabled` `[in]`: False disables bare module/package search, not direct file includes.
    ///
    /// # Returns
    /// () after changing policy; call refresh_root and flush to guarantee an immediate graph
    /// refresh.
    ///
    /// # Examples
    /// ```
    /// use cmake_analyzer::workspace::Workspace;
    /// let mut workspace = Workspace::new("/demo/CMakeLists.txt", None)?;
    /// workspace.set_module_lookup_enabled(false);
    /// workspace.refresh_root();
    /// # Ok::<(), std::io::Error>(())
    /// ```
    pub fn set_module_lookup_enabled(&mut self, enabled: bool) {
        self.name_cache = Default::default();
        self.module_lookup_enabled = enabled;
        self.graph_invalidated = true;
    }
    /// Replace independent external include/package policies and configured package prefixes. Marks
    /// graph invalidated; does not itself queue root work or read files.
    ///
    /// # Arguments
    /// - `self` `[in,out]`: Workspace; overwrites policies and package search paths.
    /// - `include` `[in]`: Policy for includes outside canonical root boundary.
    /// - `find_package` `[in]`: Policy for external package entrypoints.
    /// - `package_search_paths` `[in]` : Replacement prefixes; relative paths use visit source
    ///   directories.
    ///
    /// # Returns
    /// () after policy replacement; call refresh_root and flush when changing an already analyzed
    /// workspace.
    ///
    /// # Examples
    /// ```
    /// use cmake_analyzer::workspace::Workspace;
    /// use cmake_analyzer::workspace::model::ExternalIndex;
    /// let mut workspace = Workspace::new("/demo/CMakeLists.txt", None)?;
    /// workspace.set_external_indexing(ExternalIndex::Functions, ExternalIndex::None, vec![]);
    /// workspace.refresh_root();
    /// # Ok::<(), std::io::Error>(())
    /// ```
    pub fn set_external_indexing(
        &mut self,
        include: super::model::ExternalIndex,
        find_package: super::model::ExternalIndex,
        package_search_paths: Vec<PathBuf>,
    ) {
        self.external_include = include;
        self.external_find_package = find_package;
        self.package_search_paths = package_search_paths;
        self.graph_invalidated = true;
    }
    /// Replace explicit module search paths, clear discovery and queue root traversal. Does not
    /// inspect directories.
    ///
    /// # Arguments
    /// - `self` `[in,out]`: Workspace; overwrites paths and invalidates/marks root dirty.
    /// - `paths` `[in]`: Replacement paths, joined to each visit's source directory during lookup.
    ///
    /// # Returns
    /// () after queuing refresh.
    ///
    /// # Examples
    /// ```
    /// use cmake_analyzer::workspace::Workspace;
    /// let mut workspace = Workspace::new("/demo/CMakeLists.txt", None)?;
    /// workspace.set_module_paths(vec!["cmake/Modules".into()]);
    /// # Ok::<(), std::io::Error>(())
    /// ```
    pub fn set_module_paths(&mut self, paths: Vec<PathBuf>) {
        self.name_cache = Default::default();
        self.module_paths = paths;
        self.refresh_root();
    }
    /// Replace the supplied builtin-module directory and queue root traversal. No executable
    /// discovery or directory read occurs.
    ///
    /// # Arguments
    /// - `self` `[in,out]` : Workspace; clears discovery, overwrites directory on success and
    ///   refreshes root.
    /// - `path` `[in]` : Optional builtin directory; None removes it, Some is normalized to
    ///   absolute.
    ///
    /// # Returns
    /// Ok(()) after refresh is queued; Err on relative-path normalization failure.
    ///
    /// # Errors
    /// Current-directory lookup for a relative directory can fail; discovery is already cleared.
    ///
    /// # Examples
    /// ```
    /// use cmake_analyzer::workspace::Workspace;
    /// let mut workspace = Workspace::new("/demo/CMakeLists.txt", None)?;
    /// workspace.set_builtin_module_directory(Some("/opt/cmake/Modules".into()))?;
    /// # Ok::<(), std::io::Error>(())
    /// ```
    pub fn set_builtin_module_directory(&mut self, path: Option<PathBuf>) -> io::Result<()> {
        self.name_cache = Default::default();
        self.builtin_module_directory = path.as_deref().map(absolute).transpose()?;
        self.refresh_root();
        Ok(())
    }
    /// Release workspace-owned parse data for inactive files but preserve stable IDs and readers'
    /// retained Arcs. No I/O.
    ///
    /// # Arguments
    /// - `self` `[in,out]` : Workspace; clears completion retention if no full package visit
    ///   remains, then drops unneeded snapshots and analysis caches.
    ///
    /// # Returns
    /// () after releasing nonoverlay, clean, nondependency, nonsupplemental parse data.
    ///
    /// # Algorithm
    /// Clear supplemental retention when no full-indexed package visit remains. For each document
    /// keep parse data if it is overlaid, dirty, a dependency/search input, or a retained
    /// completion source; otherwise drop only the workspace cache references, not the stable file
    /// identity.
    ///
    /// # Flow
    /// ```text
    /// [any full package visit?]
    ///         /              \
    ///       yes              no
    ///        |        [clear completion set]
    ///        +---------------+
    ///                |
    ///         [document loop]
    ///                |
    ///  <overlay/dirty/dependency/supplement?>
    ///             /             \
    ///           keep        drop parse caches
    ///             \             /
    ///              [IDs/readers retained]
    /// ```
    ///
    /// # Examples
    /// ```text
    /// workspace.release_unused_snapshots();
    /// ```
    pub(super) fn release_unused_snapshots(&mut self) {
        if !self.graph.as_ref().is_some_and(|graph| {
            graph
                .edges
                .iter()
                .filter(|edge| {
                    edge.kind == crate::parser::build_struct::model::DependencyKind::Package
                })
                .filter_map(|edge| edge.invocation)
                .any(|visit| graph.invocations[visit].index == super::model::ExternalIndex::Full)
        }) {
            self.completion_files.clear();
        }
        // Drop unused parse data, but keep file IDs. Readers may still hold the old results.
        for (path, document) in &mut self.documents {
            if document.overlay.is_none()
                && !self.dirty.contains(path)
                && !self.dependencies.contains(path)
                && !self.completion_files.contains(path)
            {
                document.snapshot = None;
                document.analysis = None;
            }
        }
    }
    /// Override filename-based project defaults for an explicitly selected -P entry.
    /// Set explicit -P source/binary working context and suppress filename-based project defaults.
    /// Changed context clears runtime and queues root refresh; no filesystem access.
    ///
    /// # Arguments
    /// - `self` `[in,out]`: Workspace; overwrites script working directory when changed.
    /// - `directory` `[in]`: Working directory normalized lexically, not checked for existence.
    ///
    /// # Returns
    /// Ok(()) after update or no change; Err on normalization failure.
    ///
    /// # Errors
    /// Current-directory lookup for relative input can fail.
    ///
    /// # Examples
    /// ```
    /// use cmake_analyzer::workspace::Workspace;
    /// let mut workspace = Workspace::new("/demo/CMakeLists.txt", None)?;
    /// workspace.set_script_working_directory("/demo")?;
    /// # Ok::<(), std::io::Error>(())
    /// ```
    pub fn set_script_working_directory(&mut self, directory: impl AsRef<Path>) -> io::Result<()> {
        let directory = absolute(directory.as_ref())?;
        if self.root.script_working_directory.as_ref() != Some(&directory) {
            self.root.script_working_directory = Some(directory);
            self.clear_runtime();
            self.refresh_root();
        }
        Ok(())
    }
    /// Borrow the selected entry and its configured directories. No side effects.
    ///
    /// # Arguments
    /// - `self` `[in]`: Workspace whose root is read.
    ///
    /// # Returns
    /// Reference to this workspace's single Root record.
    ///
    /// # Examples
    /// ```
    /// use cmake_analyzer::workspace::Workspace;
    /// let mut workspace = Workspace::new("/demo/CMakeLists.txt", None)?;
    /// let entry = workspace.root();
    /// assert!(entry.file.is_absolute());
    /// # Ok::<(), std::io::Error>(())
    /// ```
    pub fn root(&self) -> &Root {
        &self.root
    }
}
