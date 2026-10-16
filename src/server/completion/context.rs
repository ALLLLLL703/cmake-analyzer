use super::model::Site;
use crate::{
    parser::lexer::model::{LexContent, LexError},
    workspace::model::DocumentSnapshot,
};

pub(super) fn identifier(name: &str) -> bool {
    let mut bytes = name.bytes();
    bytes
        .next()
        .is_some_and(|byte| byte.is_ascii_alphabetic() || byte == b'_')
        && bytes.all(word)
}
fn word(byte: u8) -> bool {
    byte.is_ascii_alphanumeric() || byte == b'_'
}
pub(super) fn in_comment(snapshot: &DocumentSnapshot, offset: usize) -> bool {
    let text = &snapshot.text;
    let lexed = &snapshot.lexed;
    for span in &lexed.comments {
        let Some(raw) = span.text(text) else { continue };
        let block = raw
            .strip_prefix("#[")
            .is_some_and(|rest| rest.trim_start_matches('=').starts_with('['));
        let unclosed = offset == span.end_byte
            && block
            && lexed.errors.iter().any(|error| {
                matches!(error.content, LexError::UnclosedBracketComment)
                    && error.span.start_byte == span.start_byte
            });
        if span.start_byte <= offset
            && (offset < span.end_byte || offset == span.end_byte && (!block || unclosed))
        {
            return true;
        }
    }
    false
}
pub(super) fn site(snapshot: &DocumentSnapshot, offset: usize) -> Option<Site> {
    let text = &snapshot.text;
    let lexed = &snapshot.lexed;
    if in_comment(snapshot, offset) {
        return None;
    }
    if lexed.parenthesis_depth(offset) != 0 {
        return None;
    }
    if lexed.errors.iter().any(|error| {
        matches!(
            error.content,
            LexError::UnclosedStringLiteral | LexError::UnclosedBracketArgument
        ) && error.span.start_byte.saturating_sub(1) <= offset
            && offset <= error.span.end_byte
    }) {
        return None;
    }
    let token = lexed
        .lex
        .get(
            lexed
                .lex
                .partition_point(|token| token.span.end_byte < offset),
        )
        .filter(|token| token.span.start_byte <= offset && offset <= token.span.end_byte);
    if token.is_some_and(|token| {
        matches!(
            token.content,
            LexContent::StringLiteral | LexContent::BracketArgument
        ) && offset < token.span.end_byte
    }) {
        return None;
    }
    let bytes = text.as_bytes();
    let mut start = offset;
    let mut end = offset;
    while start > 0 && word(bytes[start - 1]) {
        start -= 1;
    }
    while end < bytes.len() && word(bytes[end]) {
        end += 1;
    }
    if start != end {
        if !identifier(&text[start..end]) {
            return None;
        }
        if token.is_some_and(|token| {
            token.content == LexContent::Identifier
                && (token.span.start_byte != start || token.span.end_byte != end)
        }) {
            return None;
        }
    } else if start > 0 && !bytes[start - 1].is_ascii_whitespace() && bytes[start - 1] != b')' {
        return None;
    }
    Some(Site {
        replace: start..end,
        prefix: text[start..offset].to_ascii_lowercase(),
        has_parentheses: text[end..].trim_start().starts_with('('),
    })
}
