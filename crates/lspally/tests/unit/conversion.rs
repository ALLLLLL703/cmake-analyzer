// AI-generated tests.
use super::convert::{
    data_to_wire, decode_completion_item, encode_completion_item, encode_semantic_tokens,
};
use crate::server::model as domain;

#[test]
fn completion_round_trip_preserves_nested_application_data() {
    let item = lsp_types::CompletionItem {
        label: "你好".into(),
        kind: Some(lsp_types::CompletionItemKind::FUNCTION),
        data: Some(serde_json::json!({"list":[null,true,42,"🌟"],"object":{"key":"value"}})),
        ..Default::default()
    };
    let expected = serde_json::to_value(&item).unwrap();
    let result = encode_completion_item(decode_completion_item(item).unwrap()).unwrap();
    assert_eq!(serde_json::to_value(result).unwrap(), expected);
}

#[test]
fn semantic_tokens_encode_as_flat_five_integer_records() {
    let result = encode_semantic_tokens(domain::SemanticTokens {
        result_id: Some("v2".into()),
        data: vec![domain::SemanticToken {
            delta_line: 1,
            delta_start: 2,
            length: 3,
            token_type: 4,
            token_modifiers_bitset: 5,
        }],
    })
    .unwrap();
    assert_eq!(
        serde_json::to_value(result).unwrap(),
        serde_json::json!({"resultId":"v2","data":[1,2,3,4,5]})
    );
}

#[test]
fn non_finite_application_numbers_are_errors_not_silent_nulls() {
    for number in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
        assert_eq!(
            data_to_wire(domain::Data::Float(number)).unwrap_err().code,
            -32603
        );
    }
}
