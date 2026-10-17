//! Classify the cursor and request only discovery supported by active schema roles.
//! Client positions become UTF-8 byte offsets before syntax queries. No I/O occurs here.

use super::{
    argument_context, context,
    model::{Input, Prepared},
    roles,
};
use crate::{
    parser::semanticer::schema::NameRole, server::text::TextIndex,
    workspace::completion::model::Request,
};
use lspally::server::model::*;

/// Choose a command or argument edit and owner-aligned discovery requests.
/// Reads immutable inputs only; no I/O or source mutation.
///
/// # Arguments
/// - `inputs` `[in]`: Owner-qualified snapshots; the first text defines the shared edit site.
/// - `position` `[in]`: Zero-based cursor in `encoding` units.
/// - `encoding` `[in]`: Negotiated UTF-8, UTF-16 or UTF-32 client coordinate units.
/// - `cancel` `[in]`: Cooperative stop signal checked before work and per owner.
///
/// # Returns
/// `Ok(Some(plan))` contains a byte replacement site and any discovery flags; `Ok(None)` means no input or unsupported syntax.
///
/// # Errors
/// `Err` reports cancellation or an invalid client position, including positions inside an encoded character.
///
/// # Algorithm
/// Convert the first snapshot position to bytes. Prefer innermost ordinary expansion names, otherwise a bare argument payload. Probe each owner's schema roles for module/package/target discovery; expansions request none. Outside arguments, try a top-level command site.
///
/// # Flow
/// ```text
///  [Client position]                 [Owner snapshots]
///          |                                |
///          v                                v
///  [TextIndex::byte] ----------------> <argument?>
///                                      /        \
///                                    yes        no
///                                     |          |
///                            <expansion/bare?> [command site]
///                               /       \
///                            [none]  [per-owner role flags]
///                                       |
///                                       v
///                                  [Prepared plan]
/// ```
///
/// # Examples
/// ```text
/// let plan = prepare::query(&inputs, &position, encoding, &cancel)?;
/// ```
pub(super) fn query(
    inputs: &[Input],
    position: &Position,
    encoding: CoordinateEncoding,
    cancel: &Cancellation,
) -> ServerResult<Option<Prepared>> {
    if cancel.is_cancelled() {
        return Err(ServerError::cancelled());
    }
    let Some(input) = inputs.first() else {
        return Ok(None);
    };
    let offset = TextIndex::new(&input.snapshot.text, encoding).byte(position)?;
    if let Some(argument) = argument_context::argument(&input.snapshot, offset) {
        let expansion = argument_context::expansion(&input.snapshot, &argument, offset);
        let is_expansion = expansion.is_some();
        let Some(site) =
            expansion.or_else(|| argument_context::bare(&input.snapshot, &argument, offset))
        else {
            return Ok(None);
        };
        let mut requests = Vec::with_capacity(inputs.len());
        for input in inputs {
            if cancel.is_cancelled() {
                return Err(ServerError::cancelled());
            }
            let request = if !is_expansion {
                argument_context::argument(&input.snapshot, offset)
                    .map(|argument| {
                        let accepted = roles::choices(input, &argument, &site.prefix, cancel).roles;
                        Request {
                            command: Some(argument.command.id),
                            modules: accepted.contains(&NameRole::Module),
                            packages: accepted.contains(&NameRole::Package),
                            targets: accepted.iter().any(|role| {
                                matches!(
                                    role,
                                    NameRole::Target
                                        | NameRole::Library
                                        | NameRole::Executable
                                        | NameRole::CustomTarget
                                )
                            }),
                        }
                    })
                    .unwrap_or_default()
            } else {
                Request::default()
            };
            requests.push(request);
        }
        Ok(Some(Prepared::Arguments {
            site,
            expansion: is_expansion,
            requests,
        }))
    } else {
        Ok(context::site(&input.snapshot, offset).map(Prepared::Commands))
    }
}
