> AI-generated content.

# Input-path warnings

Schema roles distinguish InputFile, InputDirectory, InputPath, ModulePath, and
OutputPath. Generic Path has no existence requirement. Command extraction caches
role-matched argument positions through the existing structural schema matcher.

Candidates use command/visit-qualified reads, directory context, or matching
runtime expanded arguments. Earlier unquoted splitting, unknown values, generator
expressions, and ambiguous source matches prevent a guessed path. Static inputs
in conditional/deferred bodies are skipped; observed executions can supply them.

Checks warn for missing paths, unreadable metadata, and file/directory mismatches.
Optional includes suppress missing-path warnings. Existing graph resolution owns
include/subdirectory diagnostics in project/script graphs, avoiding duplicates.
Outputs, known generated files, and configure build directories are excluded.
Known outputs include file writes, configure_file, file(GENERATE), custom command
outputs/byproducts, and source files marked GENERATED. Arbitrary generators or
unmodeled functions cannot be reconstructed statically.

Metadata is checked asynchronously and deduplicated within a refresh. Per-project
results refresh with analysis/save/runtime changes and configuration replacement;
hover and unchanged reports reuse them. Diagnostics enable controls both checks
and publication. No filesystem watcher or general parameter validator is added.
