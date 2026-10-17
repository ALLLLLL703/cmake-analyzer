//! Select bound symbol occurrences and convert deduplicated byte spans to client ranges.
//! Owner-local file maps prevent unrelated numeric IDs from crossing owners. No I/O occurs.

use super::model::{Input, Query};
use crate::{
    parser::semanticer::model::{ReferenceKind, ReferenceLocation},
    server::text::TextIndex,
    workspace::model::DocumentSnapshot,
};
use lspally::server::model::*;
use std::{collections::BTreeMap, sync::Arc};

/// URI and half-open byte spans retain the immutable text needed for coordinate conversion.
/// Example: `let results = Results::new();` starts a deduplicated navigation result set.
pub(super) type Results = BTreeMap<(String, usize, usize), Arc<DocumentSnapshot>>;
/// Append a reference range only through this owner's file map; no I/O.
///
/// # Arguments
/// - `input` `[in]`: Owner-local file IDs, URIs and immutable target snapshots.
/// - `reference` `[in]`: Bound reference whose file and byte span select the location.
/// - `output` `[in,out]`: Deduplicates by URI/start/end; inserts a retained snapshot without overwriting existing entries.
///
/// # Returns
/// `()`: missing file IDs are ignored; otherwise a unique range is retained.
///
/// # Examples
/// ```text
/// add(input, reference, &mut results);
/// ```
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
/// Append locations only if the cursor selects this symbol's bound occurrences.
/// Does not guess by spelling or perform I/O.
///
/// # Arguments
/// - `input` `[in]`: Owner-qualified queried snapshot and target-file map.
/// - `offset` `[in]`: UTF-8 byte cursor; occurrence matching uses half-open payload spans.
/// - `definition` `[in]`: Optional bound source declaration, absent for source-less native commands.
/// - `references` `[in]`: Occurrences already associated with one variable/function/parameter/target.
/// - `query` `[in]`: Definition or references and declaration policy.
/// - `output` `[in,out]`: Appends/deduplicates selected ranges; existing results are preserved.
/// - `cancel` `[in]`: Stop signal checked before matching and while adding occurrences.
///
/// # Returns
/// `Ok(())` whether no occurrence is selected or matching locations were added.
///
/// # Errors
/// `Err` reports observed cancellation; previously appended results are not rolled back. A cancellation during matching can yield no additions before the outer query detects it.
///
/// # Algorithm
/// Match file ID and half-open name span, not equal spelling. Definitions add at most one location. References omit both explicit Definition records and any occurrence with the declaration's exact file/span when include_declaration is false, handling repeated includes recorded as writes.
///
/// # Flow
/// ```text
///  [Bound occurrence list]
///            |
///  <cursor hits this file/name?> --no--> [No additions]
///            | yes
///            v
///       <query policy?>
///        /           \
///  [Definition]   [References]
///        |           |
///  [add if any] [filter declarations]
///        +-----------+
///                    v
///           [Deduplicated output]
/// ```
///
/// # Examples
/// ```text
/// symbol(input, offset, definition, references, Query::Definition, &mut results, &cancel)?;
/// ```
pub(super) fn symbol(
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
/// Resolve bound symbol/target locations and convert byte spans to negotiated coordinates.
/// Only local result maps and indexes are built; no I/O or snapshot mutation.
///
/// # Arguments
/// - `inputs` `[in]`: Retained owner-qualified indexes and file maps, moved into the worker.
/// - `position` `[in]`: Client cursor validated separately against each queried snapshot.
/// - `query` `[in]`: Definition or references and declaration-inclusion policy.
/// - `encoding` `[in]`: Negotiated UTF-8/UTF-16/UTF-32 units for both input and output.
/// - `cancel` `[in]`: Cooperative stop signal checked across owners, symbols and output locations.
///
/// # Returns
/// `Ok(locations)` sorted/deduplicated by URI and byte range, including all selected owner bindings; may be empty.
///
/// # Errors
/// `Err` reports cancellation, invalid client positions or invalid/unrepresentable analysis ranges; partial result maps are discarded.
///
/// # Algorithm
/// Convert the cursor to bytes in each owner. Adapt target bindings, then use its graph reference index or replay local_index without persisting a duplicate index. Visit variable/function bindings, deduplicate by URI/byte span, and build one TextIndex per grouped target URI to render ranges from retained target text.
///
/// # Flow
/// ```text
///  [Owner Input]          [Client position + encoding]
///         |                           |
///         +-------------+-------------+
///                       v
///                  [Byte cursor]
///                       |
///            +----------+-----------+
///            v                      v
///     [Target bindings]    <graph index present?>
///            |               /             \
///            |           [yes]       [local replay]
///            |               +------+------+
///            |                      v
///            |             [Variable/function bindings]
///            +---------------+------+
///                            v
///                 [URI/byte-range deduplication]
///                            |
///                            v
///                  [Target-text range conversion]
/// ```
///
/// # Examples
/// ```text
/// let locations = locations::query(inputs, position, Query::Definition, encoding, &cancel)?;
/// ```
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
        super::targets::query(input, offset, query, &mut output, cancel)?;
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
