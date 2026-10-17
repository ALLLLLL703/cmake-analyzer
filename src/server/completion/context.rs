//! Recognize top-level command-name sites using shared lexer tokens and comments.
//! Offsets and replacement ranges are UTF-8 bytes, not client character coordinates. No I/O occurs.

use super::model::Site;
use crate::{
    parser::lexer::model::{LexContent, LexError},
    workspace::model::DocumentSnapshot,
};

/// Check the ASCII command-name spelling accepted by completion; no side effects.
///
/// # Arguments
/// - `name` `[in]`: Whole candidate spelling; empty names and non-ASCII letters are rejected.
///
/// # Returns
/// `true` for a letter/underscore followed only by letters, digits or underscores; `false` otherwise.
///
/// # Examples
/// ```text
/// assert!(context::identifier("my_command2"));
/// assert!(!context::identifier("2bad"));
/// ```
pub(super) fn identifier(name: &str) -> bool {
    let mut bytes = name.bytes();
    bytes
        .next()
        .is_some_and(|byte| byte.is_ascii_alphabetic() || byte == b'_')
        && bytes.all(word)
}
/// Check one byte for command-name continuation; no side effects.
///
/// # Arguments
/// - `byte` `[in]`: Potential ASCII letter, digit or underscore.
///
/// # Returns
/// `true` for ASCII alphanumeric bytes or `_`; `false` otherwise.
///
/// # Examples
/// ```text
/// assert!(word(b'_'));
/// assert!(!word(b'-'));
/// ```
fn word(byte: u8) -> bool {
    byte.is_ascii_alphanumeric() || byte == b'_'
}
/// Test whether a byte cursor is inside a lexer-recorded comment; no side effects.
///
/// # Arguments
/// - `snapshot` `[in]`: Matching immutable text, comment spans and lexer errors.
/// - `offset` `[in]`: UTF-8 byte cursor, including a possible comment-end insertion point.
///
/// # Returns
/// `true` inside a comment, at a line-comment end, or at an unclosed bracket-comment end; a closed bracket-comment end is outside.
///
/// # Algorithm
/// Read lexer-recorded spans rather than scanning source again. Comment interiors are
/// suppressed. At an end boundary, line comments and unfinished bracket comments stay
/// suppressed, but a completed bracket comment permits a following command.
///
/// # Flow
/// ```text
/// [Recorded comment] ---> <cursor in interior?> --yes--> [true]
///                                  | no
///                                  v
///                            <cursor at end?> --no--> [Next span]
///                                  | yes
///                                  v
///                     <line or unfinished bracket?>
///                             /             \
///                          [true]       [Next span]
/// ```
///
/// # Examples
/// ```text
/// let suppressed = context::in_comment(&snapshot, snapshot.text.len());
/// ```
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
/// Find a safe top-level command-name edit, retaining any following call syntax.
/// Read-only; no I/O or source mutation.
///
/// # Arguments
/// - `snapshot` `[in]`: Matching immutable lexer and source text.
/// - `offset` `[in]`: In-bounds UTF-8 character boundary, normally validated by TextIndex::byte.
///
/// # Returns
/// `Some(site)` replaces the whole ASCII identifier and lowercases only the prefix before the cursor; `None` rejects comments, arguments, literals and invalid token boundaries.
///
/// # Panics
/// An offset past the source or not at a character boundary can panic during byte indexing or string slicing; callers validate it first.
///
/// # Algorithm
/// Exclude comments, nonzero parenthesis depth and literal interiors. Extend ASCII word boundaries on both sides of the cursor, validate against the lexer token, then detect a following opening parenthesis without consuming it.
///
/// # Flow
/// ```text
///  [Byte cursor]
///        |
///        v
///  <comment/argument/literal?> --yes--> [None]
///        | no
///        v
///  [Extend word boundaries]
///        |
///  <valid identifier/boundary?> --no--> [None]
///        | yes
///        v
///  [Site: full range + prefix + existing-call flag]
/// ```
///
/// # Examples
/// ```text
/// let site = context::site(&snapshot, snapshot.text.len());
/// ```
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
