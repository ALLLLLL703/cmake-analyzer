use super::{
    arguments,
    model::{Candidate, Kind, Site},
};
use crate::server::text::TextIndex;
use lspally::server::model::*;

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
