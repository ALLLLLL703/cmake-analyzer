// AI-generated performance tests.
use super::session::Session;
use serde_json::json;
use std::time::Instant;

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
