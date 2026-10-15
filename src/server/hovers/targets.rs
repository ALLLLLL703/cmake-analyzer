use super::{builtins, model::*};
use crate::{
    parser::semanticer::{model::*, parsing::references::argument_name_span},
    server::text::TextIndex,
    workspace::model::DocumentSnapshot,
};
use lspally::server::model::*;
fn contains(span: crate::model::TextSpan, offset: usize) -> bool {
    span.start_byte <= offset && offset < span.end_byte
}
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
pub(super) fn locate(
    source: &DocumentSnapshot,
    position: &Position,
    encoding: CoordinateEncoding,
) -> ServerResult<Option<Target>> {
    let index = TextIndex::new(&source.text, encoding);
    let offset = index.byte(position)?;
    let found = if let Some((name, location)) = variable(source, offset) {
        Some((name, location, TargetKind::Variable))
    } else if let Some(function) = source
        .semantics
        .functions
        .iter()
        .find(|function| contains(function.definition.span, offset))
    {
        Some((
            function.name.clone(),
            function.definition.clone(),
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
                builtins::variable(name)?;
                Some((
                    name.to_owned(),
                    ReferenceLocation {
                        file: source.semantics.id,
                        command: command.id,
                        span,
                        kind: ReferenceKind::Read,
                        possible_value: None,
                    },
                    TargetKind::Variable,
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
