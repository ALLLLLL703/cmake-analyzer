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
