> AI-generated content.

# Reference analysis benchmark

Run alone, in release mode:

```sh
cargo test --locked --release --test reference_performance -- --ignored --nocapture --test-threads=1
```

`references.rs` emits JSON sample times in milliseconds. Each case has 3 warmups and 21 measured samples. Fixtures pin CMake 3.31, avoid user configuration, use literal dependencies, and have no machine-specific pass/fail thresholds. Normal test runs ignore this benchmark.

## Comparison recorded for reference indexing

Baseline: commit `46cd5e0` (before reference/function/warning indexing). Current: the uncommitted implementation including typed reference IDs. The same benchmark source was compiled against each version in isolated source directories, using the same Cargo.lock, release profile, rustflags, and target directory. Binaries were copied out before compiling the other version. Pending process-limit changes do not participate in these direct parser/Workspace workloads.

Machine: Intel Core Ultra 7 255H, Linux 7.2.7-zen1-1-zen x86_64, rustc 1.98.1. Three serial process runs per version, alternating order: baseline/current, current/baseline, baseline/current. Reported values are the median of the three per-run medians. These are synthetic local measurements, not editor round-trip latency or portable performance guarantees.

| Case | Baseline ms | Current ms | Change |
| --- | ---: | ---: | ---: |
| Full lexer/nparser/semantic, 300 set/message pairs | 1.915 | 2.131 | +11.3% |
| Full lexer/nparser/semantic, 3000 pairs | 15.301 | 15.218 | -0.5%, approximately unchanged |
| Fresh Workspace, root plus 4 modules of 300 pairs | 12.765 | 14.879 | +16.6% |
| Warm Workspace, one root literal edit, same modules | 0.154 | 1.036 | +574.4%, 6.74x |

Each pair declares one variable and reads it twice. The Workspace cold case creates a fresh analyzer and reads files with warm OS caches; it is not a cold-disk benchmark. The edit alternates equal-length values and asserts that all five dependency invocations are reused. Workspace timings include the normal full parser/build/reference work for that operation; direct full-parser timings stop after semantic analysis.

The warm edit has the clearest regression: about 0.882 ms added here. The reference index currently replays all dependency visits even when graph structure and unchanged module snapshots are reused. These aggregate passes are not fine-grained incremental. The large full-parser result is within observed variation and is not evidence of an optimization.

Per-run medians, in milliseconds:

| Case | Baseline runs | Current runs |
| --- | --- | --- |
| Full 300 | 1.857724, 1.932065, 1.914977 | 2.130809, 2.116851, 2.217438 |
| Full 3000 | 14.334557, 15.300622, 15.361809 | 15.218163, 15.319541, 15.122862 |
| Workspace cold | 12.159924, 13.259794, 12.765238 | 12.887003, 14.879069, 15.700659 |
| Workspace edit | 0.153614, 0.143983, 0.322504 | 1.036011, 0.985134, 1.492718 |

Raw samples for this session are retained in `/tmp/cmake-reference-performance-results.json`; process logs are `/tmp/cmake-reference-{baseline,current}-{1,2,3}.log`. The persistent benchmark source can regenerate samples, but those temporary files are not durable project artifacts. These measurements do not isolate allocations, RSS by operation, function-call-heavy inputs, warning publication, or LSP transport cost.
