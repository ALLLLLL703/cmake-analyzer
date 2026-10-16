use crate::parser::semanticer::model::{
    ArgumentReference, PossibleValue, ReferenceIndex, ReferenceKind, ReferenceLocation,
    SemanticCommandRepresent, SemanticedBuffer, ValueBindings,
};

/// Resolve only values attached to this read position; conflicting visits stay unknown.
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
fn unanimous<'a>(
    mut values: impl Iterator<Item = Option<&'a PossibleValue>>,
) -> Option<PossibleValue> {
    let first = values.next()??;
    values
        .all(|value| value == Some(first))
        .then(|| first.clone())
}
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
