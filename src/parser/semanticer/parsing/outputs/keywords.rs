//! Offer literal keywords reachable at an unfinished argument slot. No I/O.
// Contains AI-generated documentation additions.
use super::{MatchRole, OutputMatch, sequence};
use crate::parser::semanticer::schema::{ArgSchemaKind, CommandSchema};
use std::collections::BTreeSet;

/// Append all literal keyword spellings in a schema subtree to a sorted set. No I/O.
///
/// # Arguments
/// - `kind` `[in]`: schema subtree.
/// - `result` `[in,out]`: inserts borrowed spellings, retaining existing entries and deduplicating.
///
/// # Returns
/// () after recursively visiting groups and alternatives.
///
/// # Examples
/// ```text
/// let mut catalog = BTreeSet::new();
/// words(&ArgSchemaKind::Keyword("PUBLIC".into()), &mut catalog);
/// ```
fn words<'a>(kind: &'a ArgSchemaKind, result: &mut BTreeSet<&'a str>) {
    match kind {
        ArgSchemaKind::Keyword(word) => {
            result.insert(word);
        }
        ArgSchemaKind::Optional(args)
        | ArgSchemaKind::Repeat { args, .. }
        | ArgSchemaKind::AllOf(args)
        | ArgSchemaKind::Grammar { args, .. } => {
            for arg in args {
                words(arg, result);
            }
        }
        ArgSchemaKind::Choice(choices) => {
            for arg in choices.iter().flatten() {
                words(arg, result);
            }
        }
        _ => {}
    }
}
/// Offer reachable literal keywords at the active prefix slot, never inferring opaque tails. Invokes the caller cancellation predicate; no I/O is performed here.
///
/// # Arguments
/// - `schema` `[in]`: grammar owning returned keyword strings.
/// - `arguments` `[in]`: source-slot payloads ending in the active prefix.
/// - `stop` `[in]`: cancellation callback checked before and during candidate scanning; its own effects are caller-defined.
///
/// # Returns
/// Sorted borrowed candidate words; empty for no active slot or failed prefix. Cancellation can return a partial list that callers must discard.
///
/// # Algorithm
/// Determine the strongest completed-prefix keyword evidence. Gather grammar keyword spellings, filter by the typed prefix and probe each canonical replacement. Keep only candidates reaching the active slot without weakening completed-branch specificity.
///
/// # Flow
/// ```text
///  [completed prefix]       [grammar keywords]
///          |                        |
///  [strongest specificity]     [prefix filter]
///          |                        |
///          +-----------+------------+
///                      v
///                   <stop?>
///                  /       \
///           [partial return] [probe candidate]
///                               |
///                         <reachable/strong?>
///                            /        \
///                         [offer]    [omit]
/// ```
///
/// # Examples
/// ```text
/// let words = completion_keywords(schema, &["PU"], || false);
/// ```
pub(crate) fn completion_keywords<'a>(
    schema: &'a CommandSchema,
    arguments: &[&str],
    stop: impl Fn() -> bool,
) -> Vec<&'a str> {
    let Some(prefix) = arguments.last() else {
        return Vec::new();
    };
    let position = arguments.len() - 1;
    if stop() {
        return Vec::new();
    }
    let roots: Vec<_> = schema.args.iter().map(|argument| &argument.kind).collect();
    // Preserve the strongest completed branch even when a proposed word cannot enter its next slot.
    let Some(specificity) = sequence(
        &roots,
        vec![OutputMatch::default()],
        &arguments[..position],
        MatchRole::CompletedPrefix,
    )
    .into_iter()
    .filter(|state| state.position == position)
    .map(|state| state.keywords)
    .max() else {
        return Vec::new();
    };
    let mut catalog = BTreeSet::new();
    for argument in &schema.args {
        words(&argument.kind, &mut catalog);
    }
    let mut result = Vec::new();
    for word in catalog.into_iter().filter(|word| word.starts_with(*prefix)) {
        if stop() {
            break;
        }
        // Probe the canonical replacement, not the partial spelling, at the active slot.
        if sequence(
            &roots,
            vec![OutputMatch::default()],
            arguments,
            MatchRole::Keyword(word),
        )
        .iter()
        .any(|state| {
            state.position == arguments.len()
                && state.keywords >= specificity
                && state.indices.contains(&position)
        }) {
            result.push(word);
        }
    }
    result
}
