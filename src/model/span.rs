use std::{fmt::format, ops::Range, str::Chars};

use crate::model::Spanned;

use super::TextSpan;

impl TextSpan {
    pub fn range(&self) -> Range<usize> {
        self.start_byte..self.end_byte
    }

    /// Iterates byte offsets, including offsets inside multibyte characters.
    pub fn iter(&self) -> Range<usize> {
        self.range()
    }

    /// Returns None for reversed/out-of-bounds ranges or invalid UTF-8 boundaries.
    pub fn text<'a>(&self, source: &'a str) -> Option<&'a str> {
        source.get(self.range())
    }

    /// Iterates Unicode scalar values, not bytes or grapheme clusters.
    pub fn iter_text<'a>(&self, source: &'a str) -> Option<Chars<'a>> {
        self.text(source).map(str::chars)
    }

    pub fn combine_with_middle(span1: Self, span2: Self) -> Self {
        let follow_span = if span1.end_byte > span2.end_byte {
            span1
        } else {
            span2
        };
        let based = if span1.start_byte < span2.start_byte {
            span1
        } else {
            span2
        };

        let mut new_span = TextSpan::default();
        (
            new_span.column,
            new_span.row,
            new_span.start_byte,
            new_span.end_byte,
        ) = (
            based.column,
            based.row,
            based.start_byte,
            follow_span.end_byte,
        );

        new_span
    }

    pub fn self_combine_with_middle(&mut self, span2: Self) {
        let follow_span = if self.end_byte > span2.end_byte {
            *self
        } else {
            span2
        };
        let based = if self.start_byte < span2.start_byte {
            *self
        } else {
            span2
        };

        self.column = based.column;
        self.row = based.row;
        self.start_byte = based.start_byte;
        self.end_byte = follow_span.end_byte;
    }

    pub fn loc_info(&self) -> String {
        format!(
            "{{ col: {} row: {} end_byte: {} }}",
            self.column, self.row, self.end_byte
        )
    }
}

impl<T> Spanned<T> {
    pub fn new(content: T, span: TextSpan) -> Self {
        Spanned { content, span }
    }
}

impl IntoIterator for TextSpan {
    type Item = usize;
    type IntoIter = Range<usize>;
    fn into_iter(self) -> Self::IntoIter {
        self.range()
    }
}

impl IntoIterator for &TextSpan {
    type Item = usize;
    type IntoIter = Range<usize>;
    fn into_iter(self) -> Self::IntoIter {
        self.range()
    }
}
