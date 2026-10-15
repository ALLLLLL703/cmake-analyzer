> nearly not readable doc(so if you are not Ai(or Si(America)) do not read this)
> AI-generated content.

# Workspace

One Workspace owns one root project, its file results, and one dependency graph. Multiple projects need separate Workspaces. It reads code; it does not configure or build the project.

## Terminal display

`workspace.display(colored)` returns a borrowed `WorkspaceDisplay`. Its public field `DependcyGraphDisplay` can be printed separately. The graph uses ASCII branches and `owo-colors`, with unique visit IDs even when files repeat. Missing, unresolved, conditional, deferred, and unvisited edges remain visible. Snapshot diagnostics appear once per file revision. Pending/stale graphs require `flush()` first.

```sh
cargo run --bin dependency-graph -- CMakeLists.txt --color
cargo run --bin dependency-graph -- CMakeLists.txt --module-path cmake
```

File, `-`/stdin, and `--text` work like the other inspection CLIs. Stdin/text uses `cwd/CMakeLists.txt` as its path. `--binary-directory` and `--builtin-module-directory` are optional. No configure runs. Exit codes: 0 clean/broken pipe, 1 diagnostics, 2 I/O errors.

## Parts

| File | Job |
| --- | --- |
| `../workspace.rs` | Module entrypoint |
| `model.rs` | Files, one root/graph, saved results, visits, scopes, limits, display adapters |
| `display.rs` | Workspace overview, colored ASCII dependency tree, scopes, diagnostics |
| `documents.rs` | Editor text, disk fallback, paths, root-bound construction, queries |
| `parser.rs` | Process pending changes; save results; refresh this project |
| `graph.rs` | Follow includes/subdirectories and track their scopes |

`BuildStructAnalyzer` lives in `../parser/build_struct/`. It reads semantic results and returns ordered facts. Workspace decides which files to load and which facts to apply.

## Main flow

```text
new(root_file) / set_overlay / invalidate_disk
                    |
               pending paths
                    |
                 flush()
                    |
       editor text, otherwise disk text
                    |
     Lexer -> NParser -> Semanticer -> BuildStruct
                    |
          save this file's results
                    |
      visit dependencies in command order
                    |
           replace this project's graph
```

- `Workspace::new(file, binary_directory)` requires one root file, not a directory. `with_limits` also accepts job limits. No root IDs or multi-root tables are needed; replacing a project means creating another Workspace.
- `set_overlay` replaces editor text and bumps its version. Several edits before `flush` leave only the newest text to parse.
- `close_overlay` returns to disk text; it does not remove project membership.
- `flush` also parses open files with no root. These get local results, not a guessed project.
- Disk reads are async. Parsing runs on a worker thread; its CMake-version query uses the existing Tokio runtime.
- A semaphore caps worker count. **Today each flush waits for jobs in order; it is not a concurrent LSP manager.** Edits cannot enter the same Workspace while its `&mut` flush is running.

## Saved results, not copied files

`DocumentSnapshot` holds text, `SemanticedBuffer`, and `BuildStruct`. Readers share it through `Arc`; later edits create a new result rather than changing the old one.

An unchanged file reuses its saved result. A changed file reparses fully; this is not token-level incremental parsing. Source IDs include file/version because command IDs only identify entries in one result.

A **visit** means reading a file at one point in a root's command sequence:

```cmake
include(extra.cmake)
include(extra.cmake)
```

These share one parse result but create two visits. Each has its caller and scope. `RootGraph.facts` stores `(visit index, fact index)` instead of copying each BuildStruct.

## Visit order and scopes

`graph.rs` uses three stack steps:

```text
Enter file -> Resume next fact -> Exit file
                  |
             dependency found
                  |
       visit child -> resume caller
```

The child is read before the caller's next command. Files stay on the active call path until their children finish. `A -> B -> A` is reported as a loop; two later visits to B are allowed.

| Command | Variable scope | Source directory |
| --- | --- | --- |
| `include(file)` | Caller scope | Unchanged |
| `add_subdirectory(dir)` | New child scope | Child directory |
| Variable-scoping `block()` | New local scope | Unchanged |

Example:

```cmake
set(parent yes)
include(extra.cmake)       # extra's writes join the caller scope
add_subdirectory(child)   # child starts with the names known here
set(after yes)            # this later write does not reach the child
```

Scopes track project names and **possible write names**, not variable values. Shared name sets are copied only when one side changes. Local scope IDs are mapped separately for every visit.

Facts inside conditions, loops, functions, and macros are kept but not applied. Their dependency edges remain unresolved candidates. Cache/parent writes are also kept without guessing their destination.

## Paths and updates

- Paths become absolute; `.` and `..` are cleaned without requiring the file to exist. Symlink aliases still get separate IDs.
- Relative file includes use the current source directory, not automatically the included file's directory.
- Bare module names search supplied module paths, then the supplied builtin module directory. A builtin caller reverses that order. Workspace does not yet discover these paths from variable values or the executable.
- Missing paths and earlier module-search choices remain recorded. If a missing module appears, `invalidate_disk(path)` can refresh its callers. Path setters mark this root for refresh.
- A set holds this project's files and searched paths. If any changes, `flush` rebuilds the graph and replaces the set; removed includes stop triggering rebuilds. Unchanged file results are reused.
- `snapshot(path)` returns results matching the known document version. Call `flush` to refresh disk changes. `root_graph()` returns `None` while that root's inputs/search paths are pending or out of date.
- Unused cached results are dropped; file IDs and results already held by readers remain valid.

Default limits: 2 workers, 256 loaded files per root pass, depth 64, and 1024 file-entry attempts. Cycles, limits, and read failures become graph diagnostics. A missing `OPTIONAL` include is allowed. Parser diagnostics stay in `semantics`; extraction diagnostics stay in `build`.

## Usage

Inside an async Tokio caller:

```rust
use cmake_analyzer::workspace::Workspace;

let mut workspace = Workspace::new("demo/CMakeLists.txt", None)?;
workspace.set_overlay(
    "demo/CMakeLists.txt",
    "cmake_minimum_required(VERSION 3.31)\nproject(Demo)",
)?;
workspace.flush().await?;
let file_result = workspace.snapshot("demo/CMakeLists.txt")?;
let project_links = workspace.root_graph();
```

## Current limits

No CMake execution beyond version detection, variable evaluation, policy execution, cross-file function calls, automatic root discovery, membership actions, or File API import. Version/schema selection still uses the standalone Semanticer rules. Global catalogs and concurrent editor scheduling are later work.
