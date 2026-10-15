// AI-authored diagnostic publication and symbol queries.
use super::{
    super::{diagnostics, model::*, symbols, text::TextIndex, uri},
    internal,
};
use lspally::server::model::*;
use std::{collections::BTreeSet, sync::Arc};

impl Service {
    pub(super) async fn reports(&self) -> ServerResult<Vec<PreparedReport>> {
        let mut inputs = Vec::with_capacity(self.documents.len());
        for (path, document) in &self.documents {
            let owners = self.owners(path);
            let enabled = owners
                .iter()
                .any(|owner| self.projects[*owner].settings.diagnostics.enable);
            if !enabled
                && self
                    .published
                    .get(path)
                    .is_none_or(|previous| !previous.enabled)
            {
                continue;
            }
            let mut snapshot = None;
            let mut extra = Vec::new();
            let mut contextual = false;
            let mut resolved_calls = Vec::new();
            for owner in owners {
                if !self.projects[owner].settings.diagnostics.enable {
                    continue;
                }
                let workspace = &self.projects[owner].workspace;
                if snapshot.is_none() {
                    snapshot = workspace.snapshot(path).map_err(internal)?;
                }
                if let Some(graph) = workspace.root_graph() {
                    if let Some(visit) = graph.invocations.iter().find(|visit| visit.file == *path)
                    {
                        contextual = true;
                        let file = visit.snapshot.semantics.id;
                        for warning in graph
                            .references
                            .warnings
                            .iter()
                            .filter(|warning| warning.location.file == file)
                        {
                            extra.push((
                                warning.warning.span,
                                2,
                                warning.warning.content.message(),
                            ));
                        }
                        resolved_calls.extend(
                            graph
                                .references
                                .resolved_calls
                                .iter()
                                .filter(|location| location.file == file)
                                .map(|location| location.command),
                        );
                    }
                    for diagnostic in &graph.diagnostics {
                        // Failed child reads carry the parent's source range, not a range in the missing child.
                        let origin = diagnostic
                            .source
                            .as_ref()
                            .and_then(|source| {
                                graph
                                    .invocations
                                    .iter()
                                    .find(|visit| visit.analysis.file == source.analysis.file)
                            })
                            .map(|visit| &visit.file)
                            .unwrap_or(&diagnostic.file);
                        if origin == path {
                            extra.push((
                                diagnostic
                                    .source
                                    .as_ref()
                                    .map_or_default(|source| source.span),
                                1,
                                if origin != &diagnostic.file {
                                    format!("{}: {}", diagnostic.file.display(), diagnostic.message)
                                } else {
                                    diagnostic.message.clone()
                                },
                            ));
                        }
                    }
                }
            }
            resolved_calls.sort_by_key(|id| id.id);
            resolved_calls.dedup();
            let key = ReportKey {
                version: document.version,
                enabled,
                snapshot: snapshot.as_ref().map(Arc::downgrade),
                extra,
                contextual,
                resolved_calls,
            };
            if self
                .published
                .get(path)
                .is_some_and(|previous| previous.same(&key))
            {
                continue;
            }
            inputs.push(ReportInput {
                path: path.clone(),
                uri: Uri(document.uri.0.clone()),
                key,
                snapshot,
            });
        }
        let encoding = self.encoding;
        let converted = if inputs.is_empty() {
            Vec::new()
        } else {
            tokio::task::spawn_blocking(move || convert_reports(inputs, encoding))
                .await
                .map_err(internal)??
        };
        let mut reports: Vec<_> = self
            .clears
            .iter()
            .map(|uri| {
                Ok(PreparedReport {
                    path: uri::to_path(uri)?,
                    key: None,
                    report: DiagnosticReport {
                        uri: Uri(uri.0.clone()),
                        version: None,
                        diagnostics: Vec::new(),
                    },
                })
            })
            .collect::<ServerResult<_>>()?;
        reports.extend(converted);
        Ok(reports)
    }
    pub(super) async fn symbols(
        &mut self,
        uri: Uri,
    ) -> ServerResult<Option<DocumentSymbolResponse>> {
        let path = uri::to_path(&uri)?;
        self.ensure_owners(&path).await?;
        self.flush().await?;
        let owners = self.ensure_owners(&path).await?;
        let owner = *owners
            .first()
            .ok_or_else(|| ServerError::new(-32603, "Document has no Workspace"))?;
        if !self.projects[owner].settings.document_symbols.enable {
            return Ok(None);
        }
        let snapshot = self.projects[owner]
            .workspace
            .load_snapshot(&path)
            .await
            .map_err(internal)?;
        let input = SymbolInput {
            uri,
            snapshot,
            encoding: self.encoding,
            hierarchical: self.hierarchical,
        };
        tokio::task::spawn_blocking(move || symbols::document_symbols(input))
            .await
            .map_err(internal)?
    }
}
fn convert_reports(
    inputs: Vec<ReportInput>,
    encoding: CoordinateEncoding,
) -> ServerResult<Vec<PreparedReport>> {
    inputs
        .into_iter()
        .map(|input| {
            let mut items = if !input.key.enabled {
                Vec::new()
            } else if let Some(snapshot) = &input.snapshot {
                let mut items = diagnostics::snapshot_diagnostics(
                    snapshot,
                    encoding,
                    input.key.contextual,
                    &input.key.resolved_calls,
                )?;
                let index = TextIndex::new(&snapshot.text, encoding);
                for (span, severity, message) in &input.key.extra {
                    let mut item = diagnostics::diagnostic(&index, *span, message.clone())?;
                    item.severity = Some(DiagnosticSeverity(i32::from(*severity)));
                    items.push(item);
                }
                items
            } else {
                vec![diagnostics::diagnostic(
                    &TextIndex::new("", encoding),
                    Default::default(),
                    "Document analysis is unavailable".into(),
                )?]
            };
            let mut seen = BTreeSet::new();
            items.retain(|item| {
                seen.insert((
                    item.range.start.line,
                    item.range.start.character,
                    item.range.end.line,
                    item.range.end.character,
                    item.message.clone(),
                ))
            });
            Ok(PreparedReport {
                path: input.path,
                report: DiagnosticReport {
                    uri: input.uri,
                    version: Some(input.key.version),
                    diagnostics: items,
                },
                key: Some(input.key),
            })
        })
        .collect()
}
