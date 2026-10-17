// AI-generated tests.
//! Flat workspace symbols across owners, dependencies and standalone overlays.
//! Queries deliberately leave filtering to the client; disabled symbol settings omit results.
use super::session::{HEADER, Project, Session, uri};
use serde_json::{Value, json};
/// Request workspace symbols with a deliberately nonmatching query and require success.
/// Tests expect server-side results to remain unfiltered for client-side filtering.
///
/// # Arguments
/// - `client` `[in,out]`: writes the request and consumes its reply; unrelated messages append
///   to pending.
/// - `id` `[in]`: request/response correlation ID.
///
/// # Returns
/// An owned clone of result, normally the flat symbol array.
///
/// # Panics
/// Panics on transport/deadline failure or a response error.
///
/// # Examples
/// ```text
/// let items = symbols(&mut client, 10).await;
/// ```
async fn symbols(client: &mut Session, id: i32) -> Value {
    client.send(json!({"jsonrpc":"2.0","id":id,"method":"workspace/symbol","params":{"query":"client-filters-this"}})).await;
    let response = client.response(id).await;
    assert!(response.get("error").is_none(), "{response}");
    response["result"].clone()
}
#[tokio::test]
/// Check workspace symbols flatten nested scopes, deduplicate owners and read pending overlays.
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
/// Two roots share a function module and a standalone file is open. Require
/// CURRENT/SECOND/Shared/INNER/DETACHED exactly once, no disk OLD, and INNER
/// container/URI/line. Replace CURRENT with UPDATED and immediately require UPDATED with
/// CURRENT absent.
///
/// # Flow
/// ```text
/// [Temporary sources + settings]      [Expected: flat unique current symbols]
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
/// related_buffers_flatten_nested_symbols_deduplicate_owners_and_observe_edits(); // #[tokio::test] supplies the runtime
/// ```
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
/// Check document_symbols.enable=false excludes that workspace from workspace/symbol.
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
/// Write HIDDEN and disable document symbols before startup. Send a nonempty workspace query
/// and require an empty array; do not open the document or execute source.
///
/// # Flow
/// ```text
/// [Temporary sources + settings]      [Expected: empty workspace symbols]
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
/// symbol_settings_apply_to_workspace_results(); // #[tokio::test] supplies the runtime
/// ```
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
