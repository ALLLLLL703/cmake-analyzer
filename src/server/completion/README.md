> AI-generated content.

# Command-name completion

- `query.rs` captures immutable owner snapshots/indexes after preceding edits.
- `context.rs` uses shared lexer tokens, comments, errors, and checkpoint depth.
  Command names are completed at parenthesis depth zero, including block bodies;
  arguments, strings, comments, and invalid identifiers are excluded. No relexing.
- `candidates.rs` merges versioned native schema layers, used external schemas,
  and known document/Workspace function and macro declarations. Names are filtered
  by case-insensitive prefix and deduplicated/sorted by lowercase name. Source
  declarations override native entries; suggestions do not prove execution or scope visibility.
- `arguments.rs` extracts required schema parameters and renders escaped snippets.
- `render.rs` creates name-range edits using the negotiated position encoding.
- `model.rs` keeps protocol-independent candidate/input records.

Keyword schemas use CompletionItemKind.Keyword. Native project/target, dependency,
module, install/export, and CTest commands use Module; other commands use Function.
User functions/macros use Function even when shadowing a native command. The
client/theme chooses the actual glyph. Unknown/unsupported versions use the
CMake 3.0 baseline. Native layers are borrowed rather than copied into a new catalog.

Default edits replace only the complete identifier, preserving arguments and
surrounding text. With completion.reserve_arg = true and client snippet support,
a new call includes parentheses, required parameter placeholders, and a final
cursor stop. Existing parentheses suppress snippet insertion. Optional arguments
are omitted, repeats include only their minimum required entries, and overloads
retain only a common required prefix; no overload or unmodeled tail is invented.
Required keyword tokens remain literal. Conflicting source signatures fall back
to name-only edits. Shared owners must all enable reserve_arg to insert snippets.

Completion is an ordered cancellable service query with joined worker computation.
It does not inspect native help, execute project code, or run configure/scripts.
There is no argument/value completion, completion-item resolve, documentation
lookup, or fuzzy matching in this implementation. Configuration reload is shared
with other features; failed reload leaves live completion settings unchanged.
