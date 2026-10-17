//! Exact CMake bracket-delimiter recognition; advances cursor only, no I/O.
//! Scanning retains raw delimiters; payload stripping is a separate text view.
//! Contains AI-generated documentation additions.

use crate::model::{Spanned, TextSpan};

use super::model::{LexError, LexResult, Lexer};

impl Lexer<'_> {
    /// Recognize `[` followed by zero or more `=` and another `[` at the cursor; no side effects.
    ///
    /// # Arguments
    /// - `self` `[in]`: scanner at a possible opener; unchanged.
    ///
    /// # Returns
    /// `Some` opening-delimiter byte length, at least two; `None` for invalid offset or non-opener.
    ///
    /// # Examples
    /// ```text
    /// let lexer = Lexer::new(RawBuffer::new("[==[x]==]"));
    /// assert_eq!(lexer.bracket_opening_len(), Some(4));
    /// ```
    pub(super) fn bracket_opening_len(&self) -> Option<usize> {
        let rest = self
            .buffer
            .text
            .as_bytes()
            .get(self.buffer.cursor.offset..)?;
        if rest.first() != Some(&b'[') {
            return None;
        }
        let equals = rest[1..].iter().take_while(|byte| **byte == b'=').count();
        (rest.get(equals + 1) == Some(&b'[')).then_some(equals + 2)
    }

    /// Consume an exact bracket-delimited unit, advancing past the close or to EOF; no I/O.
    /// Does not strip or rewrite source bytes; the token payload view strips wrappers later.
    ///
    /// # Arguments
    /// - `self` `[in,out]`: scanner positioned on a validated bracket opener; cursor advanced in place.
    /// - `opening_len` `[in]`: validated opening-delimiter byte length, at least two.
    /// - `span` `[in]`: diagnostic start; local copy gets EOF as end on failure. For comments it starts at `#`.
    ///
    /// # Returns
    /// `Ok(())` after a matching close; `Err` located from the supplied start through EOF.
    ///
    /// # Errors
    /// Returns `UnclosedBracketArgument` after advancing to EOF if no exact closing
    /// delimiter exists. The comment caller relabels it `UnclosedBracketComment`.
    ///
    /// # Panics
    /// An unvalidated opener length below two can underflow; a length past the source
    /// can make the initial content slice panic. Call `bracket_opening_len` first.
    ///
    /// # Algorithm
    /// Begin after the opener and at most one initial LF/CRLF. Search for `]`, the exact
    /// number of `=` bytes, and `]`. Do not nest brackets or process escapes. A rejected
    /// candidate leaves its final `]` available as the start of another candidate.
    ///
    /// # Flow
    /// ```text
    ///         [validated opener]
    ///                  |
    ///     [skip one initial LF/CRLF]
    ///                  |
    ///         [find next closing candidate] <----+
    ///                  |                        |
    ///           <equals count + final ]?>        |
    ///              /                \          |
    ///            yes                no ---------+
    ///             |                  | EOF
    ///     [cursor past close]   [cursor = EOF]
    ///             |                  |
    ///          [Ok(())]           [Err(span)]
    /// ```
    ///
    /// # Examples
    /// ```text
    /// let mut lexer = Lexer::new(RawBuffer::new("[=[\nraw]=]"));
    /// let span = lexer.buffer.current_byte_to_span();
    /// let len = lexer.bracket_opening_len().unwrap();
    /// lexer.advance_bracket_argument(len, span).unwrap();
    /// ```
    pub(super) fn advance_bracket_argument(
        &mut self,
        opening_len: usize,
        mut span: TextSpan,
    ) -> LexResult<()> {
        let text = self.buffer.text;
        let bytes = text.as_bytes();
        let equals = opening_len - 2;
        let mut content_start = self.buffer.cursor.offset + opening_len;
        if bytes[content_start..].starts_with(b"\r\n") {
            content_start += 2;
        } else if bytes[content_start..].starts_with(b"\n") {
            content_start += 1;
        }

        let mut offset = content_start;
        while offset < bytes.len() {
            if bytes[offset] != b']' {
                offset += 1;
                continue;
            }
            offset += 1;
            let equals_start = offset;
            while bytes.get(offset) == Some(&b'=') {
                offset += 1;
            }
            if offset - equals_start == equals && bytes.get(offset) == Some(&b']') {
                self.buffer.cursor.offset = offset + 1;
                return Ok(());
            }
            // A mismatched final ']' may itself begin the real closing delimiter.
        }

        self.buffer.cursor.offset = bytes.len();
        span.end_byte = bytes.len();
        Err(Spanned {
            content: LexError::UnclosedBracketArgument,
            span,
        })
    }
}
