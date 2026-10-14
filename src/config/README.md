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

All feature switches default to enabled. Builtin and custom module directories default to empty. Relative module directories are based on the file defining them. Disabling `modules` disables these configured search directories; literal include paths and subdirectories are still analyzed. Builtin modules are not discovered automatically. Directories may be missing when configuration is loaded.

Disabling diagnostics clears previously published reports and suppresses new ones. Disabling document symbols returns null; capability advertisement remains stable for multiple projects and later reloads. Shared files receive diagnostics from enabled owners.

## Interpreter commands

`parser/interpreter_commands` tokenizes commands and returns typed service requirements. It never executes them or expands environment variables, shell substitutions, or `~`. Quoted paths are supported.

- `cmake -S <source> [-B <build>]`: project graph rooted at `<source>/CMakeLists.txt`, with an optional binary-directory context.
- `cmake -P <script>`: script graph rooted at that file, including static dependencies.

Joined options such as `-Ssource` are accepted. Other options, mixed modes, repeated options, and other programs are rejected in this initial implementation. File keys and command paths are relative to the server's startup cwd, not the configuration file. Absolute paths are accepted. Analysis settings follow the selected entry directory.

Explicit file mappings take precedence over ordinary Workspace ownership. Files without mappings retain dependency-first project discovery and standalone-file fallback. No general CMake evaluation is introduced: variable-expanded paths and conditional/deferred dependencies retain the existing static-analysis limits.

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
