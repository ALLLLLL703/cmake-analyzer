use lspally::server::model::*;

pub(super) fn append(content: &mut String, text: &str, limit: usize) {
    let remaining = limit.saturating_sub(content.len());
    let mut end = text.len().min(remaining);
    while !text.is_char_boundary(end) {
        end -= 1;
    }
    content.push_str(&text[..end]);
}
pub(super) fn section(content: &mut String, text: &str, limit: usize) {
    if text.is_empty() {
        return;
    }
    if !content.is_empty() {
        append(content, "\n\n", limit);
    }
    append(content, text, limit);
}
pub(super) fn hover(mut content: String, range: Range, limit: usize, markdown: bool) -> Hover {
    if content.len() > limit {
        let mut end = limit;
        while !content.is_char_boundary(end) {
            end -= 1;
        }
        content.truncate(end);
    }
    Hover {
        contents: HoverContents::Markup(MarkupContent {
            kind: if markdown {
                MarkupKind::Markdown
            } else {
                MarkupKind::PlainText
            },
            value: content,
        }),
        range: Some(range),
    }
}
