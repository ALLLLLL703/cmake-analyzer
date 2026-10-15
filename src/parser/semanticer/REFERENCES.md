> AI-generated content.

# References, functions, and warnings

`reference_model.rs` defines the shared records, reexported through `semanticer::model`:

- `ReferenceLocation`: FileId, CommandId, name span, and Definition/Read/Write/Call kind.
- `ArgumentReference`: variable name, argument index, and location; stored on each SemanticCommandRepresent before binding.
- `SemanticVariable.references`: local definitions, reads, and writes. `builtin` distinguishes catalogue descriptors from source variables; `first_write` remains the existing possible Out-write range, not proof of initialization.
- `SemanticFunction`: function/macro name and kind, block/scope, formal parameters, definition, and references including the definition.
- `SemanticWarn`: variable name and PossiblyUnset/Unset/Empty classification, stored as Vec<Spanned<SemanticWarn>> on SemanticedBuffer.
- `UnresolvedReference`: name, target kind, and location when binding is unavailable.

ReferenceEnvironmentId identifies a collector environment; ReferenceInvocationId identifies an owning RootGraph visit. Private indexed-variable/function IDs and snapshot-variable/function IDs are separate newtypes, not interchangeable integer aliases. Arena access converts through index accessors. Call/write clocks, argument positions, and byte offsets are numeric values rather than identities.

Positions cover names, not whole commands. Variable expansions select the name inside `${...}`. Quoted/bracket literal names select the payload. FileId is Workspace-local; CommandId and spans belong to the associated snapshot. Retain the owning buffer/RootGraph when retaining locations. Different Workspaces do not share an ID namespace.

## Collection

`parsing/references.rs` scans original argument text during command extraction. It recognizes static `${NAME}` expansions and schema-modeled literal Variable arguments. Escapes and bracket arguments suppress expansion. Nested dynamic names are not guessed; their static inner expansions can still be recorded. ENV/CACHE namespaces are not treated as ordinary variables. Bare conditions are not guessed from arbitrary strings.

The schema matcher shares its grammar traversal for Out and Variable roles. Function/macro formal parameters are declarations, not bare reads. `parsing/functions.rs` collects function/macro definitions after block/scope parsing. `references.rs` then binds source-order references and records literal state facts; `references/model.rs` contains its private analysis state.

Formal parameters have unknown values. Normal literal empty assignments and unset operations can establish Empty/Unset; dynamic values stay unknown. Existing known bindings with unknown values are not reported as missing. Missing bindings and branch-dependent declarations produce PossiblyUnset warnings. Branch merges discard conflicting value facts. Reads through parent bindings remain visible; child writes do not change the parent scope, and removing a child binding reveals its parent.

Function/macro bodies are indexed in isolated deferred environments, not executed. Later declarations are potential targets for unresolved deferred calls. User calls invalidate known value facts without interpreting their bodies. Known list/string mutations also discard previous literal value facts. Cache/environment writes are excluded from ordinary bindings; PARENT_SCOPE writes remain unresolved locations. Runtime caller scopes, cache fallback, arbitrary expansion, and general CMake evaluation are not implemented.

## Workspace aggregation

`RootGraph.references` is a ReferenceIndex rebuilt by `workspace/references.rs` over resolved dependency visits in command order. Includes share the caller environment; subdirectories fork it. Repeated visits retain separate contextual warnings while symbol reference locations are deduplicated. The index owns aggregate records rather than mutating shared document snapshots.

Local buffers retain local warnings. Project diagnostic publications use contextual warnings for visited files and suppress local unknown-command errors when an external declaration resolves the call. Files without a graph visit use local warnings. Warnings are LSP severity 2 and follow the diagnostics enable switch. Inspection displays include warnings; warnings alone do not produce an error exit status.

Command extraction/reference spans are reused and remapped by incremental semantic analysis. Function inventories, local binding/warning results, and the Workspace reference index are rebuilt for affected analysis updates. Index rebuilding runs on a bounded blocking worker even when dependency visits are reused. This is not edit-size-only or per-reference incremental computation. The release benchmark and recorded before/after measurements are in [Reference analysis benchmark](../../../tests/performance/REFERENCES.md).
