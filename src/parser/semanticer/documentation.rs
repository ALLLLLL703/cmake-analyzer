use super::model::*;
use crate::model::TextSpan;
use std::sync::Arc;

fn adjacent(source: &str, start: usize, end: usize) -> bool {
    source.get(start..end).is_some_and(|gap| {
        gap.chars().all(char::is_whitespace)
            && gap.bytes().filter(|byte| *byte == b'\n').count() <= 1
    })
}
fn standalone(source: &str, span: TextSpan) -> bool {
    let start = source[..span.start_byte]
        .rfind('\n')
        .map_or(0, |offset| offset + 1);
    source[start..span.start_byte]
        .chars()
        .all(char::is_whitespace)
}
impl Semanticer {
    /// Rebind leading comment groups even when the command payload was reused.
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
    /// Render only the bound ranges from their owning source snapshot, with a UTF-8 byte budget.
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
