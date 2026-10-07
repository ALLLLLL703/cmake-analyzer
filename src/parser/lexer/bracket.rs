use crate::model::{Spanned, TextSpan};

use super::model::{LexError, LexResult, Lexer};

impl Lexer<'_> {
    /// Returns the byte length of a bracket opener at the current cursor.
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

    /// Called at a validated opener. Removes delimiters and one initial newline.
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
