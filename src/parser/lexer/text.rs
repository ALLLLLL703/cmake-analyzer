//! Borrow lexical payloads and query checkpoint nesting without source mutation or I/O.
//! Raw token spans remain unchanged even when payload views omit wrappers.
//! Contains AI-generated documentation additions.

use super::model::LexContent;
use crate::model::{Spanned, TextSpan};

impl super::model::LexedBuffer {
    /// Read checkpoint nesting at a byte boundary; no side effects.
    /// Quoted/bracket payloads and comments do not affect depth. Inside a whole token
    /// the result is its preceding checkpoint state, not a character-by-character rescan.
    ///
    /// # Arguments
    /// - `self` `[in]`: lexical result with sorted checkpoints; unchanged.
    /// - `offset` `[in]`: source byte boundary to query; not validated against the source.
    ///
    /// # Returns
    /// Depth from the last checkpoint at or before offset, or zero if none.
    ///
    /// # Examples
    /// ```
    /// use cmake_analyzer::parser::lexer::model::{Lexer, RawBuffer};
    /// let result = Lexer::new(RawBuffer::new("f(x)")).parse();
    /// assert_eq!(result.parenthesis_depth(2), 1);
    /// ```
    pub fn parenthesis_depth(&self, offset: usize) -> usize {
        self.checkpoints[..self
            .checkpoints
            .partition_point(|checkpoint| checkpoint.state.position.start_byte <= offset)]
            .last()
            .map_or(0, |checkpoint| checkpoint.state.depth)
    }
}

impl Spanned<LexContent> {
    /// Derive a payload range without modifying the stored raw token span; no side effects.
    ///
    /// # Arguments
    /// - `self` `[in]`: located token; original span/kind remain unchanged.
    /// - `source` `[in]`: matching UTF-8 snapshot used to validate the raw span and wrappers.
    ///
    /// # Returns
    /// `Some` raw span for words/parentheses, quote-stripped span for strings, or bracket-stripped span with one initial LF/CRLF omitted. `None` for invalid raw bounds or rejected wrappers.
    ///
    /// # Algorithm
    /// Validate the raw slice. Leave unquoted forms unchanged. Remove ASCII quote or
    /// bracket delimiter widths, adjust the starting scalar column, then skip at most
    /// one initial bracket newline and update its row. This is a view, not unescaping.
    /// It assumes lexer-produced bracket tokens; it is not a replacement delimiter lexer.
    ///
    /// # Flow
    /// ```text
    ///           [validate raw span]
    ///                   |
    ///               <token kind?>
    ///            /      |       \
    ///       [word]   [quoted]  [bracket]
    ///          |        |         |
    ///    [unchanged] [trim 1] [trim delimiter]
    ///          |        |         |
    ///          |        |   [skip initial newline]
    ///          +--------+---------+
    ///                   |
    ///           [return payload span]
    /// ```
    ///
    /// # Examples
    /// ```
    /// use cmake_analyzer::parser::lexer::model::{Lexer, RawBuffer};
    /// let source = "[=[\n中]=]";
    /// let result = Lexer::new(RawBuffer::new(source)).parse();
    /// let payload = result.lex[0].payload_span(source).unwrap();
    /// assert_eq!(payload.text(source), Some("中"));
    /// assert_eq!((payload.row, payload.column), (1, 0));
    /// ```
    pub fn payload_span(&self, source: &str) -> Option<TextSpan> {
        let raw = self.span.text(source)?;
        let mut span = self.span;
        match self.content {
            LexContent::Identifier | LexContent::LeftParentheses | LexContent::RightParentheses => {
            }
            LexContent::StringLiteral => {
                raw.strip_prefix('"')?.strip_suffix('"')?;
                span.start_byte += 1;
                span.end_byte -= 1;
                span.column += 1;
            }
            LexContent::BracketArgument => {
                let bytes = raw.as_bytes();
                if bytes.first() != Some(&b'[') {
                    return None;
                }
                let opening = bytes[1..].iter().take_while(|byte| **byte == b'=').count() + 2;
                if bytes.get(opening - 1) != Some(&b'[') {
                    return None;
                }
                let closing = raw.len().checked_sub(opening)?;
                if closing < opening
                    || bytes.get(closing) != Some(&b']')
                    || bytes.last() != Some(&b']')
                    || !bytes[closing + 1..bytes.len() - 1]
                        .iter()
                        .all(|byte| *byte == b'=')
                {
                    return None;
                }
                span.start_byte += opening;
                span.end_byte -= opening;
                span.column += opening as u64;
                let payload = span.text(source)?;
                let newline = if payload.starts_with("\r\n") {
                    2
                } else if payload.starts_with('\n') {
                    1
                } else {
                    0
                };
                if newline > 0 {
                    span.start_byte += newline;
                    span.row += 1;
                    span.column = 0;
                }
            }
        }
        Some(span)
    }

    /// Borrow token payload text without allocation, expansion or other side effects.
    ///
    /// # Arguments
    /// - `self` `[in]`: located token whose raw span remains unchanged.
    /// - `source` `[in]`: matching source snapshot to borrow from.
    ///
    /// # Returns
    /// `Some` payload slice using [`Self::payload_span`] rules; `None` for invalid bounds or wrappers.
    ///
    /// # Examples
    /// ```
    /// use cmake_analyzer::parser::lexer::model::{Lexer, RawBuffer};
    /// let source = "\"hi\"";
    /// let result = Lexer::new(RawBuffer::new(source)).parse();
    /// assert_eq!(result.lex[0].text(source), Some("hi"));
    /// ```
    pub fn text<'a>(&self, source: &'a str) -> Option<&'a str> {
        self.payload_span(source)?.text(source)
    }
}
