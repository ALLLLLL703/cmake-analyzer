//! Resolve argument text only from agreeing read-position observations. No I/O.
// Contains AI-generated documentation additions.
use crate::parser::semanticer::model::{
    ArgumentReference, PossibleValue, ReferenceIndex, ReferenceKind, ReferenceLocation,
    SemanticCommandRepresent, SemanticedBuffer, ValueBindings,
};

/// Infer one argument using only text facts attached to its read positions. Conflicting visits stay unknown; no mutation or I/O.
///
/// # Arguments
/// - `buffer` `[in]`: local observations used when no workspace index is supplied.
/// - `index` `[in]`: optional aggregate reference observations; when present, no local fallback occurs.
/// - `command` `[in]`: source argument forms and extracted reads.
/// - `argument` `[in]`: zero-based source slot.
///
/// # Returns
/// Some expanded Static text if every needed read agrees and expansion is supported; None for invalid slot, missing/conflicting facts or unsupported syntax.
///
/// # Examples
/// ```text
/// let value = argument_value(&buffer, Some(&index), &buffer.commands[0], 1);
/// ```
pub(crate) fn argument_value(
    buffer: &SemanticedBuffer,
    index: Option<&ReferenceIndex>,
    command: &SemanticCommandRepresent,
    argument: usize,
) -> Option<PossibleValue> {
    let arg = command.args.get(argument)?;
    let mut values = ValueBindings::default();
    for reference in command
        .variable_references
        .iter()
        .filter(|reference| reference.argument == argument)
    {
        values.assign(&reference.name, reference_value(buffer, index, reference));
    }
    values.expand(arg, None)
}
/// Clone a fact only when every supplied observation agrees, including its origin tag. Consumes the iterator without other effects.
///
/// # Arguments
/// - `values` `[in]`: iterator of optional borrowed facts.
///
/// # Returns
/// Some cloned first fact if the iterator is nonempty and all entries equal that known fact; None for empty, unknown or conflicting input.
///
/// # Examples
/// ```text
/// let fact = PossibleValue::Static("yes".into());
/// assert_eq!(unanimous([Some(&fact), Some(&fact)].into_iter()), Some(fact));
/// ```
fn unanimous<'a>(
    mut values: impl Iterator<Item = Option<&'a PossibleValue>>,
) -> Option<PossibleValue> {
    let first = values.next()??;
    values
        .all(|value| value == Some(first))
        .then(|| first.clone())
}
/// Resolve one exact read position from workspace or local observations, requiring agreement. No side effects.
///
/// # Arguments
/// - `buffer` `[in]`: local value observations.
/// - `index` `[in]`: aggregate index, used exclusively when Some.
/// - `reference` `[in]`: name and exact file/command/span read location.
///
/// # Returns
/// Known cloned fact only when matching observations are present and unanimous; None otherwise.
///
/// # Algorithm
/// Choose exactly one observation source: the workspace index when supplied, otherwise local observations. Match the case-sensitive name and exact file/command/span/read location, then require all facts to agree.
///
/// # Flow
/// ```text
///  [requested read position]
///              |
///        <workspace index?>
///           /          \
///     [aggregate]    [local values]
///           |          |
///           +-----+----+
///                 v
///      [exact location/name filter]
///                 |
///         <known and unanimous?>
///            /            \
///       [cloned fact]   [unknown]
/// ```
///
/// # Examples
/// ```text
/// let value = reference_value(&buffer, None, &command.variable_references[0]);
/// ```
fn reference_value(
    buffer: &SemanticedBuffer,
    index: Option<&ReferenceIndex>,
    reference: &ArgumentReference,
) -> Option<PossibleValue> {
    let matches = |location: &ReferenceLocation| {
        location.file == reference.location.file
            && location.command == reference.location.command
            && location.span == reference.location.span
            && location.kind == ReferenceKind::Read
    };
    if let Some(index) = index {
        unanimous(
            index
                .variables
                .iter()
                .filter(|variable| variable.name == reference.name)
                .flat_map(|variable| &variable.references)
                .filter(|location| matches(location))
                .map(|location| location.possible_value.as_ref()),
        )
    } else {
        unanimous(
            buffer
                .values
                .iter()
                .filter(|value| value.name == reference.name && matches(&value.location))
                .map(|value| value.possible_value.as_ref()),
        )
    }
}
