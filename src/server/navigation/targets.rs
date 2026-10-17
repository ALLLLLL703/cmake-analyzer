//! Adapt revision-qualified target bindings to shared symbol navigation.
//! Raw argument spans narrow to payloads so wrappers are not clickable names. No I/O occurs.

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

/// Adapt a target occurrence only when its source revision still matches.
/// Reads immutable syntax and build data; no side effects.
///
/// # Arguments
/// - `input` `[in]`: Owner-local file map with retained source revisions.
/// - `reference` `[in]`: Target source revision, command ID and raw argument span.
///
/// # Returns
/// `Some(location)` narrows the argument to its name payload and preserves reference kind; `None` for missing files/commands/arguments or stale build analysis.
///
/// # Algorithm
/// Find the owner-local file, verify its build analysis identity, then locate
/// the source command and exact raw argument span. Narrow that argument to its
/// payload only after all checks; a stale target record cannot borrow newer text.
///
/// # Flow
/// ```text
/// [Target SourceRef] ---> [Owner-local snapshot]
///                                  |
///                        <same build analysis?> --no--> [None]
///                                  | yes
///                                  v
///                       [Command + raw argument]
///                                  |
///                         <exact span exists?> --no--> [None]
///                                  | yes
///                                  v
///                       [Payload ReferenceLocation]
/// ```
///
/// # Examples
/// ```text
/// let occurrence = location(input, target_reference);
/// ```
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
/// Append locations for one selected bound target; no I/O or binding mutation.
///
/// # Arguments
/// - `input` `[in]`: Queried source revision and owner-local file map.
/// - `target` `[in]`: Declaration with already-bound target occurrences.
/// - `offset` `[in]`: UTF-8 byte cursor tested against narrowed payloads, not quote/bracket wrappers.
/// - `query` `[in]`: Definition/reference selection policy.
/// - `out` `[in,out]`: Appends/deduplicates matching ranges; existing results remain.
/// - `cancel` `[in]`: Stop signal checked during selection and shared symbol rendering.
///
/// # Returns
/// `Ok(())` after adding locations, or with no additions if this target is not selected.
///
/// # Errors
/// `Err` reports cancellation; earlier output additions are preserved.
///
/// # Algorithm
/// First scan references in the queried build revision and narrow only potential hits. Materialize all valid adapted references only for the selected target, find its Definition record, then delegate declaration filtering and range collection to locations::symbol.
///
/// # Flow
/// ```text
///  [Target raw occurrences]
///             |
///  <revision + payload hit?> --no--> [No additions]
///             | yes
///             v
///  [Adapt selected target occurrences]
///             |
///             v
///  [Find declaration] ---> [Shared symbol policy]
///                                   |
///                                   v
///                            [Append ranges]
/// ```
///
/// # Examples
/// ```text
/// target(input, declaration, offset, Query::Definition, &mut results, &cancel)?;
/// ```
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
/// Visit owner target bindings or file-local target facts; no I/O.
///
/// # Arguments
/// - `input` `[in]`: Owner-qualified target index or local build facts.
/// - `offset` `[in]`: UTF-8 cursor in the queried snapshot.
/// - `query` `[in]`: Definition/reference selection policy.
/// - `out` `[in,out]`: Appends/deduplicates target ranges without clearing existing results.
/// - `cancel` `[in]`: Stop signal passed to each target selection.
///
/// # Returns
/// `Ok(())` after visiting target declarations, including no matches.
///
/// # Errors
/// `Err` reports cancellation detected during target selection; earlier output remains.
///
/// # Examples
/// ```text
/// targets::query(input, offset, Query::Definition, &mut results, &cancel)?;
/// ```
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
