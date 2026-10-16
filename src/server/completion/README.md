> AI-generated content.

# Completion

- `query.rs` dispatches command-name or argument-name completion on a joined worker.
- `inputs.rs` captures owner snapshots/indexes and unique dependency snapshots,
  retaining directory-scope ancestry for inherited names.
- `context.rs` handles outer command positions and comments using shared lexer data.
- `argument_context.rs` locates arguments, unfinished literals, and innermost ${...}
  names without relexing. Edits preserve surrounding quotes/brackets/braces.
- `candidates.rs` merges versioned native schemas, used external schemas, and known
  document/Workspace function and macro declarations.
- `roles.rs` resolves active schema name roles through the shared parser matcher.
  Position-qualified values may resolve earlier simple expansions; unknown unquoted
  splitting/empty removal suppresses later role inference.
- `names.rs` collects variable/target names. Deduplication borrows source names;
  only returned labels are copied.
- `arguments.rs` renders command-call parameter snippets; `render.rs` converts
  edits using the negotiated position encoding. `model.rs` contains feature models.

## Command names

Command names are offered at parenthesis depth zero, including block bodies, but
not inside comments, strings, or invalid identifiers. Prefix matching and name
merging are case-insensitive. Source declarations override native entries.
Keyword schemas use Keyword; native build/module commands use Module; other commands
use Function. User functions/macros use Function even when shadowing native names.
The client/theme chooses actual glyphs. Unknown/unsupported versions use the
CMake 3.0 baseline; native schema layers are borrowed, not copied into a new catalog.

Default edits replace only the complete identifier. With completion.reserve_arg = true
and client snippet support, new calls include parentheses, required parameter
placeholders, and a final cursor stop. Existing parentheses suppress snippets.
Optional arguments are omitted, repeats use their minimum count, and overloads
retain only a common required prefix. Keywords remain literal. Conflicting source
signatures use name-only edits; shared owners must all enable reserve_arg.

## Variable and target names

Variable names use Variable icons in schema Variable/Out/Unset slots and ordinary
${...} references, including unfinished quoted arguments. Sources include builtin
and specialized descriptors, current-file declarations, visible function/macro
parameters, and global names from included/ancestor-directory snapshots. Explicit
unrelated function-local scopes and child-directory globals are excluded.
Escaped references, bracket-literal expansion, ENV/CACHE namespaces, and computed
outer reference names are not completed; a simple nested inner reference is supported.

Target names use EnumMember icons in schema Target/TargetExe/TargetLib/CustomTarget
slots. Candidates come from literal target declarations in the document and owning
dependency snapshots, including imported targets and aliases. Executable/library
slots filter by declaration kind; directly declared nonglobal imports from unrelated
child directories are excluded. Alias-directory visibility and full runtime scope
are not modeled. Dynamically named declarations are not evaluated into new targets.

Argument prefixes follow Keyword/Optional/Repeat/Choice/AllOf schemas without
requiring a completed call. Completed keywords select more specific branches;
equally supported name alternatives are offered together. This prefix mode shares
the binding matcher but does not change its full-call intersection policy.
The target_link_libraries schema models repeated target/link items with visibility
and debug/optimized/general selectors. Unmodeled tails do not invent name roles.
Known user-command shadows do not inherit native parameter roles.

Variable/target prefixes are case-sensitive. Edits replace only the active literal
name or reference name and never insert snippets, quotes, or closing braces.
Simple names are supported; whitespace, escapes, list separators, and unresolved
template names are excluded. Suggestions describe known names, not proof of runtime
availability. General values, enum keywords, properties, paths, and runtime-only
names are not completed. Trigger characters include `(` and `{`.

Queries observe preceding edits, support cancellation, and retain immutable inputs.
They do not inspect native help or execute CMake/project code. There is no item
resolve, documentation lookup, or fuzzy matching. Successful configuration reload
applies new settings; failed reload preserves live settings.
