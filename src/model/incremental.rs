// AI-authored incremental source mapping.
use super::TextSpan;
use std::ops::Range;

#[derive(Clone, Debug, Default)]
pub struct IncrementalWork {
    pub recomputed: usize,
    pub reused: usize,
}

/// One replacement envelope; multiple edits between snapshots may share an envelope.
#[derive(Clone, Debug)]
pub struct SourceChange {
    pub old: Range<usize>,
    pub new: Range<usize>,
}
impl SourceChange {
    pub fn between(old: &str, new: &str) -> Self {
        let mut prefix = old
            .bytes()
            .zip(new.bytes())
            .take_while(|(a, b)| a == b)
            .count();
        while !old.is_char_boundary(prefix) || !new.is_char_boundary(prefix) {
            prefix -= 1;
        }
        let mut suffix = old[prefix..]
            .bytes()
            .rev()
            .zip(new[prefix..].bytes().rev())
            .take_while(|(a, b)| a == b)
            .count();
        while !old.is_char_boundary(old.len() - suffix) || !new.is_char_boundary(new.len() - suffix)
        {
            suffix -= 1;
        }
        Self {
            old: prefix..old.len() - suffix,
            new: prefix..new.len() - suffix,
        }
    }
    pub fn offset(&self, old: usize) -> Option<usize> {
        if old < self.old.start {
            Some(old)
        } else if old >= self.old.end {
            old.checked_sub(self.old.end)?.checked_add(self.new.end)
        } else {
            None
        }
    }
    pub fn range(&self, range: Range<usize>) -> Option<Range<usize>> {
        if range.end <= self.old.start {
            Some(range)
        } else if range.start >= self.old.end {
            Some(self.offset(range.start)?..self.offset(range.end)?)
        } else {
            None
        }
    }
}

/// Rebuild positions from byte offsets; columns remain Unicode scalar counts.
pub struct SourcePositions {
    lines: Vec<usize>,
    wide: Vec<(usize, usize)>,
}
impl SourcePositions {
    pub fn new(source: &str) -> Self {
        let mut lines = vec![0];
        let mut wide = Vec::new();
        let mut extra = 0;
        for (offset, ch) in source.char_indices() {
            if ch == '\n' {
                lines.push(offset + 1);
            }
            if ch.len_utf8() > 1 {
                extra += ch.len_utf8() - 1;
                wide.push((offset, extra));
            }
        }
        Self { lines, wide }
    }
    // Only multibyte characters need an index; ASCII files do not allocate one entry per character.
    fn character_offset(&self, byte: usize) -> usize {
        let index = self.wide.partition_point(|(offset, _)| *offset < byte);
        byte - index.checked_sub(1).map_or(0, |i| self.wide[i].1)
    }
    pub fn span(&self, range: Range<usize>) -> TextSpan {
        let row = self.lines.partition_point(|offset| *offset <= range.start) - 1;
        TextSpan {
            start_byte: range.start,
            end_byte: range.end,
            row: row as u64,
            column: (self.character_offset(range.start) - self.character_offset(self.lines[row]))
                as u64,
        }
    }
    pub fn remap(&self, change: &SourceChange, span: TextSpan) -> Option<TextSpan> {
        change
            .range(span.start_byte..span.end_byte)
            .map(|range| self.span(range))
    }
}
