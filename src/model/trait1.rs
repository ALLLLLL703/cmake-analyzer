//! Cursor and source-buffer contracts; no I/O is required.
//! The lexer cursor implements [`ICursor`]; [`IBuffer`] is currently an unused abstraction.
//!
//! Contains AI-generated documentation additions.

use crate::model::TextSpan;

/// Source buffer inspection contract; implementations choose storage and cursor state.
/// No implementation is provided here. The lexer offers corresponding inherent methods
/// on [`crate::parser::lexer::model::RawBuffer`]; it does not implement this trait.
/// [`ICursor`] supplies the associated point-location contract.
///
/// # Examples
/// ```
/// # use cmake_analyzer::{model::{TextSpan, trait1::IBuffer}, parser::lexer::model::RawBuffer};
/// # struct View<'a>(RawBuffer<'a>);
/// # impl IBuffer for View<'_> {
/// #     fn current_byte_to_span(&self) -> TextSpan { self.0.current_byte_to_span() }
/// #     fn buffer_advance_length(&self) -> usize { self.0.buffer_advance_length() }
/// #     fn span_to_text(&self, span: TextSpan) -> Option<&str> { self.0.span_to_text(span) }
/// # }
/// let buffer = View(RawBuffer::new("中"));
/// let point = IBuffer::current_byte_to_span(&buffer);
/// assert_eq!(IBuffer::span_to_text(&buffer, point), Some(""));
/// ```
pub trait IBuffer {
    /// Describe the current byte as a point span without advancing the buffer. No side effects.
    ///
    /// # Arguments
    /// - `self` `[in]`: source buffer to inspect; unchanged.
    ///
    /// # Returns
    /// Zero-width span at the cursor.
    ///
    /// # Examples
    /// ```
    /// # use cmake_analyzer::{model::{TextSpan, trait1::IBuffer}, parser::lexer::model::RawBuffer};
    /// # struct View<'a>(RawBuffer<'a>);
    /// # impl IBuffer for View<'_> {
    /// #     fn current_byte_to_span(&self) -> TextSpan { self.0.current_byte_to_span() }
    /// #     fn buffer_advance_length(&self) -> usize { self.0.buffer_advance_length() }
    /// #     fn span_to_text(&self, span: TextSpan) -> Option<&str> { self.0.span_to_text(span) }
    /// # }
    /// let buffer = View(RawBuffer::new("中"));
    /// let point = IBuffer::current_byte_to_span(&buffer);
    /// assert_eq!(point.range(), 0..0);
    /// ```
    fn current_byte_to_span(&self) -> TextSpan;
    /// Report the buffer length without changing it. No side effects.
    ///
    /// # Arguments
    /// - `self` `[in]`: source buffer to inspect; unchanged.
    ///
    /// # Returns
    /// Buffer extent in bytes, not remaining characters.
    ///
    /// # Examples
    /// ```
    /// # use cmake_analyzer::{model::{TextSpan, trait1::IBuffer}, parser::lexer::model::RawBuffer};
    /// # struct View<'a>(RawBuffer<'a>);
    /// # impl IBuffer for View<'_> {
    /// #     fn current_byte_to_span(&self) -> TextSpan { self.0.current_byte_to_span() }
    /// #     fn buffer_advance_length(&self) -> usize { self.0.buffer_advance_length() }
    /// #     fn span_to_text(&self, span: TextSpan) -> Option<&str> { self.0.span_to_text(span) }
    /// # }
    /// let buffer = View(RawBuffer::new("中"));
    /// assert_eq!(IBuffer::buffer_advance_length(&buffer), 3);
    /// ```
    fn buffer_advance_length(&self) -> usize;
    /// Borrow a source slice without moving the cursor. No side effects.
    ///
    /// # Arguments
    /// - `self` `[in]`: source buffer to borrow from; unchanged.
    /// - `span` `[in]`: requested half-open byte range.
    ///
    /// # Returns
    /// `Some` source slice if the span is valid; `None` otherwise.
    ///
    /// # Examples
    /// ```
    /// # use cmake_analyzer::{model::{TextSpan, trait1::IBuffer}, parser::lexer::model::RawBuffer};
    /// # struct View<'a>(RawBuffer<'a>);
    /// # impl IBuffer for View<'_> {
    /// #     fn current_byte_to_span(&self) -> TextSpan { self.0.current_byte_to_span() }
    /// #     fn buffer_advance_length(&self) -> usize { self.0.buffer_advance_length() }
    /// #     fn span_to_text(&self, span: TextSpan) -> Option<&str> { self.0.span_to_text(span) }
    /// # }
    /// let buffer = View(RawBuffer::new("中"));
    /// let span = TextSpan { end_byte: 3, ..Default::default() };
    /// assert_eq!(IBuffer::span_to_text(&buffer, span), Some("中"));
    /// ```
    fn span_to_text(&self, span: TextSpan) -> Option<&str>;
}

/// Convert a cursor into a point span; conversion does not advance it.
/// Implemented by [`crate::parser::lexer::model::RawBufferCursor`] for lexer setup
/// and diagnostics. [`TextSpan`] keeps bytes distinct from scalar columns.
///
/// # Examples
/// ```
/// use cmake_analyzer::{model::trait1::ICursor, parser::lexer::model::RawBufferCursor};
/// let cursor = RawBufferCursor::new_with_len(3);
/// assert_eq!(cursor.to_span("中").start_byte, 0);
/// ```
pub trait ICursor {
    /// Locate the cursor in the supplied source without advancing it. No side effects.
    ///
    /// # Arguments
    /// - `self` `[in]`: cursor to locate; unchanged.
    /// - `content` `[in]`: source corresponding to the cursor byte offset.
    ///
    /// # Returns
    /// Point span whose row/column describe the cursor start.
    ///
    /// # Examples
    /// ```
    /// use cmake_analyzer::{model::trait1::ICursor, parser::lexer::model::RawBufferCursor};
    /// let cursor = RawBufferCursor::new_with_len(3);
    /// let point = ICursor::to_span(&cursor, "中");
    /// assert_eq!(point.range(), 0..0);
    /// ```
    fn to_span(&self, content: &str) -> TextSpan;
}
