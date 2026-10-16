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

[document_symbols]
enable = true

[modules]
enable = true
search_paths = ["cmake/modules", "/opt/company/cmake"]
builtin_directory = "/usr/share/cmake/Modules"

[interpreter_commands]
"tools/check.cmake" = "cmake -P tools/check.cmake"
"cmake/helper.cmake" = "cmake -S . -B build"
```

All feature switches default to enabled. Default module search paths are `cmake`, `cmake/Modules`, and `cmake/modules`, relative to each analysis-entry directory. Explicit `search_paths` replaces this list; `search_paths = []` removes the project-directory defaults. Configured relative module directories remain based on the file defining them. Directories may be missing when configuration is loaded.

When modules are enabled and `builtin_directory` is omitted, the server locates CMake on PATH, resolves its executable symlink, and queries `cmake --version`. It checks `<prefix>/share/cmake-<major>.<minor>/Modules`, then `<prefix>/share/cmake/Modules`. This is best-effort discovery for standard installation layouts; missing CMake, failed detection, or nonstandard layouts leave the builtin directory unset. An explicit `builtin_directory` always wins. No configure, system-information probe, or project script is run.

Discovery results, including failures, are cached once per ConfigStore and retried after explicit configuration reload. Explicit builtin overrides and disabled modules skip discovery. Disabling `modules` disables both custom/default search directories and builtin lookup; literal include paths and subdirectories are still analyzed.

Disabling diagnostics clears previously published reports and suppresses new ones. Disabling document symbols returns null; capability advertisement remains stable for multiple projects and later reloads. Shared files receive diagnostics from enabled owners.

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
