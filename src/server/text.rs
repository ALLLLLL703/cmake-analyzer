//! Translate UTF-8 byte spans and negotiated protocol coordinates.
//! A line-start table and sparse multibyte table avoid rescanning whole lines for
//! every conversion. Line terminators are excluded from incoming edit columns.
//! Sequential changes address the text produced by earlier changes, not the original
//! snapshot. These operations are memory-only and do not mutate borrowed source.
// AI-authored LSP byte/coordinate mapping.
use lspally::server::model::*;
use std::ops::Range as ByteRange;

/// Borrowed source with sparse indexes for protocol coordinate conversion.
/// Diagnostics, symbols, navigation, formatting and editor changes share this mapper.
/// [`Self::byte`] validates incoming positions; [`Self::position`] validates outgoing
/// byte endpoints. [`apply_changes`] rebuilds an index after each ranged edit.
/// Construction allocates tables but performs no I/O and never changes source text.
///
/// # Flow
/// ```text
///  [Borrowed UTF-8 source]        [CoordinateEncoding]
///            |                            |
///            +---------------+------------+
///                            v
///                        [TextIndex]
///                       /           \
///                      v             v
///                 [Line starts] [Multibyte excess]
///                      |             |
///                      +------+------+
///                             v
///                  [Position <-> byte mapping]
/// ```
/// # Examples
/// ```text
/// let index = TextIndex::new("éx", CoordinateEncoding::Utf16);
/// assert_eq!(index.position(2)?.character, 1);
/// ```
pub(super) struct TextIndex<'a> {
    /// Exact analyzed/editor text borrowed for bounds and character-boundary validation.
    /// Example: `assert_eq!(index.source, "éx")` after constructing from that text.
    source: &'a str,
    /// Byte offset of every line start, beginning with zero; LF and lone CR create lines.
    /// Example: `TextIndex::new("a\nb", encoding).lines[1] == 2`.
    lines: Vec<usize>,
    // Multibyte entries: byte offset, encoded units before the character, cumulative byte excess.
    /// One entry per multibyte UTF-8 character: its byte offset, global encoded units
    /// before it, and cumulative UTF-8 bytes minus encoded units through it. These are
    /// index metadata, not protocol positions; UTF-8 encoding keeps the excess zero.
    /// Example: for `"é"` with UTF-16, `index.wide[0] == (0, 0, 1)`.
    /// Tuple example: `let (byte, units_before, excess_after) = index.wide[0]` reads all three values.
    wide: Vec<(usize, usize, usize)>,
}
impl<'a> TextIndex<'a> {
    /// Scan source once into line starts and multibyte encoding adjustments. No I/O or source mutation.
    ///
    /// # Arguments
    /// - `source` `[in]`: Borrowed text retained for the index lifetime.
    /// - `encoding` `[in]`: UTF-8 bytes, UTF-16 code units or UTF-32 scalar units used for columns.
    ///
    /// # Returns
    /// An index borrowing the original source and owning sparse conversion tables.
    ///
    /// # Algorithm
    /// Record LF and lone-CR starts, treating CRLF as one break. For every multibyte character record global units before it and cumulative byte excess after it. This enables binary-search conversions without scanning line contents.
    ///
    /// # Flow
    /// ```text
    ///  [Source characters]        [Selected encoding]
    ///           |                         |
    ///           +-----------+-------------+
    ///                       v
    ///                 <Line break?> ---- yes ---> [Append line start]
    ///                       |
    ///                 <Multibyte?> ----- yes ---> [Append offset/units/excess]
    ///                       |
    ///                  [Next character]
    ///                       |
    ///                  [Return index]
    /// ```
    ///
    /// # Examples
    /// ```text
    /// let index = TextIndex::new("é\n🌟", CoordinateEncoding::Utf16);
    /// assert_eq!(index.position(3)?.line, 1);
    /// ```
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
    /// Subtract cumulative byte excess before a known byte boundary. No side effects.
    ///
    /// # Arguments
    /// - `self` `[in]`: Sparse index for the same source and encoding.
    /// - `byte` `[in]`: Valid character-boundary offset expected from internal callers; not validated here.
    ///
    /// # Returns
    /// Global encoded-unit offset before byte; not a line-relative column.
    ///
    /// # Panics
    /// Arbitrary invalid offsets may underflow the unchecked subtraction; callers must supply valid source boundaries.
    ///
    /// # Examples
    /// ```text
    /// let index = TextIndex::new("éx", CoordinateEncoding::Utf16);
    /// assert_eq!(index.units_before(2), 1);
    /// ```
    fn units_before(&self, byte: usize) -> usize {
        let count = self.wide.partition_point(|entry| entry.0 < byte);
        byte - count.checked_sub(1).map_or(0, |i| self.wide[i].2)
    }
    /// Validate a protocol position and convert it to a UTF-8 byte boundary. No I/O or mutation.
    ///
    /// # Arguments
    /// - `self` `[in]`: Index of the exact current text.
    /// - `position` `[in]`: Zero-based line and encoded-unit column; line terminators are not valid edit columns.
    ///
    /// # Returns
    /// `Ok(byte)` for a position within line content or at its end; `Err` for missing lines, out-of-line columns, arithmetic overflow or split encoded characters.
    ///
    /// # Errors
    /// Invalid-parameters errors for line/column outside text or a byte that is not a UTF-8 boundary, including UTF-16 positions splitting a surrogate pair.
    ///
    /// # Algorithm
    /// Find the line bounds, trim its CR/LF tail, add the column to the global encoded start, then binary-search cumulative byte excess. Require the result lies in that line and on a UTF-8 character boundary.
    ///
    /// # Flow
    /// ```text
    ///  [Line + encoded column]
    ///               |
    ///        [Find content bounds]
    ///               |
    ///  [Global units + sparse byte excess]
    ///               |
    ///       <In line + boundary?>
    ///          /             \
    ///         no             yes
    ///         |               |
    ///      [Error]        [Byte offset]
    /// ```
    ///
    /// # Examples
    /// ```text
    /// let index = TextIndex::new("🌟x", CoordinateEncoding::Utf16);
    /// assert_eq!(index.byte(&Position { line: 0, character: 2 })?, 4);
    /// ```
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
    /// Convert a valid byte boundary into a zero-based encoded protocol position. No side effects.
    ///
    /// # Arguments
    /// - `self` `[in]`: Index for the source containing the byte offset.
    /// - `byte` `[in]`: Offset up to source length, on a UTF-8 boundary; newline bytes may be represented too.
    ///
    /// # Returns
    /// `Ok(Position)` with line and encoded column; `Err` for invalid boundaries/out-of-bounds or u32 coordinate overflow.
    ///
    /// # Errors
    /// Internal errors for invalid analysis byte ranges, too many lines or a column too large for protocol u32 fields.
    ///
    /// # Examples
    /// ```text
    /// let index = TextIndex::new("éx", CoordinateEncoding::Utf16);
    /// assert_eq!(index.position(2)?.character, 1);
    /// ```
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
    /// Convert both byte endpoints to protocol positions without separately checking range order. No side effects.
    ///
    /// # Arguments
    /// - `self` `[in]`: Index for the span's source text.
    /// - `bytes` `[in]`: Consumed half-open byte range; callers supply ordered endpoints.
    ///
    /// # Returns
    /// `Ok(Range)` with converted start and end; `Err` if either endpoint fails position conversion.
    ///
    /// # Errors
    /// Invalid byte endpoints or coordinate overflow; reversed valid endpoints are not rejected here.
    ///
    /// # Examples
    /// ```text
    /// let range = index.range(0..source.len())?;
    /// ```
    pub fn range(&self, bytes: ByteRange<usize>) -> ServerResult<Range> {
        Ok(Range {
            start: self.position(bytes.start)?,
            end: self.position(bytes.end)?,
        })
    }
    /// Validate an incoming edit range and convert it to ordered byte endpoints. No side effects.
    ///
    /// # Arguments
    /// - `self` `[in]`: Index of the text to which this edit applies.
    /// - `range` `[in]`: Borrowed protocol range with zero-based encoded coordinates.
    ///
    /// # Returns
    /// `Ok(start..end)` including empty insertions; `Err` for invalid endpoints or start after end.
    ///
    /// # Errors
    /// All byte-position validation errors plus an invalid-parameters error for reversed ranges.
    ///
    /// # Examples
    /// ```text
    /// let bytes = index.edit_range(&Range {
    ///     start: Position { line: 0, character: 0 },
    ///     end: Position { line: 0, character: 1 },
    /// })?;
    /// ```
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
/// Apply a sequence of editor changes to owned working text and return the final contents.
/// No I/O and no mutation of borrowed source; partial edits are not exposed when a later change fails.
///
/// # Arguments
/// - `source` `[in]`: Borrowed starting editor text, cloned into the working output.
/// - `changes` `[in]`: Consumed changes in protocol order; a missing range replaces all text, range_length is ignored.
/// - `encoding` `[in]`: Coordinate units of every ranged change.
///
/// # Returns
/// `Ok(String)` with all changes applied, or a copy of source for no changes; `Err` if any ranged change is invalid.
///
/// # Errors
/// Invalid line/column, split encoded character, overflow or reversed range. Borrowed source and caller-owned editor state remain unchanged.
///
/// # Algorithm
/// For each change use the current working text, rebuilding the coordinate index for ranged edits; replace the selected bytes or overwrite the entire working string.
///
/// # Flow
/// ```text
///  [Original text] ---> [Owned working text]
///                               |
///                        [Next change]
///                               |
///                          <Has range?>
///                           /        \
///                         yes        no
///                          |          |
///                  [Index + validate] [Overwrite all]
///                          |          |
///                   [Replace bytes]   |
///                          +----+-----+
///                               v
///                      [Next / return final]
/// ```
///
/// # Examples
/// ```text
/// let changes = vec![TextDocumentContentChangeEvent {
///     range: None, range_length: None, text: "project(demo)".into(),
/// }];
/// assert_eq!(apply_changes("old", changes, CoordinateEncoding::Utf16)?, "project(demo)");
/// ```
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
