//! Map negotiated text coordinates to variable, command or literal fallback hover sites; no I/O.

use super::model::*;
use crate::{
    parser::semanticer::{model::*, parsing::references::argument_name_span},
    server::text::TextIndex,
    workspace::model::DocumentSnapshot,
};
use lspally::server::model::*;
/// Test a half-open byte span without side effects.
///
/// # Arguments
/// - `span` `[in]`: source interval including start but excluding end.
/// - `offset` `[in]`: byte position to test.
///
/// # Returns
/// True when start_byte <= offset < end_byte.
///
/// # Examples
/// Local server context:
/// ```text
/// assert!(contains(crate::model::TextSpan { start_byte: 2, end_byte: 4, ..Default::default() }, 2));
/// ```
fn contains(span: crate::model::TextSpan, offset: usize) -> bool {
    span.start_byte <= offset && offset < span.end_byte
}
/// Locate a variable site at a byte offset using existing snapshot facts; no I/O or mutation.
///
/// # Arguments
/// - `source` `[in]`: immutable source and semantic facts.
/// - `offset` `[in]`: cursor byte offset, compared against name spans.
///
/// # Returns
/// Some((owned name, cloned/synthesized reference)) for a variable reference or ordinary Out argument;
/// None otherwise. ENV/CACHE output spellings are excluded only from the Out fallback.
///
/// # Algorithm
/// Search semantic variable references, then command argument references, then unresolved variable
/// references. Finally inspect literal ordinary output arguments and synthesize a Write reference over
/// the narrowed name span.
///
/// # Flow
/// ```text
///  [Cursor offset]
///        |
///  [Known reference?] --yes--> [Clone name + location]
///        | no
///  [Unresolved variable?] --yes--> [Clone name + location]
///        | no
///  [Ordinary literal Out?] --yes--> [Synthesize Write]
///        | no
///      [None]
/// ```
///
/// # Examples
/// Local server context:
/// ```text
/// let site = variable(&snapshot, 7);
/// ```
fn variable(source: &DocumentSnapshot, offset: usize) -> Option<(String, ReferenceLocation)> {
    source
        .semantics
        .variables
        .iter()
        .find_map(|variable| {
            variable
                .references
                .iter()
                .find(|reference| contains(reference.span, offset))
                .map(|reference| (variable.name.clone(), reference.clone()))
        })
        .or_else(|| {
            source
                .semantics
                .commands
                .iter()
                .flat_map(|command| &command.variable_references)
                .find(|reference| contains(reference.location.span, offset))
                .map(|reference| (reference.name.clone(), reference.location.clone()))
        })
        .or_else(|| {
            source
                .semantics
                .unresolved_references
                .iter()
                .find(|reference| {
                    reference.target == ReferenceTarget::Variable
                        && contains(reference.location.span, offset)
                })
                .map(|reference| (reference.name.clone(), reference.location.clone()))
        })
        .or_else(|| {
            source.semantics.commands.iter().find_map(|command| {
                command
                    .args
                    .iter()
                    .filter(|arg| arg.kind.iter().any(|kind| matches!(kind, ArgKind::Out)))
                    .find_map(|arg| {
                        let name = arg.literal_text().filter(|name| {
                            !name.starts_with("ENV{") && !name.starts_with("CACHE{")
                        })?;
                        let span = argument_name_span(&source.text, arg);
                        contains(span, offset).then(|| {
                            (
                                name.to_owned(),
                                ReferenceLocation {
                                    file: source.semantics.id,
                                    command: command.id,
                                    span,
                                    kind: ReferenceKind::Write,
                                    possible_value: None,
                                },
                            )
                        })
                    })
            })
        })
}
/// Convert a hover position into a snapshot-qualified name site; no I/O or source mutation.
///
/// # Arguments
/// - `source` `[in]`: immutable source text and semantic records.
/// - `position` `[in]`: client position in the negotiated encoding.
/// - `encoding` `[in]`: UTF-8/16/32 coordinate interpretation.
///
/// # Returns
/// Ok(Some(Target)) with name, reference, route and name-only client range; Ok(None) outside a supported
/// name site.
///
/// # Errors
/// Invalid client coordinates or an unrepresentable returned source range.
///
/// # Algorithm
/// Convert coordinates once. Prefer a variable, then a function definition, then a command name, then a
/// literal argument as a possible native variable. Narrow argument wrappers before range conversion.
///
/// # Flow
/// ```text
///  [Client position]
///         |
///  [Byte conversion] --invalid--> [Error]
///         |
///  [Variable site?] --yes--+
///         | no           |
///  [Definition/command?]-+
///         | no           |
///  [Literal argument?] -+
///         | none         v
///       [None]      [Range + Target]
/// ```
///
/// # Examples
/// Local server context:
/// ```text
/// let site = targets::locate(&snapshot, &Position { line: 0, character: 4 }, encoding)?;
/// ```
pub(super) fn locate(
    source: &DocumentSnapshot,
    position: &Position,
    encoding: CoordinateEncoding,
) -> ServerResult<Option<Target>> {
    let index = TextIndex::new(&source.text, encoding);
    let offset = index.byte(position)?;
    let found = if let Some((name, location)) = variable(source, offset) {
        Some((name, location, TargetKind::Variable))
    } else if let Some((function, definition)) = source
        .semantics
        .functions
        .iter()
        .filter_map(|function| {
            function
                .definition
                .as_ref()
                .map(|definition| (function, definition))
        })
        .find(|(_, definition)| contains(definition.span, offset))
    {
        Some((
            function.name.clone(),
            definition.clone(),
            TargetKind::Command,
        ))
    } else if let Some(command) = source
        .semantics
        .commands
        .iter()
        .find(|command| contains(command.span, offset))
    {
        Some((
            command.name.clone(),
            ReferenceLocation {
                file: source.semantics.id,
                command: command.id,
                span: command.span,
                kind: ReferenceKind::Call,
                possible_value: None,
            },
            TargetKind::Command,
        ))
    } else {
        source.semantics.commands.iter().find_map(|command| {
            command.args.iter().find_map(|arg| {
                let span = argument_name_span(&source.text, arg);
                if !contains(span, offset) {
                    return None;
                }
                let name = arg.literal_text()?;
                Some((
                    name.to_owned(),
                    ReferenceLocation {
                        file: source.semantics.id,
                        command: command.id,
                        span,
                        kind: ReferenceKind::Read,
                        possible_value: None,
                    },
                    TargetKind::BuiltinVariable,
                ))
            })
        })
    };
    found
        .map(|(name, location, kind)| {
            index.range(location.span.range()).map(|range| Target {
                name,
                location,
                kind,
                range,
            })
        })
        .transpose()
}
