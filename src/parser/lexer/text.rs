use super::model::LexContent;
use crate::model::{Spanned, TextSpan};

impl Spanned<LexContent> {
    /// Payload view; the stored span continues to cover the complete token.
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

    pub fn text<'a>(&self, source: &'a str) -> Option<&'a str> {
        self.payload_span(source)?.text(source)
    }
}
