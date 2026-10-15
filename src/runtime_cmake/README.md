> AI-generated content.

# Explicit CMake runtime analysis

`execute` selects Configure or Script from ConfigureOptions.mode (`configure`
remains an alias). Configure uses -S/-B; Script uses -P with an absolute entry
file and source_directory as cwd, ignoring build_directory. Both enable
--trace-expand and --trace-format=json-v1. Neither builds targets. JSON trace v1 minor 2
(`global_frame`) is required; older trace formats are rejected.

- `model.rs`: typed run/event IDs, frame depth, configure options, trace events,
  and observed include/subdirectory targets.
- `process.rs`: asynchronous child execution, cancellation, timeout, bounded
  stderr, bounded failure details, and child reaping.
- `trace.rs`: blocking JSON decoding, declaration-aware command classification,
  and execution-order dependency discovery.
- `../workspace/runtime/`: immutable execution-derived Workspace data.
- `../server/action/`: shared configure/script action entrypoints.
- `../server/service/automatic.rs`: optional, user/explicit-authorized on-save scheduling; default execution remains manual.

CMake receives no shell interpolation. It uses the source directory as cwd,
null stdin/stdout, and captured stderr. The stderr limit includes JSON trace,
warnings, and ordinary messages. Only successful execution/trace results can
be installed. Recognized command events with malformed required fields fail the run; other
stderr lines are ignored. Cancellation kills
and reaps the direct CMake child; it is not a portable descendant-process-tree
sandbox.

Frame depth is not a variable-scope ID. Repeated events retain distinct event
IDs, even for the same file, command, and frame depth. Trace arguments expose source-argument expansion before unquoted list splitting
and empty-argument removal, not always the final logical parameter list. They
do not expose every variable at that point.

The Workspace layer records ordinary `set`/`unset` events, unknown cache/parent
assignments, forced-cache values, function/macro declarations/calls, and source
reads. Assignments affected by opaque unquoted lists/empty arguments retain an
unknown namespace/value rather than interpreting a raw trace string as the
assigned value. A whole quoted `"${NAME}"` argument can prove a read value when earlier
arguments cannot shift its position. Other reads remain `None`. Environment
writes, other command mutations, arbitrary scope propagation, and complete
cache state are not reconstructed. Observations must not be carried forward
across unmodeled commands as if they were a full variable store.

Include/subdirectory targets use the next deeper trace event where available.
Successful required empty-file includes/subdirectories can also use their
expanded file paths and directory context. Empty optional/module-name includes
without trace evidence remain unresolved. Missing temporary sources retain raw
trace events without invented source locations. Ambiguous same-line command
matches also have no source `CommandId`. Events retain shared source-directory
context, and RuntimeLayer.source_commands maps event IDs to optional source
commands for path checks without repeated source scans.
