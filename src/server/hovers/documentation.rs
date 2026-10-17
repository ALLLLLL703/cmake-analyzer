//! Resolve owner-local snapshots and deduplicate bound source comments in memory; no file reads or CMake
//! execution.

use super::render;
use crate::server::model::Service;
use crate::{
    parser::semanticer::model::{Documentation, FileId, ReferenceKind, ReferenceLocation},
    workspace::{Workspace, model::DocumentSnapshot},
};
use std::{
    collections::HashSet,
    path::{Path, PathBuf},
    sync::Arc,
};

/// File IDs are resolved within their owning Workspace, never across owners.
/// Resolve a FileId only within its owning workspace; no disk reads or mutation.
///
/// # Arguments
/// - `workspace` `[in]`: owner-local graph, runtime layer and cached snapshots.
/// - `file` `[in]`: snapshot-local file identity to resolve.
/// - `fallback` `[in]`: current document path tried only after graph/runtime sources.
///
/// # Returns
/// Some((owned path, shared snapshot)) for the first matching graph/runtime/fallback source; None if
/// absent or fallback lookup fails.
///
/// # Algorithm
/// Prefer the first graph visit with this FileId, then an installed runtime snapshot, then a cached
/// fallback snapshot only if its identity matches. Clone shared handles rather than source text.
///
/// # Flow
/// ```text
///  [Owner-local FileId]
///           |
///  [Graph visit match?] --yes--> [Path + shared snapshot]
///           | no                         ^
///  [Runtime file match?] --yes------------+
///           | no                         |
///  [Fallback cached identity?] --yes------+
///           | no
///         [None]
/// ```
///
/// # Examples
/// Local server context:
/// ```text
/// let source = documentation::source(workspace, definition.file, &path);
/// ```
pub(super) fn source(
    workspace: &Workspace,
    file: FileId,
    fallback: &Path,
) -> Option<(PathBuf, Arc<DocumentSnapshot>)> {
    if let Some(visit) = workspace.root_graph().and_then(|graph| {
        graph
            .invocations
            .iter()
            .find(|visit| visit.snapshot.semantics.id == file)
    }) {
        return Some((visit.file.clone(), visit.snapshot.clone()));
    }
    if let Some(layer) = workspace.runtime() {
        if let Some((path, snapshot)) = layer
            .documents
            .iter()
            .find(|(_, snapshot)| snapshot.semantics.id == file)
        {
            return Some((path.clone(), snapshot.clone()));
        }
    }
    workspace
        .snapshot(fallback)
        .ok()
        .flatten()
        .filter(|snapshot| snapshot.semantics.id == file)
        .map(|snapshot| (fallback.to_owned(), snapshot))
}
/// Compare owner-local variable site identity and read-versus-write class; no side effects.
///
/// # Arguments
/// - `reference` `[in]`: indexed candidate reference.
/// - `location` `[in]`: hover reference, whose file field is not used directly.
/// - `file` `[in]`: current owner snapshot FileId.
///
/// # Returns
/// True for matching file, command, span endpoints and equal Read status; non-read roles are treated
/// alike.
///
/// # Examples
/// Local server context:
/// ```text
/// let matched = same(&reference, &target.location, snapshot.semantics.id);
/// ```
fn same(reference: &ReferenceLocation, location: &ReferenceLocation, file: FileId) -> bool {
    reference.file == file
        && reference.command == location.command
        && reference.span.start_byte == location.span.start_byte
        && reference.span.end_byte == location.span.end_byte
        && (reference.kind == ReferenceKind::Read) == (location.kind == ReferenceKind::Read)
}
/// Append one previously unseen bound comment section in memory; no I/O.
///
/// # Arguments
/// - `content` `[in,out]`: string receiving a bounded rendered comment section; appends only.
/// - `seen` `[in,out]`: inserts (path, first-comment start byte) even if the rendered section has no
///   remaining budget.
/// - `path` `[in]`: owning source path used in the deduplication key.
/// - `source` `[in]`: snapshot text containing the comment spans.
/// - `doc` `[in]`: bound documentation spans; empty spans cause no change.
/// - `limit` `[in]`: total byte cap for content.
///
/// # Returns
/// (); no appended text for an empty or already-seen comment group.
///
/// # Examples
/// Local server context:
/// ```text
/// let mut text = String::new();
/// let mut seen = HashSet::new();
/// documentation::add(&mut text, &mut seen, &path, &snapshot, &doc, 4096);
/// ```
pub(super) fn add(
    content: &mut String,
    seen: &mut HashSet<(PathBuf, usize)>,
    path: &Path,
    source: &DocumentSnapshot,
    doc: &Documentation,
    limit: usize,
) {
    if let Some(span) = doc.spans.first() {
        if seen.insert((path.to_owned(), span.start_byte)) {
            render::section(
                content,
                &doc.render(&source.text, limit.saturating_sub(content.len())),
                limit,
            );
        }
    }
}
/// Find the nearest documented preceding variable write for each matching owner binding; no I/O.
///
/// # Arguments
/// - `service` `[in]`: owner projects and immutable cached semantic data.
/// - `path` `[in]`: hover document path.
/// - `name` `[in]`: case-sensitive variable name.
/// - `location` `[in]`: hover reference to qualify the binding and source order.
/// - `limit` `[in]`: maximum returned comment bytes.
///
/// # Returns
/// Deduplicated comment text, or an empty string if no documented binding is found.
///
/// # Algorithm
/// Prefer graph bindings matched to the exact hover site. Walk preceding non-read references backward to
/// the nearest documented set command. If no graph binding matches, use local variable writes whose
/// commands precede the hover; stop owner work once the byte budget is full.
///
/// # Flow
/// ```text
///  [Owners + variable site]
///             |
///  [Graph binding matched?] --yes--> [Walk prior writes backward]
///             | no                           |
///  [Local references/output span]      [Nearest documented set]
///             |                              |
///  [Latest documented local write]           |
///             +------------------------------+
///                            v
///                 [Deduplicate + append]
/// ```
///
/// # Examples
/// Local server context (requires the initialized service and its client):
/// ```text
/// let comments = documentation::variable(service, &path, "VALUE", &target.location, 4096);
/// ```
pub(super) fn variable(
    service: &Service,
    path: &Path,
    name: &str,
    location: &ReferenceLocation,
    limit: usize,
) -> String {
    let mut content = String::new();
    let mut seen = HashSet::new();
    for owner in service.owners(path) {
        if content.len() >= limit {
            break;
        }
        let workspace = &service.projects[owner].workspace;
        let Some(snapshot) = workspace.snapshot(path).ok().flatten() else {
            continue;
        };
        let mut indexed = false;
        if let Some(graph) = workspace.root_graph() {
            for variable in graph
                .references
                .variables
                .iter()
                .filter(|variable| variable.name == name)
            {
                let Some(position) = variable
                    .references
                    .iter()
                    .position(|reference| same(reference, location, snapshot.semantics.id))
                else {
                    continue;
                };
                indexed = true;
                for reference in variable.references[..=position]
                    .iter()
                    .rev()
                    .filter(|reference| reference.kind != ReferenceKind::Read)
                {
                    let Some((file, source)) = source(workspace, reference.file, path) else {
                        continue;
                    };
                    let Some(command) =
                        source.semantics.commands.get(reference.command.id as usize)
                    else {
                        continue;
                    };
                    if let Some(doc) = &command.documentation {
                        if command.name.eq_ignore_ascii_case("set") {
                            add(&mut content, &mut seen, &file, &source, doc, limit);
                            break;
                        }
                    }
                }
            }
        }
        if indexed {
            continue;
        }
        for variable in snapshot.semantics.variables.iter().filter(|variable| {
            variable.name == name
                && (variable
                    .references
                    .iter()
                    .any(|reference| same(reference, location, snapshot.semantics.id))
                    || variable.changing.iter().any(|write| {
                        write.command == location.command
                            && write.span.start_byte <= location.span.start_byte
                            && location.span.end_byte <= write.span.end_byte
                    }))
        }) {
            if let Some(doc) = variable
                .changing
                .iter()
                .rev()
                .filter(|write| write.command.id <= location.command.id)
                .find_map(|write| write.documentation.as_ref())
            {
                add(&mut content, &mut seen, path, &snapshot, doc, limit);
            }
        }
    }
    content
}
