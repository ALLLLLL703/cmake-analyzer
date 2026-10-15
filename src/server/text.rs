// AI-authored LSP byte/coordinate mapping.
use lspally::server::model::*;
use std::ops::Range as ByteRange;

pub(super) struct TextIndex<'a> {
    source: &'a str,
    lines: Vec<usize>,
    // Multibyte entries: byte offset, encoded units before the character, cumulative byte excess.
    wide: Vec<(usize, usize, usize)>,
}
impl<'a> TextIndex<'a> {
    pub fn new(source: &'a str, encoding: CoordinateEncoding) -> Self {
        let mut lines = vec![0];
        let mut wide = Vec::new();
        let mut extra = 0;
        for (byte, ch) in source.char_indices() {
            if ch == '\n' || (ch == '\r' && source.as_bytes().get(byte + 1) != Some(&b'\n')) {
                lines.push(byte + 1);
            }
            if ch.len_utf8() > 1 {
                let units = byte - extra;
                let width = match encoding {
                    CoordinateEncoding::Utf8 => ch.len_utf8(),
                    CoordinateEncoding::Utf16 => ch.len_utf16(),
                    CoordinateEncoding::Utf32 => 1,
                };
                extra += ch.len_utf8() - width;
                wide.push((byte, units, extra));
            }
        }
        Self {
            source,
            lines,
            wide,
        }
    }
    fn units_before(&self, byte: usize) -> usize {
        let count = self.wide.partition_point(|entry| entry.0 < byte);
        byte - count.checked_sub(1).map_or(0, |i| self.wide[i].2)
    }
    pub fn byte(&self, position: &Position) -> ServerResult<usize> {
        let start = *self
            .lines
            .get(position.line as usize)
            .ok_or_else(|| ServerError::invalid("Edit line is outside the document"))?;
        let end = self
            .lines
            .get(position.line as usize + 1)
            .copied()
            .unwrap_or(self.source.len());
        let end = start + self.source[start..end].trim_end_matches(['\r', '\n']).len();
        let units = self
            .units_before(start)
            .checked_add(position.character as usize)
            .ok_or_else(|| ServerError::invalid("Edit character is outside the document"))?;
        let count = self.wide.partition_point(|entry| entry.1 < units);
        let extra = count.checked_sub(1).map_or(0, |i| self.wide[i].2);
        let byte = units
            .checked_add(extra)
            .ok_or_else(|| ServerError::invalid("Edit character is outside the document"))?;
        // A UTF-16 position inside a surrogate pair maps inside its UTF-8 character, and is rejected.
        if byte >= start && byte <= end && self.source.is_char_boundary(byte) {
            Ok(byte)
        } else {
            Err(ServerError::invalid(
                "Edit character is outside a line or inside an encoded character",
            ))
        }
    }
    pub fn position(&self, byte: usize) -> ServerResult<Position> {
        if byte > self.source.len() || !self.source.is_char_boundary(byte) {
            return Err(ServerError::new(-32603, "Invalid analysis byte range"));
        }
        let line = self.lines.partition_point(|start| *start <= byte) - 1;
        let character = self.units_before(byte) - self.units_before(self.lines[line]);
        Ok(Position {
            line: u32::try_from(line)
                .map_err(|_| ServerError::new(-32603, "Document has too many lines"))?,
            character: u32::try_from(character)
                .map_err(|_| ServerError::new(-32603, "Line is too long"))?,
        })
    }
    pub fn range(&self, bytes: ByteRange<usize>) -> ServerResult<Range> {
        Ok(Range {
            start: self.position(bytes.start)?,
            end: self.position(bytes.end)?,
        })
    }
    pub fn edit_range(&self, range: &Range) -> ServerResult<ByteRange<usize>> {
        let start = self.byte(&range.start)?;
        let end = self.byte(&range.end)?;
        if start > end {
            Err(ServerError::invalid("Edit range is reversed"))
        } else {
            Ok(start..end)
        }
    }
}
pub(super) fn apply_changes(
    source: &str,
    changes: Vec<TextDocumentContentChangeEvent>,
    encoding: CoordinateEncoding,
) -> ServerResult<String> {
    let mut text = source.to_owned();
    for change in changes {
        if let Some(range) = change.range {
            let bytes = TextIndex::new(&text, encoding).edit_range(&range)?;
            text.replace_range(bytes, &change.text);
        } else {
            text = change.text;
        }
    }
    Ok(text)
}
