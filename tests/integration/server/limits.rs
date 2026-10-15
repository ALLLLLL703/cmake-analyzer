// AI-generated configurable service-limit tests.
use super::session::{Project, Session, uri};
use serde_json::json;

async fn reload(client: &mut Session, id: i32) -> serde_json::Value {
    client.send(json!({"jsonrpc":"2.0","id":id,"method":"cmake-analyzer/reloadConfig","params":{"path":"limits.toml"}})).await;
    client.response(id).await
}

#[tokio::test]
async fn limits_apply_on_reload_and_reject_reductions_below_live_counts() {
    let project = Project::new().await;
    let other = Project::new().await;
    let root = project.write("CMakeLists.txt", "set(ROOT x)\n").await;
    let second = project.write("second.cmake", "set(SECOND x)\n").await;
    let foreign = other.write("CMakeLists.txt", "set(FOREIGN x)\n").await;
    let mut client = Session::start(&project.0, vec![], true).await;
    client.open(&root, "set(ROOT x)\n").await;
    client.diagnostics(&root, 1).await;
    client.open(&second, "set(SECOND x)\n").await;
    client.diagnostics(&second, 1).await;
    let path = project.0.join("limits.toml");
    tokio::fs::write(&path, "[limits]\nmax_open_documents=1\nmax_workspaces=1\n")
        .await
        .unwrap();
    assert_eq!(reload(&mut client, 10).await["error"]["code"], -32602);
    client.send(json!({"jsonrpc":"2.0","method":"textDocument/didClose","params":{"textDocument":{"uri":uri(&second)}}})).await;
    client
        .wait(|value| {
            value["method"] == "textDocument/publishDiagnostics"
                && value["params"]["uri"] == uri(&second)
                && value["params"]["version"].is_null()
        })
        .await;
    assert_eq!(reload(&mut client, 11).await["result"]["reloaded"], true);
    client.open(&second, "set(SECOND x)\n").await;
    // The rejected notification must not consume a document slot.
    assert_eq!(reload(&mut client, 12).await["result"]["reloaded"], true);
    assert_eq!(client.symbols(&foreign, 13).await["error"]["code"], -32000);
    tokio::fs::write(&path, "[limits]\nmax_open_documents=2\nmax_workspaces=2\n")
        .await
        .unwrap();
    assert_eq!(reload(&mut client, 14).await["result"]["reloaded"], true);
    client.open(&second, "set(SECOND x)\n").await;
    client.diagnostics(&second, 1).await;
    assert!(client.symbols(&foreign, 15).await["result"].is_array());
    tokio::fs::write(&path, "[limits]\nmax_open_documents=2\nmax_workspaces=1\n")
        .await
        .unwrap();
    assert_eq!(reload(&mut client, 16).await["error"]["code"], -32602);
    assert!(client.symbols(&foreign, 17).await["result"].is_array());
    client.finish().await;
    project.remove().await;
    other.remove().await;
}
