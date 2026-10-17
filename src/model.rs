//! Source-backed values shared by syntax and analysis; no I/O.
//! Byte ranges slice UTF-8; rows and columns locate the start using LF lines and
//! Unicode scalar counts (one Rust `char` per scalar, not a user-perceived character).
//! For `🦀x`, `x` starts at byte 4, scalar column 1 and UTF-16 column 2. Tabs
//! count as one scalar; only LF starts a new row here. LSP coordinates require
//! the server encoding mapper and its negotiated UTF-8/UTF-16/UTF-32 encoding.
//! [`TextSpan::text`] borrows raw text; [`Spanned`] attaches meaning without copying it.
//!
//! Contains AI-generated documentation additions.

mod span;
// AI-authored incremental source mapping.
pub mod incremental;
pub mod trait1;

/// Half-open UTF-8 byte range with a zero-based start location; no side effects.
/// Columns count Unicode scalar values, not grapheme clusters or LSP UTF-16 units.
/// Lexer, parser and semantic records use this to share source locations.
/// [`Self::text`] validates slicing; [`incremental::SourcePositions`] rebuilds coordinates.
/// Fields are not validated at construction. A span belongs to one source snapshot.
///
/// # Examples
/// ```
/// use cmake_analyzer::model::TextSpan;
/// let span = TextSpan { start_byte: 0, end_byte: 3, ..Default::default() };
/// assert_eq!(span.text("中"), Some("中"));
/// ```
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub struct TextSpan {
    /// Start column in Unicode scalar values; tabs count as one. Example: `span.column = 2`.
    pub column: u64,
    /// Start line, counting LF characters from zero. Example: `span.row = 1`.
    pub row: u64,
    /// Inclusive UTF-8 byte offset in the owning source. Example: `span.start_byte = 0`.
    pub start_byte: usize,
    /// Exclusive byte offset; may equal the start for a cursor. Example: `span.end_byte = 3`.
    pub end_byte: usize,
}

/// Syntax or analysis value paired with its source range; no side effects.
/// Lexer and parser buffers store these instead of copied source strings.
/// [`TextSpan::text`] reads the raw range; [`Self::new`] associates any payload.
///
/// # Examples
/// ```
/// use cmake_analyzer::model::{Spanned, TextSpan};
/// let located = Spanned::new("name", TextSpan::default());
/// assert_eq!(located.content, "name");
/// ```
#[derive(Debug, Clone)]
pub struct Spanned<T> {
    /// Meaning associated with the range, not necessarily the source text. Example: `located.content = "other"`.
    pub content: T,
    /// Range in the owning snapshot. Example: `located.span.end_byte = 4`.
    pub span: TextSpan,
}
