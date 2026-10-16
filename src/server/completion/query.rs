use super::{candidates, context, model::Input, render};
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
            .filter_map(|owner| {
                let workspace = &self.projects[owner].workspace;
                Some(Input {
                    snapshot: workspace.snapshot(&path).ok().flatten()?,
                    index: workspace.root_graph().map(|graph| graph.references.clone()),
                })
            })
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
            let Some(site) = context::site(&input.snapshot, offset) else {
                return Ok(Vec::new());
            };
            let candidates = candidates::collect(
                &inputs,
                &site.prefix,
                reserve && !site.has_parentheses,
                &cancel,
            )?;
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
