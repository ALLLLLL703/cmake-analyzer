// AI-generated contextual warning publication tests.
//! Context-dependent warning publications, not references-request navigation.
//! A root edit changes an imported file report without changing that file version.
use super::session::{Project, Session, uri};
use serde_json::json;

#[tokio::test]
/// Check imported bindings suppress unknowns and root value changes republish unchanged module
/// versions.
/// Creates temporary files and an analyzer session; successful completion stops the session and
/// removes fixtures.
///
/// # Arguments
/// None.
///
/// # Returns
/// `()` if every assertion passes; no output parameter.
///
/// # Panics
/// Panics on any failed assertion or unwrapped fixture/transport/process failure.
/// Cleanup is explicit on the success path; a panic may leave a fixture tree.
///
/// # Algorithm
/// Open a root defining V and calling imported external_fn, plus a module reading V: both
/// reports are empty. Change only root V to empty and require exactly one severity-2 is empty
/// warning on module line 1, still at module version 1.
///
/// # Flow
/// ```text
/// [Temporary sources + settings]      [Expected: one contextual module warning]
///              |                              |
///              v                              |
/// [LSP scenario messages]                 |
///              |                              |
///              +-------------+----------------+
///                            v
///                  [Compare assertions]
///                            |
///                            v
///             [Finish session + remove tree]
/// ```
///
/// # Examples
/// ```text
/// external_definitions_suppress_local_unknowns_and_context_changes_republish_warnings(); // #[tokio::test] supplies the runtime
/// ```
async fn external_definitions_suppress_local_unknowns_and_context_changes_republish_warnings() {
    let project = Project::new().await;
    let root = project
        .write(
            "CMakeLists.txt",
            "set(V value)\ninclude(read.cmake)\nexternal_fn()\n",
        )
        .await;
    let module = project
        .write(
            "read.cmake",
            "message(\"${V}\")\nfunction(external_fn)\nendfunction()\n",
        )
        .await;
    let mut client = Session::start(&project.0, vec![], true).await;
    client
        .open(&root, "set(V value)\ninclude(read.cmake)\nexternal_fn()\n")
        .await;
    assert_eq!(
        client.diagnostics(&root, 1).await["params"]["diagnostics"],
        json!([])
    );
    client
        .open(
            &module,
            "message(\"${V}\")\nfunction(external_fn)\nendfunction()\n",
        )
        .await;
    assert_eq!(
        client.diagnostics(&module, 1).await["params"]["diagnostics"],
        json!([])
    );
    client.send(json!({"jsonrpc":"2.0","method":"textDocument/didChange","params":{"textDocument":{"uri":uri(&root),"version":2},"contentChanges":[{"text":"cmake_minimum_required(VERSION 3.31)\nset(V \"\")\ninclude(read.cmake)\nexternal_fn()\n"}]}})).await;
    let report = client.diagnostics(&module, 1).await;
    let diagnostics = report["params"]["diagnostics"].as_array().unwrap();
    assert_eq!(diagnostics.len(), 1, "{report}");
    assert_eq!(diagnostics[0]["severity"], 2);
    assert!(
        diagnostics[0]["message"]
            .as_str()
            .unwrap()
            .contains("is empty")
    );
    assert_eq!(diagnostics[0]["range"]["start"]["line"], 1);
    client.finish().await;
    project.remove().await;
}
