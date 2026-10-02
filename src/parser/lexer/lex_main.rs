use super::{
    arguments::{bracket_end, bracket_open, quoted_end, unquoted_end},
    cursor::Cursor,
    model::{LexBuffer, LexError, LexErrorKind, LexKind, LexPart, PreBuffer},
};
use crate::model::trait1::IBuffer;

pub(super) fn scan(buffer: PreBuffer<'_>) -> LexBuffer<'_> {
    let mut result = LexBuffer {
        text: buffer.text,
        parts: Vec::new(),
        errors: Vec::new(),
    };
    if !buffer.text.is_char_boundary(buffer.current_byte) {
        result.errors.push(LexError {
            kind: LexErrorKind::InvalidOffset,
            span: buffer.current_byte_to_span(),
        });
        return result;
    }

    let mut cursor = Cursor::new(buffer);
    while cursor.offset < cursor.text.len() {
        let (kind, end, error) = next_part(cursor.text, cursor.offset);
        if let Some(kind) = error {
            result.errors.push(LexError {
                kind,
                span: cursor.span(end),
            });
        }
        result.parts.push(LexPart {
            kind,
            span: cursor.span(end),
        });
        cursor.advance_to(end);
    }
    result
}

fn next_part(text: &str, start: usize) -> (LexKind, usize, Option<LexErrorKind>) {
    let bytes = text.as_bytes();
    if start == 0 && text.starts_with('\u{feff}') {
        return (LexKind::Bom, 3, None);
    }
    match bytes[start] {
        b' ' | b'\t' | b'\r' if !is_crlf(bytes, start) => {
            let mut end = start + 1;
            while matches!(bytes.get(end), Some(b' ' | b'\t' | b'\r')) && !is_crlf(bytes, end) {
                end += 1;
            }
            (LexKind::Space, end, None)
        }
        b'\n' => (LexKind::Newline, start + 1, None),
        b'\r' => (LexKind::Newline, start + 2, None),
        b'(' => (LexKind::ParenLeft, start + 1, None),
        b')' => (LexKind::ParenRight, start + 1, None),
        b'"' => match quoted_end(bytes, start) {
            Some(end) => (LexKind::ArgumentQuoted, end, None),
            None => (
                LexKind::ArgumentQuoted,
                bytes.len(),
                Some(LexErrorKind::UnterminatedQuotedArgument),
            ),
        },
        b'#' => {
            if let Some((content, equals)) = bracket_open(bytes, start + 1) {
                bracket_part(bytes, content, equals, true)
            } else {
                let mut end = start + 1;
                while end < bytes.len() && bytes[end] != b'\n' && !is_crlf(bytes, end) {
                    end += 1;
                }
                (LexKind::CommentLine, end, None)
            }
        }
        b'[' if bracket_open(bytes, start).is_some() => {
            // The opening delimiter has already been recognized.
            let mut content = start + 1;
            while bytes[content] == b'=' {
                content += 1;
            }
            bracket_part(bytes, content + 1, content - start - 1, false)
        }
        _ => {
            let end = unquoted_end(text, start);
            if end == start {
                return (
                    LexKind::Invalid,
                    start + 1,
                    Some(LexErrorKind::UnexpectedCharacter),
                );
            }
            let word = &bytes[start..end];
            let identifier = (word[0].is_ascii_alphabetic() || word[0] == b'_')
                && word
                    .iter()
                    .all(|byte| byte.is_ascii_alphanumeric() || *byte == b'_');
            let kind = if identifier {
                LexKind::Identifier
            } else {
                LexKind::ArgumentUnquoted
            };
            (kind, end, None)
        }
    }
}

fn bracket_part(
    bytes: &[u8],
    content: usize,
    equals: usize,
    comment: bool,
) -> (LexKind, usize, Option<LexErrorKind>) {
    let kind = if comment {
        LexKind::CommentBracket
    } else {
        LexKind::ArgumentBracket
    };
    match bracket_end(bytes, content, equals) {
        Some(end) => (kind, end, None),
        None => (
            kind,
            bytes.len(),
            Some(if comment {
                LexErrorKind::UnterminatedBracketComment
            } else {
                LexErrorKind::UnterminatedBracketArgument
            }),
        ),
    }
}

fn is_crlf(bytes: &[u8], offset: usize) -> bool {
    bytes.get(offset) == Some(&b'\r') && bytes.get(offset + 1) == Some(&b'\n')
}
