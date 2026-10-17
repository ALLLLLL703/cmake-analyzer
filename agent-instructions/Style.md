> AI-generated content.

# Rustdoc

- Document every module, function/method, struct, field, enum, variant and variant field, regardless of visibility; include test helpers and trait items. Update generators for generated code.
- Use `//!` for module contents; `///` for declarations. Start with the item's meaning, behavior/effect and side effects; explicitly state when none occur. Describe the item, not just its name/type.
- Struct/enum docs: meaning, likely call sites, related APIs with rustdoc links and their roles, and a minimal usage example. For complex internals/impls, add an explanation and a text/Mermaid flow diagram showing calls, state changes and side effects.
- Function docs: purpose/effects first; `# Arguments` lists **every** parameter, including receivers, with `[in]`, `[out]` or `[in,out]`. Explicitly identify output parameters and whether they overwrite/append. `# Returns` explains every returned value, including tuple parts, `Option` cases, `Result` outcomes or `()`. Add `# Errors`, `# Panics` and `# Safety` when applicable.
- Complex functions also need `# Algorithm` and `# Flow`: explain the principle and diagram branches, calls, mutation/I/O and outputs. Avoid line-by-line narration.
- Use plain English and short sentences. Avoid jargon/acronyms; explain unavoidable terms once in everyday words.
- Every function/method, struct field, enum variant and variant field needs its own usage example: call the function, read/write the field, or construct/use/match the variant. Use `# Examples` for functions; short inline examples suffice for fields/variants. An enclosing type's example alone is not enough.
- Keep docs short; link shared explanations. Examples must match the API; complete code blocks must compile after adapting template imports. No undocumented obvious-item exception.

## Example

`example` below is the containing crate's placeholder name. Diagrams demonstrate the required format; trivial implementations need no diagram.

```rust
//! Keep or shorten byte buffers through [`Buffer::apply`]; no file/network access.

/// Bytes that callers can shorten through [`Self::apply`].
/// [`Mode`] chooses how many bytes to keep; applying it changes bytes and a counter.
/// No file/network access.
///
/// # Flow
/// ```text
/// caller -> Mode -> apply -> bytes + removed counter + returned length
/// ```
/// # Examples
/// ```
/// # use example::{Buffer, Mode};
/// let mut buffer = Buffer { bytes: vec![1, 2, 3] };
/// let mut removed = 0;
/// assert_eq!(buffer.apply(Mode::Trim { len: 2 }, &mut removed), 2);
/// assert_eq!(removed, 1);
/// ```
pub struct Buffer {
    /// Stored bytes; [`Buffer::apply`] may remove bytes from the end.
    /// Example: `buffer.bytes.push(1)` adds a byte; `buffer.bytes.len()` reads the count.
    pub bytes: Vec<u8>,
}

/// How many bytes [`Buffer::apply`] should keep.
/// Creating this choice changes nothing; applying it may remove bytes.
///
/// # Examples
/// ```
/// # use example::Mode;
/// let policy = Mode::Trim { len: 2 };
/// ```
pub enum Mode {
    /// Keep all bytes; applying this choice removes nothing.
    /// Example: `buffer.apply(Mode::Keep, &mut removed)` keeps the buffer unchanged.
    Keep,
    /// Keep only the first bytes; applying this choice removes the rest.
    /// Example: `buffer.apply(Mode::Trim { len: 2 }, &mut removed)` keeps at most two.
    Trim {
        /// Maximum bytes to keep; larger values leave bytes unchanged.
        /// Example: `Mode::Trim { len: 2 }` sets this limit to two bytes.
        len: usize,
    },
}

impl Buffer {
    /// Keep the requested bytes, overwrite the removed-byte counter and return the length.
    /// Changes this buffer and `removed`; no file/network access.
    ///
    /// # Arguments
    /// - `self` [in,out]: buffer that may lose bytes from its end.
    /// - `mode` [in]: whether to keep all bytes or only the first bytes.
    /// - `removed` [out]: overwritten with the number of discarded bytes.
    ///
    /// # Returns
    /// Remaining byte count; never exceeds the original length.
    ///
    /// # Algorithm
    /// Record the old length, remove excess bytes, then count how many were removed.
    ///
    /// # Flow
    /// ```text
    /// old length -> Keep / Trim -> change bytes -> write removed -> return length
    /// ```
    /// # Examples
    /// ```
    /// # use example::{Buffer, Mode};
    /// let mut buffer = Buffer { bytes: vec![1, 2, 3] };
    /// let mut removed = 0;
    /// assert_eq!(buffer.apply(Mode::Trim { len: 2 }, &mut removed), 2);
    /// assert_eq!(removed, 1);
    /// ```
    pub fn apply(&mut self, mode: Mode, removed: &mut usize) -> usize {
        let before = self.bytes.len();
        if let Mode::Trim { len } = mode {
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
