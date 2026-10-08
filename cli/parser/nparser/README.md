> AI-generated content.

# Nparser CLI

Inspect the existing lexer → nparser pipeline:

```sh
cargo run --bin nparser -- path/to/CMakeLists.txt
cargo run --bin nparser -- --text 'message("hello" [[raw]])'
printf 'message(hello)\n' | cargo run --bin nparser
printf 'message(hello)\n' | cargo run --bin nparser -- -
cargo run --bin nparser -- --color path/to/CMakeLists.txt
```

A file and `--text` are mutually exclusive. Input must be UTF-8. Without a file or `--text`, input is read from stdin. `-` also means stdin. ANSI colors are disabled unless `--color` is supplied.

Input uses asynchronous I/O; parsing and buffered stdout output run on a blocking worker. Source text remains alive while spans are rendered.

Exit codes:

- `0`: no error nodes or unclosed commands reported by the current parser; also used when the output consumer closes the pipe.
- `1`: error nodes or unclosed commands were reported. Inspection output is still written to stdout.
- `2`: argument, input, output, or worker failure; I/O failures are reported on stderr.

The CLI uses the current handwritten parser; it does not independently validate CMake or change its recovery/span behavior. Its exit status reflects the errors that parser actually reports, not a guarantee of valid CMake. Missing-parenthesis lexer and parser diagnostics may both appear.
