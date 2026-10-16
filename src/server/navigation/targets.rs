use super::{
    locations::{Results, symbol},
    model::{Input, Query},
};
use crate::parser::{
    build_struct::model::{BuildEffect, TargetDeclaration, TargetReference},
    semanticer::{
        model::{FileId, ReferenceKind, ReferenceLocation},
        parsing::references::argument_name_span,
    },
};
use lspally::server::model::{Cancellation, ServerError, ServerResult};

fn location(input: &Input, reference: &TargetReference) -> Option<ReferenceLocation> {
    let file = FileId {
        id: reference.source.analysis.file,
    };
    let (_, snapshot) = input.files.get(&file)?;
    if snapshot.build.analysis != reference.source.analysis {
        return None;
    }
    let command = snapshot
        .semantics
        .commands
        .get(reference.source.command.id as usize)?;
    let argument = command
        .args
        .iter()
        .find(|argument| argument.span == reference.span)?;
    Some(ReferenceLocation {
        file,
        command: command.id,
        span: argument_name_span(&snapshot.text, argument),
        kind: reference.kind,
        possible_value: None,
    })
}
fn target(
    input: &Input,
    target: &TargetDeclaration,
    offset: usize,
    query: Query,
    out: &mut Results,
    cancel: &Cancellation,
) -> ServerResult<()> {
    // Only materialize adapted locations for the selected target, not the whole catalog.
    let mut selected = false;
    for reference in &target.references {
        if cancel.is_cancelled() {
            return Err(ServerError::cancelled());
        }
        if reference.source.analysis == input.snapshot.build.analysis
            && reference.span.start_byte <= offset
            && offset < reference.span.end_byte
        {
            if let Some(location) = location(input, reference) {
                selected |= location.span.start_byte <= offset && offset < location.span.end_byte;
            }
        }
    }
    if !selected {
        return Ok(());
    }
    let references: Vec<_> = target
        .references
        .iter()
        .filter_map(|reference| location(input, reference))
        .collect();
    let definition = references
        .iter()
        .find(|reference| reference.kind == ReferenceKind::Definition);
    symbol(input, offset, definition, &references, query, out, cancel)
}
pub(super) fn query(
    input: &Input,
    offset: usize,
    query: Query,
    out: &mut Results,
    cancel: &Cancellation,
) -> ServerResult<()> {
    if let Some(targets) = &input.targets {
        for declaration in targets.iter() {
            target(input, declaration, offset, query, out, cancel)?;
        }
    } else {
        for fact in &input.snapshot.build.facts {
            if let BuildEffect::Target(declaration) = &fact.effect {
                target(input, declaration, offset, query, out, cancel)?;
            }
        }
    }
    Ok(())
}
