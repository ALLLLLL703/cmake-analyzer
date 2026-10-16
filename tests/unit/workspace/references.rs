// AI-generated Workspace reference-index regressions.
use super::Project;
use cmake_analyzer::parser::semanticer::model::{ReferenceKind, SemanticWarnKind};
use std::sync::Arc;

#[tokio::test]
async fn include_and_subdirectory_bind_external_references_without_mutating_snapshots() {
    let project = Project::new().await;
    project.write("CMakeLists.txt","set(ROOT value)\ninclude(module.cmake)\nfn(${ROOT})\nadd_subdirectory(child)\nmessage(\"${MODULE} ${CHILD_ONLY}\")\n").await;
    project.write("module.cmake","function(Fn P)\nmessage(\"${P} ${ROOT}\")\nendfunction()\nset(MODULE value)\nmessage(\"${ROOT}\")\n").await;
    project
        .write(
            "child/CMakeLists.txt",
            "message(\"${ROOT} ${MODULE}\")\nset(ROOT local)\nset(CHILD_ONLY local)\n",
        )
        .await;
    let mut workspace = project.workspace();
    workspace.flush().await.unwrap();
    let graph = workspace.root_graph().unwrap();
    assert_eq!(graph.invocations.len(), 3);
    let root = workspace
        .snapshot(project.0.join("CMakeLists.txt"))
        .unwrap()
        .unwrap();
    let module = workspace
        .snapshot(project.0.join("module.cmake"))
        .unwrap()
        .unwrap();
    let variable = graph
        .references
        .variables
        .iter()
        .find(|variable| {
            variable.name == "ROOT"
                && variable
                    .definition
                    .as_ref()
                    .is_some_and(|definition| definition.file == root.semantics.id)
        })
        .unwrap();
    assert!(
        variable
            .references
            .iter()
            .any(|reference| reference.file == module.semantics.id
                && reference.kind == ReferenceKind::Read)
    );
    for reference in &variable.references {
        let visit = graph
            .invocations
            .iter()
            .find(|visit| visit.snapshot.semantics.id == reference.file)
            .unwrap();
        assert!(
            visit
                .snapshot
                .semantics
                .commands
                .get(reference.command.id as usize)
                .is_some()
        );
        assert_eq!(reference.span.text(&visit.snapshot.text), Some("ROOT"));
    }
    let function = graph
        .references
        .functions
        .iter()
        .find(|function| function.name == "Fn")
        .unwrap();
    assert_eq!(
        function.definition.as_ref().unwrap().file,
        module.semantics.id
    );
    assert!(
        function
            .references
            .iter()
            .any(|reference| reference.file == root.semantics.id
                && reference.kind == ReferenceKind::Call)
    );
    assert_eq!(
        module
            .semantics
            .functions
            .iter()
            .find(|function| function.name == "Fn")
            .unwrap()
            .references
            .len(),
        1
    );
    assert_eq!(
        graph.references.warnings.len(),
        1,
        "{:?}",
        graph.references.warnings
    );
    assert_eq!(
        graph.references.warnings[0].warning.content.variable,
        "CHILD_ONLY"
    );
}

#[tokio::test]
async fn unsetting_a_child_binding_reveals_the_parent_binding() {
    let project = Project::new().await;
    project
        .write(
            "CMakeLists.txt",
            "set(V value)\nadd_subdirectory(child)\nmessage(\"${V}\")\n",
        )
        .await;
    project
        .write(
            "child/CMakeLists.txt",
            "set(V \"\")\nunset(V)\nmessage(\"${V}\")\n",
        )
        .await;
    let mut workspace = project.workspace();
    workspace.flush().await.unwrap();
    let graph = workspace.root_graph().unwrap();
    assert!(
        graph.references.warnings.is_empty(),
        "{:?}",
        graph.references.warnings
    );
    let root = graph.invocations[0].snapshot.semantics.id;
    let variable = graph
        .references
        .variables
        .iter()
        .find(|variable| {
            variable.name == "V"
                && variable
                    .definition
                    .as_ref()
                    .is_some_and(|definition| definition.file == root)
        })
        .unwrap();
    assert!(
        variable
            .references
            .iter()
            .any(|reference| reference.file != root && reference.kind == ReferenceKind::Read)
    );
}

#[tokio::test]
async fn repeated_include_warnings_use_each_invocation_and_refresh_after_local_edits() {
    let project = Project::new().await;
    project
        .write(
            "CMakeLists.txt",
            "set(V \"\")\ninclude(read.cmake)\nset(V value)\ninclude(read.cmake)\n",
        )
        .await;
    project.write("read.cmake", "message(\"${V}\")\n").await;
    let mut workspace = project.workspace();
    workspace.flush().await.unwrap();
    let module = workspace
        .snapshot(project.0.join("read.cmake"))
        .unwrap()
        .unwrap();
    let graph = workspace.root_graph().unwrap();
    assert_eq!(graph.references.warnings.len(), 1);
    assert_eq!(
        graph.references.warnings[0].warning.content.kind,
        SemanticWarnKind::Empty
    );
    assert_eq!(graph.references.warnings[0].invocation.index(), 1);
    let before = graph.references.clone();
    workspace.set_overlay(project.0.join("CMakeLists.txt"),"cmake_minimum_required(VERSION 3.31)\nset(V value)\ninclude(read.cmake)\nset(V value)\ninclude(read.cmake)\n").unwrap();
    workspace.flush().await.unwrap();
    assert!(!workspace.graph_rebuilt);
    assert!(
        workspace
            .root_graph()
            .unwrap()
            .references
            .warnings
            .is_empty()
    );
    assert_eq!(before.warnings.len(), 1);
    assert!(Arc::ptr_eq(
        &module,
        &workspace
            .snapshot(project.0.join("read.cmake"))
            .unwrap()
            .unwrap()
    ));
}
