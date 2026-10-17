//! Build byte-limited UTF-8 hover text in memory, preserving native help as literal text; no I/O.

use lspally::server::model::*;

/// Append as much text as fits the total byte limit, stopping at a UTF-8 boundary; no I/O.
///
/// # Arguments
/// - `content` `[in,out]`: existing string; appends only, never overwrites or shrinks it.
/// - `text` `[in]`: text whose prefix may be copied.
/// - `limit` `[in]`: maximum total bytes; an already oversized content string is left oversized.
///
/// # Returns
/// (); appended prefix may be empty when no budget remains.
///
/// # Examples
/// Local server context:
/// ```text
/// let mut content = String::new();
/// append(&mut content, "é!", 2);
/// assert_eq!(content, "é");
/// ```
pub(super) fn append(content: &mut String, text: &str, limit: usize) {
    let remaining = limit.saturating_sub(content.len());
    let mut end = text.len().min(remaining);
    while !text.is_char_boundary(end) {
        end -= 1;
    }
    content.push_str(&text[..end]);
}
/// Append a nonempty section separated from existing content by two newlines, within the byte budget; no
/// I/O.
///
/// # Arguments
/// - `content` `[in,out]`: text receiving separator and section prefixes; never overwritten.
/// - `text` `[in]`: section; empty input adds no separator.
/// - `limit` `[in]`: total byte cap passed to append.
///
/// # Returns
/// (); separator alone may consume the remaining budget.
///
/// # Examples
/// Local server context:
/// ```text
/// let mut content = "Value".to_owned();
/// section(&mut content, "Comment", 64);
/// assert_eq!(content, "Value\n\nComment");
/// ```
pub(super) fn section(content: &mut String, text: &str, limit: usize) {
    if text.is_empty() {
        return;
    }
    if !content.is_empty() {
        append(content, "\n\n", limit);
    }
    append(content, text, limit);
}
/// CMake help is reStructuredText, not Markdown; preserve its literal formatting.
/// Append native reStructuredText as plaintext or indented Markdown code; no I/O.
/// Markdown indentation preserves literal help formatting rather than interpreting it as Markdown.
///
/// # Arguments
/// - `content` `[in,out]`: existing text; appends a separator and help prefixes, never overwrites.
/// - `text` `[in]`: native help body.
/// - `limit` `[in]`: total UTF-8 byte cap.
/// - `markdown` `[in]`: true adds four spaces per line; false uses a plain section.
///
/// # Returns
/// (); output may end partway through a line or indentation at the byte limit.
///
/// # Algorithm
/// Plaintext uses section unchanged. Markdown first adds a separator if needed, then iterates lines,
/// adding newline and four-space prefixes through the same UTF-8 bounded append helper.
///
/// # Flow
/// ```text
///  [Native help + existing text]
///               |
///         [Markdown?] --no--> [Plain section]
///               | yes
///         [Separator if needed]
///               |
///         [Next line] <------------------+
///               |                        |
///         [Room remains?] --yes--> [Indent + append]
///               | no / end
///             [Return]
/// ```
///
/// # Examples
/// Local server context:
/// ```text
/// let mut content = String::new();
/// local_help(&mut content, "set(<variable>)", 128, true);
/// assert!(content.starts_with("    set"));
/// ```
pub(super) fn local_help(content: &mut String, text: &str, limit: usize, markdown: bool) {
    if !markdown {
        section(content, text, limit);
        return;
    }
    if !content.is_empty() {
        append(content, "\n\n", limit);
    }
    for (index, line) in text.lines().enumerate() {
        if content.len() >= limit {
            break;
        }
        if index > 0 {
            append(content, "\n", limit);
        }
        append(content, "    ", limit);
        append(content, line, limit);
    }
}
/// Wrap text in a client hover, truncating at a UTF-8 boundary; no I/O.
///
/// # Arguments
/// - `content` `[in]`: owned text, locally shortened if over budget.
/// - `range` `[in]`: selected name range attached to the result.
/// - `limit` `[in]`: maximum returned content bytes.
/// - `markdown` `[in]`: selects Markdown rather than PlainText markup.
///
/// # Returns
/// Hover with Markup contents and Some(range); may contain an empty value.
///
/// # Examples
/// Local server context:
/// ```text
/// let rendered = hover("VALUE".into(), range, 3, false);
/// ```
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
