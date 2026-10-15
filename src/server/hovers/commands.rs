use super::{builtins, documentation, model::Target, render};
use crate::{
    parser::semanticer::model::{ReferenceKind, ReferenceLocation},
    server::model::Service,
};
use lspally::server::model::*;
use std::{collections::HashSet, path::Path};
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
pub(super) fn hover(service: &Service, path: &Path, target: Target, limit: usize) -> Option<Hover> {
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
            known = true;
            let Some((file, source)) =
                documentation::source(workspace, function.definition.file, path)
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
        let doc = builtins::command(&target.name)?;
        render::append(&mut content, doc, limit);
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
    Some(render::hover(
        content,
        target.range,
        limit,
        service.hover_markdown,
    ))
}
