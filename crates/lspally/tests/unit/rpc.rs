// AI-generated tests.

//! Check JSON-RPC envelope validation, typed response IDs, and UTF-8 frame lengths.
//! All fixtures are in-memory values; no files, network access, or child processes.

use lspally::rpc;
use serde_json::json;

/// Decode five envelopes with null request ID, conflicting response payloads, absent payload, wrong protocol version, or scalar parameters.
/// No external side effects.
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
/// Require every packet to fail Message deserialization rather than fall back to another envelope kind.
///
/// # Examples
/// Local test-module context; run through the test harness (ignored measurements require explicit selection).
/// ```text
/// malformed_envelopes_cannot_be_reclassified();
/// ```
#[test]
fn malformed_envelopes_cannot_be_reclassified() {
    for packet in [
        json!({"jsonrpc":"2.0","id":null,"method":"x"}),
        json!({"jsonrpc":"2.0","id":1,"result":null,"error":{"code":-1,"message":"bad"}}),
        json!({"jsonrpc":"2.0","id":1}),
        json!({"jsonrpc":"1.0","id":1,"method":"x"}),
        json!({"jsonrpc":"2.0","method":"x","params":42}),
    ] {
        assert!(serde_json::from_value::<rpc::Message>(packet).is_err());
    }
}

/// Deserialize and reserialize a null success with ID 7, then decode a success without ID.
/// No external side effects.
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
/// Require exact round-trip JSON and rejection of the missing-ID message.
///
/// # Examples
/// Local test-module context; run through the test harness (ignored measurements require explicit selection).
/// ```text
/// null_result_is_success_and_id_is_required();
/// ```
#[test]
fn null_result_is_success_and_id_is_required() {
    let value = json!({"jsonrpc":"2.0","id":7,"result":null});
    let response: rpc::Response = serde_json::from_value(value.clone()).unwrap();
    assert_eq!(serde_json::to_value(response).unwrap(), value);
    assert!(
        serde_json::from_value::<rpc::Message>(json!({"jsonrpc":"2.0","result":null})).is_err()
    );
}

/// Frame a typed log notification containing 你好🌟.
/// No external side effects; framing stays in memory.
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
/// Require Content-Length to equal body bytes, exceed character count, and preserve the decoded message.
///
/// # Examples
/// Local test-module context; run through the test harness (ignored measurements require explicit selection).
/// ```text
/// unicode_content_length_counts_bytes_not_characters();
/// ```
#[test]
fn unicode_content_length_counts_bytes_not_characters() {
    let notification = rpc::Notification::new::<lsp_types::notification::LogMessage>(
        lsp_types::LogMessageParams {
            typ: lsp_types::MessageType::INFO,
            message: "你好🌟".into(),
        },
    )
    .unwrap();
    let frame = notification.display().unwrap();
    let (header, body) = frame.split_once("\r\n\r\n").unwrap();
    assert_eq!(header, format!("Content-Length: {}", body.len()));
    assert!(body.len() > body.chars().count());
    assert_eq!(
        serde_json::from_str::<serde_json::Value>(body).unwrap()["params"]["message"],
        "你好🌟"
    );
}

/// Create a typed shutdown request with ID 1 and success replies with IDs 2 and 1.
/// No external side effects.
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
/// Require mismatched-ID rejection and successful decoding of the matching response.
///
/// # Examples
/// Local test-module context; run through the test harness (ignored measurements require explicit selection).
/// ```text
/// typed_response_requires_matching_request_id();
/// ```
#[test]
fn typed_response_requires_matching_request_id() {
    use lsp_types::request::Shutdown;
    let request = rpc::Request::new::<Shutdown>(lsp_types::NumberOrString::Number(1), ()).unwrap();
    let wrong =
        rpc::Response::success::<Shutdown>(lsp_types::NumberOrString::Number(2), ()).unwrap();
    assert!(request.decode_response::<Shutdown>(wrong).is_err());
    let correct =
        rpc::Response::success::<Shutdown>(lsp_types::NumberOrString::Number(1), ()).unwrap();
    request.decode_response::<Shutdown>(correct).unwrap();
}
