//! Probe schema keywords and name roles after conservative prior-argument expansion.
//! Unknown unquoted splitting and native user shadows suppress guesses. No I/O or binding mutation occurs.

use super::model::{ArgumentChoices, ArgumentSite, Input};
use crate::parser::semanticer::{
    model::{ReferenceKind, SemanticArgForm},
    parsing::{completion_keywords, completion_roles},
    schema::CommandSchemaFrom,
    values::argument_value,
};

/// Check whether an owner-local command call is bound to a source function/macro.
/// Read-only; no side effects.
///
/// # Arguments
/// - `input` `[in]`: Owner index and local semantic declarations.
/// - `command` `[in]`: Snapshot-local command ID being tested.
/// - `file` `[in]`: File ID qualified by this input owner.
///
/// # Returns
/// `true` if resolved graph calls or local source call references match file/command; `false` otherwise.
///
/// # Examples
/// ```text
/// let user_call = roles::shadowed(input, command.id, input.snapshot.semantics.id);
/// ```
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
/// Probe reachable schema keywords and name roles for the active argument.
/// Does not bind references, change values or perform I/O.
///
/// # Arguments
/// - `input` `[in]`: Owner-qualified snapshot and position-qualified known values.
/// - `argument` `[in]`: Command and active slot; its ordinal cannot exceed the parsed argument count.
/// - `prefix` `[in]`: Case-preserving active payload prefix appended after prior arguments.
/// - `cancel` `[in]`: Stop signal forwarded to keyword probing; this function itself does not return a cancellation error.
///
/// # Returns
/// Borrowed keywords and name-role alternatives; empty when schema is absent, a native call is user-shadowed, or prior unquoted expansion may change slot positions. Cancellation can shorten keyword results; outer collection checks it.
///
/// # Panics
/// Panics if the active ordinal exceeds command.args.len(); argument_context supplies a valid parsed or trailing slot.
///
/// # Algorithm
/// Prepare preceding values once. Use literal text or known position-qualified expansions; quoted unknowns retain their raw contents, but unknown/empty/list-producing unquoted expansions stop inference. Append the active prefix and run shared schema role/keyword matching. External schemas keep their own roles.
///
/// # Flow
/// ```text
///  [Command schema] --missing/native shadow--> [Empty choices]
///         | usable
///         v
///  [Prepare earlier arguments]
///         |
///  <unknown unquoted split?> --yes--> [Empty choices]
///         | no
///         v
///  [Append active prefix]
///         |
///         +------------------+
///         v                  v
///  [Name-role matcher] [Cancellable keyword probes]
///         +------------------+
///                  v
///           [ArgumentChoices]
/// ```
///
/// # Examples
/// ```text
/// let choices = roles::choices(input, &slot, "PRI", &cancel);
/// ```
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
