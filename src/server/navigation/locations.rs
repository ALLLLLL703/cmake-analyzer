use super::model::{Input, Query};
use crate::{
    parser::semanticer::model::{ReferenceKind, ReferenceLocation},
    server::text::TextIndex,
    workspace::model::DocumentSnapshot,
};
use lspally::server::model::*;
use std::{collections::BTreeMap, sync::Arc};

type Results = BTreeMap<(String, usize, usize), Arc<DocumentSnapshot>>;
fn add(input: &Input, reference: &ReferenceLocation, output: &mut Results) {
    if let Some((uri, snapshot)) = input.files.get(&reference.file) {
        output
            .entry((
                uri.clone(),
                reference.span.start_byte,
                reference.span.end_byte,
            ))
            .or_insert_with(|| snapshot.clone());
    }
}
fn symbol(
    input: &Input,
    offset: usize,
    definition: Option<&ReferenceLocation>,
    references: &[ReferenceLocation],
    query: Query,
    output: &mut Results,
    cancel: &Cancellation,
) -> ServerResult<()> {
    if cancel.is_cancelled() {
        return Err(ServerError::cancelled());
    }
    if !references
        .iter()
        .take_while(|_| !cancel.is_cancelled())
        .any(|reference| {
            reference.file == input.snapshot.semantics.id
                && reference.span.start_byte <= offset
                && offset < reference.span.end_byte
        })
    {
        return Ok(());
    }
    match query {
        Query::Definition => {
            if let Some(definition) = definition {
                add(input, definition, output);
            }
        }
        Query::References {
            include_declaration,
        } => {
            for reference in references {
                if cancel.is_cancelled() {
                    return Err(ServerError::cancelled());
                }
                // A repeated include can record its declaration span again as a write.
                let declaration = reference.kind == ReferenceKind::Definition
                    || definition.is_some_and(|definition| {
                        definition.file == reference.file
                            && definition.span.start_byte == reference.span.start_byte
                            && definition.span.end_byte == reference.span.end_byte
                    });
                if include_declaration || !declaration {
                    add(input, reference, output);
                }
            }
        }
    }
    Ok(())
}
pub(super) fn query(
    inputs: Vec<Input>,
    position: Position,
    query: Query,
    encoding: CoordinateEncoding,
    cancel: &Cancellation,
) -> ServerResult<Vec<Location>> {
    let mut output = Results::new();
    for input in &inputs {
        if cancel.is_cancelled() {
            return Err(ServerError::cancelled());
        }
        let offset = TextIndex::new(&input.snapshot.text, encoding).byte(&position)?;
        let local;
        let index = if let Some(index) = &input.index {
            index.as_ref()
        } else {
            local = crate::parser::semanticer::references::local_index(
                &input.snapshot.semantics,
                &input.snapshot.text,
            );
            &local
        };
        for variable in &index.variables {
            symbol(
                input,
                offset,
                variable.definition.as_ref(),
                &variable.references,
                query,
                &mut output,
                cancel,
            )?;
        }
        for function in &index.functions {
            symbol(
                input,
                offset,
                function.definition.as_ref(),
                &function.references,
                query,
                &mut output,
                cancel,
            )?;
        }
    }
    let mut locations = Vec::with_capacity(output.len());
    let mut current = None;
    // Grouped URI ordering builds each target text index only once.
    for ((uri, start, end), snapshot) in &output {
        if cancel.is_cancelled() {
            return Err(ServerError::cancelled());
        }
        if current
            .as_ref()
            .is_none_or(|(previous, _)| *previous != uri)
        {
            current = Some((uri, TextIndex::new(&snapshot.text, encoding)));
        }
        if let Some((_, index)) = &current {
            locations.push(Location {
                uri: Uri(uri.clone()),
                range: index.range(*start..*end)?,
            });
        }
    }
    Ok(locations)
}
