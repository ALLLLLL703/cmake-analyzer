> AI-generated content.

# Definition and references

- `definition.rs` and `references.rs` implement the LSP entrypoints.
- `mod.rs` sends cancellable requests through the ordered service actor.
- `query.rs` captures each owner's immutable reference index and source snapshots.
- `locations.rs` matches half-open name spans, collects bound symbol locations,
  deduplicates URI/range pairs, and converts bytes with the negotiated encoding.
- `model.rs` keeps request kinds and owner-qualified worker inputs.

Project queries use RootGraph.references without cloning its records. File IDs
are resolved only within that owner's source map. Shared files can return several
possible definitions from different projects or visits. Queries wait for preceding
edits; conversion runs on a joined blocking worker and checks cancellation.
Closed-file targets use encoded file URIs; open files retain their client URI.

Standalone queries replay the existing reference collector on the worker. This
retains deferred macro/parameter environments that the flattened variable view
cannot distinguish. No second permanent reference index is stored per file.

Definitions use the index's optional definition, not a name-only search or nearest
assignment guess. Missing definitions return null. References return an empty
array for an unbound target and honor includeDeclaration; repeated visits do not
repeat physical locations. Writes at a repeated declaration span are excluded
when declarations are excluded.

Used native commands have FunctionKind::Builtin, no definition/body block, no
formal parameters, and Call references. Names are case-insensitive; known user
shadowing remains separate. Unused builtins are not materialized. Native commands
therefore support references but have no source definition target.

Navigation uses static reference relations. It does not execute CMake, infer
cache/nonlocal bindings, synthesize trace-only links, or add include-path navigation.
