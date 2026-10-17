// AI-generated performance tests.

//! Measure private completion and semantic-token adapters through wrapper test wiring.
//! Ignored measurements allocate payloads and print timings; no files or child processes.

use super::convert::{decode_completion_item, encode_completion_item, encode_semantic_tokens};
use crate::server::model as domain;
use std::{hint::black_box, time::Instant};

/// Compare moving 1,000 wire completions over 20 rounds against decoding and reencoding equivalent owned payloads.
/// Allocates memory and prints stdout only; ignored measurement, preferably release mode.
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
/// Prepare clones outside timers, include output destruction, assert the first item's JSON is preserved, and print per-item timing without a threshold.
///
/// # Flow
/// ```text
/// [1000-item prototype] --> [clone 20 batches per path]
///                                     |             |
///                                     v             v
///                               [wire move]  [decode / encode]
///                                     |             |
///                                     +------+------+
///                                            v
/// [print timings / item] <-- [JSON equality of first item]
/// ```
///
/// # Examples
/// Local test-module context; run through the test harness (ignored measurements require explicit selection).
/// ```text
/// completion_adapters_against_wire_only_move();
/// ```
#[test]
#[ignore = "measurement only; run in release mode with --ignored --nocapture"]
fn completion_adapters_against_wire_only_move() {
    const ITEMS: usize = 1000;
    const ROUNDS: usize = 20;
    let prototype: Vec<_> = (0..ITEMS)
        .map(|i| lsp_types::CompletionItem {
            label: format!("completion_{i}_你好"),
            kind: Some(lsp_types::CompletionItemKind::FUNCTION),
            data: Some(
                serde_json::json!({"index":i,"nested":["one","two",{"uri":"file:///a.cmake"}]}),
            ),
            ..Default::default()
        })
        .collect();
    // Prepare and clone outside the timers. Both paths consume equivalent owned payloads.
    let baseline: Vec<_> = (0..ROUNDS).map(|_| prototype.clone()).collect();
    let converted: Vec<_> = (0..ROUNDS).map(|_| prototype.clone()).collect();
    let expected = serde_json::to_value(&prototype[0]).unwrap();
    let start = Instant::now();
    for items in baseline {
        black_box(items.into_iter().map(black_box).collect::<Vec<_>>());
    }
    let wire_elapsed = start.elapsed();
    let start = Instant::now();
    for items in converted {
        let output: Vec<_> = items
            .into_iter()
            .map(|item| {
                encode_completion_item(decode_completion_item(black_box(item)).unwrap()).unwrap()
            })
            .collect();
        black_box(output);
    }
    let conversion_elapsed = start.elapsed();
    let result = encode_completion_item(
        decode_completion_item(prototype.into_iter().next().unwrap()).unwrap(),
    )
    .unwrap();
    assert_eq!(serde_json::to_value(result).unwrap(), expected);
    println!(
        "completion: {} items, wire-only {:?}, decode+encode {:?}, {:.1} ns/item; includes output drop, excludes construction/JSON serialization",
        ITEMS * ROUNDS,
        wire_elapsed,
        conversion_elapsed,
        conversion_elapsed.as_nanos() as f64 / (ITEMS * ROUNDS) as f64
    );
}

/// Encode fifty preconstructed vectors of 10,000 semantic tokens.
/// Allocates memory and prints stdout only; ignored measurement, preferably release mode.
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
/// Require every encoded vector to retain 10,000 records and print per-token timing; construction and JSON serialization are excluded.
///
/// # Flow
/// ```text
/// [prepare 50 x 10000 tokens]
///              |
///              v
///       [start timer]
///              |
///              v
///       [encode one vector] <------+
///              |                   | next vector
///              v                   |
///    [require 10000 records] ------+
///              |
///              v
///       [print ns / token]
/// ```
///
/// # Examples
/// Local test-module context; run through the test harness (ignored measurements require explicit selection).
/// ```text
/// semantic_token_encoding_large_vectors();
/// ```
#[test]
#[ignore = "measurement only; run in release mode with --ignored --nocapture"]
fn semantic_token_encoding_large_vectors() {
    const TOKENS: usize = 10000;
    const ROUNDS: usize = 50;
    let inputs: Vec<_> = (0..ROUNDS)
        .map(|_| domain::SemanticTokens {
            result_id: Some("revision-1".into()),
            data: (0..TOKENS)
                .map(|_| domain::SemanticToken {
                    delta_line: 0,
                    delta_start: 1,
                    length: 4,
                    token_type: 2,
                    token_modifiers_bitset: 0,
                })
                .collect(),
        })
        .collect();
    let start = Instant::now();
    for input in inputs {
        let result = encode_semantic_tokens(black_box(input)).unwrap();
        assert_eq!(result.data.len(), TOKENS);
        black_box(result);
    }
    let elapsed = start.elapsed();
    println!(
        "semantic tokens: {} tokens, encode {:?}, {:.1} ns/token; includes output drop, excludes construction/JSON serialization",
        TOKENS * ROUNDS,
        elapsed,
        elapsed.as_nanos() as f64 / (TOKENS * ROUNDS) as f64
    );
}
