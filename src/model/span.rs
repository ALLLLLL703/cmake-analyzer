use std::{ops::Range, str::Chars};

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

    pub fn combine_with_middle(span1: Self, span2: Self) -> Self {}
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
