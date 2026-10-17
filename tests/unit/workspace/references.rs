// AI-generated Workspace reference-index regressions.

//! Check owner-qualified variable/function references and invocation-specific warnings.
//! Writes temporary fixtures and analyzes overlays; no project execution.

use super::Project;
use cmake_analyzer::parser::semanticer::model::{ReferenceKind, SemanticWarnKind};
use std::sync::Arc;

/// Analyze ROOT, an included Fn/MODULE definition, a child-local ROOT/CHILD_ONLY, and cross-file reads.
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
/// Require three visits, owner-valid ROOT spans/commands, cross-file function calls, unchanged local function references, and one CHILD_ONLY warning.
///
/// # Flow
/// ```text
/// [root ROOT / Fn call]
///        |            \
///        v             v
/// [included Fn/MODULE] [child-local ROOT / CHILD_ONLY]
///        |                     |
///        +----------+----------+
///                   v
///       [owner-qualified reference index]
///                   |                  \
///                   v                   v
///  [valid cross-file spans/calls] [one CHILD_ONLY warning]
///                   |
///                   v
///  [local function snapshot still definition-only]
/// ```
///
/// # Examples
/// Local test-module context, outside any running Tokio runtime:
/// the macro-generated test wrapper runs its own runtime and returns `()`.
/// Prefer the test harness; ignored measurements require explicit selection.
/// ```text
/// include_and_subdirectory_bind_external_references_without_mutating_snapshots();
/// ```
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

/// Set V in the root, shadow it with an empty child value, unset it, and read it again.
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
/// Require no warnings and a child read attached to the root definition.
///
/// # Flow
/// ```text
/// [root V=value] --> [child V=empty]
///        |                 |
///        |                 v
///        |            [child unset V]
///        |                 |
///        +---------------->v
///                 [child read binds root V]
///                          |
///                          v
///                   [no warnings]
/// ```
///
/// # Examples
/// Local test-module context, outside any running Tokio runtime:
/// the macro-generated test wrapper runs its own runtime and returns `()`.
/// Prefer the test harness; ignored measurements require explicit selection.
/// ```text
/// unsetting_a_child_binding_reveals_the_parent_binding();
/// ```
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

/// Read V from a twice-included module first with an empty value, then a nonempty value; overlay the first write to nonempty.
/// Writes/reads/removes temporary files and changes an overlay; no project execution.
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
/// Require one Empty warning at invocation 1 initially, no graph rebuild or new warnings after edit, an unchanged saved index, and shared module snapshot.
///
/// # Flow
/// ```text
/// [V empty / include]      [V value / include]
///           |                       |
///           v                       v
///  [Empty at visit 1]       [no Empty warning]
///           |                       |
///           +-----------+-----------+
///                       v
///           [overlay first V to value / flush]
///                       |
///                       v
///  [no new warnings; old index and module unchanged]
/// ```
///
/// # Examples
/// Local test-module context, outside any running Tokio runtime:
/// the macro-generated test wrapper runs its own runtime and returns `()`.
/// Prefer the test harness; ignored measurements require explicit selection.
/// ```text
/// repeated_include_warnings_use_each_invocation_and_refresh_after_local_edits();
/// ```
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
