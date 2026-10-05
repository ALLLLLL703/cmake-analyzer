# Structural and Static Semantic Analysis

The handwritten pipeline is:

```text
Lexer -> NParser -> Semanticer -> SemanticBuffer
```

```rust
use cmake_analyzer::parser::semanticer::Semanticer;

let result = Semanticer::analyze("set(value hello)\nmessage(\"${value}\")");
println!("{}", result.display(true));
assert!(!result.has_errors());
```

`Semanticer::new(parsed).parse()` consumes an existing `NParsedBuffer`. Command and argument strings are moved, not cloned. Reusing the same analyzer after parsing produces an empty snapshot. `Name` holds a command index, optional argument index, and a payload byte range; `buffer.name(&name)` borrows the original text. All IDs belong to one snapshot.

## Structure

Blocks form a flat arena rather than a recursively owned tree. `if`, `foreach`, `while`, `function`, `macro`, and `block` record their parent, opening/closing command indices, body scope, and optional branch markers. `command_blocks` and `command_scopes` map commands into these arenas. Deep nesting is processed and dropped without recursive ownership.

Branch diagnostics cover misplaced `else`/`elseif`, duplicate `else`, and `elseif` after `else`. End markers match block kinds case-insensitively. A mismatched end recovers at the nearest matching opener, diagnosing intervening unclosed blocks. Unmatched ends and remaining open blocks are errors. Function/macro optional end names are compared when literal.

Scopes include the directory, function bodies, macro analysis containers, and variable-enabled `block` bodies. `block(SCOPE_FOR POLICIES)` alone does not create a variable scope. Ordinary conditions and loops do not create variable scopes. Macro containers organize placeholders and navigation; they are not a claim that CMake macros create runtime variable scopes.

## Facts and Binding

- Literal `set`, `unset`, `option`, and `foreach` names, cache/environment namespaces, unset markers, and function/macro parameters.
- Function/macro declarations and case-insensitive command references, including calls that override ordinary built-ins. Variables and targets retain case-sensitive names.
- `${...}`, `$ENV{...}`, and `$CACHE{...}` references. The scanner tracks escaped characters, UTF-8, newlines, and nested expansions with an explicit stack. Dynamic outer names remain unresolved while their inner references are indexed. Bracket arguments never expand.
- Common implicit reads in conditions, `foreach(IN LISTS ...)`, and list mutations/queries; selected list/string/math/property output variables.
- Target definitions and aliases, target command receivers, dependencies, property receivers, and link-library candidates.
- Selected command arity checks and minimum literal function/macro parameter counts. Unquoted expansion or list-valued syntax suppresses count-based checks; dynamic formal parameter lists do too. Multiple callable candidates do not produce a guessed signature error.

Variable references select the nearest preceding binding in the analysis scope chain; writes on the same command do not bind its input reads. Unset normal bindings permit cache fallback. `PARENT_SCOPE` writes are recorded separately and are not invented as local assignments. Writes inside callable bodies are not propagated to the directory as though the callable had executed. Targets and callables may bind forward to candidates in visible analysis scopes. Target duplicates in one scope are warnings, since mutually exclusive branches may declare the same name. Unresolved references are not errors: built-ins, imported names, system libraries, and configured values can have no local declaration.

Declaration and implicit-read spans cover their owning argument. Expansion references have precise name byte ranges and zero-based row/Unicode-scalar column positions. Columns are not LSP UTF-16 offsets. Quoted/bracket payloads remain unwrapped in the underlying nparser result.

## Inspection and Limits

`buffer.display(colored)` first displays the existing nparser result, then blocks, scopes, symbols, references, candidate IDs, and diagnostics. Plain `Display` disables colors. Text is escaped; formatting errors propagate. `has_errors()` combines upstream lexical/parser errors with semantic error diagnostics; warnings alone do not fail.

This is a single-file static candidate index, not a CMake interpreter. It does not evaluate branches, calls, policies, variable values, generator expressions, external includes/packages, or exact runtime scope effects. Loop restoration and `block(PROPAGATE ...)` effects are not simulated. It does not provide incremental updates or a complete command-signature catalog. Literal fact extraction skips escape-dependent/dynamic names rather than guessing their evaluated values.

Lexical and command syntax support is inherited from the handwritten lexer/nparser. In particular, comments and full unquoted-argument rules are not implemented there yet, and nparser does not enforce all whitespace/newline separation rules. This stage does not replace or broaden those parsers.

See the [Semanticer CLI](../../../cli/parser/semanticer/README.md) for file, text, and stdin inspection.
