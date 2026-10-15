> AI-generated content.

# Incremental analysis

Read `src/workspace/incremental.rs` first. It connects four caches:

```text
new source
  -> IncrementalLexer.update()
  -> IncrementalNParser.update()
  -> IncrementalSemanticer.update()
  -> IncrementalBuildStruct.update()
  -> Workspace graph refresh or replay
```

The original `parse()` / `analyze()` functions remain full-analysis references. Inspection CLIs still use them. Workspace uses incremental caches automatically after the first snapshot.

## One ordinary edit

Change `set(A one)` to `set(A two)` between other commands:

1. Lexer resumes at a whole-unit checkpoint before the replacement. It reuses the suffix only after byte alignment, parenthesis depth, and relevant opening-parenthesis location agree.
2. Nparser reparses that command. Unchanged closed commands have no parsing dependency beyond their own token interval.
3. Semanticer reuses other commands and their Out argument matches. The declaration name `A` did not change, so template specialization and block/scope structure can be reused.
4. BuildStruct extracts that command's effects again. Reused effects receive the new revision, command ID, span, block, and scope references.
5. The graph's declared write names are unchanged. Workspace keeps visits/scopes and updates snapshot and edge references instead of resolving dependencies again.

Reference spans stored on reused commands are remapped with their command IDs. Function inventories and local reference/warning binding are rebuilt after command reuse; Workspace rebuilds its contextual reference index on a bounded blocking worker even when graph visits are reused. These aggregate passes can remain linear. See [References](semanticer/REFERENCES.md).

`DocumentSnapshot.work` reports recomputed/reused units at each layer. Lexer counts emitted tokens, nparser counts parsing units, and semantic/build caches count commands. Workspace exposes `graph_rebuilt` and `reused_invocations` for the latest flush. These are work counters, not elapsed-time measurements.

## Invalidation

| Change | Work that expands |
| --- | --- |
| Quote, bracket delimiter, escape, comment | Lexer may scan to EOF; parser errors are rebuilt |
| Command insertion/deletion or structural keyword | Rebuild dense command/block/scope links; unchanged command payloads and Out matches can still be reused |
| Project/language/package/config/feature/template trigger | Rebuild specialized variable declarations |
| Minimum-version declaration | Rebuild all schema-dependent semantic and build results |
| Project/write/dependency graph input, scope layout, fact count/order | Replay the dependency graph |
| Module search settings or missing/failed dependency | Replay dependency resolution |

A host CMake version detected without a declaration is pinned to the semantic cache lifetime. Create a new cache to detect environment changes. Incremental semantic construction currently uses builtin schemas; the original Semanticer remains available for injected schemas.

## Source mapping and ownership

`src/model/incremental.rs` finds one UTF-8-safe replacement envelope between snapshots. Unchanged ranges map to the new source; overlapping ranges cannot be reused. Positions are recalculated with line starts and a sparse multibyte-character index. Columns are Unicode scalar counts, not LSP UTF-16 units.

Workspace snapshots share `Arc<SemanticedBuffer>` and `Arc<BuildStruct>` with their caches. Old snapshots remain unchanged. Token/node materialization, argument/result copies for owned output models, position mapping, variable write-list assembly, and dense-ID relinking can still be linear. This is incremental computation, not an O(edit-size) guarantee.

## Correctness checks

`tests/unit/parser/incremental.rs` compares tokens, errors, parser nodes, complete semantic output, and build facts against fresh full analysis. It covers Unicode, malformed delimiters, comments, structural/template/version changes, and character replacements/deletions. Reuse assertions prevent a full reparse from masquerading as incrementality.

`tests/unit/workspace.rs` verifies local-edit reuse, old-snapshot immutability, refreshed edge revisions, and graph replay after an inherited write-name change.
