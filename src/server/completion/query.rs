use super::{candidates, inputs, model::Prepared, names, prepare, render};
use crate::server::{model::Service, service::internal, text::TextIndex, uri};
use lspally::server::model::*;

impl Service {
    pub(in crate::server) async fn complete(
        &mut self,
        document: Uri,
        position: Position,
        snippets: bool,
        cancel: Cancellation,
    ) -> ServerResult<Option<CompletionResponse>> {
        let path = uri::to_path(&document)?;
        let owners = self.owners(&path);
        let reserve = snippets
            && owners
                .iter()
                .all(|owner| self.projects[*owner].settings.completion.reserve_arg);
        let (owners, inputs): (Vec<_>, Vec<_>) = owners
            .into_iter()
            .filter_map(|owner| {
                inputs::capture(&self.projects[owner].workspace, &path).map(|input| (owner, input))
            })
            .unzip();
        let encoding = self.encoding;
        let stop = cancel.clone();
        let (mut inputs, prepared) = tokio::task::spawn_blocking(move || {
            let prepared = prepare::query(&inputs, &position, encoding, &stop)?;
            Ok::<_, ServerError>((inputs, prepared))
        })
        .await
        .map_err(internal)??;
        let Some(prepared) = prepared else {
            return Ok(Some(CompletionResponse::Array(Vec::new())));
        };
        if let Prepared::Arguments { requests, .. } = &prepared {
            for ((owner, input), request) in owners.into_iter().zip(&mut inputs).zip(requests) {
                let stop = cancel.clone();
                input.catalog = self.projects[owner]
                    .workspace
                    .completion_catalog(&path, request, move || stop.is_cancelled())
                    .await
                    .map_err(|error| {
                        if error.kind() == std::io::ErrorKind::Interrupted {
                            ServerError::cancelled()
                        } else {
                            internal(error)
                        }
                    })?;
            }
        }
        let items = tokio::task::spawn_blocking(move || {
            if cancel.is_cancelled() {
                return Err(ServerError::cancelled());
            }
            let Some(input) = inputs.first() else {
                return Ok(Vec::new());
            };
            let index = TextIndex::new(&input.snapshot.text, encoding);
            let (site, candidates) = match prepared {
                Prepared::Commands(site) => {
                    let choices = candidates::collect(
                        &inputs,
                        &site.prefix,
                        reserve && !site.has_parentheses,
                        &cancel,
                    )?;
                    (site, choices)
                }
                Prepared::Arguments {
                    site, expansion, ..
                } => {
                    let offset = site.replace.start + site.prefix.len();
                    let choices =
                        names::collect(&inputs, offset, &site.prefix, expansion, &cancel)?;
                    (site, choices)
                }
            };
            let mut items = Vec::with_capacity(candidates.len());
            for candidate in candidates {
                if cancel.is_cancelled() {
                    return Err(ServerError::cancelled());
                }
                items.push(render::item(candidate, &site, &index)?);
            }
            Ok(items)
        })
        .await
        .map_err(internal)??;
        Ok(Some(CompletionResponse::Array(items)))
    }
}
