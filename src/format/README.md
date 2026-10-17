> AI-generated content.

# CMake formatting

`format::render` consumes source, LexedBuffer, and SemanticedBuffer from the same
immutable analysis snapshot. It uses semantic command/block structure, not the debug
display serializer. Raw argument and comment lexemes are copied unchanged.

- `layout.rs`: normalize command/argument spacing and indent control/function blocks.
  Calls fitting line_width stay on one line. Long calls, multiline arguments, and
  internal comments use one argument per continuation line with a separate close.
  Nested parentheses receive nested continuation indentation.
- `trivia.rs`: retain comments, attachment, and surrounding blank lines. Literal
  contents and comment bodies are never wrapped or reindented internally.
- `writer.rs`: track columns/tabs, use the first source newline style (LF/CRLF),
  and end nonempty output with one final newline.
- `model.rs`: protocol-independent options, errors, and writer state.

Line width counts Unicode scalar columns, with tabs advancing by indent_width.
Atomic literals and command names may exceed the width; their text is never split.
Argument layouts can change, but case, argument order, quoting/brackets, escapes,
and literal contents do not. Unknown command names remain formattable. Syntax,
structural-block, or unsupported-version errors reject a rewrite. Empty files stay
empty. Cancellation is checked between commands, arguments, and comment spans.

## Project CLI

```sh
cmake-analyzer format
cmake-analyzer format /path/to/project
cmake-analyzer format /path/to/project/CMakeLists.txt --check
cmake-analyzer --config style.toml format /path/to/project
```

The entry must be CMakeLists.txt or its directory. The CLI loads the normal layered
configuration and static Workspace dependency graph. Only resolved, project-local
visits are formatted; it does not recursively collect every *.cmake file. Unused
files, unvisited conditional/deferred/dynamic imports, project-external modules,
configured build outputs, .git/target contents, and non-source CMakeCache directories
are excluded. Known source directories remain eligible for in-source builds.
Canonical paths prevent symlinked external modules from being written.

Graph read/traversal failures abort. Existing Workspace traversal limits apply
(defaults: 256 files, depth 64, 1024 visits); exhaustion never silently writes a
partial traversal. All eligible files are rendered before the first write, so a
syntax error leaves the whole batch untouched. Changed files are replaced via
same-directory temporary files with permissions retained. Each replacement is
atomic, not a project-wide transaction for I/O failures. Concurrent disk changes
detected before writing are rejected; coordinate CLI use with editors.

--check prints paths needing formatting and exits 1 without writing. A clean check
exits 0. Only CLI mode emits a file/summary report; LSP stdout remains protocol-only.
No configure, project script, or execute_process body is executed.

## LSP and save behavior

The server advertises textDocument/formatting and textDocument/willSaveWaitUntil.
Both are ordered, cancellable snapshot queries on a joined worker. They return
client-applied TextEdits for the current open document and never write its disk file.
Syntax errors return no edits. Identical output also returns no edits.

format.on_save defaults true. The pre-save request uses configured indentation,
spaces/tabs, and line width; manual formatting uses client tabSize/insertSpaces and
configured line width. Shared owners must agree on effective style; disagreement
returns an error instead of choosing an arbitrary owner. Any owner disabling
on_save suppresses save edits. Successful reload changes subsequent requests;
failed reload preserves live settings.

Clients that support willSaveWaitUntil request formatting before saving. Others
need a save hook; merely receiving didSave cannot safely rewrite an editor buffer.
A fallback hook should request willSaveWaitUntil, not manual formatting, so that
format.on_save=false remains effective. See the Neovim fallback example below.

```lua
vim.api.nvim_create_autocmd("LspAttach", {
  callback = function(event)
    local client = vim.lsp.get_client_by_id(event.data.client_id)
    if not client or client.name ~= "cmake-analyzer" then return end
    local sync = client.capabilities.textDocument.synchronization or {}
    if sync.willSaveWaitUntil then return end -- Native client support handles it.
    vim.api.nvim_create_autocmd("BufWritePre", {
      buffer = event.buf,
      callback = function()
        local tick = vim.api.nvim_buf_get_changedtick(event.buf)
        local params = {
          textDocument = { uri = vim.uri_from_bufnr(event.buf) }, reason = 1,
        }
        local timeout = vim.g.cmake_analyzer_format_timeout_ms or 1000
        local response = client:request_sync(
          "textDocument/willSaveWaitUntil", params, timeout, event.buf
        )
        if response and not response.err and response.result
            and tick == vim.api.nvim_buf_get_changedtick(event.buf) then
          vim.lsp.util.apply_text_edits(response.result, event.buf, client.offset_encoding)
        end
      end,
    })
  end,
})
```

The fallback timeout is client-side milliseconds and can be changed through
vim.g.cmake_analyzer_format_timeout_ms. Client identifiers may differ by setup.
