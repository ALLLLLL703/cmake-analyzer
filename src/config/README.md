> AI-generated content.

# Configuration

Configuration is merged field by field, in this order:

1. `~/.config/cmake-analyzer/cmake-analyzer.toml` (optional).
2. `<analysis-entry-directory>/.cmake-analyzer.toml` (optional).
3. `--config PATH.toml` or the file selected by reloadConfig (required when specified).

Missing fields inherit defaults/lower layers. Arrays replace earlier arrays; interpreter mappings merge by normalized file path. Unknown fields, invalid types, and unsupported interpreter commands are errors. Project settings are cached until explicit reload; no configuration watcher is installed.

```toml
[diagnostics]
enable = true

[diagnostics.rules]
unknown_function = "error"
unknown_variable = "warn"

[document_symbols]
enable = true

[completion]
reserve_arg = false

[format]
on_save = true
indent_width = 2
insert_spaces = true
line_width = 100

[modules]
enable = true
search_paths = ["cmake/modules", "/opt/company/cmake"]
builtin_directory = "/usr/share/cmake/Modules"
external_include = "full"
external_find_package = "full"
package_search_paths = ["/usr/local", "/usr"]

[interpreter_commands]
"tools/check.cmake" = "cmake -P tools/check.cmake"
"cmake/helper.cmake" = "cmake -S . -B build"
```

Diagnostics, document symbols, and module lookup default to enabled. Default module search paths are `cmake`, `cmake/Modules`, and `cmake/modules`, relative to each analysis-entry directory. Explicit `search_paths` replaces this list; `search_paths = []` removes the project-directory defaults. Configured relative module directories remain based on the file defining them. Directories may be missing when configuration is loaded.

When modules are enabled and `builtin_directory` is omitted, the server locates CMake on PATH, resolves its executable symlink, and queries `cmake --version`. It checks `<prefix>/share/cmake-<major>.<minor>/Modules`, then `<prefix>/share/cmake/Modules`. This is best-effort discovery for standard installation layouts; missing CMake, failed detection, or nonstandard layouts leave the builtin directory unset. An explicit `builtin_directory` always wins. No configure, system-information probe, or project script is run.

Discovery results, including failures, are cached once per ConfigStore and retried after explicit configuration reload. Explicit builtin overrides and disabled modules skip discovery. Disabling modules disables module/package lookup, including variable/environment search paths and builtin lookup; literal include paths and subdirectories are still analyzed.

Disabling diagnostics clears previously published reports and suppresses new ones. Disabling document symbols returns null for document queries and omits that owner's buffers from workspace/symbol; capability advertisement remains stable for multiple projects and later reloads. Shared files receive diagnostics from enabled owners.

## External modules and packages

modules.external_include and modules.external_find_package independently accept
full (default), functions, or none. They apply to files outside the analysis entry
directory, including symlinked external files. full performs ordinary conservative
static indexing; functions imports only unconditional top-level function/macro
symbols and their statically resolved dependency declarations, without external
variable/target/value effects. none keeps file navigation but imports no symbols.
Project-local files and add_subdirectory sources retain normal analysis. Explicitly
opened external files remain locally analyzable.

modules.package_search_paths supplies package prefixes or exact config directories.
Defaults are /usr/local and /usr on Unix, empty elsewhere; an explicit array replaces
them, and [] removes these defaults. Relative entries resolve against the defining
configuration file. Static <Package>_DIR/<Package>_ROOT, CMAKE_PREFIX_PATH, environment
paths, and literal HINTS/PATHS also contribute. Find-module search uses module paths,
including known CMAKE_MODULE_PATH values. MODULE/CONFIG/NO_MODULE and
CMAKE_FIND_PACKAGE_PREFER_CONFIG determine entrypoint precedence.

All fields inherit user/project/explicit layers and apply on transactional reload.
Package version/component/toolchain selection is not executed or fully emulated;
see [static discovery and limits](../workspace/packages/README.md). No source downloads
or FetchContent/ExternalProject checkout discovery occur.

## Diagnostic rules

The diagnostics configuration module defines independent unknown_function and
unknown_variable rules. Each accepts error, warn, info, hint, or none. Defaults are
error and warn respectively. unknown_variable controls unset/possibly-unset reads;
known-empty variable warnings retain their ordinary warning level. Resolved imported
functions suppress unknown-command reports regardless of level. Syntax, dependency,
and missing-input diagnostics retain their existing policies.

Rules merge independently through user/project/explicit layers. Unknown rule names,
levels, or types reject reload without changing live settings. Severity changes
republish diagnostics even without a document edit. For shared files, the strongest
level requested by an enabled owner wins; none suppresses a rule only when all enabled
owners disable it. diagnostics.enable remains the master switch.

## Completion

Command-name completion is available outside argument parentheses, including block
bodies. `completion.reserve_arg` is a boolean, default `false`: only the command
name is inserted. Set it to `true` to insert new-call parentheses and snippet
placeholders for known required parameters, provided the client supports snippets.
Existing parentheses/arguments remain unchanged. Optional/unmodeled arguments and
ambiguous overload suffixes are not inserted. This field affects command-call snippets
only; variable/target names and schema keyword completions remain plain text.

Variable names are completed in schema Variable/Out/Unset positions and ordinary
${...} references. Target names are completed in schema target positions with
EnumMember icons. Explicit schema keywords are completed in reachable argument
positions with Keyword icons. These features do not require reserve_arg; arbitrary
values, other scalar enums, and general paths are not completed. ModulePath/PackageName
slots offer discovered module/package names using Module items. Imported library names
use existing Target slots; full-indexed conditional package export files can supplement
these candidates without execution. LanguageType slots offer
version-supported CMake languages as EnumMember items, plus NONE in project.

The field follows user/project/explicit precedence. Missing values inherit; an
explicit `false` overrides a lower-layer `true`. Shared files require all owners
to enable it. Successful explicit reload applies the setting to subsequent
completion queries; invalid types/unknown fields reject reload without changing
live settings. There is no configuration watcher. See [completion](../server/completion/README.md).

## Formatting

`format.on_save` defaults to true and controls pre-save LSP edits for the current
open document. Set false to disable automatic save formatting; explicit formatting
and the project CLI remain available. No save request formats other project files
or writes the editor's disk file directly. Clients need willSaveWaitUntil support
or a pre-save hook.

`indent_width` defaults to 2 columns and accepts 1–255. `insert_spaces` defaults to
true; false selects one tab per indentation level, with indent_width as its tab
stop width. `line_width` defaults to 100 positive Unicode scalar columns; atomic
literals are never split to enforce it. These settings apply to CLI/pre-save
formatting. Explicit LSP formatting uses client tabSize/insertSpaces instead of
configured indentation and keeps the configured line width.

All fields follow user/project/explicit precedence and omission-preserving reload.
Invalid values/types/fields reject reload without changing live settings. Shared
owners must agree on effective style; any owner disabling on_save suppresses save
edits. See [formatting and client setup](../format/README.md).

## Process limits

Set these in user configuration or an explicit override file:

```toml
[limits]
max_open_documents = 256
max_workspaces = 32
```

Both values are positive counts, shared by the entire server process. Project-local settings cannot override them; a file explicitly selected as a global source may contain them. User values inherit named defaults, and explicit values override user values field by field.

Limits are enforced at initialization, document admission, and Workspace creation. Reload applies new limits atomically; values below current open-document or Workspace counts are rejected without evicting anything. Close documents/remove Workspaces before lowering their limits. Internal queue capacity and edit batch size remain named static policies, not configuration fields. Workspace traversal/worker limits are unchanged.

## Interpreter commands

`parser/interpreter_commands` tokenizes commands and returns typed service requirements. It never executes them or expands environment variables, shell substitutions, or `~`. Quoted paths are supported.

- `cmake -S <source> [-B <build>]`: project graph rooted at `<source>/CMakeLists.txt`, with an optional binary-directory context.
- `cmake -P <script>`: script graph rooted at that file, including static dependencies.

Joined options such as `-Ssource` are accepted. Other options, mixed modes, repeated options, and other programs are rejected in this initial implementation. File keys and command paths are relative to the server's startup cwd, not the configuration file. Absolute paths are accepted. Analysis settings follow the selected entry directory.

Explicit file mappings take precedence over ordinary Workspace ownership. Files without mappings retain dependency-first project discovery and standalone-file fallback. No execution is introduced by interpreter mappings. Simple known variable-expanded paths can resolve statically; unknown values and conditional/deferred dependencies remain limited.

## Runtime analysis

```toml
[runtime]
enable = true
mode = "action" # Default; explicit manual action.
executable = "cmake"
# Omit to use <project-source>/.cmake-analyzer/build.
# build_directory = "analysis-build"
arguments = ["-DCMAKE_BUILD_TYPE=Debug"]
timeout_seconds = 120
max_output_bytes = 67108864
max_view_bytes = 65536
```

The default `mode = "action"` runs only after an explicit action. Select **Run
CMake for … (executes project code)**; `enable = false` disables execution and
hides runtime actions. Automatic Code Action requests return none. Unmapped
standalone files cannot run; configured scripts use their script entry.
See [actions](../server/README.md#manual-code-actions).

To authorize automatic configure and mapped script execution, set this in
**user configuration or an explicit `--config` file**, not a project file:

```toml
[runtime]
enable = true
mode = "on_save"
```

A project may disable execution or select `mode = "action"`; a project-local
`on_save` is rejected unless that file was explicitly selected as a global
configuration source. Project settings cannot grant automatic execution when
the effective user/explicit runtime settings disable it. Normal explicit-layer
precedence still applies. Loading/reloading configuration, opening files, and
unsaved edits do not start a run. A subsequent didSave of an open related file
schedules its owning entries. Related files must all be saved; otherwise the
run is skipped and logged. Repeated queued saves coalesce per entry/profile;
a save cancels an affected active run before its replacement starts. Jobs share
max_workspaces, cancellation, timeout, and saved-input checks with manual actions.

Relative executable paths containing directory components and explicit build
directories use the defining configuration file's directory. Bare executable
names use PATH. The omitted build directory is project-relative, not relative
to the user configuration. A source-equivalent build directory and identical
concurrently used build directories are rejected. A dedicated directory is
**not a sandbox**: project code may spawn processes, download, or write elsewhere.

Timeout is positive seconds per run. max_output_bytes is a positive per-run
stderr/trace byte limit; max_view_bytes bounds formatted views and failure
details and variable hover output in bytes (including static hover). Native builtin
help uses only the user/explicit timeout and output limits: seconds per help child,
bytes per captured help/catalog output and total cached help-body bytes. Project
executable/arguments/enable/mode do not affect help inspection; the inspector uses
the startup PATH CMake, and missing help has no fallback. The effective per-project
max_view_bytes bounds combined hover values/comments/help. Successful reload clears
the native help cache without starting help processes. Defaults are named in
model.rs. Successful reload applies
new settings, cancels active runs, and clears runtime results; failed reload
preserves live settings/results.

Arguments allow -D/-U cache settings, -G/-A/-T generator settings, -W warnings,
--toolchain, --fresh, and --no-warn-unused-cli. Source/build, trace redirection,
script/build/install modes, presets, positional arguments, and other flags are
rejected. Values are passed directly, without a shell. Configure requires JSON
trace v1 minor 2 global_frame support.

## Configured script execution

```toml
[interpreter_commands]
"tools/check.cmake" = "cmake -P tools/check.cmake"

[runtime]
arguments = ["-DINPUT=example"]
```

After saving/reloading configuration, open the mapped file and select **Run
CMake script for … (executes project code)**. The explicit mapping wins over
nearest-project and filename inference, even for a file named CMakeLists.txt.
Nearest-project (otherwise document-directory) settings are checked first, then startup-directory
settings for a matching mapping. This also supports nested scripts without a
CMakeLists.txt ancestor. Keys/entry paths remain relative to startup cwd.

Runtime settings come from the profile selecting the script mapping; other
analysis settings still follow the analysis entry directory. The process uses
startup cwd and executes cmake with trace flags, runtime arguments, and -P
followed by the mapped absolute script path. No -S/-B or build directory is used;
runtime.build_directory is ignored. Source/binary directory builtins in the
script Workspace use that cwd, and filename-based implicit-project defaults
are disabled. Scripts may still write anywhere their CMake code permits.

Script runtime arguments permit only -D/-U, -W, and --no-warn-unused-cli.
Configure-only options such as -G and --fresh are rejected before launching.
CMake's own non-scriptable command restrictions still apply. Cancellation,
time/output limits, dynamic include/value/function observations, and edit/reload
invalidation are shared with configure. Configuration discovery never executes
the script. Authorized `on_save` applies to mapped scripts as well as configure.

## Dynamic reload

Send a JSON-RPC request, not a notification:

```json
{
  "jsonrpc": "2.0",
  "id": 10,
  "method": "cmake-analyzer/reloadConfig",
  "params": { "path": "config/local.toml" }
}
```

The path is relative to startup cwd and becomes the highest-priority override for all Workspaces. Use `params: {}` to reread the current sources without switching the explicit file. Success returns `{ "reloaded": true, "workspaces": 2 }` (count varies).

Reload is an ordered service barrier. It stages configuration, reconstructs active Workspace ownership/graphs, and preserves open text/version overlays. Read/parse failures preserve the old sources and live state. Diagnostic publications follow the response. Ordinary edits still use incremental analysis; explicit reload may rebuild all active Workspaces.

Reading order: `model.rs` defines settings/sources; `patch_model.rs` defines partial configuration models; `loader.rs` loads and merges files; `server/configuration.rs` validates RPC parameters; `server/service/configuration.rs` stages and applies reloads.
