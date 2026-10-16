use super::{MatchRole, OutputMatch, sequence};
use crate::parser::semanticer::schema::{ArgSchemaKind, CommandSchema};
use std::collections::BTreeSet;

fn words<'a>(kind: &'a ArgSchemaKind, result: &mut BTreeSet<&'a str>) {
    match kind {
        ArgSchemaKind::Keyword(word) => {
            result.insert(word);
        }
        ArgSchemaKind::Optional(args)
        | ArgSchemaKind::Repeat { args, .. }
        | ArgSchemaKind::AllOf(args) => {
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
/// Offer only reachable literal keywords at the active prefix slot; never infer opaque tails.
/// The caller discards partial results when its cancellation predicate stops the scan.
pub(crate) fn completion_keywords<'a>(
    schema: &'a CommandSchema,
    arguments: &[&str],
    stop: impl Fn() -> bool,
) -> Vec<&'a str> {
    let Some(prefix) = arguments.last() else {
        return Vec::new();
    };
    let position = arguments.len() - 1;
    let mut catalog = BTreeSet::new();
    for argument in &schema.args {
        words(&argument.kind, &mut catalog);
    }
    let roots: Vec<_> = schema.args.iter().map(|argument| &argument.kind).collect();
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
        .any(|state| state.position == arguments.len() && state.indices.contains(&position))
        {
            result.push(word);
        }
    }
    result
}
