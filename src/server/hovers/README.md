> AI-generated content.

# Hover documentation

- `targets.rs` locates variable names, command names, and function/macro declaration
  names using the negotiated position encoding on a joined worker.
- `variables.rs` retains Static:/Observed: values and bound source comments.
- `commands.rs` resolves local, Workspace, and observed function/macro definitions
  before considering native command help. User shadows do not receive builtin help.
- `documentation.rs` resolves comment ranges in their owning snapshot/Workspace.
- `builtins.rs` caches local help; builtins/model.rs, names.rs, and process.rs hold
  its state, exact/template lookup, and bounded asynchronous child handling.
- `render.rs` shares UTF-8-safe output bounds and preserves native reStructuredText
  as preformatted text when the client prefers Markdown.

## Local builtin help

On demand, resolve cmake from the server startup PATH and query
--help-command-list / --help-variable-list, then --help-command NAME or
--help-variable NAME. No fixed installation path, HTML package, embedded manual,
network fallback, or repository-generated documentation is used. Exact variable
names precede template matches. Function argument variables reuse the native
function page. Missing help simply contributes no builtin documentation; source
comments and values remain available.

The service caches catalogs and canonical help bodies, including unavailable
results, until a successful reloadConfig. Unknown source spellings are not cached.
Only user/explicit runtime.timeout_seconds and runtime.max_output_bytes control
help inspection: timeout is per child; the byte limit bounds each captured output
and the total cached help-body bytes. Failed/oversized catalog captures are not
partially indexed. Documents that exceed the remaining cache budget can still be
shown without caching. Project runtime.executable, arguments, mode, and enable do
not select or authorize this inspector. stdin/stderr are null; timeout, overflow,
and cancellation kill/reap the direct child. No project configure/script is run.

Values and documentation together share the effective runtime.max_view_bytes
budget. Runtime.enable does not disable source or builtin help. Failed reloads
preserve the old cache; successful reloads discard it lazily without querying help.

## Source comments

Line and bracket comments immediately preceding function, macro, or set commands
form source documentation. Consecutive comments combine; blank lines, intervening
code, or a trailing comment break attachment. Comment text is read from recorded
TextSpans only when needed. For example:

```cmake
# Location of application resources.
set(RESOURCE_DIR "${CMAKE_CURRENT_SOURCE_DIR}/resources")

#[=[
Loads the application's resource definitions.
]=]
function(load_resources)
endfunction()
```

Semantics attach shared comment-range records to commands, function/macro records,
and set-derived VariableChanging entries. Comment-only edits rebind these ranges
without requiring command payload reparsing. Static reference bindings retain
existing limits for deferred execution, cache fallback, and nonlocal writes;
missing bindings do not invent documentation from an unrelated declaration.
