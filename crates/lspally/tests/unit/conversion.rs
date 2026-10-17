// AI-generated tests.

//! Check private wrapper conversions for nested data, semantic tokens, and invalid floats.
//! Loaded from the wrapper module to keep adapters private; no external side effects.

use super::convert::{
    data_to_wire, decode_completion_item, encode_completion_item, encode_semantic_tokens,
};
use crate::server::model as domain;

/// Round-trip a Unicode FUNCTION completion with nested null, boolean, number, string, and object data.
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
/// Require the reencoded JSON value to equal the original serialized item.
///
/// # Examples
/// Local test-module context; run through the test harness (ignored measurements require explicit selection).
/// ```text
/// completion_round_trip_preserves_nested_application_data();
/// ```
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

/// Encode one semantic token with fields 1 through 5 and result ID v2.
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
/// Require JSON data `[1,2,3,4,5]` and the unchanged result ID.
///
/// # Examples
/// Local test-module context; run through the test harness (ignored measurements require explicit selection).
/// ```text
/// semantic_tokens_encode_as_flat_five_integer_records();
/// ```
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

/// Convert NaN, positive infinity, and negative infinity as application floats.
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
/// Require each conversion to fail with code -32603 instead of producing JSON null.
///
/// # Examples
/// Local test-module context; run through the test harness (ignored measurements require explicit selection).
/// ```text
/// non_finite_application_numbers_are_errors_not_silent_nulls();
/// ```
#[test]
fn non_finite_application_numbers_are_errors_not_silent_nulls() {
    for number in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
        assert_eq!(
            data_to_wire(domain::Data::Float(number)).unwrap_err().code,
            -32603
        );
    }
}
