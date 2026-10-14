// AI-generated implementation.
// review over
use super::model::{DocumentAnalysis, DocumentSnapshot, Workspace};
use crate::parser::build_struct::model::AnalysisKey;
use std::{io, path::Path, sync::Arc};

impl Workspace {
    /// create the document_snapshot(the parsed) ,or_else just return the existing one
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
        self.release_unused_snapshots();
        Ok(())
    }
    /// Analyze pending local documents without traversing a project dependency graph.
    /// Intended for standalone file Workspaces; project callers should use flush().
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
    pub async fn load_snapshot(
        &mut self,
        path: impl AsRef<Path>,
    ) -> io::Result<Arc<DocumentSnapshot>> {
        let path = super::absolute(path.as_ref())?;
        self.ensure_snapshot(&path).await
    }
    pub fn refresh_root(&mut self) {
        self.graph_invalidated = true;
        self.dirty.insert(self.root.file.clone());
    }
}
