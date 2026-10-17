//! Combine command-qualified static values, recorded observations and source comments in memory; never
//! execute CMake.

// AI-authored variable hover with position-qualified values and documentation.
use super::{documentation, model::Target, render};
use crate::{
    parser::semanticer::model::{PossibleValue, ReferenceKind},
    server::model::Service,
};
use std::path::Path;
/// Render a possible value as a debug-escaped string, or Unknown; no side effects.
/// Reserve sixfold space for debug escapes before adding quotes and an optional ellipsis; this helper
/// alone is not a hard final byte cap.
///
/// # Arguments
/// - `value` `[in]`: optional static or observed value.
/// - `budget` `[in]`: approximate display allowance; raw text is limited to budget/6 bytes at a UTF-8
///   boundary.
///
/// # Returns
/// Unknown when absent; otherwise quoted escaped text with an ellipsis if shortened.
///
/// # Examples
/// Local server context:
/// ```text
/// assert_eq!(value_text(None, 60), "Unknown");
/// ```
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
/// Combine variable values and bound comments across owners without I/O or source mutation.
/// Static observations are command/role-qualified; installed runtime observations remain separately
/// labeled. Unknown values are not treated as proven initialization.
///
/// # Arguments
/// - `service` `[in]`: owner snapshots, graphs, runtime layers and Markdown preference.
/// - `path` `[in]`: document path used for owner-local lookup.
/// - `target` `[in]`: selected name and snapshot-local reference site.
/// - `limit` `[in]`: view-byte allowance used for value/comment construction; final render::hover enforces
///   the hard cap.
///
/// # Returns
/// Owned hover text headed Variable, with Static/Observed lines as available and deduplicated comments.
/// This intermediate string can exceed the final cap after Markdown indentation.
///
/// # Algorithm
/// For each owner, collect matching graph values and recorded values until their reserves are exhausted.
/// Fall back to the target reference value only if no contextual static value exists. Indent value text
/// for Markdown, then append bound comments.
///
/// # Flow
/// ```text
///  [Target + owners]
///          |
///  [Graph static values]    [Recorded values]
///          |                      |
///          +----------+-----------+
///                     |
///  [No static context?] --yes--> [Reference fallback]
///                     |                |
///                     +----------------+
///                     v
///            [Indent + append comments]
///                     |
///               [Return text]
/// ```
///
/// # Examples
/// Local server context (requires the initialized service and its client):
/// ```text
/// let text = variables::content(service, &path, &target, 4096);
/// ```
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
