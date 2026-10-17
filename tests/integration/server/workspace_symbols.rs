// AI-generated tests.
use super::session::{HEADER, Project, Session, uri};
use serde_json::{Value, json};
async fn symbols(client: &mut Session, id: i32) -> Value {
    client.send(json!({"jsonrpc":"2.0","id":id,"method":"workspace/symbol","params":{"query":"client-filters-this"}})).await;
    let response = client.response(id).await;
    assert!(response.get("error").is_none(), "{response}");
    response["result"].clone()
}
#[tokio::test]
async fn related_buffers_flatten_nested_symbols_deduplicate_owners_and_observe_edits() {
    let p = Project::new().await;
    let a = p
        .write(
            "a/CMakeLists.txt",
            "include(../shared.cmake)\nset(OLD yes)\n",
        )
        .await;
    p.write(
        "b/CMakeLists.txt",
        "include(../shared.cmake)\nset(SECOND yes)\n",
    )
    .await;
    let shared = p
        .write(
            "shared.cmake",
            "function(Shared ARG)\nset(INNER yes)\nendfunction()\n",
        )
        .await;
    let detached = p.write("detached.cmake", "set(DETACHED yes)\n").await;
    let folders = ["a", "b"]
        .map(|name| json!({"uri":uri(&p.0.join(name)),"name":name}))
        .to_vec();
    let mut client = Session::start(&p.0, folders, true).await;
    client
        .open(&a, "include(../shared.cmake)\nset(CURRENT yes)\n")
        .await;
    client.diagnostics(&a, 1).await;
    client.open(&detached, "set(DETACHED yes)\n").await;
    client.diagnostics(&detached, 1).await;
    let result = symbols(&mut client, 10).await;
    let items = result.as_array().unwrap();
    for name in ["CURRENT", "SECOND", "Shared", "INNER", "DETACHED"] {
        assert_eq!(
            items.iter().filter(|item| item["name"] == name).count(),
            1,
            "{name}: {result}"
        );
    }
    assert!(!items.iter().any(|item| item["name"] == "OLD"));
    let inner = items.iter().find(|item| item["name"] == "INNER").unwrap();
    assert_eq!(inner["containerName"], "Shared");
    assert_eq!(inner["location"]["uri"], uri(&shared));
    assert_eq!(inner["location"]["range"]["start"]["line"], 2);
    client.send(json!({"jsonrpc":"2.0","method":"textDocument/didChange","params":{"textDocument":{"uri":uri(&a),"version":2},"contentChanges":[{"text":format!("{HEADER}include(../shared.cmake)\nset(UPDATED yes)\n")}]}})).await;
    let changed = symbols(&mut client, 11).await;
    assert!(
        changed
            .as_array()
            .unwrap()
            .iter()
            .any(|item| item["name"] == "UPDATED")
    );
    assert!(
        !changed
            .as_array()
            .unwrap()
            .iter()
            .any(|item| item["name"] == "CURRENT")
    );
    client.finish().await;
    p.remove().await;
}
#[tokio::test]
async fn symbol_settings_apply_to_workspace_results() {
    let p = Project::new().await;
    p.write("CMakeLists.txt", "set(HIDDEN yes)\n").await;
    tokio::fs::write(
        p.0.join(".cmake-analyzer.toml"),
        "[document_symbols]\nenable=false\n",
    )
    .await
    .unwrap();
    let mut client = Session::start(&p.0, vec![], true).await;
    assert_eq!(symbols(&mut client, 10).await, json!([]));
    client.finish().await;
    p.remove().await;
}
