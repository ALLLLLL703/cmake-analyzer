> AI-generated content.

# Definition and references

- `definition.rs` and `references.rs` implement the LSP entrypoints.
- `dependencies.rs` handles native include/module, find_package entrypoints, and add_subdirectory source paths.
- `targets.rs` adapts TargetDeclaration.references to shared symbol-location rendering.
- `mod.rs` sends cancellable requests through the ordered service actor.
- `query.rs` captures each owner's immutable reference/target indexes and snapshots.
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
cannot distinguish. No second permanent variable/function reference index is stored per file.

Variable/function definitions use the index's optional definition, not a name-only search or nearest
assignment guess. Missing definitions return null. References return an empty
array for an unbound target and honor includeDeclaration; repeated visits do not
repeat physical locations. Writes at a repeated declaration span are excluded
when declarations are excluded.

Used native commands have FunctionKind::Builtin, no definition/body block, no
formal parameters, and Call references. Names are case-insensitive; known user
shadowing remains separate. Unused builtins are not materialized. Native commands
therefore support references but their command names have no source definition target.

## Build targets

TargetDeclaration.references stores TargetReference records with snapshot-qualified
SourceRef, raw argument span, and Definition/Read kind. BuildStruct contains file-local
bindings; RootGraph.targets contains owner-qualified declaration copies with cross-file
references. Immutable source/argument arrays are shared, not copied per owner.

Closed calls use explicit Target/TargetExe/TargetLib/CustomTarget schema positions.
Full-call matching intersects equally preferred branches rather than using completion's
prefix union. Position-qualified simple expansions can establish earlier slot counts;
unknown splitting or empty removal suppresses later bindings. Native command shadows
do not inherit native target roles. Only known
literal declarations and literal operands bind; ordinary values, file names, external
link items without declarations, unknown expansions, and ambiguous names do not.

Regular and GLOBAL imported targets are project-visible. Nonglobal imports are visible
in their declaring CMake directory and descendants. Includes retain the caller directory,
including repeated visits to the same physical file. Repeated global alias declarations at the same source site merge.
Aliases have separate definitions;
the original-target operand references the original declaration. Aliases of nonglobal
imports retain directory visibility; aliases of known global targets are global.

Navigation selects argument payloads without quotes/bracket delimiters, honors
includeDeclaration, and keeps names inside ${...} on variable navigation. Cached local
references are rebound after incremental edits; graph aggregation never mutates shared
snapshots. Shared-owner results remain qualified and deduplicate physical locations.

Top-level condition TARGET operands also use the schema target role. These are static
structural candidates, not proof of execution. Dynamic target names, generator-expression
target uses, general recursive condition parsing, IMPORTED_GLOBAL promotion, and runtime
caller-directory evaluation are not modeled.

## Dependency paths

Definition on include's first argument opens its resolved module/file, including
unfinished calls and first quoted/bracket operands recovered from lexer errors.
Definition on find_package's first argument opens a statically resolved Find module
or config entrypoint. Module/config preference and search limits are described in
[package discovery](../../workspace/packages/README.md). File jumps remain available
when external symbol importing is disabled.
Definition on add_subdirectory's first argument opens the source directory's CMakeLists.txt,
not its binary directory. Targets use an empty range at file start. Native command
names, later arguments/options, and names inside ${...} keep symbol navigation.
Known user shadows are not treated as imports.

Resolved dependency edges preserve each visit's source directory. Literal paths
in conditional/deferred commands and standalone files reuse Workspace module
precedence through resolve_dependency_at without loading/traversing their contents.
Installed runtime dependency observations can supply executed dynamic targets;
navigation never starts a run. Unknown expressions, missing files, and directories
without CMakeLists.txt have no target. Open unsaved target files remain usable.
Shared callers retain distinct directory contexts; repeated target URIs are deduplicated.

Navigation does not execute CMake, infer cache/nonlocal bindings, or synthesize
runtime-only variable/function links.
