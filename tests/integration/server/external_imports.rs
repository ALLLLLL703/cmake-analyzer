// AI-generated tests.
//! External include indexing modes across diagnostics, symbols and definitions.
//! Configuration reloads must change analysis without executing the marker-writing source.
use super::{
    navigation::definition,
    session::{Project, Session, uri},
};
use serde_json::{Value, json};
/// Request all workspace symbols using an empty query and require success.
///
/// # Arguments
/// - `client` `[in,out]`: writes the request and consumes its reply; unrelated messages append
///   to pending.
/// - `id` `[in]`: request/response correlation ID.
///
/// # Returns
/// An owned clone of the result array; the response envelope is consumed.
///
/// # Panics
/// Panics on transport/deadline failure or a response error.
///
/// # Examples
/// ```text
/// let items = workspace_symbols(&mut client, 11).await;
/// ```
async fn workspace_symbols(client: &mut Session, id: i32) -> Value {
    client
        .send(json!({"jsonrpc":"2.0","id":id,"method":"workspace/symbol","params":{"query":""}}))
        .await;
    let result = client.response(id).await;
    assert!(result.get("error").is_none(), "{result}");
    result["result"].clone()
}
#[tokio::test]
/// Check full/functions/none external indexing changes semantic features without source
/// execution.
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
/// Cycle external_include modes with reload. Full/functions suppress the ext_call
/// unknown-command diagnostic and retain its definition/symbol; none restores the diagnostic
/// and removes those results. EXTERNAL_VALUE and ExternalTarget symbols survive only in full;
/// EXTERNAL_VALUE diagnostics appear in functions/none. Include-path definition always reaches
/// the external module. Require no marker in app or modules.
///
/// # Flow
/// ```text
/// [Temporary sources + settings]      [Expected: mode-specific semantic results]
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
/// reload_changes_external_symbols_diagnostics_and_navigation_without_running_sources(); // #[tokio::test] supplies the runtime
/// ```
async fn reload_changes_external_symbols_diagnostics_and_navigation_without_running_sources() {
    let p = Project::new().await;
    let body = "include(External)\next_call()\nmessage(\"${EXTERNAL_VALUE}\")\n";
    let root = p.write("app/CMakeLists.txt", body).await;
    let external=p.write("modules/External.cmake","set(EXTERNAL_VALUE yes)\nadd_library(ExternalTarget INTERFACE)\nfunction(ext_call)\nendfunction()\nfile(WRITE marker.txt should-not-run)\n").await;
    let config = p.0.join("app/.cmake-analyzer.toml");
    tokio::fs::write(
        &config,
        "[modules]\nsearch_paths=[\"../modules\"]\npackage_search_paths=[]\n",
    )
    .await
    .unwrap();
    let mut client = Session::start(&p.0.join("app"), vec![], true).await;
    client.open(&root, body).await;
    let initial = client.diagnostics(&root, 1).await;
    assert!(
        !initial["params"]["diagnostics"]
            .as_array()
            .unwrap()
            .iter()
            .any(|item| item["message"].as_str().unwrap().contains("EXTERNAL_VALUE")),
        "{initial}"
    );
    for (index, mode) in ["full", "functions", "none"].into_iter().enumerate() {
        tokio::fs::write(&config,format!("[modules]\nsearch_paths=[\"../modules\"]\npackage_search_paths=[]\nexternal_include=\"{mode}\"\n")).await.unwrap();
        let id = index as i32 * 10 + 10;
        client.send(json!({"jsonrpc":"2.0","id":id,"method":"cmake-analyzer/reloadConfig","params":null})).await;
        let result = client.response(id).await;
        assert!(result.get("error").is_none(), "{result}");
        let report = client.diagnostics(&root, 1).await;
        let items = report["params"]["diagnostics"].as_array().unwrap();
        assert_eq!(
            items
                .iter()
                .any(|item| item["message"].as_str().unwrap().contains("ext_call")),
            mode == "none",
            "{mode}: {report}"
        );
        assert_eq!(
            items
                .iter()
                .any(|item| item["message"].as_str().unwrap().contains("EXTERNAL_VALUE")),
            mode != "full",
            "{mode}: {report}"
        );
        let symbols = workspace_symbols(&mut client, id + 1).await;
        for (name, present) in [
            ("ext_call", mode != "none"),
            ("EXTERNAL_VALUE", mode == "full"),
            ("ExternalTarget", mode == "full"),
        ] {
            assert_eq!(
                symbols
                    .as_array()
                    .unwrap()
                    .iter()
                    .any(|item| item["name"] == name && item["location"]["uri"] == uri(&external)),
                present,
                "{mode}: {symbols}"
            );
        }
        let module = definition(&mut client, &root, 0, 10, id + 2).await;
        assert_eq!(module["result"][0]["uri"], uri(&external), "{module}");
        let function = definition(&mut client, &root, 1, 2, id + 3).await;
        assert_eq!(
            function["result"].is_null(),
            mode == "none",
            "{mode}: {function}"
        );
    }
    assert!(!p.0.join("app/marker.txt").exists());
    assert!(!p.0.join("modules/marker.txt").exists());
    client.finish().await;
    p.remove().await;
}
