//! Read source-bound function/macro comments across static owners and recorded runtime calls; no I/O.

use super::{documentation, model::Target, render};
use crate::{
    parser::semanticer::model::{ReferenceKind, ReferenceLocation},
    server::model::Service,
};
use std::{collections::HashSet, path::Path};
/// Compare command and byte-name identity within an explicitly supplied owner-local file; no side
/// effects.
///
/// # Arguments
/// - `reference` `[in]`: candidate definition/call/reference.
/// - `target` `[in]`: hover location; its file field is not compared directly.
/// - `file` `[in]`: current snapshot FileId that the candidate must belong to.
///
/// # Returns
/// True for equal command and span endpoints in file, regardless of reference role.
///
/// # Examples
/// Local server context:
/// ```text
/// let matched = same(&reference, &target.location, snapshot.semantics.id);
/// ```
fn same(
    reference: &ReferenceLocation,
    target: &ReferenceLocation,
    file: crate::parser::semanticer::model::FileId,
) -> bool {
    reference.file == file
        && reference.command == target.command
        && reference.span.start_byte == target.span.start_byte
        && reference.span.end_byte == target.span.end_byte
}
/// Resolve source-bound function or macro documentation, suppressing native-help fallback for known user
/// commands; no I/O.
///
/// # Arguments
/// - `service` `[in]`: owner workspaces and installed runtime layers.
/// - `path` `[in]`: document path for ownership and source-command matching.
/// - `target` `[in]`: hover spelling and selected reference.
/// - `limit` `[in]`: byte cap for appended comments or fallback heading.
///
/// # Returns
/// Some(text) for a known source/runtime function even without comments; None when no bound user
/// definition/call matches. A zero budget can yield Some(empty).
///
/// # Algorithm
/// Match owner-local static function references and their definitions, then recorded function
/// calls/declarations mapped to the same source command. Read bound comments from owning snapshots and
/// deduplicate them. If known but undocumented, emit a Function heading.
///
/// # Flow
/// ```text
///  [Each owner]
///       |
///  [Static bound function]   [Mapped runtime call]
///       |                            |
///       +--------------+-------------+
///                      v
///              [Read/deduplicate comments]
///                      |
///               [Known definition?] --no--> [None]
///                      | yes
///               [Comments or heading]
/// ```
///
/// # Examples
/// Local server context (requires the initialized service and its client):
/// ```text
/// let user_doc = commands::content(service, &path, &target, 4096);
/// ```
pub(super) fn content(
    service: &Service,
    path: &Path,
    target: &Target,
    limit: usize,
) -> Option<String> {
    let mut content = String::new();
    let mut known = false;
    let mut seen = HashSet::new();
    for owner in service.owners(path) {
        let workspace = &service.projects[owner].workspace;
        let Some(snapshot) = workspace.snapshot(path).ok().flatten() else {
            continue;
        };
        let functions = workspace
            .root_graph()
            .map_or(snapshot.semantics.functions.as_slice(), |graph| {
                graph.references.functions.as_slice()
            });
        for function in functions.iter().filter(|function| {
            function.name.eq_ignore_ascii_case(&target.name)
                && function
                    .references
                    .iter()
                    .any(|reference| same(reference, &target.location, snapshot.semantics.id))
        }) {
            let Some(definition) = function.definition.as_ref() else {
                continue;
            };
            known = true;
            let Some((file, source)) = documentation::source(workspace, definition.file, path)
            else {
                continue;
            };
            if let Some(doc) = &function.documentation {
                documentation::add(&mut content, &mut seen, &file, &source, doc, limit);
            }
        }
        if let Some(layer) = workspace.runtime() {
            for function in layer
                .functions
                .iter()
                .filter(|function| function.name.eq_ignore_ascii_case(&target.name))
            {
                let matches = |id: crate::runtime_cmake::model::EventId| {
                    layer.trace.events[id.index()].file == path
                        && layer.source_commands[id.index()] == Some(target.location.command)
                };
                if !function.calls.iter().copied().any(matches)
                    && !(target.location.kind == ReferenceKind::Definition
                        && matches(function.declaration))
                {
                    continue;
                }
                known = true;
                let event = &layer.trace.events[function.declaration.index()];
                let Some(source) = layer.documents.get(&event.file) else {
                    continue;
                };
                let Some(id) = layer.source_commands[event.id.index()] else {
                    continue;
                };
                let Some(doc) = source.semantics.commands[id.id as usize]
                    .documentation
                    .as_ref()
                else {
                    continue;
                };
                documentation::add(&mut content, &mut seen, &event.file, source, doc, limit);
            }
        }
    }
    if !known {
        return None;
    } else if content.is_empty() {
        render::append(
            &mut content,
            &format!(
                "Function {:?}",
                target.name.chars().take(limit / 6).collect::<String>()
            ),
            limit,
        );
    }
    Some(content)
}
