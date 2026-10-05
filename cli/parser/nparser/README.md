# NParser CLI

Inspect flat CMake command calls and their arguments:

```sh
cargo run --bin nparser -- CMakeLists.txt
cargo run --bin nparser -- --text 'message("hello" [=[raw]=])'
printf 'if(A AND (B OR C))\nendif()\n' | cargo run --bin nparser
printf 'message(hello)\n' | cargo run --bin nparser -- -
cargo run --bin nparser -- --text 'message("hello")' --color
cargo run --bin nparser -- --help
```

The positional file and `--text` are mutually exclusive. With neither argument, or with `-` as the file, input comes from stdin. Input must be UTF-8. `--color` explicitly enables ANSI colors; output is plain by default, including when redirected.

Output groups arguments under each `Command` line. Argument labels distinguish `Unquoted`, `Quoted`, and the model's existing `Bracked` variant. Internal parentheses are indented argument markers. Strings use escaped debug formatting so source newlines and terminal control characters cannot change the output structure.

For example:

```text
Command: message
  Quoted: "hello"
  Bracked: "raw"
```

Diagnostics follow partial successful results, with their `TextSpan` byte ranges and zero-based row/Unicode-scalar column positions. These columns are not LSP UTF-16 offsets or terminal display widths.

Exit codes:

- `0`: success, including empty input and broken output pipes.
- `1`: lexical or parsing errors; partial results are still displayed.
- `2`: argument, input, output, or worker errors.

Input is read asynchronously. Lexing, parsing, formatting, and buffered stdout writes run in one awaited blocking worker; source and token strings are moved rather than cloned.

This is a syntax inspection tool, not a CMake interpreter or complete validator. See [Command Parser](../../../src/parser/nparser/README.md) for current parsing behavior and limitations.
