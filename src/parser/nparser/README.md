# Command Parser

`NParser` consumes a `LexedBuffer` and produces flat command calls with spanned names and arguments. It does not evaluate variables, validate built-in command signatures, or group control blocks.

## Scanner Contracts

- `peek()` borrows the current token without advancing.
- `advance_a_function_name()` consumes one command identifier matching `[A-Za-z_][A-Za-z0-9_]*`. Invalid names are not consumed by this helper.
- `advance_a_function_args()` starts at the opening `(` and consumes through the matching outer `)`. Internal parentheses are preserved as argument markers, not parsed as nested commands.
- `advance_a_arg()` consumes one token, preserving its span and mapping its argument kind.
- All scanners return `NParseResult<T>`; EOF and invalid input are explicit errors rather than panics.

String allocations are moved from lexer tokens into the result. Consumed token strings in `parser.buffer` become empty; their kinds and spans remain. Do not reset the cursor to reparse consumed data. Quoted/bracket text retains the lexer's existing unwrapping behavior; bracket arguments map to the existing `Bracked` variant.

`parse()` forwards lexical errors even for an empty token stream. It skips invalid top-level tokens and, when an opening parenthesis is missing, leaves the next token available as a potential command. An unterminated argument list is diagnosed at its opening parenthesis; incomplete commands are omitted. Missing closing parentheses may consume the rest of the token stream, since command-like text can legally occur inside arguments.

Normal EOF between commands is not an error. This implementation does not yet enforce whitespace/newline separation rules or validate control-block matching.
