// AI-generated contextual builtin/path and incremental invalidation tests.
use super::Project;
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
