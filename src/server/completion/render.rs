//! Convert byte-based completion plans into negotiated client-coordinate edits.
//! Rendering changes no server text and performs no I/O; the client applies the returned edit.

use super::{
    arguments,
    model::{Candidate, Kind, Site},
};
use crate::server::text::TextIndex;
use lspally::server::model::*;

/// Render one client edit, replacing only the planned byte span.
/// Does not apply the edit or perform I/O.
///
/// # Arguments
/// - `candidate` `[in]`: Owned label, presentation kind and optional required snippet parameters.
/// - `site` `[in]`: Full replacement span and existing-parenthesis flag; wrappers and other arguments lie outside the span.
/// - `index` `[in]`: TextIndex for this exact source revision and negotiated encoding.
///
/// # Returns
/// `Ok(item)` contains one text edit and plain-text format, or snippet format when parameters exist and no parentheses already follow.
///
/// # Errors
/// `Err` reports invalid analysis byte boundaries or client-coordinate overflow during range conversion.
///
/// # Examples
/// ```text
/// let item = render::item(candidate, &site, &index)?;
/// ```
pub(super) fn item(
    candidate: Candidate,
    site: &Site,
    index: &TextIndex,
) -> ServerResult<CompletionItem> {
    let snippet = (!site.has_parentheses)
        .then_some(candidate.parameters.as_deref())
        .flatten();
    let new_text = snippet.map_or_else(
        || candidate.name.clone(),
        |parameters| arguments::snippet(&candidate.name, parameters),
    );
    Ok(CompletionItem {
        label: candidate.name,
        kind: Some(CompletionItemKind(match candidate.kind {
            Kind::Keyword => 14,
            Kind::Module => 9,
            Kind::Function => 3,
            Kind::Variable => 6,
            Kind::EnumMember => 20,
        })),
        text_edit: Some(CompletionTextEdit::Edit(TextEdit {
            range: index.range(site.replace.clone())?,
            new_text,
        })),
        insert_text_format: Some(InsertTextFormat(if snippet.is_some() { 2 } else { 1 })),
        label_details: None,
        detail: None,
        documentation: None,
        deprecated: None,
        preselect: None,
        sort_text: None,
        filter_text: None,
        insert_text: None,
        insert_text_mode: None,
        additional_text_edits: None,
        command: None,
        commit_characters: None,
        data: None,
        tags: None,
    })
}
