> AI-generated content.

# Rust Style

## Type names

- Every struct and enum name must include a meaningful domain, feature or responsibility qualifier in the identifier itself. Use `RenderOptions`, `SemanticerOptions` or `RenderMode`, not bare `Options`, `Context`, `Model` or `Mode`.
- Apply this rule regardless of visibility, including test helpers and generated types. Different modules do not justify unqualified names; a module path or import alias does not replace a qualified declaration name.
- Choose a short qualifier that explains ownership or purpose; do not mechanically copy the full module path into the name.

## Rustdoc

- Document every module, function/method, struct, field, enum, variant and variant field, regardless of visibility; include test helpers and trait items. Update generators that remain in use; one-time generated files may be documented directly.
- Use `//!` for module contents; `///` for declarations. Start with the item's meaning, behavior/effect and side effects; explicitly state when none occur. Describe the item, not just its name/type.
- Struct/enum docs: meaning, likely call sites, related APIs with rustdoc links and their roles, and a minimal usage example. For complex internals/impls, add an explanation and a text/Mermaid flow diagram showing calls, state changes and side effects.
- Function docs: purpose/effects first; `# Arguments` lists **every** parameter, including receivers, with `[in]`, `[out]` or `[in,out]`. Explicitly identify output parameters and whether they overwrite/append. `# Returns` explains every returned value, including tuple parts, `Option` cases, `Result` outcomes or `()`. Add `# Errors`, `# Panics` and `# Safety` when applicable.
- Complex functions also need `# Algorithm` and `# Flow`: explain the principle, calls, state changes, file/network access and outputs. Avoid line-by-line narration.
- Diagrams must be genuinely two-dimensional: multiple rows/columns with connected nodes, not a one-line arrow chain. Choose a flowchart, state/sequence diagram, ownership map or component diagram to fit the item. Show real branches/loops, inputs, changed state and side effects where relevant. Prefer boxed ASCII for portable rustdoc; Mermaid requires renderer support.
- Use plain English and short sentences. Avoid jargon/acronyms; explain unavoidable terms once in everyday words.
- Every function/method, struct field, enum variant and variant field needs its own usage example: call the function, read/write the field, or construct/use/match the variant. Use `# Examples` for functions; short inline examples suffice for fields/variants. An enclosing type's example alone is not enough.
- Keep docs short; link shared explanations. Examples must match the API; complete code blocks must compile after adapting template imports. No undocumented obvious-item exception.

### Example

`example` below is the containing crate's placeholder name. Diagrams demonstrate the required format; trivial implementations need no diagram.

```rust
//! Keep or shorten byte buffers through [`ByteBuffer::apply`]; no file/network access.

/// Bytes that callers can shorten through [`Self::apply`].
/// [`ByteBufferMode`] chooses how many bytes to keep; applying it changes bytes and a counter.
/// No file/network access.
///
/// # Flow
/// ```text
///    [Caller]            [policy]
///        |                   |
///        +--------+----------+
///                 v
///      +---------------------+
///      | ByteBuffer::apply   |
///      +----------+----------+
///                 |
///         +-------+--------+
///         v                v
///   [bytes changed]  [removed written]
///         |
///         v
///   [length returned]
/// ```
/// # Examples
/// ```
/// # use example::{ByteBuffer, ByteBufferMode};
/// let mut buffer = ByteBuffer { bytes: vec![1, 2, 3] };
/// let mut removed = 0;
/// assert_eq!(buffer.apply(ByteBufferMode::Trim { len: 2 }, &mut removed), 2);
/// assert_eq!(removed, 1);
/// ```
pub struct ByteBuffer {
    /// Stored bytes; [`ByteBuffer::apply`] may remove bytes from the end.
    /// Example: `buffer.bytes.push(1)` adds a byte; `buffer.bytes.len()` reads the count.
    pub bytes: Vec<u8>,
}

/// How many bytes [`ByteBuffer::apply`] should keep.
/// Creating this choice changes nothing; applying it may remove bytes.
///
/// # Examples
/// ```
/// # use example::ByteBufferMode;
/// let policy = ByteBufferMode::Trim { len: 2 };
/// ```
pub enum ByteBufferMode {
    /// Keep all bytes; applying this choice removes nothing.
    /// Example: `buffer.apply(ByteBufferMode::Keep, &mut removed)` keeps the buffer unchanged.
    Keep,
    /// Keep only the first bytes; applying this choice removes the rest.
    /// Example: `buffer.apply(ByteBufferMode::Trim { len: 2 }, &mut removed)` keeps at most two.
    Trim {
        /// Maximum bytes to keep; larger values leave bytes unchanged.
        /// Example: `ByteBufferMode::Trim { len: 2 }` sets this limit to two bytes.
        len: usize,
    },
}

impl ByteBuffer {
    /// Keep the requested bytes, overwrite the removed-byte counter and return the length.
    /// Changes this buffer and `removed`; no file/network access.
    ///
    /// # Arguments
    /// - `self` `[in,out]`: buffer that may lose bytes from its end.
    /// - `mode` `[in]`: whether to keep all bytes or only the first bytes.
    /// - `removed` `[out]`: overwritten with the number of discarded bytes.
    ///
    /// # Returns
    /// Remaining byte count; never exceeds the original length.
    ///
    /// # Algorithm
    /// Record the old length, remove excess bytes, then count how many were removed.
    ///
    /// # Flow
    /// ```text
    ///          [Save old length]
    ///                  |
    ///             < policy? >
    ///              /       \
    ///           Keep       Trim
    ///            |           |
    ///       [No change] [Remove excess]
    ///            |           |
    ///            +-----+-----+
    ///                  v
    ///           [Write removed]
    ///                  |
    ///                  v
    ///           [Return length]
    /// ```
    /// # Examples
    /// ```
    /// # use example::{ByteBuffer, ByteBufferMode};
    /// let mut buffer = ByteBuffer { bytes: vec![1, 2, 3] };
    /// let mut removed = 0;
    /// assert_eq!(buffer.apply(ByteBufferMode::Trim { len: 2 }, &mut removed), 2);
    /// assert_eq!(removed, 1);
    /// ```
    pub fn apply(&mut self, mode: ByteBufferMode, removed: &mut usize) -> usize {
        let before = self.bytes.len();
        if let ByteBufferMode::Trim { len } = mode {
            self.bytes.truncate(len);
        }
        *removed = before - self.bytes.len();
        self.bytes.len()
    }
}
```

For other return shapes, document each outcome explicitly:

```rust
/// # Returns
/// `Ok(Some((name, span)))`: resolved name and its source range.
/// `Ok(None)`: no match; no output parameter is changed.
/// `Err(error)`: lookup failed; see Errors for mutation guarantees.
/// # Errors
/// Describe each actual failure category and any partial output/state changes.
```
