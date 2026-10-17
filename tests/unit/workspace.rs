// AI-generated tests.

//! Check Workspace snapshots, graph reuse, overlays, and dependency traversal.
//! [`Project`] writes version-pinned temporary fixtures and removes them on drop.
//! Child modules cover imports, references, targets, values, and an explicit real-CMake runtime test.

mod imports;
mod references;
mod runtime;
mod targets;
mod values;
use cmake_analyzer::workspace::model::{Workspace, WorkspaceLimits};
use std::{
    path::PathBuf,
    sync::{
        Arc,
        atomic::{AtomicU64, Ordering},
    },
};
/// Process-local suffix counter preventing temporary fixture directory collisions within this test process.
static NEXT: AtomicU64 = AtomicU64::new(0);

// AI-generated incremental regression tests.
/// Analyze a root with a literal write, include, and target, then edit a value and finally the write name.
/// Writes/removes temporary fixture files and updates overlays; no project execution.
///
/// # Arguments
/// None; the test constructs its own fixtures.
///
/// # Returns
/// `()` after all checks pass.
///
/// # Panics
/// If fixture setup or any documented assertion fails.
///
/// # Algorithm
/// Require one recomputed command per analysis layer, reused child snapshot/two visits, refreshed analysis keys, immutable old text, and graph replay only for the changed write inventory.
///
/// # Flow
/// ```text
/// [root + child] --> [initial snapshots / graph]
///                            |
///                            v
///                    [overlay A: one -> two]
///                            |
///                            v
///                 [require local reuse / new keys]
///                            |
///                            v
///                 [overlay write name A -> NEW]
///                            |
///                            v
///                 [require graph rebuild / NEW writes]
/// ```
///
/// # Examples
/// Local test-module context, outside any running Tokio runtime:
/// the macro-generated test wrapper runs its own runtime and returns `()`.
/// Prefer the test harness; ignored measurements require explicit selection.
/// ```text
/// local_edits_reuse_analysis_and_dependency_visits();
/// ```
#[tokio::test]
async fn local_edits_reuse_analysis_and_dependency_visits() {
    let p = Project::new().await;
    p.write(
        "CMakeLists.txt",
        "set(A one)\ninclude(child.cmake)\nadd_library(core STATIC a.cpp)",
    )
    .await;
    p.write("child.cmake", "set(CHILD value)").await;
    let mut workspace = p.workspace();
    workspace.flush().await.unwrap();
    let root = p.0.join("CMakeLists.txt");
    let child = p.0.join("child.cmake");
    let before = workspace.snapshot(&root).unwrap().unwrap();
    let child_before = workspace.snapshot(&child).unwrap().unwrap();
    workspace.set_overlay(&root,"cmake_minimum_required(VERSION 3.31)\nset(A two)\ninclude(child.cmake)\nadd_library(core STATIC a.cpp)").unwrap();
    workspace.flush().await.unwrap();
    let after = workspace.snapshot(&root).unwrap().unwrap();
    assert_eq!(after.work.parser.recomputed, 1);
    assert_eq!(after.work.semantics.recomputed, 1);
    assert_eq!(after.work.build.recomputed, 1);
    assert!(!workspace.graph_rebuilt);
    assert_eq!(workspace.reused_invocations, 2);
    assert!(Arc::ptr_eq(
        &child_before,
        &workspace.snapshot(&child).unwrap().unwrap()
    ));
    assert!(before.text.contains("one"));
    assert!(after.text.contains("two"));
    assert_eq!(
        workspace.root_graph().unwrap().invocations[0].analysis,
        after.build.analysis
    );
    assert_eq!(
        workspace.root_graph().unwrap().edges[0].source.analysis,
        after.build.analysis
    );
    // Changing the declared write name changes inherited graph state and must replay it.
    workspace.set_overlay(&root,"cmake_minimum_required(VERSION 3.31)\nset(NEW two)\ninclude(child.cmake)\nadd_library(core STATIC a.cpp)").unwrap();
    workspace.flush().await.unwrap();
    assert!(workspace.graph_rebuilt);
    let graph = workspace.root_graph().unwrap();
    assert!(graph.scopes[0].possible_writes.contains("NEW"));
    assert!(!graph.scopes[0].possible_writes.contains("A"));
}

/// Temporary fixture root shared by Workspace regression tests and their child modules.
/// [`Self::write`] pins CMake 3.31 for static analysis; [`Self::workspace`] builds an unloaded
/// [`Workspace`]. Creation writes a directory; Drop recursively removes it, ignoring removal errors.
/// The runtime child test explicitly configures only its own trusted compiler-free fixture.
///
/// # Flow
/// ```text
/// [Project::new] --> [temporary directory]
///                          |          |
///                          v          v
///                  [write fixtures] [workspace / flush]
///                          |          |
///                          +-----+----+
///                                v
///                       [Drop: remove tree]
/// ```
///
/// # Examples
/// Local asynchronous test context:
/// ```text
/// let project = Project::new().await;
/// project.write("CMakeLists.txt", "set(A one)").await;
/// let mut workspace = project.workspace();
/// workspace.flush().await.unwrap();
/// ```
struct Project(
    /// Temporary project directory, named with process ID and a process-local counter.
    /// Example: `project.0.join("CMakeLists.txt")` locates the root fixture file.
    PathBuf,
);
impl Project {
    /// Create a process/counter-qualified directory beneath the system temporary directory.
    /// Mutates the filesystem and increments NEXT; never runs project code.
    ///
    /// # Arguments
    /// None.
    ///
    /// # Returns
    /// Project owning the directory path; Drop attempts recursive cleanup.
    ///
    /// # Panics
    /// If creating the temporary directory fails.
    ///
    /// # Examples
    /// Local test-module context:
    /// ```text
    /// let project = Project::new().await;
    /// ```
    async fn new() -> Self {
        let path = std::env::temp_dir().join(format!(
            "cmake-analyzer-tests-{}-{}",
            std::process::id(),
            NEXT.fetch_add(1, Ordering::Relaxed)
        ));
        tokio::fs::create_dir_all(&path).await.unwrap();
        Self(path)
    }
    /// Create parent directories and overwrite a fixture file with a CMake 3.31 preamble plus body.
    /// Writes filesystem content only; no project execution.
    ///
    /// # Arguments
    /// - `self` `[in]`: project path, unchanged; its directory contents are mutated.
    /// - `name` `[in]`: path joined to the project directory; callers use trusted relative names.
    /// - `body` `[in]`: fixture text appended after the version preamble; the file is overwritten, not appended.
    ///
    /// # Returns
    /// `()` after the complete file is written.
    ///
    /// # Panics
    /// If the joined path has no parent or creating/writing directories/files fails.
    ///
    /// # Examples
    /// Local test-module context:
    /// ```text
    /// project.write("child/CMakeLists.txt", "set(V one)").await;
    /// ```
    async fn write(&self, name: &str, body: &str) {
        let path = self.0.join(name);
        tokio::fs::create_dir_all(path.parent().unwrap())
            .await
            .unwrap();
        tokio::fs::write(
            path,
            format!("cmake_minimum_required(VERSION 3.31)\n{body}"),
        )
        .await
        .unwrap();
    }
    /// Create an unloaded Workspace rooted at this project's CMakeLists.txt.
    /// Does not read fixture text or configure a project; later flush reads and analyzes it.
    ///
    /// # Arguments
    /// - `self` `[in]`: project directory supplying the root path, unchanged.
    ///
    /// # Returns
    /// Workspace with no explicit binary directory and default limits.
    ///
    /// # Panics
    /// If Workspace::new cannot normalize the root or initialize its state.
    ///
    /// # Examples
    /// Local test-module context:
    /// ```text
    /// let mut workspace = project.workspace();
    /// ```
    fn workspace(&self) -> Workspace {
        Workspace::new(self.0.join("CMakeLists.txt"), None).unwrap()
    }
}
impl Drop for Project {
    /// Attempt to remove the entire temporary project directory, including any generated build tree.
    /// Removal errors are deliberately ignored; no subprocess is started.
    ///
    /// # Arguments
    /// - `self` `[in,out]`: retiring fixture; stored path is unchanged while its directory is removed.
    ///
    /// # Returns
    /// `()` whether removal succeeds or fails.
    ///
    /// # Examples
    /// Local test-module context:
    /// ```text
    /// drop(project);
    /// ```
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

/// Analyze project(Disk), replace it with an Editor overlay, repeat the same overlay, then close it.
/// Writes/removes a temporary root and changes in-memory overlays; no project execution.
///
/// # Arguments
/// None; the test constructs its own fixtures.
///
/// # Returns
/// `()` after all checks pass.
///
/// # Panics
/// If fixture setup or any documented assertion fails.
///
/// # Algorithm
/// Require pending results to disappear, revision to increase, unchanged-overlay snapshot identity, and restored disk text.
///
/// # Flow
/// ```text
/// [project(Disk)] --> [save original snapshot]
///                            |
///                            v
/// [Editor overlay] --> [pending results hidden]
///                            |
///                            v
///                     [flush: newer revision]
///                            |
/// [repeat same text] --> [same snapshot identity]
///                            |
///                            v
/// [close overlay] ----> [flush: Disk text restored]
/// ```
///
/// # Examples
/// Local test-module context, outside any running Tokio runtime:
/// the macro-generated test wrapper runs its own runtime and returns `()`.
/// Prefer the test harness; ignored measurements require explicit selection.
/// ```text
/// overlay_revisions_hide_stale_results_and_close_restores_disk();
/// ```
#[tokio::test]
async fn overlay_revisions_hide_stale_results_and_close_restores_disk() {
    let p = Project::new().await;
    p.write("CMakeLists.txt", "project(Disk)").await;
    let root = p.0.join("CMakeLists.txt");
    let mut w = p.workspace();
    w.flush().await.unwrap();
    let original = w.snapshot(&root).unwrap().unwrap();
    let overlay = "cmake_minimum_required(VERSION 3.31)\nproject(Editor)";
    w.set_overlay(&root, overlay).unwrap();
    assert!(w.root_graph().is_none());
    assert!(w.snapshot(&root).unwrap().is_none());
    w.flush().await.unwrap();
    let edited = w.snapshot(&root).unwrap().unwrap();
    assert_eq!(&*edited.text, overlay);
    assert!(edited.build.analysis.revision > original.build.analysis.revision);
    w.set_overlay(&root, overlay).unwrap();
    w.flush().await.unwrap();
    assert!(Arc::ptr_eq(&edited, &w.snapshot(&root).unwrap().unwrap()));
    w.close_overlay(&root).unwrap();
    w.flush().await.unwrap();
    assert!(
        w.snapshot(&root)
            .unwrap()
            .unwrap()
            .text
            .contains("project(Disk)")
    );
}

/// Include the same child twice from a temporary root.
/// Writes/reads/removes temporary fixtures; no project execution.
///
/// # Arguments
/// None; the test constructs its own fixtures.
///
/// # Returns
/// `()` after all checks pass.
///
/// # Panics
/// If fixture setup or any documented assertion fails.
///
/// # Algorithm
/// Require three visits, two edges with distinct invocation IDs, a shared child snapshot, and shared include scope.
///
/// # Flow
/// ```text
/// [root includes child twice]
///             |
///             v
///        [flush graph]
///             |
///         +---+----------------+
///         v                    v
/// [3 visits / 2 edges]   [same child snapshot]
///         |                    |
///         v                    v
/// [distinct visit IDs]   [shared include scope]
/// ```
///
/// # Examples
/// Local test-module context, outside any running Tokio runtime:
/// the macro-generated test wrapper runs its own runtime and returns `()`.
/// Prefer the test harness; ignored measurements require explicit selection.
/// ```text
/// repeated_includes_share_snapshots_but_have_separate_visits();
/// ```
#[tokio::test]
async fn repeated_includes_share_snapshots_but_have_separate_visits() {
    let p = Project::new().await;
    p.write(
        "CMakeLists.txt",
        "include(child.cmake)\ninclude(child.cmake)",
    )
    .await;
    p.write("child.cmake", "set(CHILD value)").await;
    let mut w = p.workspace();
    w.flush().await.unwrap();
    let graph = w.root_graph().unwrap();
    assert_eq!(graph.invocations.len(), 3);
    assert_eq!(graph.edges.len(), 2);
    assert_ne!(graph.edges[0].invocation, graph.edges[1].invocation);
    assert!(Arc::ptr_eq(
        &graph.invocations[1].snapshot,
        &graph.invocations[2].snapshot
    ));
    assert_eq!(graph.invocations[0].scope, graph.invocations[1].scope);
}

/// Analyze an optional absent include, create it, and invalidate its disk candidate.
/// Creates/reads/removes temporary files and mutates Workspace caches; no project execution.
///
/// # Arguments
/// None; the test constructs its own fixtures.
///
/// # Returns
/// `()` after all checks pass.
///
/// # Panics
/// If fixture setup or any documented assertion fails.
///
/// # Algorithm
/// Require the edge initially to have no invocation, pending graph to be hidden, and a visit after flush.
///
/// # Flow
/// ```text
/// [optional absent later.cmake] --> [edge without visit]
///               |
///               v
///      [write later.cmake]
///               |
///               v
///      [invalidate candidate] ----> [pending graph hidden]
///               |
///               v
///            [flush] -------------> [edge has visit]
/// ```
///
/// # Examples
/// Local test-module context, outside any running Tokio runtime:
/// the macro-generated test wrapper runs its own runtime and returns `()`.
/// Prefer the test harness; ignored measurements require explicit selection.
/// ```text
/// missing_include_candidates_are_invalidated_when_file_appears();
/// ```
#[tokio::test]
async fn missing_include_candidates_are_invalidated_when_file_appears() {
    let p = Project::new().await;
    p.write("CMakeLists.txt", "include(later.cmake OPTIONAL)")
        .await;
    let mut w = p.workspace();
    w.flush().await.unwrap();
    assert!(w.root_graph().unwrap().edges[0].invocation.is_none());
    p.write("later.cmake", "set(FOUND yes)").await;
    w.invalidate_disk(p.0.join("later.cmake")).unwrap();
    assert!(w.root_graph().is_none());
    w.flush().await.unwrap();
    assert!(w.root_graph().unwrap().edges[0].invocation.is_some());
}

/// Create a root-child include cycle and analyze it normally and with a one-invocation budget.
/// Writes/reads/removes temporary files; no project execution.
///
/// # Arguments
/// None; the test constructs its own fixtures.
///
/// # Returns
/// `()` after all checks pass.
///
/// # Panics
/// If fixture setup or any documented assertion fails.
///
/// # Algorithm
/// Require diagnostics with two visits for the cycle and diagnostics with one visit for the constrained traversal.
///
/// # Flow
/// ```text
/// [root <--> child include cycle]
///                |
///          +-----+----------------+
///          v                      v
///  [default limits]       [invocations limit = 1]
///          |                      |
///          v                      v
/// [2 visits + diagnostic] [1 visit + diagnostic]
/// ```
///
/// # Examples
/// Local test-module context, outside any running Tokio runtime:
/// the macro-generated test wrapper runs its own runtime and returns `()`.
/// Prefer the test harness; ignored measurements require explicit selection.
/// ```text
/// cycles_and_visit_limits_terminate_with_diagnostics();
/// ```
#[tokio::test]
async fn cycles_and_visit_limits_terminate_with_diagnostics() {
    let p = Project::new().await;
    p.write("CMakeLists.txt", "include(child.cmake)").await;
    p.write("child.cmake", "include(CMakeLists.txt)").await;
    let mut w = p.workspace();
    w.flush().await.unwrap();
    assert!(!w.root_graph().unwrap().diagnostics.is_empty());
    assert_eq!(w.root_graph().unwrap().invocations.len(), 2);
    let mut limited = Workspace::with_limits(
        p.0.join("CMakeLists.txt"),
        None,
        WorkspaceLimits {
            invocations: 1,
            ..Default::default()
        },
    )
    .unwrap();
    limited.flush().await.unwrap();
    assert_eq!(limited.root_graph().unwrap().invocations.len(), 1);
    assert!(!limited.root_graph().unwrap().diagnostics.is_empty());
}
