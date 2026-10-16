// AI-authored variable hover with position-qualified values and documentation.
use super::{documentation, model::Target, render};
use crate::{
    parser::semanticer::model::{PossibleValue, ReferenceKind},
    server::model::Service,
};
use std::path::Path;
fn value_text(value: Option<&PossibleValue>, budget: usize) -> String {
    value.map_or_else(
        || "Unknown".into(),
        |value| {
            let text = value.text();
            let mut end = text.len().min(budget / 6);
            while !text.is_char_boundary(end) {
                end -= 1;
            }
            format!(
                "{:?}{}",
                &text[..end],
                if end < text.len() { " …" } else { "" }
            )
        },
    )
}
pub(super) fn content(service: &Service, path: &Path, target: &Target, limit: usize) -> String {
    let name = target.name.as_str();
    let location = &target.location;
    let mut content = format!(
        "Variable {:?}\n",
        name.chars().take(limit / 6).collect::<String>()
    );
    let mut contextual = false;
    for owner in service.owners(path) {
        let workspace = &service.projects[owner].workspace;
        let Some(snapshot) = workspace.snapshot(path).ok().flatten() else {
            continue;
        };
        let file = snapshot.semantics.id;
        if let Some(graph) = workspace.root_graph() {
            for value in graph.values.iter().filter(|value| {
                value.name == name
                    && value.location.file == file
                    && value.location.command == location.command
                    && (value.location.kind == ReferenceKind::Read)
                        == (location.kind == ReferenceKind::Read)
            }) {
                contextual = true;
                if content.len() + 64 >= limit {
                    break;
                }
                content.push_str(&format!(
                    "Static: {}\n",
                    value_text(
                        value.possible_value.as_ref(),
                        limit.saturating_sub(content.len() + 64)
                    )
                ));
            }
        }
        if let Some(layer) = workspace.runtime() {
            for value in layer.values.iter().filter(|value| {
                value.name == name
                    && value.location.as_ref().is_some_and(|source| {
                        source.file == file
                            && source.command == location.command
                            && (source.kind == ReferenceKind::Read)
                                == (location.kind == ReferenceKind::Read)
                    })
            }) {
                if content.len() + 80 >= limit {
                    break;
                }
                content.push_str(&format!(
                    "Observed: {}\n",
                    value_text(
                        value.possible_value.as_ref(),
                        limit.saturating_sub(content.len() + 80)
                    )
                ));
            }
        }
    }
    if !contextual && content.len() + 16 < limit {
        content.push_str(&format!(
            "Static: {}",
            value_text(
                location.possible_value.as_ref(),
                limit.saturating_sub(content.len() + 16)
            )
        ));
    }
    if service.hover_markdown {
        content = format!("    {}", content.trim_end().replace('\n', "\n    "));
    }
    let comments = documentation::variable(
        service,
        path,
        name,
        location,
        limit.saturating_sub(content.len() + 2),
    );
    render::section(&mut content, &comments, limit);
    content
}
