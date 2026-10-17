//! Capture diagnostics and document symbols from immutable analysis snapshots.
//! Publication keys include text version, feature rules, snapshot identity and
//! owner-dependent observations. Conversion runs on a joined blocking worker and
//! does not publish; the runtime loop decides whether its results are still current.
// AI-authored diagnostic publication and symbol queries.
use super::{
    super::{diagnostics, model::*, symbols, text::TextIndex, uri},
    internal,
};
use lspally::server::model::*;
use std::{collections::BTreeSet, sync::Arc};

impl Service {
    /// Prepare changed open-file diagnostics and pending close-clear reports without publishing them.
    /// Captures shared snapshots and joins blocking conversion; does not mutate the publication cache.
    ///
    /// # Arguments
    /// - `self` `[in]`: Documents/projects/publication keys and pending clear URIs read without mutation.
    ///
    /// # Returns
    /// `Ok(reports)` with close clears first, then changed versioned reports; may be empty. `Err` on snapshot/URI/conversion-worker/range failures.
    ///
    /// # Errors
    /// Workspace snapshot lookup, invalid pending-clear URI, worker join failure or byte-range conversion error. No publication state is changed.
    ///
    /// # Algorithm
    /// For each open file combine enabled owners using strongest rule levels, first available snapshot, contextual/path/dependency warnings and resolved calls. Generated missing-child paths are excluded; child read failures use parent call ranges. Sort/deduplicate call IDs, compare the complete key against published, convert changed inputs on a worker, and prepend pending empty clears. Disabling diagnostics emits a clearing versioned report only if previously enabled.
    ///
    /// # Flow
    /// ```text
    ///  [Open documents]         [Pending close URIs]
    ///          |                        |
    ///  [Owners + policy + observations] |
    ///          |                        |
    ///  <Key changed?> -- no --> [Skip]   |
    ///          | yes                    |
    ///  [Blocking conversion]     [Empty clear reports]
    ///          |                        |
    ///          +------------+-----------+
    ///                       v
    ///                [Prepared, not sent]
    /// ```
    ///
    /// # Examples
    /// ```text
    /// let prepared = service.reports().await?;
    /// // runtime::drive rechecks queue freshness before publishing.
    /// ```
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
            let rules = owners
                .iter()
                .filter(|owner| self.projects[**owner].settings.diagnostics.enable)
                .fold(
                    crate::config::DiagnosticRules {
                        unknown_function: crate::config::DiagnosticLevel::None,
                        unknown_variable: crate::config::DiagnosticLevel::None,
                    },
                    |mut combined, owner| {
                        let rules = self.projects[*owner].settings.diagnostics.rules;
                        combined.unknown_function =
                            combined.unknown_function.min(rules.unknown_function);
                        combined.unknown_variable =
                            combined.unknown_variable.min(rules.unknown_variable);
                        combined
                    },
                );
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
                if let Some(layer) = workspace.runtime() {
                    if let Some(calls) = layer.resolved_calls.get(path) {
                        resolved_calls.extend(calls);
                    }
                }
                let paths = &self.projects[owner].path_state;
                for warning in paths
                    .warnings
                    .iter()
                    .filter(|warning| warning.file == *path)
                {
                    extra.push((warning.span, 2, warning.message.clone()));
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
                            if let Some(severity) =
                                diagnostics::variable_severity(warning.warning.content.kind, rules)
                            {
                                extra.push((
                                    warning.warning.span,
                                    severity,
                                    warning.warning.content.message(),
                                ));
                            }
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
                        let read_failure = diagnostic.kind
                            == crate::workspace::model::WorkspaceDiagnosticKind::ReadFailure;
                        if read_failure && paths.is_generated(&diagnostic.file) {
                            continue;
                        }
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
                                if read_failure && diagnostic.source.is_some() {
                                    2
                                } else {
                                    1
                                },
                                diagnostic.message.clone(),
                            ));
                        }
                    }
                }
            }
            resolved_calls.sort_by_key(|id| id.id);
            resolved_calls.dedup();
            let key = ReportKey {
                rules,
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
    /// Ensure ownership, flush pending analysis and load a document snapshot for symbol conversion.
    /// May admit projects and read source; conversion runs on a joined worker and does not mutate the snapshot.
    ///
    /// # Arguments
    /// - `self` `[in,out]`: Ownership/projects and Workspace snapshot caches may be populated; open text remains unchanged.
    /// - `uri` `[in]`: Consumed local URI of the requested file; file need not already be open.
    ///
    /// # Returns
    /// `Ok(Some(response))` with nested or flat symbols (possibly empty), `Ok(None)` when the first owner disables symbols, otherwise `Err`.
    ///
    /// # Errors
    /// URI, owner admission, overlay/flush, missing-owner, source loading or worker/range conversion errors.
    ///
    /// # Algorithm
    /// Ensure owners, flush, resolve owners again because ownership may have changed, select the first owner, honor its feature switch, load its snapshot and convert with negotiated encoding/hierarchy.
    ///
    /// # Flow
    /// ```text
    ///  [Document URI]
    ///         |
    ///  [Ensure owners + flush]
    ///         |
    ///  [Resolve first owner again]
    ///         |
    ///  <Symbols enabled?>
    ///      /         \
    ///     no         yes
    ///     |           |
    ///  [None]   [Load + convert snapshot]
    /// ```
    ///
    /// # Examples
    /// ```text
    /// let symbols = service.symbols(uri).await?;
    /// ```
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
            functions_only: false,
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
/// Convert captured diagnostic inputs to push payloads and retain publication keys.
/// Memory-only: neither sends reports nor updates the service cache.
///
/// # Arguments
/// - `inputs` `[in]`: Consumed captured snapshots, keys and document identities.
/// - `encoding` `[in]`: Coordinate units for outgoing diagnostic ranges.
///
/// # Returns
/// `Ok(Vec<PreparedReport>)` in input order; empty inputs yield an empty vector. `Err` aborts the batch on an invalid byte range.
///
/// # Errors
/// Invalid analysis byte spans or protocol coordinate overflow from TextIndex.
///
/// # Algorithm
/// Disabled inputs produce no diagnostics; available snapshots produce semantic/build plus owner-dependent findings; missing enabled analysis produces a fallback error. Remove duplicates by range/message (not severity), preserve the first occurrence, then attach the editor version and original key.
///
/// # Flow
/// ```text
///  [ReportInput]
///        |
///  <Enabled / snapshot?>
///     /       |        \
///  disabled present    absent
///     |       |          |
///  [Empty] [Convert] [Unavailable error]
///     +-------+----------+
///             v
///  [Deduplicate range + message]
///             |
///  [Versioned payload + key]
/// ```
///
/// # Examples
/// ```text
/// let reports = convert_reports(inputs, CoordinateEncoding::Utf16)?;
/// ```
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
                    input.key.rules,
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
