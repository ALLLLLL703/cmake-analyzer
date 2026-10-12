> AI-generated content.

# Project Architecture

## Current implementation

The `hand` branch contains a handwritten CMake lexer, command parser, and static semantic analyzer. `src/main.rs` remains a placeholder; there is no running LSP on this branch. `src/lib.rs` exports `model`, `parser`, and a user-owned experimental `rpc` scaffold. The `lexer`, `nparser`, and `semanticer` inspection binaries are under `cli/parser/`.

## Syntax and source ranges

- `src/model.rs` and `src/model/span.rs`: `Spanned<T>` and half-open UTF-8 byte ranges in `TextSpan`. Columns are Unicode scalar counts, not LSP UTF-16 coordinates.
- `src/parser/lexer/`: tokenization and payload views. Token spans preserve delimiters; payload access removes quotes or bracket delimiters.
- `src/parser/nparser/model.rs`: command nodes, raw argument forms, and `CommandState`.
- `src/parser/nparser/parsing.rs`: `NParser::parse` and `consume_function_args`. A complete command ends at its consumed outer right parenthesis, not the following token.

## Semantic data

All semantic arena models are defined in `src/parser/semanticer/model.rs`:

- `Semanticer`: parsed input, active and injected schema maps, and selected CMake version.
- `SemanticedBuffer`: commands, variables, diagnostics, blocks, scopes, and root symbols.
- `SemanticCommandRepresent`: command/name ranges, original argument forms, closure state, block/scope IDs, and an optional shared `Arc<CommandSchema>`.
- `SemanticBlock`: kind, direct command IDs including its own opening/closing commands, child block IDs, parent ID, closure state, and full range. Descendant commands are not duplicated in ancestor membership lists.
- `SemanticScope`: independent scope record associated with a block by `BlockId`, with an optional parent `ScopeId`.
- `SemanticVariable`: name, initial-value placeholder, optional scope ID, and command IDs recording possible writes. Global variables use `None`; scoped variables are distinguished by `(scope, name)`.
- `SemanticSymbol`: a range and an ID reference to a top-level block or command. It does not duplicate commands, blocks, or their children.

IDs are indices into one analysis snapshot, not persistent identities across parses.

## Analysis pipeline

`Semanticer::parse(source, file_id).await` in `src/parser/semanticer/parsing.rs` assembles these stages:

1. `scan_version` in `parsing/version.rs`: select the minimum declared by `cmake_minimum_required(VERSION ...)`; otherwise asynchronously run `cmake --version`. Version ranges select their lower bound.
2. Load versioned schemas and builtin variable records, then overlay constructor-injected schemas.
3. `scan_commands`: convert nparser commands to semantic records and diagnose syntax errors and commands absent from the schema map.
4. `scan_blocks` in `parsing/block.rs`: recognize `if`, `while`, `foreach`, `function`, `macro`, and `block` pairs using builtin keyword commands. Diagnose missing or unexpected ends, invalid branch placement, repeated `else`, and `elseif` after `else`. An ancestor end can recover past unfinished inner blocks; those inner blocks remain unclosed and receive diagnostics.
5. `scan_scopes` in `parsing/scope.rs`: establish function scopes and statically identified variable scopes from `block()`. Plain `block()` creates a variable scope; `SCOPE_FOR POLICIES` alone does not. Macro and control-flow blocks inherit their surrounding variable scope. Opening declarations belong to the surrounding scope; their bodies belong to the newly established scope.
6. `track_variable_changes` in `parsing/outputs.rs`: traverse semantic commands only, locate schema `Out` arguments, and record writes in the owning scope. It does not revisit nparser nodes or source text.
7. `collect_symbols` in `parsing/symbols.rs`: assemble root block/command references in source order. Nested members remain accessible through the block arena.

## Schema and output matching

- `src/parser/semanticer/schema.rs`: scalar argument roles and `Keyword`, `Optional`, `Repeat`, `Choice`, and `AllOf` grammar nodes.
- `schema/builtin.rs`: lazy static command tables with version additions and replacements; `schemas_for_version` applies the layers.
- `variables/builtin.rs`: lazy static `SemanticVariable` tables; `variables_for_version` applies variable additions. Initial values are empty placeholders, and template names are not expanded into runtime variables.
- `outputs.rs`: `sequence`, `group`, and `step` locate outputs structurally without validating scalar parameter types. `merge` prefers more literal keyword matches at the same position and intersects equally preferred output candidates. Unresolved output names or earlier unquoted expansion/list splitting prevent a guessed write.

## Semantic inspection CLI

- `src/parser/semanticer/display.rs`: borrowed, streaming `SemanticedBuffer::display(colored)` adapter. It prints version, root symbols, commands and raw argument forms, blocks, scopes, variables/write IDs, and diagnostics without cloning the analysis arenas. Source-derived strings are debug-escaped; Color formatting uses the existing `owo-colors` dependency with a reusable `Style` palette; no handwritten ANSI escapes.
- `SemanticDisplay::with_all_variables`: opt into unchanged builtin variable records, which are hidden by default to avoid flooding the output.
- `cli/parser/semanticer/args.rs`: positional file or `-`/stdin, `--text`, `--color`, and `--all-variables`.
- `io.rs`: asynchronous input loading. `output.rs`: lexer/parser/semantic analysis and buffered display writing. `main.rs`: CLI assembly and a blocking worker for analysis/rendering; the worker uses the running Tokio handle for asynchronous version detection.
- Exit codes: `0` for clean analysis or broken pipe, `1` when analysis diagnostics exist, and `2` for input/output failures. Clap handles invalid CLI arguments. Color is explicit, disabled by default, and distinguishes headings, IDs, keyword/native/external commands, scopes, variables, outputs, argument forms, closure states, and errors.

## Deliberate limits

This is static structural analysis, not CMake execution. Variable references are not scanned or diagnosed. Argument validation, runtime values, function/macro call resolution, runtime caller scopes, cache namespaces, `PARENT_SCOPE`, and `PROPAGATE` effects are not implemented. Some schema branches remain explicitly `Unmodeled`. Function-body writes are snapshot-local possible writes, not evidence of a function call.

No document-symbol protocol handler exists yet; `SemanticedBuffer.symbols` is its intended structural input. Tests already present are wired through `tests/suite.rs` and `tests/integration.rs`. Under the current user instruction, do not add new tests or general documentation; maintain this requested architecture record and ownership labels.
