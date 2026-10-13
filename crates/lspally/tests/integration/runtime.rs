// AI-generated tests.
use super::session::Session;
use lspally::server::ExitStatus;
use serde_json::json;

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
