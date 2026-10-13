> AI-generated content.

# Project Architecture

## Current implementation

The `hand` branch contains a handwritten CMake lexer, command parser, and static semantic analyzer. `src/main.rs` remains a placeholder; there is no running LSP on this branch. `src/lib.rs` exports `model`, `parser`, and `workspace`; the old `rpc` scaffold has been removed. The `lexer`, `nparser`, `semanticer`, `build-struct`, and `dependency-graph` inspection binaries are under `cli/parser/`.

## Cargo workspace

- Root `Cargo.toml`: the analyzer package and `crates/lspally` library share a Cargo workspace with resolver 3. Existing `clap`, `owo-colors`, and `tokio` versions/features live in `[workspace.dependencies]`; both packages inherit them. `lspally` also inherits `serde` (derive), `serde_json`, and `lsp-types 0.97`.
- `crates/lspally/Cargo.toml` and `src/lib.rs`: edition 2024 library exporting `rpc` and `server`. `server/model.rs` declares the server framework; `server/wrapper/` implements stream transport and execution. No analyzer-backed server executable is wired yet.

## LSP JSON-RPC

- `crates/lspally/src/rpc/model.rs`: serde-compatible `Request`, `Notification`, `Response`, and untagged `Message`; JSON-RPC version is always 2.0. IDs reuse `lsp_types::NumberOrString`; parameters reuse its object/array union. Unknown method names and raw parameters remain available; there is no duplicate method enum or registry. Absent/null parameters normalize to omission. Responses contain either a JSON result (including null) or an error; null IDs are only accepted for errors.
- `rpc/methods.rs`: generic construction and parameter extraction use `lsp_types::request::Request` / `notification::Notification` markers and associated types. `take_params` consumes the JSON tree without cloning and retains the envelope/ID. `decode_response` verifies method and ID before consuming and decoding the result into the request's associated result type. Outstanding-request tracking is left to the caller. Raw progress parameters preserve partial results not modeled by lsp-types' work-done progress type.
- `rpc/serde.rs`: small adapters preserve missing-vs-null result/error fields, reject notifications carrying IDs and responses with both/neither payloads, and classify messages by field presence without falling back from malformed requests to notifications.
- `rpc/display.rs`: `to_lsp_string(&message)` and `.display()` return a fallible String containing `Content-Length: <UTF-8 bytes>\r\n\r\n<compact JSON>`. No ANSI colors or extra trailing newline. Serialization errors are returned; no framing parser or stream I/O is implemented.

## Reusable LSP server

- `server/model.rs`: `ILanguageServer` requires 51 standard request handlers, 20 notification handlers, and two custom hooks, without default bodies. `Wrapper<S>`, `ClientHandle`, `Limits`, and private session state live here. The direction inventory is in `todos/d1/lsp-methods.md`.
- `server/model/`: independent document, lifecycle, capabilities, edits, diagnostics, and feature records. Standard business APIs expose neither serde nor lsp-types. `Data` carries arbitrary application data; only `Extension` exposes raw JSON. `Position` uses zero-based coordinates in `Context::encoding`; the wrapper does not translate analyzer byte spans or manage document text.
- `model/context.rs`: business errors, negotiated UTF-8/UTF-16/UTF-32 encoding, and cooperative cancellation. `model/client.rs`: URI/version-qualified diagnostic publications and arbitrary partial-result progress.
- `wrapper/convert/`: moved field-level model adapters; private lsp-types scalar wrappers use small serde bridges. `dispatch.rs` routes standard methods into feature handlers under `handlers/`; only unknown non-reserved methods reach extension hooks. Known invalid parameters return InvalidParams, not an extension fallback.
- `wrapper/runtime.rs`: `Wrapper::run(reader, writer)` and `run_stdio`. One business worker preserves notification/query arrival order; an independent reader receives responses while a handler awaits a client call. Queues and outstanding client requests are bounded. Full request queues produce errors; full notification queues end the connection rather than silently losing document changes.
- `wrapper/lifecycle.rs`: client capability checks, dynamic-registration method checks, initialization progress token, and position encoding negotiation. The reader enforces initialization/shutdown phases and applies cancellation/trace actions before business hooks. Successful shutdown retires outgoing waits; exit/EOF cancels the session and joins tasks, including a failed worker or blocked writer.
- `wrapper/transport.rs`: bounded ASCII CRLF headers, one Content-Length, UTF-8 bodies, exact reads, and one serialized writer with flush acknowledgements. Reader-generated errors enqueue without awaiting a socket flush. Logs use stderr, not protocol stdout.
- `wrapper/client.rs` and `pending.rs`: typed standard outgoing calls, unique IDs, response correlation, capability/lifecycle checks, initialization-output restrictions, diagnostics clearing, work-done/partial progress, and trace filtering. Short pending-map locks are never held across a business await; drop guards retire abandoned waits.
- `scripts/gen/lspally-schema/`: authored generators for seed models/adapters, method inventory, registration checks, and explicit unsupported test scaffolding. Seed output is not a complete regeneration of manually adapted production files. Generated sections retain script-origin labels; manual changes are marked separately. Temporary verification sources stay outside `target/`. User-approved persistent tests now live in package-root `tests/` directories.
- Current execution is serialized, not parallel query execution. Business implementations choose advertised feature capabilities and own text/version semantics. No HTTP, reconnect, CMake Workspace integration, or automatic configure is implemented.

## Syntax and source ranges

- `src/model.rs` and `src/model/span.rs`: `Spanned<T>` and half-open UTF-8 byte ranges in `TextSpan`. Columns are Unicode scalar counts, not LSP UTF-16 coordinates.
- `src/parser/lexer/`: span-backed tokens/payloads. Drops line/bracket comments; preserves quoted, bracket, and escaped `#`. Reports unclosed bracket comments.
- `src/parser/nparser/model.rs`: command nodes, raw argument forms, and `CommandState`.
- `src/parser/nparser/parsing.rs`: `NParser::parse` and `consume_function_args`. A complete command ends at its consumed outer right parenthesis, not the following token.

## Semantic data

All semantic arena models are defined in `src/parser/semanticer/model.rs`:

- `Semanticer`: parsed input, active and injected schema maps, and selected CMake version.
- `SemanticedBuffer`: commands, variables, diagnostics, blocks, scopes, and root symbols.
- `SemanticCommandRepresent`: command/name ranges, original argument forms, closure state, block/scope IDs, and an optional shared `Arc<CommandSchema>`.
- `SemanticBlock`: kind, direct command IDs including its own opening/closing commands, child block IDs, parent ID, closure state, and full range. Descendant commands are not duplicated in ancestor membership lists.
- `SemanticScope`: independent scope record associated with a block by `BlockId`, with an optional parent `ScopeId`.
- `SemanticVariable`: name, initial-value placeholder, optional scope ID, and command IDs recording possible writes. Global variables use `None`; scoped variables are distinguished by `(scope, name)`.
- `SemanticSymbol`: a range and an ID reference to a top-level block or command. It does not duplicate commands, blocks, or their children.

IDs are indices into one analysis snapshot, not persistent identities across parses.

## Analysis pipeline

`Semanticer::parse(source, file_id).await` in `src/parser/semanticer/parsing.rs` assembles these stages:

1. `scan_version` in `parsing/version.rs`: select the minimum declared by `cmake_minimum_required(VERSION ...)`; otherwise asynchronously run `cmake --version`. Version ranges select their lower bound.
2. Load versioned schemas and builtin variable records, then overlay constructor-injected schemas.
3. `scan_commands`: convert nparser commands to semantic records and diagnose syntax errors and commands absent from the schema map.
4. `scan_blocks` in `parsing/block.rs`: recognize `if`, `while`, `foreach`, `function`, `macro`, and `block` pairs using builtin keyword commands. Diagnose missing or unexpected ends, invalid branch placement, repeated `else`, and `elseif` after `else`. An ancestor end can recover past unfinished inner blocks; those inner blocks remain unclosed and receive diagnostics.
5. `scan_scopes` in `parsing/scope.rs`: establish function scopes and statically identified variable scopes from `block()`. Plain `block()` creates a variable scope; `SCOPE_FOR POLICIES` alone does not. Macro and control-flow blocks inherit their surrounding variable scope. Opening declarations belong to the surrounding scope; their bodies belong to the newly established scope.
6. `detect_specialized_variables` in `parsing/templates.rs`: generate concrete builtin descriptors from literal triggers, remove unresolved templates from the result, and merge names without duplicates. The public method returns `Vec<SemanticVariable>` and remains usable after parsing.
7. `track_variable_changes` in `parsing/outputs.rs`: traverse semantic commands only, locate schema `Out` arguments, and record writes in the owning scope. It does not revisit nparser nodes or source text.
8. `collect_symbols` in `parsing/symbols.rs`: assemble root block/command references in source order. Nested members remain accessible through the block arena.

## Schema and output matching

- `src/parser/semanticer/schema.rs`: scalar argument roles and `Keyword`, `Optional`, `Repeat`, `Choice`, and `AllOf` grammar nodes.
- `schema/builtin.rs`: lazy static command tables with version additions and replacements; `schemas_for_version` applies the layers.
- `variables/builtin.rs`: lazy static `SemanticVariable` tables; `variables_for_version` applies variable additions. Initial values are unknown/empty placeholders. The raw catalogue retains templates; `template_names_for_version` exposes their versioned names without cloning variable records.
- `parsing/templates.rs`: specialize all nine placeholder families using project/language/package/config/linker/policy triggers, literal feature names, regex capture descriptors, known local function-call arities, and explicit concrete variable names. Combined templates need every placeholder resolved. These are descriptors, not proof of runtime initialization. Dynamic values and cross-file function-call resolution are unsupported.
- `outputs.rs`: `sequence`, `group`, and `step` locate outputs structurally without validating scalar parameter types. `merge` prefers more literal keyword matches at the same position and intersects equally preferred output candidates. Unresolved output names or earlier unquoted expansion/list splitting prevent a guessed write.

## Incremental computation

See [Incremental analysis](../src/parser/INCREMENTAL.md) for the reading order, example, and invalidation rules.

- `src/model/incremental.rs`: UTF-8-safe replacement envelopes, unchanged range mapping, line/sparse Unicode position indexes, and work counters.
- `lexer/incremental.rs`: whole-unit checkpoints retain parenthesis state; `parse_until` shares the full scanner. Relex from before the edit until unchanged suffix/state alignment, or EOF. Tokens and diagnostics retain full-analysis behavior.
- `nparser/incremental.rs`: reuse complete closed-command units; reparse changed/error units through the same `parse_next` implementation. Lexer diagnostics are inherited anew. Cache state is declared in each layer's `model.rs`.
- `semanticer/incremental.rs`: reuse unchanged command payloads/schema Arcs and cached Out matches. Rebuild block/scope links when dense IDs/keyword structure change, and specialized variables when template inputs change. Version declaration changes invalidate all schema-dependent results. Builtin-only incremental caches pin a host-detected version until reconstruction.
- `build_struct/incremental.rs`: cache per-command effect/diagnostic ranges; regenerate selected commands and refresh source references of reused facts. Schema identity prevents reuse across catalog changes.
- `workspace/incremental.rs`: connect per-file caches on the bounded parser worker. Snapshots share semantic/build Arcs instead of cloning whole outputs. `workspace/refresh.rs` retains clean dependency visits/scopes if graph inputs, scope layout, and fact positions are unchanged; otherwise the ordered dependency walk replays.
- Incremental caches expose work counters. Source comparison, output materialization, variable aggregation, and position/ID updates may remain linear. Full parse/analyze APIs remain the reference and are still used by inspection CLIs.
- Differential tests in `tests/unit/parser/incremental.rs` check all four layers against fresh full analysis and assert local reuse. Workspace tests check graph reuse, revision references, immutable old snapshots, and context invalidation.

## Semantic inspection CLI

- `src/parser/semanticer/display.rs`: borrowed, streaming `SemanticedBuffer::display(colored)` adapter. It prints version, root symbols, commands and raw argument forms, blocks, scopes, variables/write IDs, and diagnostics without cloning the analysis arenas. Source-derived strings are debug-escaped; Color formatting uses the existing `owo-colors` dependency with a reusable `Style` palette; no handwritten ANSI escapes.
- `SemanticDisplay::with_all_variables`: opt into unchanged builtin variable records, which are hidden by default to avoid flooding the output.
- `cli/parser/semanticer/args.rs`: positional file or `-`/stdin, `--text`, `--color`, and `--all-variables`.
- `io.rs`: asynchronous input loading. `output.rs`: lexer/parser/semantic analysis and buffered display writing. `main.rs`: CLI assembly and a blocking worker for analysis/rendering; the worker uses the running Tokio handle for asynchronous version detection.
- Exit codes: `0` for clean analysis or broken pipe, `1` when analysis diagnostics exist, and `2` for input/output failures. Clap handles invalid CLI arguments. Color is explicit, disabled by default, and distinguishes headings, IDs, keyword/native/external commands, scopes, variables, outputs, argument forms, closure states, and errors.

## Build structure and Workspace

See [Workspace README](../src/workspace/README.md) for flow, examples, and limits.

- `src/parser/build_struct/model.rs`: temporary borrowed `BuildStructAnalyzer`, owned ordered facts, source references qualified by file/revision, typed projects/targets/source sections/file sets, dependency requests, and possible writes.
- `build_struct/analyze.rs`: native-command dispatch and conditional/deferred ownership; injected schemas can supply Out writes but do not masquerade as native build commands. `project.rs`, `targets.rs`, and `sources.rs` extract supported literal structure and retain raw/unresolved arguments.
- `build_struct/display.rs`: borrowed `BuildStruct::display(colored)` adapter; ordered facts, raw arguments, conditional/deferred/unresolved markers, and optional borrowed semantic diagnostics. `cli/parser/build_struct/{args,io,output,main}.rs` supplies file/stdin/text and explicit colors; it does not load dependencies.
- `workspace/display.rs`: `Workspace::display(colored)` returns `WorkspaceDisplay` with the requested public `DependcyGraphDisplay` field. The field's `DependencyGraphDisplay` adapter draws an ASCII visit tree with `owo-colors`, distinguishes include/subdirectory edges and missing/unresolved/unvisited candidates, and prints scopes and diagnostics. Iterative printing indexes edges once and retains repeated visits; source labels are escaped and repeated snapshot diagnostics are shown once. Pending/stale graphs are not displayed as current results.
- `cli/parser/dependency_graph/{args,io,output,main}.rs`: root file/stdin/text, explicit colors, binary directory, module search paths, and builtin directory; asynchronous input/Workspace flush followed by buffered rendering on a blocking worker. Stdin/text uses `cwd/CMakeLists.txt` for relative dependencies. Exit codes match semantic inspection: 0 clean/broken pipe, 1 diagnostics, 2 I/O errors. No configure is run.
- `src/workspace/model.rs`: documents/snapshots, one root and optional graph per Workspace, visits, directory contexts, variable scopes, graph edges, and limits. Multiple projects require separate Workspaces; root IDs and multi-root maps are removed. Graph facts reference retained immutable snapshots rather than copying declaration payloads. Scope write sets share copy-on-write state.
- `workspace/documents.rs`: normalized absolute file identities, overlays, root-bound construction/module-path settings, snapshots, and release of unused cached snapshots. Symlink aliases are not canonicalized.
- `workspace/parser.rs`: `flush` coalesces pending edits, incrementally updates known files in bounded blocking workers, checks revisions, and refreshes or rebuilds its one graph when project inputs change. A dependency-path set replaces the old reverse-root map. Unchanged semantic snapshots are reused. `flush` is currently serialized through `&mut Workspace`; it is not yet a concurrent LSP actor. Profiles retain standalone minimum-version selection; global catalogs/context generation and cached executable probing remain M3 work.
- `workspace/graph.rs`: iterative source-order dependency traversal, dependency-path tracking, active-cycle/work limits, optional/missing module handling, and include/subdirectory scope differences. Include shares caller scope/source directory; subdirectories copy the caller-visible possible-write state. Local variable blocks get separate scopes. Conditional/deferred dependencies remain candidate edges; arbitrary variable-expanded paths are unresolved.
- Loose open files receive local analysis without root registration. Static module search paths and the builtin module directory are supplied explicitly. Policy options are recorded, not interpreted. Scope state records project names and possible writes, not evaluated values; cache/parent writes are retained as nonlocal facts without pretending they are local writes.

## Deliberate limits

This is static structural analysis, not CMake execution. Variable references are not scanned or diagnosed. Argument validation, runtime values, function/macro call resolution, runtime caller scopes, cache namespaces, `PARENT_SCOPE`, and `PROPAGATE` effects are not implemented. Some schema branches remain explicitly `Unmodeled`. Function-body writes are snapshot-local possible writes, not evidence of a function call.

No CMake-backed document-symbol handler exists yet; `SemanticedBuffer.symbols` is its intended structural input. The reusable server trait and router do declare that protocol method. Analyzer tests are wired through `tests/suite.rs` and `tests/integration.rs`. `tests/unit/parser/{semanticer,build_struct}.rs` cover scopes, templates, diagnostics, source-qualified facts, target kinds, and source file sets; `tests/unit/workspace.rs` covers overlays/revisions, repeated visits, missing candidates, cycles, and limits.

`crates/lspally/tests/suite.rs` loads RPC and duplex integration tests. The explicit 73-method fixture retains script provenance and marks manual test edits. Integration tests cover lifecycle, callbacks, ordering, cancellation, extension routing, EOF, blocked writes, and worker failure. Private conversion tests and performance checks are externally wired from `server/wrapper.rs` without widening adapter visibility. Ignored release-mode checks measure completion adapters, token encoding, and hover round trips; reproduction and timing limits are in `crates/lspally/tests/performance/README.md`.
