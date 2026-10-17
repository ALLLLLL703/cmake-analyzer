// AI-generated tests.

//! Check lifecycle, ordering, cancellation, and protocol errors over an in-memory session.
//! Fixtures exchange JSON frames and record events; no files or child processes.

use super::session::Session;
use lspally::server::ExitStatus;
use serde_json::json;

/// Initialize the duplex fixture, including its outgoing configuration callback.
/// Exchanges in-memory frames and joins tasks; no files or subprocesses.
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
/// Query events and require exactly configured, then a graceful shutdown.
///
/// # Flow
/// ```text
/// [initialize / initialized] --> [server asks configuration]
///            ^                            |
///            |                            v
///     [reply configured] <---------- [reader receives reply]
///            |
///            v
/// [events query: configured] --> [graceful shutdown]
/// ```
///
/// # Examples
/// Local test-module context, outside any running Tokio runtime:
/// the macro-generated test wrapper runs its own runtime and returns `()`.
/// Prefer the test harness; ignored measurements require explicit selection.
/// ```text
/// configuration_callback_keeps_reader_live();
/// ```
#[tokio::test]
async fn configuration_callback_keeps_reader_live() {
    let mut s = Session::new();
    s.initialize().await;
    assert_eq!(
        s.query(2, "custom/events", json!({})).await["result"],
        json!(["configured"])
    );
    s.shutdown().await;
    assert_eq!(s.finish().await.unwrap(), ExitStatus::Graceful);
}

/// Open Unicode text at version 1, send versions 2 and 3, then hover at character 4.
/// Exchanges in-memory frames and appends fixture events; the file URI is never opened.
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
/// Require URI/version-qualified empty diagnostics, partial progress, a null hover, ordered events, and graceful shutdown.
///
/// # Flow
/// ```text
/// [open v1: Unicode text] --> [empty diagnostics v1]
///            |                        |
///            v                        v
/// [changes v2 then v3]          [partial progress]
///            |
///            v
/// [hover character 4] ------> [null result]
///            |                    |
///            +---------+----------+
///                      v
///             [require ordered events]
/// ```
///
/// # Examples
/// Local test-module context, outside any running Tokio runtime:
/// the macro-generated test wrapper runs its own runtime and returns `()`.
/// Prefer the test harness; ignored measurements require explicit selection.
/// ```text
/// updates_precede_queries_and_publish_versioned_empty_diagnostics();
/// ```
#[tokio::test]
async fn updates_precede_queries_and_publish_versioned_empty_diagnostics() {
    let mut s = Session::new();
    s.initialize().await;
    let uri = "file:///tmp/unicode.cmake";
    s.send(json!({"jsonrpc":"2.0","method":"textDocument/didOpen","params":{"textDocument":{"uri":uri,"languageId":"cmake","version":1,"text":"你好🌟"}}})).await;
    let diagnostics = s.receive().await;
    assert_eq!(diagnostics["method"], "textDocument/publishDiagnostics");
    assert_eq!(diagnostics["params"]["uri"], uri);
    assert_eq!(diagnostics["params"]["version"], 1);
    assert_eq!(diagnostics["params"]["diagnostics"], json!([]));
    let progress = s.receive().await;
    assert_eq!(progress["method"], "$/progress");
    assert_eq!(progress["params"]["value"], json!(["partial"]));
    for version in [2, 3] {
        s.send(json!({"jsonrpc":"2.0","method":"textDocument/didChange","params":{"textDocument":{"uri":uri,"version":version},"contentChanges":[{"text":"new"}]}})).await;
    }
    let response = s
        .query(
            2,
            "textDocument/hover",
            json!({"textDocument":{"uri":uri},"position":{"line":0,"character":4}}),
        )
        .await;
    assert!(response.get("result").unwrap().is_null());
    assert_eq!(
        &*s.events.lock().unwrap(),
        &["configured", "open:1", "change:2", "change:3", "hover"]
    );
    s.shutdown().await;
    assert_eq!(s.finish().await.unwrap(), ExitStatus::Graceful);
}

/// Send a pre-initialize request, malformed JSON, invalid hover parameters, an unknown method, and a Unicode echo.
/// Uses in-memory transport/tasks only; no files or subprocesses.
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
/// Require codes -32002, -32700 with null ID, -32602, -32601, an exact echo, and graceful shutdown.
///
/// # Flow
/// ```text
/// [before initialize] --> [request: -32002]
///          |
///          +-- [raw malformed JSON] --> [-32700 / null ID]
///          |
///          v
///    [initialize]
///          |
///          +-- [invalid hover] ------> [-32602]
///          +-- [unknown method] -----> [-32601]
///          +-- [Unicode echo] -------> [unchanged JSON]
///          |
///          v
///  [graceful shutdown]
/// ```
///
/// # Examples
/// Local test-module context, outside any running Tokio runtime:
/// the macro-generated test wrapper runs its own runtime and returns `()`.
/// Prefer the test harness; ignored measurements require explicit selection.
/// ```text
/// invalid_packets_and_known_parameters_do_not_fall_back_to_extensions();
/// ```
#[tokio::test]
async fn invalid_packets_and_known_parameters_do_not_fall_back_to_extensions() {
    let mut s = Session::new();
    assert_eq!(
        s.query(0, "custom/echo", json!({})).await["error"]["code"],
        -32002
    );
    s.raw("{").await;
    let response = s.receive().await;
    assert_eq!(response["error"]["code"], -32700);
    assert!(response["id"].is_null());
    s.initialize().await;
    assert_eq!(
        s.query(2, "textDocument/hover", json!({})).await["error"]["code"],
        -32602
    );
    assert_eq!(
        s.query(3, "unknown", json!({})).await["error"]["code"],
        -32601
    );
    assert_eq!(
        s.query(4, "custom/echo", json!({"unicode":"🌟"})).await["result"],
        json!({"unicode":"🌟"})
    );
    s.shutdown().await;
    assert_eq!(s.finish().await.unwrap(), ExitStatus::Graceful);
}

/// Start custom/wait, cancel ID 2 twice, then send an echo with ID 3.
/// Cancels a fixture handler and exchanges in-memory frames; no files or subprocesses.
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
/// Require one -32800 response for ID 2, the next response to be the echo, and graceful shutdown.
///
/// # Flow
/// ```text
/// [custom/wait ID 2] <-- [cancel ID 2]
///          |
///          v
/// [one -32800 response]
///          |
///          +-- [cancel ID 2 again] --> [no extra response]
///          |
///          v
/// [echo ID 3] ---------------> [next frame is echo]
///          |
///          v
/// [graceful shutdown]
/// ```
///
/// # Examples
/// Local test-module context, outside any running Tokio runtime:
/// the macro-generated test wrapper runs its own runtime and returns `()`.
/// Prefer the test harness; ignored measurements require explicit selection.
/// ```text
/// cancelling_a_request_yields_one_response_and_preserves_connection();
/// ```
#[tokio::test]
async fn cancelling_a_request_yields_one_response_and_preserves_connection() {
    let mut s = Session::new();
    s.initialize().await;
    s.send(json!({"jsonrpc":"2.0","id":2,"method":"custom/wait","params":{}}))
        .await;
    s.send(json!({"jsonrpc":"2.0","method":"$/cancelRequest","params":{"id":2}}))
        .await;
    let response = s.receive().await;
    assert_eq!(response["id"], 2);
    assert_eq!(response["error"]["code"], -32800);
    // A second cancellation must not create another response; the next frame belongs to this query.
    s.send(json!({"jsonrpc":"2.0","method":"$/cancelRequest","params":{"id":2}}))
        .await;
    assert_eq!(
        s.query(3, "custom/echo", json!({})).await["result"],
        json!({})
    );
    s.shutdown().await;
    assert_eq!(s.finish().await.unwrap(), ExitStatus::Graceful);
}

/// Use one session for exit before initialization and a second for EOF during custom/wait.
/// Drops duplex client halves and joins server tasks; no files or subprocesses.
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
/// Require Abnormal for the first and Disconnected for the second.
///
/// # Flow
/// ```text
/// [session A]                    [session B]
///      |                              |
///      v                              v
/// [early exit]              [initialize / custom/wait]
///      |                              |
///      v                              v
/// [finish]                         [EOF / finish]
///      |                              |
///      v                              v
/// [Abnormal]                    [Disconnected]
/// ```
///
/// # Examples
/// Local test-module context, outside any running Tokio runtime:
/// the macro-generated test wrapper runs its own runtime and returns `()`.
/// Prefer the test harness; ignored measurements require explicit selection.
/// ```text
/// exit_without_shutdown_is_abnormal_and_eof_retires_waiting_handler();
/// ```
#[tokio::test]
async fn exit_without_shutdown_is_abnormal_and_eof_retires_waiting_handler() {
    let mut s = Session::new();
    s.send(json!({"jsonrpc":"2.0","method":"exit"})).await;
    assert_eq!(s.finish().await.unwrap(), ExitStatus::Abnormal);
    let mut s = Session::new();
    s.initialize().await;
    s.send(json!({"jsonrpc":"2.0","id":2,"method":"custom/wait","params":{}}))
        .await;
    assert_eq!(s.finish().await.unwrap(), ExitStatus::Disconnected);
}
