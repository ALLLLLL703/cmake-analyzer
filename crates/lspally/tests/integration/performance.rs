// AI-generated performance tests.

//! Measure ignored duplex hover latency, including framing, adapters, and task scheduling.
//! Prints timings only when explicitly run; no files or child processes.

use super::session::Session;
use serde_json::json;
use std::time::Instant;

/// Measure 1,000 hover requests after ten warmups against the UTF-8 duplex fixture.
/// Creates tasks/streams and prints stdout; no files or subprocesses. Ignored unless explicitly selected.
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
/// Require measured responses to contain null results; sort durations and print p50/p95/p99, then shut down. No latency threshold is asserted.
///
/// # Flow
/// ```text
/// [initialize] --> [10 warmup hovers]
///                          |
///                          v
///                  [1000 timed hovers] <---+
///                          |              | next ID
///                          +--------------+
///                          |
///                          v
///                  [null checks / sort]
///                          |
///                          v
/// [shutdown / finish] <-- [print p50 / p95 / p99]
/// ```
///
/// # Examples
/// Local test-module context, outside any running Tokio runtime:
/// the macro-generated test wrapper runs its own runtime and returns `()`.
/// Prefer the test harness; ignored measurements require explicit selection.
/// ```text
/// small_hover_round_trip_latency();
/// ```
#[tokio::test]
#[ignore = "measurement only; run in release mode with --ignored --nocapture"]
async fn small_hover_round_trip_latency() {
    let mut session = Session::new();
    session.initialize().await;
    let params = json!({"textDocument":{"uri":"file:///tmp/bench.cmake"},"position":{"line":0,"character":4}});
    for id in 2..12 {
        session
            .query(id, "textDocument/hover", params.clone())
            .await;
    }
    let mut samples = Vec::with_capacity(1000);
    for id in 12..1012 {
        let params = params.clone();
        let start = Instant::now();
        let response = session.query(id, "textDocument/hover", params).await;
        samples.push(start.elapsed());
        assert!(response.get("result").unwrap().is_null());
    }
    samples.sort_unstable();
    println!(
        "duplex hover (1000 requests, 10 warmup): p50 {:?}, p95 {:?}, p99 {:?}; includes JSON, framing, scheduling and adapters; fixture records each query",
        samples[500], samples[950], samples[990]
    );
    session.shutdown().await;
    session.finish().await.unwrap();
}
