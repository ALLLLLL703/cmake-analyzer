// AI-generated contextual warning publication tests.
use super::session::{Project, Session, uri};
use serde_json::json;

#[tokio::test]
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
