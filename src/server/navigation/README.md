> AI-generated content.

# Definition and references

- `definition.rs` and `references.rs` implement the LSP entrypoints.
- `dependencies.rs` handles native include/module and add_subdirectory source paths.
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
therefore support references but their command names have no source definition target.

## Dependency paths

Definition on include's first argument opens its resolved module/file. Definition
on add_subdirectory's first argument opens the source directory's CMakeLists.txt,
not its binary directory. Targets use an empty range at file start. Native command
names, later arguments/options, and names inside ${...} keep symbol navigation.
Known user shadows are not treated as imports.

Resolved dependency edges preserve each visit's source directory. Literal paths
in conditional/deferred commands and standalone files reuse Workspace module
precedence through resolve_dependency without loading/traversing their contents.
Installed runtime dependency observations can supply executed dynamic targets;
navigation never starts a run. Unknown expressions, missing files, and directories
without CMakeLists.txt have no target. Open unsaved target files remain usable.
Shared callers retain distinct directory contexts; repeated target URIs are deduplicated.

Navigation does not execute CMake, infer cache/nonlocal bindings, or synthesize
runtime-only variable/function links.
