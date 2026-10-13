> AI-generated content.

# LSP server trait and wrapper

Status: implemented after user approval. Verification uses temporary sources outside `target/`; generators live under `scripts/gen/lspally-schema/`.

## Agreed boundaries

- `ILanguageServer` declares every stable LSP method received by a server, plus custom request/notification handlers. Every method requires an explicit implementation; no default bodies.
- Business models are independent of `lsp-types`, serde, and RPC envelopes. Public business APIs must not expose those transport types.
- Reuse `lsp-types` inside the Wrapper for standard parameter/result conversion. Do not recreate a method enum or duplicate RPC serialization.
- `Wrapper<S: ILanguageServer>` owns protocol handling, not language analysis. Use static dispatch and async handlers with Send futures.
- Server-to-client methods belong to a typed `ClientHandle`, not incoming server handlers.
- Unsupported features return explicit business errors. Do not use `todo!()` in framework implementation paths or advertise unsupported capabilities.
- Unknown methods reach custom handlers. Known methods with invalid parameters or unsupported implementations do not.

## Files

| File | Responsibility |
| --- | --- |
| `server.rs` | Module entrypoint and public exports |
| `server/model.rs` | Trait, Wrapper, ClientHandle, and runtime state declarations |
| `server/model/document.rs` | Document identity, positions, ranges, versions, changes |
| `server/model/lifecycle.rs` | Initialization, capabilities, configuration, lifecycle data |
| `server/model/features.rs` | Feature input/output models; split by feature if large |
| `server/model/diagnostic.rs` | Diagnostics, related locations, push/pull reports |
| `server/model/client.rs` | Typed outgoing client operations and results |
| `server/model/context.rs` | Request context, cancellation, business errors |
| `server/model/extension.rs` | Custom method name and raw JSON payload boundary |
| `server/wrapper.rs` | Implementation module entrypoint |
| `server/wrapper/transport.rs` | Async frame reading and serialized frame writing |
| `server/wrapper/lifecycle.rs` | Initialization, shutdown, exit, EOF cleanup |
| `server/wrapper/dispatch.rs` | Incoming method routing and typed handler calls |
| `server/wrapper/convert/` | Focused business ↔ LSP conversion modules |
| `server/wrapper/client.rs` | Outgoing requests/notifications and capability checks |
| `server/wrapper/pending.rs` | Request IDs, response matching, cancellation |
| `server/wrapper/runtime.rs` | Ordered updates, bounded execution and queues |

Create only modules with implemented responsibilities. Keep data declarations in model files.

## Interface and models

- Use the stable LSP 3.17 methods represented by `lsp-types 0.97`; exclude proposed methods. Inventory each method by direction, request/notification kind, model, and handler before implementation.
- Cover lifecycle, text/notebook synchronization, configuration/workspace changes, queries, completion/resolution, symbols, edits, formatting, hierarchies, tokens, hints, diagnostics, and client operations.
- Treat `$/cancelRequest` and `$/setTrace` as protocol-owned actions with required business hooks where needed. A hook cannot prevent the Wrapper from applying protocol rules.
- Keep business results separate from JSON-RPC responses: hover data, completion data, locations, edits, diagnostics, etc. Preserve optional results and meaningful alternative result shapes.
- Define independent capabilities and negotiated position encoding. Positions must have documented units; conversion must not silently assume byte offsets or UTF-16.
- Separate request failure from a valid empty/null result. Map business errors to appropriate RPC errors only in the Wrapper.
- Business models should move payloads across the boundary without cloning whole documents or result trees.

## Active output and extensions

- Provide `ClientHandle` to handlers so background work can publish updates or request client actions.
- A diagnostic publication carries document URI, optional version, and `Vec<Diagnostic>`. A vector alone is insufficient to route or clear diagnostics.
- ClientHandle covers configuration, capability registration, edits, messages, progress, refresh operations, and other standard server-to-client methods.
- Separate `custom_request` and `custom_notification`. Requests return a raw JSON result or business error; notifications never receive an RPC response.
- Extensions are the explicit exception to the JSON-free business boundary: expose method name and raw parameter JSON, not frame headers or an entire RPC envelope. The Wrapper retains ID/version ownership.
- Unhandled custom requests return MethodNotFound; unknown notifications may be ignored. Keep `$/` reserved-method rules.

## Runtime rules

- Start with supplied Tokio async reader/writer streams; stdio is an adapter. No HTTP transport, reconnect logic, or automatic CMake configure in this task.
- Read UTF-8 Content-Length frames and reuse existing RPC models/framing. Malformed input must not be dispatched as a different message kind.
- Enforce initialize → initialized → running → shutdown → exit. Respect allowed initialization-time output; shutdown responds with null before exit.
- Keep the reader active while handlers await client responses. Otherwise a configuration/applyEdit request can deadlock.
- Use one serialized writer, bounded queues, bounded handler concurrency, and tracked tasks. Do not hold a global lock across a business await.
- Apply document changes in arrival order before dispatching dependent queries. Concurrent work must use a consistent document version/snapshot.
- Match outgoing responses by ID. Track incoming cancellation separately; cancelled requests still receive exactly one response.
- Retire pending requests/tasks on shutdown or EOF. Reserve stdout for protocol frames.

## Implementation steps

- [x] M1: review the direction-aware method inventory and independent model contracts; declare all required trait methods and ClientHandle operations.
- [x] M2: implement business/LSP conversions and request/notification routing; preserve extension payloads and response semantics.
- [x] M3: implement async transport and lifecycle state transitions.
- [x] M4: implement ClientHandle, pending responses, cancellation, ordered updates, and bounded execution.
- [x] M5: run end-to-end verification with an explicit minimal implementation; do not connect cmake-analyzer's Workspace until separately approved.

## Verification

Completed: workspace check/format/tests (38 existing tests), RPC smoke, and a duplex fixture implementing all 73 required methods. The fixture exercises initialization, negotiated UTF-8 positions, ordered document updates/queries, empty diagnostics, partial progress, configuration callbacks without deadlock, invalid JSON/parameters, null results, extensions, cancellation, shutdown/exit, EOF, and worker failure. Standard adapters are compile-checked; the smoke does not execute every feature method. No Workspace integration or persistent tests were added.


- Compile-check complete method coverage and a minimal implementation. Explicit unsupported handlers are acceptable; default trait implementations are not.
- Exercise lifecycle, document ordering, incoming queries, active diagnostics, client requests, cancellation, EOF, and custom methods.
- Check Unicode byte lengths, position conversion, optional/null results, invalid parameters, unsupported methods, and response ID matching.
- Demonstrate that a handler awaiting a client response does not block its receipt.
- Run workspace compilation, formatting, existing tests, and transient smoke checks. Persistent new tests require approval.
- Maintain `agent-instructions/Architect.md` for implemented behavior only, with provenance labels on Markdown/tests.
