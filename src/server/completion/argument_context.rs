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
