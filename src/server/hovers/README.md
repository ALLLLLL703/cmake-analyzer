> AI-generated content.

# Hover documentation

- `targets.rs` locates variable names, command names, and function/macro declaration
  names using the negotiated position encoding on a joined worker.
- `variables.rs` retains Static:/Observed: values and adds bound source comments
  and builtin variable help. Indexed bindings take precedence over file-local
  fallback; unrelated or later declarations are not guessed by name.
- `commands.rs` resolves function/macro definitions through local, Workspace, or
  observed runtime calls before considering builtin command help. User-defined
  commands that shadow builtins do not receive the builtin's documentation.
- `documentation.rs` resolves comment ranges in their owning snapshot/Workspace.
- `include.rs` is the generated compile-time index for cmake_builtin_docs.
- `builtins.rs` handles exact/template lookup; `render.rs` shares UTF-8-safe output
  bounds. The client's first supported hover format selects Markdown or plaintext.

Line and bracket comments immediately preceding function, macro, or set commands
form source documentation. Consecutive comments combine; blank lines, intervening
code, or a trailing comment break attachment. Comment text is read from recorded
TextSpans only when needed for hover. For example:

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

Hover never executes CMake. Values and documentation together share the configured
runtime.max_view_bytes budget; runtime.enable does not disable static help.
