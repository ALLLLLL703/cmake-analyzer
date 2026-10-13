> AI-generated content.

# LSP performance checks

```sh
cargo test -p lspally --release -- --ignored --nocapture --test-threads=1
```

These ignored tests print measurements, not machine-dependent pass/fail limits.

- `completion_adapters_against_wire_only_move`: 20 batches of 1,000 completions, including private scalar kinds and nested application data. Compare wire-only moves with wire → business → wire conversion. Inputs are prepared before timing; output destruction is timed. JSON text serialization is excluded.
- `semantic_token_encoding_large_vectors`: encode 50 owned vectors of 10,000 tokens. Input construction and JSON serialization are excluded; output destruction is included. The optimizer may reuse the vector allocation.
- `small_hover_round_trip_latency`: 10 warmup requests followed by 1,000 measured requests over Tokio duplex streams. Reports p50/p95/p99, including framing, JSON, adapters, scheduling, and fixture event recording. No language analysis or operating-system pipe is measured.

## Local sample

One serial release run with rustc 1.98.1; timings vary with allocator, CPU, and load.

| Path | Measurement |
| --- | --- |
| Completion, wire-only (20,000 items) | 11.86 ms |
| Completion, decode + encode (20,000 items) | 21.58 ms |
| Semantic token encoding (500,000 tokens) | 0.93 ms |
| Duplex hover p50 / p95 / p99 | 5.35 / 5.64 / 9.90 µs |

The completion difference is about 0.49 µs/item for this payload and run. It is not a general LSP overhead estimate. These tests do not measure allocation counts or peak memory.
