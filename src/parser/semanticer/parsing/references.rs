//! Extract simple variable reads and schema path roles before binding names.
//!
//! Nested/dynamic variable names are not resolved; escaped and bracket text stays literal.
// Contains AI-generated documentation additions.
// AI-authored variable-reference extraction before binding.
use super::outputs::variable_arguments;
use crate::{
    model::{Spanned, TextSpan, incremental::SourcePositions},
    parser::{lexer::model::LexContent, semanticer::model::*},
};

/// Remove lexical argument delimiters from a source name range when possible. No side effects.
///
/// # Arguments
/// - `source` `[in]`: owning source snapshot.
/// - `arg` `[in]`: semantic argument with raw span and lexical form.
///
/// # Returns
/// Payload span for quoted/bracket names; original raw span if payload extraction fails or no delimiters apply.
///
/// # Examples
/// ```text
/// let span = argument_name_span(source, &command.args[0]);
/// ```
pub(crate) fn argument_name_span(source: &str, arg: &SemanticArg) -> TextSpan {
    let content = match arg.form {
        SemanticArgForm::Quoted => LexContent::StringLiteral,
        SemanticArgForm::Bracket => LexContent::BracketArgument,
        _ => LexContent::Identifier,
    };
    Spanned::new(content, arg.span)
        .payload_span(source)
        .unwrap_or(arg.span)
}
/// Overwrite unbound variable reads/path roles and refresh ReferencingVar flags. Scans only simple source substitutions and explicit schema-variable inputs; no I/O or execution.
///
/// # Arguments
/// - `source` `[in]`: source matching command/argument spans.
/// - `file` `[in]`: owning file identity.
/// - `command` `[in,out]`: replaces variable_references/paths and removes/readds ReferencingVar flags; other roles remain.
///
/// # Returns
/// () after extraction. Unclosed/declaration commands keep expansion reads but skip explicit schema-variable inputs.
///
/// # Algorithm
/// Classify schema paths, then scan quoted/unquoted raw bytes for unescaped simple ${name} forms. Skip empty/dynamic names and translate local offsets into source spans. Finally add literal schema-variable reads unless an earlier unquoted dynamic/list argument could change slot correspondence.
///
/// # Flow
/// ```text
///  [raw args]             [schema]
///      |                      |
///      v                      v
///  <quoted/unquoted?>    [path/variable slots]
///    /          \             |
///  [skip] [simple ${name}]      |
///              |               |
///       [source read spans]    |
///              +-------+-------+
///                      v
///        [filter unstable source slots]
///                      |
///                      v
///            [reads + role flags]
/// ```
///
/// # Examples
/// ```text
/// scan_command(source, FileId { id: 1 }, &mut command);
/// ```
pub(super) fn scan_command(source: &str, file: FileId, command: &mut SemanticCommandRepresent) {
    command.variable_references.clear();
    command.paths = super::outputs::path_arguments(command);
    for (argument, arg) in command.args.iter_mut().enumerate() {
        arg.kind
            .retain(|kind| !matches!(kind, ArgKind::ReferencingVar));
        if !matches!(
            arg.form,
            SemanticArgForm::Quoted | SemanticArgForm::Unquoted
        ) {
            continue;
        }
        let Some(raw) = arg.span.text(source) else {
            continue;
        };
        if !raw.contains("${") {
            continue;
        }
        let positions = SourcePositions::new(raw);
        let bytes = raw.as_bytes();
        let mut cursor = 0;
        while cursor + 1 < bytes.len() {
            if bytes[cursor] == b'\\' {
                cursor += 2;
                continue;
            }
            if bytes[cursor..].starts_with(b"${") {
                let start = cursor + 2;
                if let Some(end) = raw[start..].find('}').map(|offset| start + offset) {
                    let name = &raw[start..end];
                    if !name.is_empty() && !name.contains(['$', '{', '\\']) {
                        let mut span = positions.span(start..end);
                        if span.row == 0 {
                            span.column += arg.span.column;
                        }
                        span.row += arg.span.row;
                        span.start_byte += arg.span.start_byte;
                        span.end_byte += arg.span.start_byte;
                        command.variable_references.push(ArgumentReference {
                            name: name.to_owned(),
                            argument,
                            location: ReferenceLocation {
                                file,
                                command: command.id,
                                span,
                                kind: ReferenceKind::Read,
                                possible_value: None,
                            },
                        });
                        cursor = end + 1;
                        continue;
                    }
                }
            }
            cursor += 1;
        }
        if command
            .variable_references
            .last()
            .is_some_and(|reference| reference.argument == argument)
        {
            arg.kind.push(ArgKind::ReferencingVar);
        }
    }
    if !command.closed
        || matches!(
            command.name.to_ascii_lowercase().as_str(),
            "function" | "macro"
        )
    {
        return;
    }
    for argument in variable_arguments(command) {
        if command.args[..argument]
            .iter()
            .any(|arg| arg.form == SemanticArgForm::Unquoted && arg.content.contains(['$', ';']))
        {
            continue;
        }
        let arg = &mut command.args[argument];
        let Some(name) = arg.literal_text().filter(|name| !name.is_empty()) else {
            continue;
        };
        command.variable_references.push(ArgumentReference {
            name: name.to_owned(),
            argument,
            location: ReferenceLocation {
                file,
                command: command.id,
                span: argument_name_span(source, arg),
                kind: ReferenceKind::Read,
                possible_value: None,
            },
        });
        if !arg
            .kind
            .iter()
            .any(|kind| matches!(kind, ArgKind::ReferencingVar))
        {
            arg.kind.push(ArgKind::ReferencingVar);
        }
    }
}
