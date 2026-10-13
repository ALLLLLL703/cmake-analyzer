> AI-generated content.

# BuildStruct + Workspace

Approved: M1 + M2 + Semanticer template detection. M3–M5 await review.

## Flow

```text
Workspace -> Lexer -> NParser -> Semanticer
                                  |
                         BuildStructAnalyzer
                                  |
                             BuildStruct
                                  |
                   Workspace graph + scope bindings
```

- `BuildStructAnalyzer<'a>` temporarily borrows `&SemanticedBuffer`.
- `BuildStruct` owns one file's facts, not the whole project graph.
- One Workspace owns one root, one graph, documents, scopes, catalogs, and jobs. Multiple projects use separate Workspaces.
- Analyzer returns dependency requests; Workspace loads files.

## Files

Create modules only when needed. Models stay in `model.rs`.

| Path | Work |
| --- | --- |
| `src/parser/build_struct.rs` | Module wiring |
| `build_struct/model.rs` | Analyzer, facts, effects, source IDs |
| `build_struct/analyze.rs` | Scan commands in order |
| `build_struct/project.rs` | Project, languages, minimum version |
| `build_struct/targets.rs` | Targets, aliases, target changes |
| `build_struct/sources.rs` | Sources, headers, file sets |
| `build_struct/dependencies.rs` | Include/subdirectory requests |
| `src/workspace.rs` | Public entrypoints |
| `workspace/model.rs` | Documents, one root/graph, contexts, snapshots |
| `workspace/documents.rs` | File IDs, disk text, editor overlays |
| `workspace/parser.rs` | Job queue, workers, result publication |
| `workspace/graph.rs` | Dependencies, callers, target indexes |
| `workspace/context.rs` | Apply ordered effects to scopes |
| `workspace/registry.rs` | Shared schemas, builtin catalogs, triggers |
| `workspace/discovery.rs` | Root registration/suggestions |
| `workspace/queries.rs` | Read-only graph/snapshot queries |
| `workspace/actions.rs` | Membership edit proposals |
| `workspace/cmake.rs` | Later: explicit configure + File API |

Paths below `build_struct/` and `workspace/` are under their respective parent directories.

## BuildStruct

- Scan existing semantic commands; do not reparse text or clone the buffer.
- Record `project`, `enable_language`, and `cmake_minimum_required` fields.
- Record executable/library/custom targets, imported targets, and aliases.
- Record `target_sources`, header file sets, and raw link items.
- Record `include`/`add_subdirectory` requests and their command positions.
- Each fact has a source reference, local block/scope ID, and conditional/deferred status.
- Source references include file, revision/profile generation, and local command ID.
- Own small extracted payloads. Workspace may share immutable semantic snapshots with `Arc`.
- First support literal arguments. Keep dynamic names/paths/expressions unresolved.
- Do not execute function/macro bodies or assume conditional targets exist.
- Resolve relative sources with the declaring source-directory context, not the editor's directory.
- Headers mean CMake declarations, not C/C++ `#include` analysis.
- Keep imported-target visibility; do not merge every name into one global namespace.

## Global parser manager

1. Create Workspace with its root file; update document revisions. Open text overrides disk text.
2. Select shared schemas; read files asynchronously.
3. Run parsing/extraction in bounded blocking workers; coalesce queued edits.
4. Publish only results matching the current revision and context generation.
5. Apply supported effects in source order; load static dependencies.
6. Rebuild the graph when a project dependency changes; replace the dependency-path set.

- Cache syntax by file revision; semantics by revision/profile; applied effects by invocation context.
- Reuse syntax for repeated includes, but keep separate invocation contexts.
- `include`: caller variable scope. `add_subdirectory`: new child directory scope.
- Policy scope is separate from variable scope.
- Follow unconditional, non-deferred requests first; retain other edges as candidates.
- Bound workers, traversal depth, file count, and invocation work. Detect active traversal cycles.
- Aborting a blocking task does not stop running work; discard stale results.
- Closing an editor removes its overlay, not a file still used by a root.
- Initial bindings track unknown values/writes; full evaluation, cache, `PARENT_SCOPE`, and `PROPAGATE` come later.

## Builtin registration

- Registry shares immutable descriptors; mutable bindings belong to scopes.
- Extract typed triggers from commands; Workspace applies them in order. No event framework needed.
- Bootstrap command schemas first; project/language triggers then supply contextual variables.
- Preserve standalone `Semanticer::parse` and CLI behavior through an explicit compatibility path.
- Detect the configured executable once per generation, not once per file.
- Separate executable version, declared minimum, and effective policy version.
- Current code selects schemas by minimum version: approve migration before changing it or `version` fields.
- Approved template detection is implemented in Semanticer: expand known static domains and omit unresolved templates. Never register partially expanded names as concrete variables.
- Shared catalogs + sparse bindings avoid cloning all builtin variables into every context. Review the existing `variables` vector adapter.

## Loose files and CMake execution

- Attached files get project context; loose files retain syntax/local symbols.
- Root membership follows explicit registration/dependencies, not folder ancestry alone.
- Actions: register a root, add a child `CMakeLists.txt` via `add_subdirectory`, or add a module via `include`.
- User chooses the destination; check its revision and existing declarations before editing.
- Configure only on explicit request for a trusted project, using a chosen build directory/preset.
- Use CMake File API; keep configured results separate from static candidates.
- Configure runs disk text, not unsaved overlays. Cancellation/build-directory isolation is not a script sandbox.

## Order and review

- [x] M1: models + literal BuildStruct extraction.
- [x] M2: one root/documents + bounded parser jobs + static dependency graph/scopes (serialized `flush`; concurrent LSP scheduling remains later work).
- [x] Semanticer: nine template families; return concrete descriptors and omit unresolved template names.
- [ ] M3: shared catalogs + version split + contextual builtin triggers.
- [ ] M4: graph queries + membership actions; LSP adapts these later.
- [ ] M5: optional explicit configure/File API import.

Review: ownership/IDs, loose-file mode, supported overloads, version migration, variable storage, template migration, and permission for new tests.

Keep Out-only variable tracking; no general reference scan. Reuse dependencies. Run existing tests, formatting, compilation, and smoke checks; persistent new tests need approval. Cover scope differences, repeated includes, stale results, removed edges, conditional targets, and independent Workspaces. Update `agent-instructions/Architect.md` when code changes, not with unimplemented plans.
