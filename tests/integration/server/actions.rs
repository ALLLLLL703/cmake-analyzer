// AI-generated explicit configure/action integration tests.
use super::session::{Project, Session, uri};
use serde_json::{Value, json};
async fn actions(
    client: &mut Session,
    file: &std::path::Path,
    line: usize,
    id: i32,
    automatic: bool,
) -> Value {
    client.send(json!({"jsonrpc":"2.0","id":id,"method":"textDocument/codeAction","params":{"textDocument":{"uri":uri(file)},"range":{"start":{"line":line,"character":0},"end":{"line":line,"character":0}},"context":{"diagnostics":[],"triggerKind":if automatic{2}else{1}}}})).await;
    client.response(id).await
}
async fn execute(client: &mut Session, action: &Value, id: i32) -> Value {
    client.send(json!({"jsonrpc":"2.0","id":id,"method":"workspace/executeCommand","params":{"command":action["command"]["command"],"arguments":action["command"]["arguments"]}})).await;
    client.response(id).await
}
#[tokio::test]
async fn explicit_configure_discovers_modules_and_exposes_observed_values_then_edits_invalidate() {
    let p = Project::new().await;
    let text = "project(Action NONE)\nfile(WRITE \"${CMAKE_CURRENT_SOURCE_DIR}/configured\" yes)\nfunction(path output)\nset(${output} \"${CMAKE_CURRENT_SOURCE_DIR}/runtime\" PARENT_SCOPE)\nendfunction()\npath(DIR)\ninclude(\"${DIR}/new.cmake\")\nruntime_fn()\nmessage(\"${VALUE}\")\n";
    let root = p.write("CMakeLists.txt", text).await;
    let module = p
        .write(
            "runtime/new.cmake",
            "function(runtime_fn)\nendfunction()\nset(VALUE module)\n",
        )
        .await;
    let mut client = Session::start(&p.0, vec![], true).await;
    client.open(&root, text).await;
    let before = client.diagnostics(&root, 1).await;
    assert!(
        before["params"]["diagnostics"]
            .as_array()
            .unwrap()
            .iter()
            .any(|diagnostic| diagnostic["message"]
                .as_str()
                .unwrap()
                .contains("runtime_fn"))
    );
    assert!(!p.0.join("configured").exists());
    assert_eq!(
        actions(&mut client, &root, 7, 10, true).await["result"],
        json!([])
    );
    let listed = actions(&mut client, &root, 7, 11, false).await;
    assert_eq!(listed["result"].as_array().unwrap().len(), 1);
    assert!(!p.0.join("configured").exists());
    assert_eq!(
        execute(&mut client, &listed["result"][0], 12).await["result"]["started"],
        true
    );
    // Execution returned immediately; ordinary requests remain live.
    assert!(client.symbols(&root, 13).await.get("error").is_none());
    let completed = client
        .wait(|value| {
            value["method"] == "window/showMessage"
                && value["params"]["message"].as_str().is_some_and(|message| {
                    message.contains("completed") || message.contains("failed")
                })
        })
        .await;
    assert_eq!(completed["params"]["type"], 3, "{completed}");
    assert!(p.0.join("configured").exists());
    assert!(p.0.join(".cmake-analyzer/build/CMakeCache.txt").exists());
    let after = client.diagnostics(&root, 1).await;
    assert!(
        !after["params"]["diagnostics"]
            .as_array()
            .unwrap()
            .iter()
            .any(|diagnostic| diagnostic["message"]
                .as_str()
                .unwrap()
                .contains("CommandNotFound")
                && diagnostic["message"]
                    .as_str()
                    .unwrap()
                    .contains("runtime_fn"))
    );
    let value_line = text
        .lines()
        .position(|line| line.starts_with("message"))
        .unwrap()
        + 1;
    let listed = actions(&mut client, &root, value_line, 14, false).await;
    let expanded = listed["result"]
        .as_array()
        .unwrap()
        .iter()
        .find(|action| action["command"]["command"] == "cmake-analyzer.expandedArguments")
        .unwrap();
    let inspected = execute(&mut client, expanded, 15).await;
    let observed = &inspected["result"]["events"][0]["observed_values"];
    assert!(
        observed
            .as_array()
            .unwrap()
            .iter()
            .any(|value| value["name"] == "VALUE" && value["value"] == "module"),
        "{inspected}"
    );
    client
        .open(
            &module,
            "function(runtime_fn)\nendfunction()\nset(VALUE module)\n",
        )
        .await;
    let symbols = client.symbols(&module, 16).await;
    assert!(symbols.to_string().contains("runtime_fn"));
    client.send(json!({"jsonrpc":"2.0","method":"textDocument/didChange","params":{"textDocument":{"uri":uri(&root),"version":2},"contentChanges":[{"text":"cmake_minimum_required(VERSION 3.31)\nproject(Edited NONE)\n"}]}})).await;
    let listed = actions(&mut client, &root, 1, 17, false).await;
    assert!(
        !listed["result"]
            .as_array()
            .unwrap()
            .iter()
            .any(|action| action["command"]["command"] == "cmake-analyzer.expandedArguments")
    );
    let rejected = execute(&mut client, &listed["result"][0], 18).await;
    assert_eq!(rejected["error"]["code"], -32602);
    client.finish().await;
    p.remove().await;
}
#[cfg(unix)]
#[tokio::test]
async fn background_configure_can_be_cancelled_and_shutdown_joins_it() {
    use std::os::unix::fs::PermissionsExt;
    let p = Project::new().await;
    let root = p.write("CMakeLists.txt", "project(Cancel NONE)\n").await;
    let executable = p.0.join("slow-cmake");
    tokio::task::spawn_blocking(move|| {std::fs::write(&executable,"#!/bin/sh\n# Script-generated by tests/integration/server/actions.rs.\nexec sleep 60\n").unwrap();std::fs::set_permissions(executable,std::fs::Permissions::from_mode(0o700)).unwrap();}).await.unwrap();
    tokio::fs::write(
        p.0.join(".cmake-analyzer.toml"),
        "[runtime]\nexecutable='./slow-cmake'\n",
    )
    .await
    .unwrap();
    let mut client = Session::start(&p.0, vec![], true).await;
    client.open(&root, "project(Cancel NONE)\n").await;
    client.diagnostics(&root, 1).await;
    let listed = actions(&mut client, &root, 1, 10, false).await;
    assert_eq!(
        execute(&mut client, &listed["result"][0], 11).await["result"]["started"],
        true
    );
    assert!(client.symbols(&root, 12).await.get("error").is_none());
    let listed = actions(&mut client, &root, 1, 13, false).await;
    let cancel = listed["result"]
        .as_array()
        .unwrap()
        .iter()
        .find(|action| action["command"]["command"] == "cmake-analyzer.cancelConfigure")
        .unwrap();
    assert_eq!(execute(&mut client, cancel, 14).await["result"], true);
    tokio::time::timeout(std::time::Duration::from_secs(5), client.finish())
        .await
        .unwrap();
    p.remove().await;
}
