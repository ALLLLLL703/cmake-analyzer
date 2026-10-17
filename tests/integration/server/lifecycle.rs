// AI-generated lifecycle, edit, diagnostic, and symbol tests.
//! didOpen/didChange/didClose diagnostics, symbol shapes and EOF teardown.
//! Standalone unsaved paths need no disk file; all temporary directories are removed.
use super::session::*;
use serde_json::json;
use tokio::io::AsyncWriteExt;

#[tokio::test]
/// Check standalone UTF-16 diagnostics, hierarchical symbols and didClose clearing.
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
/// Open an unsaved function plus an emoji/unknown call; require the exact bogus range and
/// nested VAR under hello. Incrementally replace bogus with message and require empty version-2
/// diagnostics. Close the document and require empty diagnostics without a version.
///
/// # Flow
/// ```text
/// [Temporary sources + settings]      [Expected: nested symbols + cleared reports]
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
/// standalone_incremental_diagnostics_and_nested_symbols(); // #[tokio::test] supplies the runtime
/// ```
async fn standalone_incremental_diagnostics_and_nested_symbols() {
    let project = Project::new().await;
    let path = project.0.join("loose.cmake");
    let mut session = Session::start(&project.0, vec![], true).await;
    session
        .open(
            &path,
            "function(hello)\nset(VAR x)\nendfunction()\nmessage(\"🌟\") bogus()",
        )
        .await;
    let report = session.diagnostics(&path, 1).await;
    let errors = report["params"]["diagnostics"].as_array().unwrap();
    let unknown = errors
        .iter()
        .find(|error| error["message"].as_str().unwrap().contains("bogus"))
        .unwrap();
    assert_eq!(unknown["range"]["start"], json!({"line":4,"character":14}));
    let symbols = session.symbols(&path, 2).await;
    let symbols = symbols["result"].as_array().unwrap();
    let function = symbols
        .iter()
        .find(|symbol| symbol["name"] == "hello")
        .unwrap();
    assert_eq!(
        function["selectionRange"]["start"],
        json!({"line":1,"character":9})
    );
    assert_eq!(function["children"][0]["name"], "VAR");
    session.send(json!({"jsonrpc":"2.0","method":"textDocument/didChange","params":{"textDocument":{"uri":uri(&path),"version":2},"contentChanges":[{"range":{"start":{"line":4,"character":14},"end":{"line":4,"character":19}},"text":"message"}]}})).await;
    let report = session.diagnostics(&path, 2).await;
    assert!(
        report["params"]["diagnostics"]
            .as_array()
            .unwrap()
            .is_empty()
    );
    session.send(json!({"jsonrpc":"2.0","method":"textDocument/didClose","params":{"textDocument":{"uri":uri(&path)}}})).await;
    let cleared = session
        .wait(|value| {
            value["method"] == "textDocument/publishDiagnostics"
                && value["params"]["uri"] == uri(&path)
                && value["params"].get("version").is_none()
        })
        .await;
    assert!(
        cleared["params"]["diagnostics"]
            .as_array()
            .unwrap()
            .is_empty()
    );
    session.finish().await;
    project.remove().await;
}

#[tokio::test]
/// Check flat symbol containers and coalesced edits expose the newest overlay.
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
/// Start without hierarchical symbols, require A in container f at the open URI, then send full
/// changes through versions 2..5. Query symbols and require V5 plus an empty version-5
/// diagnostic report.
///
/// # Flow
/// ```text
/// [Temporary sources + settings]      [Expected: flat symbols from version 5]
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
/// flat_symbols_and_multiple_edits_use_current_text(); // #[tokio::test] supplies the runtime
/// ```
async fn flat_symbols_and_multiple_edits_use_current_text() {
    let project = Project::new().await;
    let path = project.0.join("loose.cmake");
    let mut session = Session::start(&project.0, vec![], false).await;
    session
        .open(&path, "function(f)\nset(A x)\nendfunction()")
        .await;
    let result = session.symbols(&path, 2).await;
    let variable = result["result"]
        .as_array()
        .unwrap()
        .iter()
        .find(|symbol| symbol["name"] == "A")
        .unwrap();
    assert_eq!(variable["containerName"], "f");
    assert_eq!(variable["location"]["uri"], uri(&path));
    for version in 2..=5 {
        session.send(json!({"jsonrpc":"2.0","method":"textDocument/didChange","params":{"textDocument":{"uri":uri(&path),"version":version},"contentChanges":[{"text":format!("{HEADER}set(V{version} x)")}]}})).await;
    }
    let result = session.symbols(&path, 3).await;
    assert!(
        result["result"]
            .as_array()
            .unwrap()
            .iter()
            .any(|symbol| symbol["name"] == "V5")
    );
    assert!(
        session.diagnostics(&path, 5).await["params"]["diagnostics"]
            .as_array()
            .unwrap()
            .is_empty()
    );
    session.finish().await;
    project.remove().await;
}

#[tokio::test]
/// Check client input EOF joins the wrapper/workspace service as Disconnected.
/// Creates a temporary tree and analyzer task; joins on EOF and removes the tree.
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
/// Open a standalone document and consume its diagnostics, shut down the client write half
/// without shutdown/exit, then require Disconnected from the server task within five seconds.
///
/// # Flow
/// ```text
/// [Temporary sources + settings]      [Expected: Disconnected task status]
///              |                              |
///              v                              |
/// [LSP scenario messages]                 |
///              |                              |
///              +-------------+----------------+
///                            v
///                  [Compare assertions]
///                            |
///                            v
///             [Join task + remove tree]
/// ```
///
/// # Examples
/// ```text
/// eof_stops_workspace_service(); // #[tokio::test] supplies the runtime
/// ```
async fn eof_stops_workspace_service() {
    let project = Project::new().await;
    let path = project.0.join("loose.cmake");
    let mut session = Session::start(&project.0, vec![], true).await;
    session.open(&path, "set(A x)").await;
    session.diagnostics(&path, 1).await;
    session.input.shutdown().await.unwrap();
    assert_eq!(
        tokio::time::timeout(std::time::Duration::from_secs(5), session.task)
            .await
            .unwrap()
            .unwrap(),
        lspally::server::ExitStatus::Disconnected
    );
    project.remove().await;
}
