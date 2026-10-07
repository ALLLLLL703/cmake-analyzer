# Span-Based Lexer Results

Lexer tokens are `Spanned<LexContent>`; enum variants identify token kinds without owning strings. Errors are `Spanned<LexError>`. Keep the original source alongside each result, and use that same source snapshot when resolving spans.

```rust
use cmake_analyzer::parser::lexer::model::{Lexer, RawBuffer};

let source = "message(\"hello\" [[raw]])";
let lexed = Lexer::new(RawBuffer::new(source)).parse();
for token in &lexed.lex {
    let raw = token.span.text(source); // Complete token, including delimiters.
    let payload = token.text(source);  // Unwrapped string/bracket contents.
}
println!("{}", lexed.display(false, source));
```

The stored token span remains a half-open UTF-8 byte range covering its entire lexeme. `payload_span(source)` returns a separate view with adjusted row/column coordinates. Quoted views exclude both quotes; bracket views exclude matching delimiters and exactly one initial LF or CRLF. No escapes or variables are evaluated and no text is copied.

`advance_identifier()` and `advance_string_literal()` now return `LexResult<()>`: they consume source but do not allocate result strings. `RawBuffer::span_to_text()` returns a borrowed `Option<&str>`.

## TextSpan Iteration

- `span.range()` returns `start_byte..end_byte`.
- `span.iter()`, `for offset in span`, and `for offset in &span` traverse absolute byte offsets. These include offsets inside multibyte characters; they are not necessarily valid string-slice boundaries.
- `span.text(source)` returns `Option<&str>`.
- `span.iter_text(source)` returns `Option<std::str::Chars<'_>>`, traversing Unicode scalar values without copying text.
- Text access rejects reversed, out-of-bounds, and non-UTF-8-boundary ranges with `None`. An empty range at EOF is valid.

```rust
if let Some(chars) = span.iter_text(source) {
    for ch in chars {
        // Process one Unicode scalar value.
    }
}
```

Lexer display now requires the source: `display(colored, source)`. The `lexer` CLI already supplies it.

Nparser command names are spanned lexer kinds, arguments are spanned argument kinds, and error text is recovered through its outer node span. `Spanned<NParsedArgument>::text(source)` provides the same unwrapped view. The handwritten nparser control flow and its two TODO scanners remain unfinished; this migration does not implement them. In particular, its unfinished identifier path still needs cursor advancement before the parser can be used on normal commands.

These models prepare for snapshot-based/incremental analysis but do not implement incremental updates. Spans and their row/column metadata must not be reused against edited source without updating or recomputing them.


## ai generated md
