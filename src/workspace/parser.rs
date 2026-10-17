//! Load overlay/disk text and publish revision-qualified immutable snapshots on bounded workers.
//! Flush serially processes pending paths and refreshes/rebuilds dependency and reference indexes.
//! Only schema version discovery may query CMake; project code is never executed here.
// Contains AI-generated documentation additions.
// AI-generated implementation.
// review over
use super::model::{DocumentAnalysis, DocumentSnapshot, Workspace};
use crate::parser::build_struct::model::AnalysisKey;
use std::{io, path::Path, sync::Arc};

impl Workspace {
    /// create the document_snapshot(the parsed) ,or_else just return the existing one
    /// Load overlay or disk text and publish a current immutable result. Reads disk only without an
    /// overlay; read failure clears this file's parse caches. Parsing holds a worker permit.
    ///
    /// # Arguments
    /// - `self` `[in,out]` : Workspace; creates a document if needed and replaces analysis/snapshot
    ///   on success.
    /// - `path` `[in]`: Normalized file identity.
    ///
    /// # Returns
    /// Ok shared current snapshot, reused when text/revision match; Err on read, identity,
    /// worker/join or revision failure.
    ///
    /// # Errors
    /// Filesystem/UTF-8 read errors, exhausted IDs/revisions, closed semaphore, failed worker join
    /// or obsolete analysis. Previous incremental state may have been taken before a worker
    /// failure.
    ///
    /// # Algorithm
    /// Read text, reuse matching output, otherwise advance disk revision and move incremental state
    /// into a bounded blocking task. Run lexer/parser/semantic/build updates, potentially querying
    /// host CMake version. Reject obsolete results before publishing; never execute project code.
    ///
    /// # Flow
    /// ```text
    /// [overlay?] ---- yes ---> [editor text]
    ///     | no                     |
    ///  [async disk read] ----------+
    ///                              v
    ///                     <text/revision match?>
    ///                        /              \
    ///                    reuse         [bounded worker]
    ///                                      |
    ///                               <revision current?>
    ///                                 /           \
    ///                             publish        error
    /// ```
    ///
    /// # Examples
    /// ```text
    /// let snapshot = workspace.ensure_snapshot(&absolute_path).await?;
    /// ```
    pub(super) async fn ensure_snapshot(
        &mut self,
        path: &Path,
    ) -> io::Result<Arc<DocumentSnapshot>> {
        let overlay = self.document(path)?.overlay.clone();
        let text: Arc<str> = match &overlay {
            Some(text) => text.clone(),
            None => match tokio::fs::read_to_string(path).await {
                Ok(text) => text.into(),
                Err(error) => {
                    self.document(path)?.snapshot = None;
                    self.document(path)?.analysis = None;
                    return Err(error);
                }
            },
        };
        let document = self.document(path)?;
        if let Some(snapshot) = &document.snapshot {
            if snapshot.text == text && snapshot.build.analysis.revision == document.revision {
                return Ok(snapshot.clone());
            }
        }
        // Editor edits already bumped the version; newly read disk text needs a bump here.
        if overlay.is_none() {
            document.revision = document
                .revision
                .checked_add(1)
                .ok_or_else(|| io::Error::other("Revision space exhausted"))?;
        }
        let key = AnalysisKey {
            file: document.id,
            revision: document.revision,
        };
        let previous = document.analysis.take();
        let permit = self
            .workers
            .clone()
            .acquire_owned()
            .await
            .map_err(io::Error::other)?;
        let runtime = tokio::runtime::Handle::current();
        // Parse on a worker thread. Use the existing runtime to wait for cmake --version.
        let result = tokio::task::spawn_blocking(move || {
            let _permit = permit;
            let analysis = runtime.block_on(async {
                if let Some(mut analysis) = previous {
                    analysis.update(text.clone(), key).await;
                    analysis
                } else {
                    DocumentAnalysis::new(text.clone(), key).await
                }
            });
            let snapshot = Arc::new(DocumentSnapshot {
                lexed: analysis.lexer.buffer.clone(),
                text,
                semantics: analysis.semantics.buffer.clone(),
                build: analysis.build.buffer.clone(),
                work: analysis.work(),
            });
            (snapshot, analysis)
        })
        .await
        .map_err(io::Error::other)?;
        let document = self.document(path)?;
        if document.revision != key.revision {
            return Err(io::Error::other("Obsolete analysis result"));
        }
        document.analysis = Some(result.1);
        document.snapshot = Some(result.0.clone());
        Ok(result.0)
    }

    /// Parse pending changes; rebuild this project's graph only when its inputs changed.
    /// Consume pending edits, refresh file results and rebuild/reuse project visits when a root
    /// input changed. Reads dependencies but never configures or executes project code; may inspect
    /// host CMake version.
    ///
    /// # Arguments
    /// - `self` `[in,out]` : Workspace; drains dirty paths, replaces snapshots/graph/indexes and
    ///   resets last-flush work counters.
    ///
    /// # Returns
    /// Ok(()) after processing, even when dependency failures are retained as graph diagnostics;
    /// Err on overlay analysis or reference-worker failure.
    ///
    /// # Errors
    /// Overlay analysis errors requeue the drained paths. Reference-worker errors propagate; a
    /// failed join queues root refresh. Disk/dependency read failures usually become graph
    /// diagnostics rather than returned errors.
    ///
    /// # Algorithm
    /// Take the pending set and parse known files serially. Root/dependency changes attempt
    /// snapshot-only graph reuse, otherwise traverse and replace dependency tracking. Replay
    /// references/values/targets on a bounded worker, then release unused parse data. Nonoverlay
    /// read failures invalidate traversal; overlay failure restores the pending set.
    ///
    /// # Flow
    /// ```text
    /// [pending paths]
    ///        |
    ///  [parse known documents]
    ///        |
    ///  <root/dependency changed?> -- no --> [release unused]
    ///        | yes
    ///  <safe visit reuse?>
    ///      /           \
    ///  refresh       rebuild graph
    ///      \           /
    ///        [replay indexes]
    ///               |
    ///        [release unused]
    /// ```
    ///
    /// # Examples
    /// Call inside a Tokio runtime; sources must exist unless supplied as editor overlays.
    /// ```no_run
    /// use cmake_analyzer::workspace::Workspace;
    /// # async fn example() -> std::io::Result<()> {
    /// let mut workspace = Workspace::new("/demo/CMakeLists.txt", None)?;
    /// workspace.set_overlay("/demo/CMakeLists.txt", "cmake_minimum_required(VERSION 3.31)\nproject(Demo)")?;
    /// workspace.flush().await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn flush(&mut self) -> io::Result<()> {
        //human review over
        let dirty = std::mem::take(&mut self.dirty);
        let rebuild = dirty
            .iter()
            .any(|path| *path == self.root.file || self.dependencies.contains(path));
        self.graph_rebuilt = false;
        self.reused_invocations = 0;
        for path in &dirty {
            if self.documents.contains_key(path) {
                let overlay = self
                    .documents
                    .get(path)
                    .is_some_and(|doc| doc.overlay.is_some());
                if let Err(error) = self.ensure_snapshot(path).await {
                    if overlay {
                        self.dirty.extend(dirty.iter().cloned());
                        return Err(error);
                    }
                    self.graph_invalidated = true;
                }
            }
        }
        if rebuild && !self.refresh_graph_snapshots(&dirty) {
            self.graph_rebuilt = true;
            let root = self.root.clone();
            let graph = self.build_graph(&root).await;
            // Replace the set: a removed include must stop triggering a rebuild.
            self.dependencies.clear();
            self.dependencies
                .extend(graph.invocations.iter().map(|visit| visit.file.clone()));
            for edge in &graph.edges {
                // Creating a previously missing Foo.cmake must also trigger a rebuild.
                self.dependencies.extend(edge.candidates.iter().cloned());
                self.dependencies.extend(edge.resolved.iter().cloned());
            }
            self.graph = Some(graph);
            self.graph_invalidated = false;
        }
        if rebuild {
            self.refresh_references().await?;
        }
        self.release_unused_snapshots();
        Ok(())
    }
    /// Analyze pending local documents without traversing a project dependency graph.
    /// Intended for standalone file Workspaces; project callers should use flush().
    /// Analyze pending known files without following dependencies or rebuilding contextual indexes.
    /// Intended for standalone workspaces; may read disk and inspect host CMake version, never
    /// project execution.
    ///
    /// # Arguments
    /// - `self` `[in,out]` : Workspace; drains pending paths and replaces local snapshots. On
    ///   failure requeues the drained set.
    ///
    /// # Returns
    /// Ok(()) after local analysis; Err on the first snapshot failure.
    ///
    /// # Errors
    /// Propagates identity, read, revision and bounded-worker failures from local snapshot loading.
    ///
    /// # Examples
    /// Call inside a Tokio runtime; sources must exist unless supplied as editor overlays.
    /// ```no_run
    /// use cmake_analyzer::workspace::Workspace;
    /// # async fn example(workspace: &mut Workspace) -> std::io::Result<()> {
    /// workspace.flush_documents().await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn flush_documents(&mut self) -> io::Result<()> {
        let dirty = std::mem::take(&mut self.dirty);
        for path in &dirty {
            if self.documents.contains_key(path) {
                if let Err(error) = self.ensure_snapshot(path).await {
                    self.dirty.extend(dirty.iter().cloned());
                    return Err(error);
                }
            }
        }
        Ok(())
    }
    /// Load one overlay/disk snapshot without adding the file to the dependency graph.
    /// Load one overlay/disk result without adding a dependency visit. May parse on a bounded
    /// worker and inspect host CMake version; never executes project code.
    ///
    /// # Arguments
    /// - `self` `[in,out]`: Workspace; inserts/updates this document's caches but not graph edges.
    /// - `path` `[in]`: File identity normalized lexically before loading.
    ///
    /// # Returns
    /// Ok shared immutable snapshot; Err on normalization, read, revision or worker failure.
    ///
    /// # Errors
    /// Relative-path normalization, file/UTF-8 reads, exhausted IDs/revisions, worker
    /// acquisition/join or obsolete analysis.
    ///
    /// # Examples
    /// Call inside a Tokio runtime; sources must exist unless supplied as editor overlays.
    /// ```no_run
    /// use cmake_analyzer::workspace::Workspace;
    /// # async fn example(workspace: &mut Workspace) -> std::io::Result<()> {
    /// let snapshot = workspace.load_snapshot("/demo/helper.cmake").await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn load_snapshot(
        &mut self,
        path: impl AsRef<Path>,
    ) -> io::Result<Arc<DocumentSnapshot>> {
        let path = super::absolute(path.as_ref())?;
        self.ensure_snapshot(&path).await
    }
    /// Force the next flush to revisit root dependencies rather than only refresh snapshots. No I/O
    /// or runtime execution.
    ///
    /// # Arguments
    /// - `self` `[in,out]`: Workspace; sets graph_invalidated and inserts root into pending paths.
    ///
    /// # Returns
    /// () after queuing refresh.
    ///
    /// # Examples
    /// ```
    /// use cmake_analyzer::workspace::Workspace;
    /// let mut workspace = Workspace::new("/demo/CMakeLists.txt", None)?;
    /// workspace.refresh_root();
    /// # Ok::<(), std::io::Error>(())
    /// ```
    pub fn refresh_root(&mut self) {
        self.graph_invalidated = true;
        self.dirty.insert(self.root.file.clone());
    }
}
