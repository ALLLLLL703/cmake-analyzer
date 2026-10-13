// AI-generated tests.
use lspally::rpc;
use serde_json::json;

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

#[test]
fn null_result_is_success_and_id_is_required() {
    let value = json!({"jsonrpc":"2.0","id":7,"result":null});
    let response: rpc::Response = serde_json::from_value(value.clone()).unwrap();
    assert_eq!(serde_json::to_value(response).unwrap(), value);
    assert!(
        serde_json::from_value::<rpc::Message>(json!({"jsonrpc":"2.0","result":null})).is_err()
    );
}

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
