//! Range operations and source views for [`TextSpan`]; no I/O.
//! Iteration yields bytes, including interior UTF-8 offsets; text iteration yields scalars.
//!
//! Contains AI-generated documentation additions.

use std::{fmt::format, ops::Range, str::Chars};

use crate::model::Spanned;

use super::TextSpan;

impl TextSpan {
    /// Copy the half-open byte bounds without validation or side effects.
    ///
    /// # Arguments
    /// - `self` `[in]`: range to inspect; unchanged.
    ///
    /// # Returns
    /// Owned byte range, even if reversed or outside a source.
    ///
    /// # Examples
    /// ```
    /// # use cmake_analyzer::model::TextSpan;
    /// let span = TextSpan { start_byte: 0, end_byte: 3, ..Default::default() };
    /// assert_eq!(span.range(), 0..3);
    /// ```
    pub fn range(&self) -> Range<usize> {
        self.start_byte..self.end_byte
    }

    /// Iterate byte offsets, including positions inside UTF-8 characters; no side effects.
    ///
    /// # Arguments
    /// - `self` `[in]`: range to inspect; unchanged.
    ///
    /// # Returns
    /// Range iterator yielding each offset before the exclusive end.
    ///
    /// # Examples
    /// ```
    /// # use cmake_analyzer::model::TextSpan;
    /// let span = TextSpan { start_byte: 0, end_byte: 3, ..Default::default() };
    /// assert_eq!(span.iter().collect::<Vec<_>>(), vec![0, 1, 2]);
    /// ```
    pub fn iter(&self) -> Range<usize> {
        self.range()
    }

    /// Borrow the raw source range without copying or side effects.
    ///
    /// # Arguments
    /// - `self` `[in]`: range to inspect; unchanged.
    /// - `source` `[in]`: UTF-8 source snapshot to borrow from; unchanged.
    ///
    /// # Returns
    /// `Some` slice for ordered in-bounds UTF-8 boundaries; `None` otherwise.
    ///
    /// # Examples
    /// ```
    /// # use cmake_analyzer::model::TextSpan;
    /// let span = TextSpan { start_byte: 0, end_byte: 3, ..Default::default() };
    /// assert_eq!(span.text("中"), Some("中"));
    /// ```
    pub fn text<'a>(&self, source: &'a str) -> Option<&'a str> {
        source.get(self.range())
    }

    /// Borrow an iterator of Unicode scalar values, not grapheme clusters; no side effects.
    ///
    /// # Arguments
    /// - `self` `[in]`: range to inspect; unchanged.
    /// - `source` `[in]`: UTF-8 source snapshot to borrow from; unchanged.
    ///
    /// # Returns
    /// `Some` character iterator for a valid slice; `None` for invalid byte bounds.
    ///
    /// # Examples
    /// ```
    /// # use cmake_analyzer::model::TextSpan;
    /// let span = TextSpan { start_byte: 0, end_byte: 3, ..Default::default() };
    /// assert_eq!(span.iter_text("中").unwrap().collect::<String>(), "中");
    /// ```
    pub fn iter_text<'a>(&self, source: &'a str) -> Option<Chars<'a>> {
        self.text(source).map(str::chars)
    }

    /// Build an envelope covering both spans and the gap between them; no side effects.
    ///
    /// # Arguments
    /// - `span1` `[in]`: first range, from the same snapshot as span2.
    /// - `span2` `[in]`: second range; its location wins if starts are equal.
    ///
    /// # Returns
    /// Range with the smaller start, larger end and chosen start coordinates; inputs are not validated.
    ///
    /// # Examples
    /// ```
    /// # use cmake_analyzer::model::TextSpan;
    /// let span = TextSpan { start_byte: 0, end_byte: 3, ..Default::default() };
    /// let other = TextSpan { start_byte: 5, end_byte: 6, column: 3, ..Default::default() };
    /// assert_eq!(TextSpan::combine_with_middle(span, other).range(), 0..6);
    /// ```
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

    /// Overwrite this span with the envelope of both ranges, including their gap; no I/O.
    ///
    /// # Arguments
    /// - `self` `[in,out]`: range overwritten with the envelope and earliest-start coordinates.
    /// - `span2` `[in]`: other range in the same snapshot; coordinates win on equal starts.
    ///
    /// # Returns
    /// `()`; only this span is changed.
    ///
    /// # Examples
    /// ```
    /// # use cmake_analyzer::model::TextSpan;
    /// let mut span = TextSpan { start_byte: 0, end_byte: 3, ..Default::default() };
    /// span.self_combine_with_middle(TextSpan { end_byte: 6, ..Default::default() });
    /// assert_eq!(span.range(), 0..6);
    /// ```
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

    /// Allocate a short location label for inspection output; no external side effects.
    ///
    /// # Arguments
    /// - `self` `[in]`: range to inspect; unchanged.
    ///
    /// # Returns
    /// String containing column, row and exclusive end byte, but not the start byte.
    ///
    /// # Examples
    /// ```
    /// # use cmake_analyzer::model::TextSpan;
    /// let span = TextSpan { start_byte: 0, end_byte: 3, ..Default::default() };
    /// assert!(span.loc_info().contains("end_byte: 3"));
    /// ```
    pub fn loc_info(&self) -> String {
        format!(
            "{{ col: {} row: {} end_byte: {} }}",
            self.column, self.row, self.end_byte
        )
    }
}

impl<T> Spanned<T> {
    /// Associate a value with a range without validating it; no side effects.
    ///
    /// # Arguments
    /// - `content` `[in]`: payload moved into the result.
    /// - `span` `[in]`: location copied into the result.
    ///
    /// # Returns
    /// New located value owning the payload.
    ///
    /// # Examples
    /// ```
    /// use cmake_analyzer::model::{Spanned, TextSpan};
    /// let value = Spanned::new(7, TextSpan::default());
    /// assert_eq!(value.content, 7);
    /// ```
    pub fn new(content: T, span: TextSpan) -> Self {
        Spanned { content, span }
    }
}

impl IntoIterator for TextSpan {
    /// Yielded byte offset, not a character. Example: `let offset: <TextSpan as IntoIterator>::Item = 0`.
    type Item = usize;
    /// Half-open byte range iterator. Example: `let iter: <TextSpan as IntoIterator>::IntoIter = 0..3`.
    type IntoIter = Range<usize>;
    /// Consume the span and iterate byte offsets; no side effects.
    ///
    /// # Arguments
    /// - `self` `[in]`: range whose offsets are yielded; no source access.
    ///
    /// # Returns
    /// Half-open byte iterator, not a character iterator.
    ///
    /// # Examples
    /// ```
    /// # use cmake_analyzer::model::TextSpan;
    /// let span = TextSpan { start_byte: 0, end_byte: 3, ..Default::default() };
    /// assert_eq!(span.into_iter().count(), 3);
    /// ```
    fn into_iter(self) -> Self::IntoIter {
        self.range()
    }
}

impl IntoIterator for &TextSpan {
    /// Yielded byte offset while the span stays borrowed. Example: `let offset: <&TextSpan as IntoIterator>::Item = 0`.
    type Item = usize;
    /// Owned range iterator independent of the borrow. Example: `let iter: <&TextSpan as IntoIterator>::IntoIter = 0..3`.
    type IntoIter = Range<usize>;
    /// Borrow the span and iterate byte offsets; no side effects.
    ///
    /// # Arguments
    /// - `self` `[in]`: range whose offsets are yielded; no source access.
    ///
    /// # Returns
    /// Half-open byte iterator, not a character iterator.
    ///
    /// # Examples
    /// ```
    /// # use cmake_analyzer::model::TextSpan;
    /// let span = TextSpan { start_byte: 0, end_byte: 3, ..Default::default() };
    /// assert_eq!((&span).into_iter().count(), 3);
    /// ```
    fn into_iter(self) -> Self::IntoIter {
        self.range()
    }
}
