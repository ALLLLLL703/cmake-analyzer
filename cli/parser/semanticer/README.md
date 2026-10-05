# Semanticer CLI

Inspect CMake control blocks, scopes, symbols, references, and diagnostics:

```sh
cargo run --bin semanticer -- CMakeLists.txt
cargo run --bin semanticer -- --text 'set(value hello) message("${value}")'
cargo run --bin semanticer -- --text 'function(greet who) message("${who}") endfunction() greet(world)' --color
printf 'add_executable(app app.cpp)\n' | cargo run --bin semanticer
printf 'set(value hello)\n' | cargo run --bin semanticer -- -
cargo run --bin semanticer -- --help
```

The positional file and `--text` are mutually exclusive. Omit both, or use `-`, to read stdin. Source must be UTF-8. Output is plain unless `--color` is explicitly supplied.

The display includes the original nparser commands, a flat block arena with parent/open/close/branch indices, analysis scopes, symbol declarations, reference candidates, and diagnostics. Names are escaped; spans use half-open UTF-8 byte ranges with zero-based row/Unicode-scalar column positions. Unresolved references and potential duplicate-target warnings do not fail the command.

Exit codes:

- `0`: no error diagnostics, including empty input, warnings, or broken output pipes.
- `1`: lexical, command, block, or supported semantic errors; partial results remain visible on stdout.
- `2`: argument, input, output, or worker failure; the failure is reported on stderr.

Input is read asynchronously. The complete analysis and buffered stdout output run in one awaited blocking worker. No CMake commands are executed.

This tool inherits the handwritten lexer's and nparser's syntax limitations, including currently unsupported comments. It is not a complete CMake validator. See [Structural and Static Semantic Analysis](../../../src/parser/semanticer/README.md) for supported facts, recovery, and binding semantics.
