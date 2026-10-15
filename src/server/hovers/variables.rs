// AI-authored variable-only hover with position-qualified static and observed values.
use super::*;
use crate::{
    parser::semanticer::model::{PossibleValue, ReferenceKind},
    server::{service::internal, text::TextIndex, uri},
};
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
pub(super) async fn hover(
    service: &Service,
    uri: Uri,
    position: Position,
) -> ServerResult<Option<Hover>> {
    let path = uri::to_path(&uri)?;
    let owners = service.owners(&path);
    let Some(snapshot) = owners.iter().find_map(|owner| {
        service.projects[*owner]
            .workspace
            .snapshot(&path)
            .ok()
            .flatten()
    }) else {
        return Ok(None);
    };
    let encoding = service.encoding;
    let source = snapshot.clone();
    let target = tokio::task::spawn_blocking(move || {
        let index = TextIndex::new(&source.text, encoding);
        let offset = index.byte(&position)?;
        let found = source
            .semantics
            .variables
            .iter()
            .find_map(|variable| {
                variable
                    .references
                    .iter()
                    .find(|reference| {
                        reference.span.start_byte <= offset && offset < reference.span.end_byte
                    })
                    .map(|reference| (variable.name.clone(), reference.clone()))
            })
            .or_else(|| {
                source
                    .semantics
                    .commands
                    .iter()
                    .flat_map(|command| &command.variable_references)
                    .find(|reference| {
                        reference.location.span.start_byte <= offset
                            && offset < reference.location.span.end_byte
                    })
                    .map(|reference| (reference.name.clone(), reference.location.clone()))
            })
            .or_else(|| {
                source
                    .semantics
                    .unresolved_references
                    .iter()
                    .find(|reference| {
                        reference.target
                            == crate::parser::semanticer::model::ReferenceTarget::Variable
                            && reference.location.span.start_byte <= offset
                            && offset < reference.location.span.end_byte
                    })
                    .map(|reference| (reference.name.clone(), reference.location.clone()))
            })
            .or_else(|| {
                source.semantics.commands.iter().find_map(|command| {
                    command
                        .args
                        .iter()
                        .filter(|arg| {
                            arg.kind.iter().any(|kind| {
                                matches!(kind, crate::parser::semanticer::model::ArgKind::Out)
                            })
                        })
                        .find_map(|arg| {
                            let name = arg.literal_text().filter(|name| {
                                !name.starts_with("ENV{") && !name.starts_with("CACHE{")
                            })?;
                            let span =
                                crate::parser::semanticer::parsing::references::argument_name_span(
                                    &source.text,
                                    arg,
                                );
                            (span.start_byte <= offset && offset < span.end_byte).then(|| {
                                (
                                    name.to_owned(),
                                    crate::parser::semanticer::model::ReferenceLocation {
                                        file: source.semantics.id,
                                        command: command.id,
                                        span,
                                        kind: ReferenceKind::Write,
                                        possible_value: None,
                                    },
                                )
                            })
                        })
                })
            });
        found
            .map(|(name, location)| {
                index
                    .range(location.span.range())
                    .map(|range| (name, location, range))
            })
            .transpose()
    })
    .await
    .map_err(internal)??;
    let Some((name, location, range)) = target else {
        return Ok(None);
    };
    let limit = owners
        .iter()
        .map(|owner| service.projects[*owner].settings.runtime.max_view_bytes)
        .min()
        .unwrap_or_else(|| crate::config::Runtime::default().max_view_bytes);
    let mut content = format!(
        "Variable {:?}\n",
        name.chars().take(limit / 6).collect::<String>()
    );
    let mut contextual = false;
    for owner in owners {
        let workspace = &service.projects[owner].workspace;
        if let Some(graph) = workspace.root_graph() {
            for value in graph.values.iter().filter(|value| {
                value.name == name
                    && value.location.file == snapshot.semantics.id
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
                        source.file == snapshot.semantics.id
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
    if content.len() > limit {
        let mut end = limit;
        while !content.is_char_boundary(end) {
            end -= 1;
        }
        content.truncate(end);
    }
    Ok(Some(Hover {
        contents: HoverContents::Markup(MarkupContent {
            kind: MarkupKind::PlainText,
            value: content,
        }),
        range: Some(range),
    }))
}
