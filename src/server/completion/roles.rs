use super::model::{ArgumentChoices, ArgumentSite, Input};
use crate::parser::semanticer::{
    model::{ReferenceKind, SemanticArgForm},
    parsing::{completion_keywords, completion_roles},
    schema::CommandSchemaFrom,
    values::argument_value,
};

pub(super) fn shadowed(
    input: &Input,
    command: crate::parser::semanticer::model::CommandId,
    file: crate::parser::semanticer::model::FileId,
) -> bool {
    input.index.as_ref().is_some_and(|index| {
        index
            .resolved_calls
            .iter()
            .any(|call| call.file == file && call.command == command)
    }) || input
        .snapshot
        .semantics
        .functions
        .iter()
        .filter(|function| function.definition.is_some())
        .flat_map(|function| &function.references)
        .any(|call| {
            call.file == file && call.command == command && call.kind == ReferenceKind::Call
        })
}
pub(super) fn choices<'a>(
    input: &'a Input,
    argument: &ArgumentSite<'a>,
    prefix: &str,
    cancel: &lspally::server::model::Cancellation,
) -> ArgumentChoices<'a> {
    let Some(schema) = &argument.command.schema else {
        return ArgumentChoices::default();
    };
    if matches!(
        schema.from,
        CommandSchemaFrom::Builtin | CommandSchemaFrom::KeyWord
    ) && shadowed(input, argument.command.id, input.snapshot.semantics.id)
    {
        return ArgumentChoices::default();
    }
    let mut preceding = Vec::with_capacity(argument.argument + 1);
    for (index, arg) in argument.command.args[..argument.argument]
        .iter()
        .enumerate()
    {
        if let Some(literal) = arg.literal_text() {
            preceding.push(std::borrow::Cow::Borrowed(literal));
            continue;
        }
        if let Some(expanded) = argument_value(
            &input.snapshot.semantics,
            input.index.as_deref(),
            argument.command,
            index,
        )
        .filter(|value| {
            arg.form != SemanticArgForm::Unquoted
                || !value.text().is_empty() && !value.text().contains(';')
        }) {
            preceding.push(std::borrow::Cow::Owned(expanded.text().to_owned()));
        } else if arg.form == SemanticArgForm::Quoted {
            preceding.push(std::borrow::Cow::Borrowed(arg.content.as_str()));
        } else {
            // Unknown unquoted expansion/list splitting can change every later argument position.
            return ArgumentChoices::default();
        }
    }
    preceding.push(std::borrow::Cow::Borrowed(prefix));
    let arguments: Vec<_> = preceding.iter().map(|argument| argument.as_ref()).collect();
    ArgumentChoices {
        roles: completion_roles(schema, &arguments),
        keywords: completion_keywords(schema, &arguments, || cancel.is_cancelled()),
    }
}
