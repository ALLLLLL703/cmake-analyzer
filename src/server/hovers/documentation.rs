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
fn same(reference: &ReferenceLocation, location: &ReferenceLocation, file: FileId) -> bool {
    reference.file == file
        && reference.command == location.command
        && reference.span.start_byte == location.span.start_byte
        && reference.span.end_byte == location.span.end_byte
        && (reference.kind == ReferenceKind::Read) == (location.kind == ReferenceKind::Read)
}
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
