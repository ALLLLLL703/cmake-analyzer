//! Full and resumable lexical scanning; advances bytes, never edits source or performs I/O.
//! Complete token spans include wrappers. Malformed literals stop scanning; unexpected
//! closing parentheses produce errors and scanning continues.
//! Contains AI-generated documentation additions.

use crate::{
    model::{Spanned, TextSpan},
    parser::lexer::model::{LexContent, LexError, LexResult, LexedBuffer, Lexer, RawBuffer},
};

impl<'a> Lexer<'a> {
    /// Own a raw buffer wrapper without scanning or other side effects.
    ///
    /// # Arguments
    /// - `buffer` `[in]`: borrowed source and initial cursor moved into the scanner.
    ///
    /// # Returns
    /// Lexer at the supplied byte offset.
    ///
    /// # Examples
    /// ```
    /// use cmake_analyzer::parser::lexer::model::{Lexer, RawBuffer};
    /// let lexer = Lexer::new(RawBuffer::new("f()"));
    /// assert_eq!(lexer.buffer.cursor.offset, 0);
    /// ```
    pub fn new(buffer: RawBuffer<'a>) -> Self {
        Lexer { buffer }
    }

    /// Scan from the current cursor to EOF, advancing only that cursor; no I/O.
    /// Each call starts parenthesis depth at zero; use a fresh lexer for a full parse.
    ///
    /// # Arguments
    /// - `self` `[in,out]`: scanner whose byte cursor advances through complete units or to EOF.
    ///
    /// # Returns
    /// New tokens, comments, errors and checkpoints for this scan. Normal EOF emits no EOF error; unterminated literals stop scanning.
    ///
    /// # Algorithm
    /// Initialize the scalar position at the cursor and delegate to the resumable scanner
    /// with a predicate that never stops early.
    ///
    /// # Flow
    /// ```text
    ///  [current cursor]     [depth = 0]
    ///          +----------------+
    ///                   |
    ///              [parse_until]
    ///                   |
    ///              <terminated?>
    ///               /         \
    ///         [normal EOF]   [literal error]
    ///               \         /
    ///                [return output]
    /// ```
    ///
    /// # Examples
    /// ```
    /// use cmake_analyzer::parser::lexer::model::{Lexer, RawBuffer};
    /// let mut lexer = Lexer::new(RawBuffer::new(r#"f("中") # note"#));
    /// let result = lexer.parse();
    /// assert_eq!(result.lex.len(), 4);
    /// assert_eq!(result.comments.len(), 1);
    /// ```
    pub fn parse(&mut self) -> LexedBuffer {
        let state = super::model::LexState {
            position: self.buffer.current_byte_to_span(),
            ..Default::default()
        };
        self.parse_until(state, |_| false)
    }

    // AI-authored resumable scan. Stop only between whole tokens/comments, before whitespace.
    /// Resume whole-unit scanning, advancing the byte cursor and invoking stop at boundaries; no I/O.
    ///
    /// # Arguments
    /// - `self` `[in,out]`: scanner whose cursor must agree with state.position; advanced in place.
    /// - `state` `[in]`: starting point, parenthesis depth and most recent opener.
    /// - `stop` `[in,out]`: callback invoked before whitespace at every checkpoint; may mutate captured alignment state.
    ///
    /// # Returns
    /// New chunk of tokens/comments/errors and chunk-relative checkpoint counts. A true stop suppresses final unmatched-opening diagnostics.
    ///
    /// # Algorithm
    /// Record checkpoint state before whitespace. Skip ASCII whitespace, then consume one
    /// whole token/comment and advance LF/scalar coordinates. Parentheses in payloads do
    /// not affect depth. Unexpected closes emit errors without tokens; malformed literals
    /// emit errors and stop. Only a non-stopped scan reports outstanding opening depth.
    ///
    /// # Flow
    /// ```text
    ///            [checkpoint]
    ///                  |
    ///              <stop?> ---- yes ---> [return chunk]
    ///                  | no
    ///           [skip whitespace]
    ///                  |
    ///              <unit?>
    ///         /        |         \
    ///    [EOF/error] [comment]  [token]
    ///         |        |         |
    ///  [final depth]   |    [update nesting]
    ///         |        +----+----+
    ///      [return]         |
    ///                 [advance position]
    ///                       |
    ///                       +----> [checkpoint]
    /// ```
    ///
    /// # Panics
    /// An inconsistent starting state or manually corrupted byte cursor can make source
    /// slices invalid. Supply whole-unit checkpoints from the same source prefix.
    ///
    /// # Examples
    /// ```text
    /// let state = LexState::default();
    /// let mut lexer = Lexer::new(RawBuffer::new("f()"));
    /// let chunk = lexer.parse_until(state, |s| s.position.start_byte >= 3);
    /// ```
    pub(super) fn parse_until(
        &mut self,
        state: super::model::LexState,
        mut stop: impl FnMut(super::model::LexState) -> bool,
    ) -> LexedBuffer {
        let mut result = LexedBuffer::default();
        let mut left_parentheses_num = state.depth;
        let mut last_parentheses_loc = state.last_opening;
        let mut position = state.position;
        let mut stopped = false;
        loop {
            let state = super::model::LexState {
                position,
                depth: left_parentheses_num,
                last_opening: last_parentheses_loc,
            };
            result.checkpoints.push(super::model::LexCheckpoint {
                state,
                tokens: result.lex.len(),
                errors: result.errors.len(),
                comments: result.comments.len(),
            });
            if stop(state) {
                stopped = true;
                break;
            }
            if let Err(error) = self.skip_whitespace() {
                result.add_error(error.content, error.span);
                break;
            }

            let whitespace_start = position.start_byte;
            advance_position(
                &mut position,
                &self.buffer.text[whitespace_start..self.buffer.cursor.offset],
            );
            let mut span = position;
            let content = match self.peek() {
                Err(error) if matches!(error.content, LexError::ReachTheEof) => break,
                Err(error) => {
                    result.add_error(error.content, error.span);
                    break;
                }
                Ok('"') => {
                    // Consume the opening quote; the scanner consumes the closing one.
                    self.buffer.cursor.offset += 1;
                    match self.advance_string_literal() {
                        Ok(()) => Some(LexContent::StringLiteral),
                        Err(error) => {
                            result.add_error(error.content, error.span);
                            break;
                        }
                    }
                }
                // AI-authored comment handling; quoted/bracket argument payloads never reach this branch.
                Ok('#') => {
                    self.buffer.cursor.offset += 1;
                    if let Some(opening_len) = self.bracket_opening_len() {
                        if let Err(mut error) = self.advance_bracket_argument(opening_len, span) {
                            result.comments.push(TextSpan {
                                end_byte: self.buffer.cursor.offset,
                                ..span
                            });
                            error.content = LexError::UnclosedBracketComment;
                            result.add_error(error.content, error.span);
                            break;
                        }
                    } else {
                        self.buffer.cursor.offset += self.buffer.text[self.buffer.cursor.offset..]
                            .bytes()
                            .take_while(|byte| *byte != b'\n')
                            .count();
                    }
                    result.comments.push(TextSpan {
                        end_byte: self.buffer.cursor.offset,
                        ..span
                    });
                    None
                }
                Ok('[') => {
                    let token = if let Some(opening_len) = self.bracket_opening_len() {
                        self.advance_bracket_argument(opening_len, span)
                            .map(|()| LexContent::BracketArgument)
                    } else {
                        self.advance_identifier().map(|()| LexContent::Identifier)
                    };
                    match token {
                        Ok(content) => Some(content),
                        Err(error) => {
                            result.add_error(error.content, error.span);
                            break;
                        }
                    }
                }
                Ok('(') => {
                    self.buffer.cursor.offset += 1;
                    left_parentheses_num += 1;
                    last_parentheses_loc = TextSpan {
                        end_byte: self.buffer.cursor.offset,
                        ..span
                    };
                    Some(LexContent::LeftParentheses)
                }
                Ok(')') => {
                    self.buffer.cursor.offset += 1;
                    if left_parentheses_num > 0 {
                        left_parentheses_num -= 1;
                        Some(LexContent::RightParentheses)
                    } else {
                        result.add_error(
                            LexError::UnclosedParentheses,
                            TextSpan {
                                end_byte: self.buffer.cursor.offset,
                                ..span
                            },
                        );
                        None
                    }
                }
                Ok(_) => match self.advance_identifier() {
                    Ok(()) => Some(LexContent::Identifier),
                    Err(error) => {
                        result.add_error(error.content, error.span);
                        break;
                    }
                },
            };
            span.end_byte = self.buffer.cursor.offset;
            advance_position(
                &mut position,
                &self.buffer.text[span.start_byte..span.end_byte],
            );
            if let Some(content) = content {
                result.add_lex(Spanned { content, span });
            }
        }

        if !stopped && left_parentheses_num > 0 {
            result.add_error(LexError::UnclosedParentheses, last_parentheses_loc);
        }
        result
    }

    /// Skip ASCII space, tab, LF and CR, leaving the cursor at the first other byte; no I/O.
    ///
    /// # Arguments
    /// - `self` `[in,out]`: scanner whose byte offset increases by skipped bytes; source unchanged.
    ///
    /// # Returns
    /// `Ok(())` even at normal EOF; `Err` if the current suffix cannot be sliced.
    ///
    /// # Errors
    /// Returns `ReachTheEof` without changing the cursor for an invalid slice boundary or
    /// offset beyond the source; a valid empty suffix succeeds.
    ///
    /// # Examples
    /// ```
    /// use cmake_analyzer::parser::lexer::model::{Lexer, RawBuffer};
    /// let mut lexer = Lexer::new(RawBuffer::new(" \t中"));
    /// lexer.skip_whitespace().unwrap();
    /// assert_eq!(lexer.buffer.cursor.offset, 2);
    /// ```
    pub fn skip_whitespace(&mut self) -> LexResult<()> {
        let rest = self
            .buffer
            .text
            .get(self.buffer.cursor.offset..)
            .ok_or_else(|| self.eof_error())?;
        let skipped = rest
            .bytes()
            .take_while(|byte| matches!(byte, b' ' | b'\t' | b'\n' | b'\r'))
            .count();
        self.buffer.cursor.offset += skipped;
        Ok(())
    }

    /// Read and consume one Unicode scalar value; no I/O or source mutation.
    ///
    /// # Arguments
    /// - `self` `[in,out]`: scanner whose offset advances by the character UTF-8 width on success.
    ///
    /// # Returns
    /// `Ok(character)` read before advancing; `Err` when no character is available.
    ///
    /// # Errors
    /// Returns located `ReachTheEof` at EOF or an invalid cursor boundary; the offset is unchanged on error.
    ///
    /// # Examples
    /// ```
    /// use cmake_analyzer::parser::lexer::model::{Lexer, RawBuffer};
    /// let mut lexer = Lexer::new(RawBuffer::new("中"));
    /// assert_eq!(lexer.advance().unwrap(), '中');
    /// assert_eq!(lexer.buffer.cursor.offset, 3);
    /// ```
    pub fn advance(&mut self) -> LexResult<char> {
        let ch = self
            .buffer
            .text
            .get(self.buffer.cursor.offset..)
            .and_then(|rest| rest.chars().next())
            .ok_or_else(|| self.eof_error())?;
        self.buffer.cursor.offset += ch.len_utf8();
        Ok(ch)
    }

    /// Read the next Unicode scalar without advancing; no side effects.
    ///
    /// # Arguments
    /// - `self` `[in]`: scanner whose source/cursor are inspected; unchanged.
    ///
    /// # Returns
    /// `Ok(character)` at a readable boundary; `Err` when no character is available.
    ///
    /// # Errors
    /// Returns located `ReachTheEof` for EOF, an invalid UTF-8 boundary or an out-of-range cursor.
    ///
    /// # Examples
    /// ```
    /// use cmake_analyzer::parser::lexer::model::{Lexer, RawBuffer};
    /// let lexer = Lexer::new(RawBuffer::new("中"));
    /// assert_eq!(lexer.peek().unwrap(), '中');
    /// assert_eq!(lexer.buffer.cursor.offset, 0);
    /// ```
    pub fn peek(&self) -> LexResult<char> {
        self.buffer
            .text
            .get(self.buffer.cursor.offset..)
            .and_then(|rest| rest.chars().next())
            .ok_or_else(|| self.eof_error())
    }

    /// Consume an unquoted word starting at the current byte, retaining escapes and embedded quotes; no I/O.
    ///
    /// # Arguments
    /// - `self` `[in,out]`: scanner advanced to the first unescaped separator outside embedded quotes; source unchanged.
    ///
    /// # Returns
    /// `Ok(())` after scanning (possibly zero bytes at a separator); `Err` if no nonempty valid suffix exists.
    ///
    /// # Errors
    /// Returns `ReachTheEof` without advancing for EOF or an invalid suffix. Unclosed
    /// legacy embedded quotes are not diagnosed by this helper.
    ///
    /// # Algorithm
    /// Scan bytes, toggling embedded-quote state. Backslash shields the next byte, or
    /// both CRLF bytes. Stop at whitespace, parentheses or `#` only outside quotes.
    /// Normal callers start at a non-separator UTF-8 boundary.
    ///
    /// # Flow
    /// ```text
    ///              [next byte]
    ///                   |
    ///               <kind?>
    ///       /           |            \
    ///  [backslash]    [quote]     [separator]
    ///       |           |             |
    ///  [skip escape] [toggle quoted] <outside?>
    ///       |           |         /       \
    ///       +-----------+       yes       no
    ///                   |        |        |
    ///                   |     [stop]      |
    ///                   +<----------------+
    ///                   |
    ///               [next byte]
    /// ```
    ///
    /// # Examples
    /// ```
    /// use cmake_analyzer::parser::lexer::model::{Lexer, RawBuffer};
    /// let mut lexer = Lexer::new(RawBuffer::new("hello world"));
    /// lexer.advance_identifier().unwrap();
    /// assert_eq!(lexer.buffer.cursor.offset, 5);
    /// ```
    pub fn advance_identifier(&mut self) -> LexResult<()> {
        let rest = self
            .buffer
            .text
            .get(self.buffer.cursor.offset..)
            .filter(|rest| !rest.is_empty())
            .ok_or_else(|| self.eof_error())?;
        // AI-generated: escaped '#' and legacy embedded quotes are argument text.
        let bytes = rest.as_bytes();
        let mut length = 0;
        let mut quoted = false;
        while length < bytes.len() {
            match bytes[length] {
                b'\\' => {
                    length += 1;
                    if bytes.get(length) == Some(&b'\r') && bytes.get(length + 1) == Some(&b'\n') {
                        // CRLF is one escaped newline, just like LF.
                        length += 2;
                    } else if length < bytes.len() {
                        length += 1;
                    }
                }
                b'"' => {
                    quoted = !quoted;
                    length += 1;
                }
                b' ' | b'\t' | b'\n' | b'\r' | b'(' | b')' | b'#' if !quoted => break,
                _ => length += 1,
            }
        }
        self.buffer.cursor.offset += length;
        Ok(())
    }

    /// Consume quoted content after its opening quote, including the closing quote; no I/O.
    ///
    /// # Arguments
    /// - `self` `[in,out]`: scanner positioned after an opening quote; advances past close or to EOF.
    ///
    /// # Returns
    /// `Ok(())` after the closing quote; `Err` with opening-to-EOF span if not closed.
    ///
    /// # Errors
    /// Invalid suffix returns `ReachTheEof` without advancing. Missing closing quote
    /// returns `UnclosedStringLiteral` after setting the cursor to source EOF.
    ///
    /// # Algorithm
    /// Walk scalars while tracking whether the previous backslash shields this character.
    /// The first unescaped quote ends the unit; escapes remain in the source unchanged.
    ///
    /// # Flow
    /// ```text
    ///              [next scalar]
    ///                    |
    ///                <escaped?>
    ///              yes /    \ no
    ///        [clear flag]    <kind?>
    ///              |        /      \
    ///              |    [slash]  [quote]
    ///              |        |      |
    ///              |   [set flag] [return Ok]
    ///              +--------+
    ///                       |
    ///                  [next scalar]
    ///                       | EOF
    ///              [cursor = EOF; Err]
    /// ```
    ///
    /// # Examples
    /// ```
    /// use cmake_analyzer::parser::lexer::model::{Lexer, RawBuffer};
    /// let mut lexer = Lexer::new(RawBuffer::new("\"hi\""));
    /// lexer.advance().unwrap();
    /// lexer.advance_string_literal().unwrap();
    /// assert_eq!(lexer.buffer.cursor.offset, 4);
    /// ```
    pub fn advance_string_literal(&mut self) -> LexResult<()> {
        let start = self.buffer.cursor.offset;
        let rest = self
            .buffer
            .text
            .get(start..)
            .ok_or_else(|| self.eof_error())?;
        let mut escaped = false;
        for (index, ch) in rest.char_indices() {
            if escaped {
                escaped = false;
                continue;
            }
            match ch {
                '\\' => escaped = true,
                '"' => {
                    let end = start + index;
                    self.buffer.cursor.offset = end + 1;
                    return Ok(());
                }
                _ => {}
            }
        }
        self.buffer.cursor.offset = self.buffer.text.len();
        let mut opening = RawBuffer::new(self.buffer.text);
        opening.cursor.offset = start.saturating_sub(1);
        let mut span = opening.current_byte_to_span();
        span.end_byte = self.buffer.text.len();
        Err(Spanned {
            content: LexError::UnclosedStringLiteral,
            span,
        })
    }

    /// Construct a point-located EOF signal without advancing; no side effects.
    ///
    /// # Arguments
    /// - `self` `[in]`: source/cursor used to reconstruct the location; unchanged.
    ///
    /// # Returns
    /// Spanned `ReachTheEof` at the current byte offset.
    ///
    /// # Examples
    /// ```text
    /// let lexer = Lexer::new(RawBuffer::new(""));
    /// let error = lexer.eof_error();
    /// assert!(matches!(error.content, LexError::ReachTheEof));
    /// ```
    fn eof_error(&self) -> Spanned<LexError> {
        Spanned {
            content: LexError::ReachTheEof,
            span: self.buffer.current_byte_to_span(),
        }
    }
}

// Track only newly consumed text instead of rescanning the source for every token.
/// Advance scalar coordinates over newly consumed text; no I/O.
///
/// # Arguments
/// - `position` `[in,out]`: point overwritten with advanced byte offset, LF row and scalar column.
/// - `consumed` `[in]`: exact newly consumed source slice; not retained or changed.
///
/// # Returns
/// `()`; end_byte is overwritten to equal the new start_byte.
///
/// # Examples
/// ```text
/// let mut position = TextSpan::default();
/// advance_position(&mut position, "中\nx");
/// assert_eq!((position.start_byte, position.row, position.column), (5, 1, 1));
/// ```
fn advance_position(position: &mut TextSpan, consumed: &str) {
    for ch in consumed.chars() {
        if ch == '\n' {
            position.row += 1;
            position.column = 0;
        } else {
            position.column += 1;
        }
    }
    position.start_byte += consumed.len();
    position.end_byte = position.start_byte;
}
