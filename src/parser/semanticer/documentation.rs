//! Bind nearby standalone comments and render bounded documentation text.
//!
//! Ranges always refer to the same immutable source as their declarations. No I/O.
// Contains AI-generated documentation additions.
use super::model::*;
use crate::model::TextSpan;
use std::sync::Arc;

/// Check whether a source gap contains only whitespace and at most one newline. No side effects.
///
/// # Arguments
/// - `source` `[in]`: source snapshot.
/// - `start` `[in]`: inclusive gap byte offset.
/// - `end` `[in]`: exclusive gap byte offset.
///
/// # Returns
/// True for an in-bounds UTF-8 gap satisfying the adjacency rule; false for invalid bounds or other text.
///
/// # Examples
/// ```text
/// assert!(adjacent("# doc
/// set(x y)", 5, 6));
/// ```
fn adjacent(source: &str, start: usize, end: usize) -> bool {
    source.get(start..end).is_some_and(|gap| {
        gap.chars().all(char::is_whitespace)
            && gap.bytes().filter(|byte| *byte == b'\n').count() <= 1
    })
}
/// Check whether only whitespace precedes a comment on its line. No side effects.
///
/// # Arguments
/// - `source` `[in]`: owning source snapshot.
/// - `span` `[in]`: full comment range whose start is checked.
///
/// # Returns
/// True when the line prefix before the comment is whitespace-only.
///
/// # Panics
/// Panics if span.start_byte is outside source or not a UTF-8 boundary. Parser-produced spans must match source.
///
/// # Examples
/// ```text
/// let span = TextSpan { start_byte: 2, end_byte: 7, row: 0, column: 2 };
/// assert!(standalone("  # doc", span));
/// ```
fn standalone(source: &str, span: TextSpan) -> bool {
    let start = source[..span.start_byte]
        .rfind('\n')
        .map_or(0, |offset| offset + 1);
    source[start..span.start_byte]
        .chars()
        .all(char::is_whitespace)
}
impl Semanticer {
    /// Overwrite command documentation with adjacent standalone leading comments. Only function, macro and set declarations receive groups; no I/O.
    ///
    /// # Arguments
    /// - `self` `[in]`: analyzer, unchanged.
    /// - `source` `[in]`: immutable source matching all spans.
    /// - `comments` `[in]`: full comment ranges sorted in source order.
    /// - `buffer` `[in,out]`: overwrites every command documentation field; other fields unchanged.
    ///
    /// # Returns
    /// () after rebinding groups, including commands whose payloads were reused.
    ///
    /// # Panics
    /// May panic when comment starts do not belong to source; standalone checks use direct slices.
    ///
    /// # Algorithm
    /// For each supported declaration, locate preceding comments by end offset, then walk backward while standalone comments remain adjacent. Share the retained span group with the command.
    ///
    /// # Flow
    /// ```text
    ///  [command]        [sorted comments]
    ///      |                   |
    ///      +--------+----------+
    ///               v
    ///        <supported name?>
    ///          /         \
    ///        no          yes
    ///        |            |
    ///  [clear group] [walk backward]
    ///        |            |
    ///        +-----+------+
    ///              v
    ///       [store group or None]
    /// ```
    ///
    /// # Examples
    /// ```text
    /// analyzer.bind_documentation(source, &parsed.comments, &mut buffer);
    /// ```
    pub(crate) fn bind_documentation(
        &self,
        source: &str,
        comments: &[TextSpan],
        buffer: &mut SemanticedBuffer,
    ) {
        for command in &mut buffer.commands {
            command.documentation = None;
            if !["function", "macro", "set"]
                .iter()
                .any(|name| command.name.eq_ignore_ascii_case(name))
            {
                continue;
            }
            let end = comments.partition_point(|span| span.end_byte <= command.span.start_byte);
            let mut start = end;
            let mut boundary = command.span.start_byte;
            while start > 0 {
                let span = comments[start - 1];
                if !standalone(source, span) || !adjacent(source, span.end_byte, boundary) {
                    break;
                }
                start -= 1;
                boundary = span.start_byte;
            }
            if start < end {
                command.documentation = Some(Arc::new(Documentation {
                    spans: comments[start..end].to_vec(),
                }));
            }
        }
    }
}
impl Documentation {
    /// Render bound comments with a UTF-8 byte budget. Removes line/bracket comment delimiters and .rst: marker lines; no I/O or source mutation.
    ///
    /// # Arguments
    /// - `self` `[in]`: ordered bound comment ranges.
    /// - `source` `[in]`: owning immutable source; invalid ranges are skipped.
    /// - `limit` `[in]`: maximum output bytes, including inserted newlines.
    ///
    /// # Returns
    /// Owned text no longer than limit bytes; invalid spans and non-comment slices contribute nothing.
    ///
    /// # Algorithm
    /// Read only bound ranges, strip recognized comment wrappers, trim line endings and copy whole UTF-8 characters until the byte budget is exhausted.
    ///
    /// # Flow
    /// ```text
    ///  [bound spans]      [byte budget]
    ///        |                  |
    ///        v                  |
    ///  <valid comment?> --no--> [skip]
    ///        | yes              |
    ///        v                  |
    ///  [strip wrappers]         |
    ///        |                  |
    ///        +--------+---------+
    ///                 v
    ///       [append within budget]
    ///                 |
    ///           <budget left?>
    ///             /       \
    ///           next     return
    /// ```
    ///
    /// # Examples
    /// ```
    /// use cmake_analyzer::parser::semanticer::model::*;
    /// use cmake_analyzer::model::TextSpan;
    /// let docs = Documentation { spans: vec![TextSpan { start_byte: 0, end_byte: 7, row: 0, column: 0 }] };
    /// assert_eq!(docs.render("# hello", 4), "hell");
    /// ```
    pub fn render(&self, source: &str, limit: usize) -> String {
        let mut result = String::new();
        for span in &self.spans {
            let Some(raw) = span.text(source).and_then(|text| text.strip_prefix('#')) else {
                continue;
            };
            let body = if raw.starts_with('[') {
                let equals = raw
                    .as_bytes()
                    .iter()
                    .skip(1)
                    .take_while(|byte| **byte == b'=')
                    .count();
                let width = equals + 2;
                if raw.as_bytes().get(width - 1) == Some(&b'[')
                    && raw.ends_with(&format!("]{}]", "=".repeat(equals)))
                {
                    &raw[width..raw.len() - width]
                } else {
                    raw
                }
            } else {
                raw.strip_prefix(' ').unwrap_or(raw)
            };
            for line in body.trim_matches(['\r', '\n']).lines() {
                let line = line.trim_end();
                if line == ".rst:" {
                    continue;
                }
                if !result.is_empty() && result.len() < limit {
                    result.push('\n');
                }
                let mut end = line.len().min(limit.saturating_sub(result.len()));
                while !line.is_char_boundary(end) {
                    end -= 1;
                }
                result.push_str(&line[..end]);
                if result.len() >= limit {
                    return result;
                }
            }
        }
        result
    }
}
