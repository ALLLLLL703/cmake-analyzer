// AI-generated contextual builtin/path and incremental invalidation tests.

//! Check static directory values, per-invocation values, and script working-directory context.
//! Writes temporary source and analyzes overlays; no CMake project execution.

use super::Project;
/// Use Outer/Inner project directory builtins and literal DIR expansions to traverse five files, then overlay DIR to a missing path.
/// Writes/reads/removes temporary files and changes an overlay; never executes CMake.
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
/// Require five diagnostic-free visits, the outer PROJECT_SOURCE_DIR, graph rebuild with diagnostics after edit, and preserved old DIR values.
///
/// # Flow
/// ```text
/// [Outer directory values] --> [modules/one -> two]
///            |
///            v
/// [child Inner context] ------> [child/inner.cmake]
///            |
///            v
/// [require 5 visits / no diagnostics]
///            |
///            v
/// [overlay DIR to missing] ---> [graph rebuild / diagnostics]
///            |
///            v
/// [old DIR observation still points to modules]
/// ```
///
/// # Examples
/// Local test-module context, outside any running Tokio runtime:
/// the macro-generated test wrapper runs its own runtime and returns `()`.
/// Prefer the test harness; ignored measurements require explicit selection.
/// ```text
/// builtin_and_literal_values_resolve_includes_without_executing_cmake();
/// ```
#[tokio::test]
async fn builtin_and_literal_values_resolve_includes_without_executing_cmake() {
    let project = Project::new().await;
    project.write("CMakeLists.txt","cmake_minimum_required(VERSION 3.31)\nproject(Outer NONE)\nset(DIR \"${PROJECT_SOURCE_DIR}/modules\")\ninclude(\"${DIR}/one.cmake\")\nadd_subdirectory(child)\n").await;
    project.write("modules/one.cmake","cmake_minimum_required(VERSION 3.31)\ninclude(\"${CMAKE_CURRENT_LIST_DIR}/two.cmake\")\n").await;
    project
        .write(
            "modules/two.cmake",
            "cmake_minimum_required(VERSION 3.31)\nset(DONE yes)\n",
        )
        .await;
    project.write("child/CMakeLists.txt","cmake_minimum_required(VERSION 3.31)\nproject(Inner NONE)\ninclude(\"${PROJECT_SOURCE_DIR}/inner.cmake\")\n").await;
    project
        .write(
            "child/inner.cmake",
            "cmake_minimum_required(VERSION 3.31)\nset(CHILD yes)\n",
        )
        .await;
    let mut workspace = project.workspace();
    workspace.flush().await.unwrap();
    let graph = workspace.root_graph().unwrap();
    assert!(graph.diagnostics.is_empty(), "{:?}", graph.diagnostics);
    assert_eq!(graph.invocations.len(), 5);
    let source = graph
        .values
        .iter()
        .find(|value| value.name == "PROJECT_SOURCE_DIR")
        .unwrap()
        .possible_value
        .as_ref()
        .unwrap();
    assert_eq!(source.text(), project.0.to_str().unwrap());
    let before = graph.values.clone();
    workspace.set_overlay(project.0.join("CMakeLists.txt"),"cmake_minimum_required(VERSION 3.31)\nproject(Outer NONE)\nset(DIR \"${PROJECT_SOURCE_DIR}/missing\")\ninclude(\"${DIR}/one.cmake\")\nadd_subdirectory(child)\n").unwrap();
    workspace.flush().await.unwrap();
    assert!(workspace.graph_rebuilt);
    assert!(!workspace.root_graph().unwrap().diagnostics.is_empty());
    assert!(before.iter().any(|value| {
        value.name == "DIR"
            && value
                .possible_value
                .as_ref()
                .is_some_and(|value| value.text().ends_with("/modules"))
    }));
}
/// Include one read.cmake twice with REPEAT set to one and two.
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
/// Require two reads sharing a command ID but not invocation IDs, with distinct one/two values.
///
/// # Flow
/// ```text
/// [REPEAT=one]          [REPEAT=two]
///       |                     |
///       v                     v
/// [read.cmake visit 1]  [read.cmake visit 2]
///       |                     |
///       +----------+----------+
///                  v
///     [same command; different invocations]
///                  |
///                  v
///        [observed one / two respectively]
/// ```
///
/// # Examples
/// Local test-module context, outside any running Tokio runtime:
/// the macro-generated test wrapper runs its own runtime and returns `()`.
/// Prefer the test harness; ignored measurements require explicit selection.
/// ```text
/// repeated_static_visits_qualify_values_by_invocation_not_just_command();
/// ```
#[tokio::test]
async fn repeated_static_visits_qualify_values_by_invocation_not_just_command() {
    use cmake_analyzer::parser::semanticer::model::ReferenceKind;
    let p = Project::new().await;
    p.write(
        "CMakeLists.txt",
        "set(REPEAT one)\ninclude(read.cmake)\nset(REPEAT two)\ninclude(read.cmake)\n",
    )
    .await;
    p.write("read.cmake", "message(\"${REPEAT}\")\n").await;
    let mut workspace = p.workspace();
    workspace.flush().await.unwrap();
    let reads: Vec<_> = workspace
        .root_graph()
        .unwrap()
        .values
        .iter()
        .filter(|value| value.name == "REPEAT" && value.location.kind == ReferenceKind::Read)
        .collect();
    assert_eq!(reads.len(), 2);
    assert_eq!(reads[0].location.command, reads[1].location.command);
    assert_ne!(reads[0].invocation, reads[1].invocation);
    assert_eq!(reads[0].possible_value.as_ref().unwrap().text(), "one");
    assert_eq!(reads[1].possible_value.as_ref().unwrap().text(), "two");
}
/// Treat CMakeLists.txt as a script with an explicit working directory containing dep.cmake.
/// Writes/reads/removes temporary fixtures and sets static context; does not execute a script.
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
/// Require no project defaults, two visits, source/binary directories equal to the working directory, list directory equal to the file parent, and unknown PROJECT_SOURCE_DIR.
///
/// # Flow
/// ```text
/// [CMakeLists.txt]       [explicit working directory]
///         |                         |
///         v                         v
/// [list dir = file parent] [source/binary dirs = cwd]
///         |                         |
///         +-------------+-----------+
///                       v
///              [static script graph]
///                       |
///              +--------+-----------+
///              v                    v
///       [dep.cmake visit]    [no project defaults/value]
/// ```
///
/// # Examples
/// Local test-module context, outside any running Tokio runtime:
/// the macro-generated test wrapper runs its own runtime and returns `()`.
/// Prefer the test harness; ignored measurements require explicit selection.
/// ```text
/// explicit_script_context_overrides_filename_and_seeds_cwd_not_project_defaults();
/// ```
#[tokio::test]
async fn explicit_script_context_overrides_filename_and_seeds_cwd_not_project_defaults() {
    let p = Project::new().await;
    p.write("CMakeLists.txt","message(\"${CMAKE_SOURCE_DIR}\")\nmessage(\"${CMAKE_CURRENT_LIST_DIR}\")\nmessage(\"${CMAKE_BINARY_DIR}\")\nmessage(\"${PROJECT_SOURCE_DIR}\")\ninclude(\"${CMAKE_SOURCE_DIR}/dep.cmake\")\n").await;
    p.write("working/dep.cmake", "set(FROM_SCRIPT yes)\n").await;
    let mut workspace = p.workspace();
    let cwd = p.0.join("working");
    workspace.set_script_working_directory(&cwd).unwrap();
    workspace.flush().await.unwrap();
    let graph = workspace.root_graph().unwrap();
    assert!(!graph.project_defaults);
    assert_eq!(graph.invocations.len(), 2);
    for name in ["CMAKE_SOURCE_DIR", "CMAKE_BINARY_DIR"] {
        assert_eq!(
            graph
                .values
                .iter()
                .find(|value| value.name == name)
                .unwrap()
                .possible_value
                .as_ref()
                .unwrap()
                .text(),
            cwd.to_str().unwrap()
        );
    }
    assert_eq!(
        graph
            .values
            .iter()
            .find(|value| value.name == "CMAKE_CURRENT_LIST_DIR")
            .unwrap()
            .possible_value
            .as_ref()
            .unwrap()
            .text(),
        p.0.to_str().unwrap()
    );
    assert!(
        graph
            .values
            .iter()
            .find(|value| value.name == "PROJECT_SOURCE_DIR")
            .unwrap()
            .possible_value
            .is_none()
    );
}
