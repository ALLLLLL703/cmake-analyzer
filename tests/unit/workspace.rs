// AI-generated tests.
use cmake_analyzer::workspace::model::{Workspace, WorkspaceLimits};
use std::{
    path::PathBuf,
    sync::{
        Arc,
        atomic::{AtomicU64, Ordering},
    },
};
static NEXT: AtomicU64 = AtomicU64::new(0);
struct Project(PathBuf);
impl Project {
    async fn new() -> Self {
        let path = std::env::temp_dir().join(format!(
            "cmake-analyzer-tests-{}-{}",
            std::process::id(),
            NEXT.fetch_add(1, Ordering::Relaxed)
        ));
        tokio::fs::create_dir_all(&path).await.unwrap();
        Self(path)
    }
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
    fn workspace(&self) -> Workspace {
        Workspace::new(self.0.join("CMakeLists.txt"), None).unwrap()
    }
}
impl Drop for Project {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

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
