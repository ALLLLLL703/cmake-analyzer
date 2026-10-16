use super::{argument_context, candidates, context, inputs, names, render};
use crate::server::{model::Service, service::internal, text::TextIndex, uri};
use lspally::server::model::*;

impl Service {
    pub(in crate::server) async fn complete(
        &self,
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
        let inputs: Vec<_> = owners
            .into_iter()
            .filter_map(|owner| inputs::capture(&self.projects[owner].workspace, &path))
            .collect();
        let encoding = self.encoding;
        let items = tokio::task::spawn_blocking(move || {
            if cancel.is_cancelled() {
                return Err(ServerError::cancelled());
            }
            let Some(input) = inputs.first() else {
                return Ok(Vec::new());
            };
            let index = TextIndex::new(&input.snapshot.text, encoding);
            let offset = index.byte(&position)?;
            let (site, candidates) =
                if let Some(argument) = argument_context::argument(&input.snapshot, offset) {
                    let expansion = argument_context::expansion(&input.snapshot, &argument, offset);
                    let is_expansion = expansion.is_some();
                    let Some(site) = expansion
                        .or_else(|| argument_context::bare(&input.snapshot, &argument, offset))
                    else {
                        return Ok(Vec::new());
                    };
                    let candidates =
                        names::collect(&inputs, offset, &site.prefix, is_expansion, &cancel)?;
                    (site, candidates)
                } else {
                    let Some(site) = context::site(&input.snapshot, offset) else {
                        return Ok(Vec::new());
                    };
                    let candidates = candidates::collect(
                        &inputs,
                        &site.prefix,
                        reserve && !site.has_parentheses,
                        &cancel,
                    )?;
                    (site, candidates)
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
