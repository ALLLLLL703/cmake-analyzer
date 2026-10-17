//! Find argument payloads and ordinary variable-name replacements in retained text.
//! Byte spans exclude wrappers; all lookups are read-only and perform no I/O.

use super::{
    context,
    model::{ArgumentSite, Site},
};
use crate::{
    parser::{
        lexer::model::LexError,
        semanticer::{model::SemanticArgForm, parsing::references::argument_name_span},
    },
    workspace::model::DocumentSnapshot,
};

/// Locate an argument slot, recovering an unfinished literal when parsing omitted it.
/// Reads retained syntax only; no side effects.
///
/// # Arguments
/// - `snapshot` `[in]`: Matching source, lexer and semantic command arenas.
/// - `offset` `[in]`: Validated UTF-8 byte cursor.
///
/// # Returns
/// `Some(slot)` identifies a parsed payload, recovered literal payload, or empty insertion gap; `None` rejects comments, outside calls and wrapper interiors.
///
/// # Algorithm
/// Find the containing command after excluding comments and zero parenthesis depth. Prefer parsed payloads, reject delimiter interiors, then recover quote/bracket payload starts from lexer errors (skipping the bracket's first newline). Otherwise use the source-order insertion ordinal.
///
/// # Flow
/// ```text
///  [Cursor inside call?] --no--> [None]
///            | yes
///            v
///  <parsed payload?> --yes--> [Borrow command + slot]
///            | no
///            v
///  <unfinished literal?> --yes--> [Recover payload start]
///            | no                         |
///            v                            v
///  [Empty gap + ordinal]            [Recovered slot]
/// ```
///
/// # Examples
/// ```text
/// let slot = argument_context::argument(&snapshot, snapshot.text.len());
/// ```
pub(super) fn argument(snapshot: &DocumentSnapshot, offset: usize) -> Option<ArgumentSite<'_>> {
    if context::in_comment(snapshot, offset) || snapshot.lexed.parenthesis_depth(offset) == 0 {
        return None;
    }
    let command = snapshot.semantics.commands.iter().find(|command| {
        command.span.end_byte < offset
            && (offset < command.full_span.end_byte
                || !command.closed && command.full_span.start_byte <= offset)
    })?;
    for (argument, arg) in command.args.iter().enumerate() {
        let span = argument_name_span(&snapshot.text, arg);
        if span.start_byte <= offset && offset <= span.end_byte {
            return Some(ArgumentSite {
                command,
                argument,
                payload: span.range(),
                form: arg.form,
            });
        }
        if arg.span.start_byte <= offset
            && (offset < arg.span.end_byte
                || offset == arg.span.end_byte
                    && matches!(arg.form, SemanticArgForm::Quoted | SemanticArgForm::Bracket))
        {
            return None;
        }
    }
    // An unfinished literal is a lexer error, so nparser has no current argument yet.
    for error in &snapshot.lexed.errors {
        if error.span.start_byte < offset && offset <= error.span.end_byte {
            let (start, form) = match error.content {
                LexError::UnclosedStringLiteral => {
                    (error.span.start_byte + 1, SemanticArgForm::Quoted)
                }
                LexError::UnclosedBracketArgument => {
                    let raw = error.span.text(&snapshot.text)?;
                    let opening = raw.bytes().skip(1).take_while(|byte| *byte == b'=').count() + 2;
                    let mut start = error.span.start_byte + opening;
                    let tail = snapshot.text.get(start..)?;
                    start += if tail.starts_with("\r\n") {
                        2
                    } else {
                        usize::from(tail.starts_with('\n'))
                    };
                    (start, SemanticArgForm::Bracket)
                }
                _ => continue,
            };
            if offset < start {
                return None;
            }
            return Some(ArgumentSite {
                command,
                argument: command.args.len(),
                payload: start..error.span.end_byte,
                form,
            });
        }
    }
    let argument = command
        .args
        .partition_point(|arg| arg.span.end_byte < offset);
    Some(ArgumentSite {
        command,
        argument,
        payload: offset..offset,
        form: SemanticArgForm::Unquoted,
    })
}
/// Create a plain argument payload replacement while preserving its wrappers.
/// No I/O or mutation.
///
/// # Arguments
/// - `snapshot` `[in]`: Source and lexer corresponding to the argument.
/// - `argument` `[in]`: Payload span and command slot from argument().
/// - `offset` `[in]`: Validated byte cursor within that payload.
///
/// # Returns
/// `Some(site)` replaces the entire payload with a case-preserving prefix; `None` rejects nested parentheses, invalid ranges, whitespace, escapes, lists or expansion syntax.
///
/// # Examples
/// ```text
/// let site = argument_context::bare(&snapshot, &slot, offset);
/// ```
pub(super) fn bare(
    snapshot: &DocumentSnapshot,
    argument: &ArgumentSite<'_>,
    offset: usize,
) -> Option<Site> {
    if snapshot.lexed.parenthesis_depth(offset) != 1 {
        return None;
    }
    let raw = snapshot.text.get(argument.payload.clone())?;
    if raw.chars().any(|character| {
        character.is_whitespace()
            || matches!(character, '$' | '{' | '}' | '\\' | ';' | '"' | '(' | ')')
    }) {
        return None;
    }
    Some(Site {
        replace: argument.payload.clone(),
        prefix: snapshot
            .text
            .get(argument.payload.start..offset)?
            .to_owned(),
        has_parentheses: false,
    })
}
/// Innermost ordinary ${...} only; escaped, ENV/CACHE, and bracket-literal references are excluded.
/// Find only the innermost ordinary `${...}` name; no side effects.
/// Escaped starts, ENV/CACHE namespaces and bracket-literal references are excluded.
///
/// # Arguments
/// - `snapshot` `[in]`: Source text matching the argument payload.
/// - `argument` `[in]`: Argument form and payload bounds from argument().
/// - `offset` `[in]`: Validated byte cursor inside the payload.
///
/// # Returns
/// `Some(site)` replaces only the variable name, preserving `${`, `}` and surrounding text; `None` means no supported innermost name.
///
/// # Panics
/// Invalid internal byte bounds can panic while indexing/slicing. The caller supplies a slot from the same snapshot and a validated cursor.
///
/// # Algorithm
/// Scan from payload start to the cursor with a stack: ordinary openings store name starts, ENV/CACHE openings store an excluded marker, closing braces pop, and escapes skip a byte. Use only the stack top and stop the replacement at the next brace or payload end. Nested syntax remaining in the name rejects the site.
///
/// # Flow
/// ```text
///  [Payload bytes before cursor]
///                 |
///                 v
///          <next byte sequence?>
///           /       |        \
///     [escape]  [opening]   [closing]
///        |          |          |
///     [skip]   [push start/   [pop]
///        |      excluded]      |
///        +----------+----------+
///                   | repeat
///                   v
///         [Top ordinary name?] --no--> [None]
///                   | yes
///                   v
///        [Name-only replacement span]
/// ```
///
/// # Examples
/// ```text
/// let variable_site = argument_context::expansion(&snapshot, &slot, offset);
/// ```
pub(super) fn expansion(
    snapshot: &DocumentSnapshot,
    argument: &ArgumentSite<'_>,
    offset: usize,
) -> Option<Site> {
    if argument.form == SemanticArgForm::Bracket {
        return None;
    }
    let bytes = snapshot.text.as_bytes();
    let mut cursor = argument.payload.start;
    let mut stack = Vec::new();
    while cursor < offset {
        if bytes[cursor] == b'\\' {
            cursor += 2;
            continue;
        }
        if bytes[cursor..offset].starts_with(b"${") {
            stack.push(Some(cursor + 2));
            cursor += 2;
        } else if bytes[cursor..offset].starts_with(b"$ENV{") {
            stack.push(None);
            cursor += 5;
        } else if bytes[cursor..offset].starts_with(b"$CACHE{") {
            stack.push(None);
            cursor += 7;
        } else {
            if bytes[cursor] == b'}' {
                stack.pop();
            }
            cursor += 1;
        }
    }
    let start = stack.last().copied().flatten()?;
    let tail = snapshot.text.get(start..argument.payload.end)?;
    let end = start + tail.find('}').unwrap_or(tail.len());
    if offset > end || snapshot.text[start..end].contains(['$', '{', '\\']) {
        return None;
    }
    Some(Site {
        replace: start..end,
        prefix: snapshot.text.get(start..offset)?.to_owned(),
        has_parentheses: false,
    })
}
