//! UTF-8-safe change envelopes and scalar position indexes for incremental syntax.
//! No I/O. Callers must use offsets belonging to the corresponding source snapshots.
//! Contains AI-generated documentation additions.

// AI-authored incremental source mapping.
use super::TextSpan;
use std::ops::Range;

/// Work counts for the latest construction or update; no side effects.
/// Workspace aggregates these to explain reuse, not elapsed time.
/// [`SourceChange`] defines the replacement envelope; each layer chooses its unit:
/// lexer tokens, parser attempts/closed commands, or semantic/build commands.
/// Counts omit comparison, cloning and position-index rebuilding costs.
///
/// # Examples
/// ```
/// use cmake_analyzer::model::incremental::IncrementalWork;
/// let work = IncrementalWork { recomputed: 1, reused: 4 };
/// assert_eq!(work.reused, 4);
/// ```
#[derive(Clone, Debug, Default)]
pub struct IncrementalWork {
    /// Units freshly processed in this update. Example: `work.recomputed = 1`.
    pub recomputed: usize,
    /// Previously computed units retained this update. Example: `work.reused = 4`.
    pub reused: usize,
}

/// One replacement envelope between two source snapshots; no side effects.
/// Multiple separated edits may share this envelope, losing reuse between them.
/// Incremental layers use [`Self::range`] to reject overlapping units and
/// [`SourcePositions::remap`] to rebuild unchanged locations.
/// `between` gives matching starts and UTF-8-safe boundaries; public fields allow
/// unvalidated manual envelopes, so callers must preserve that invariant.
///
/// # Examples
/// ```
/// use cmake_analyzer::model::incremental::SourceChange;
/// let change = SourceChange::between("a中z", "a文z");
/// assert_eq!(change.old, 1..4);
/// assert_eq!(change.new, 1..4);
/// ```
#[derive(Clone, Debug)]
pub struct SourceChange {
    /// Replaced half-open byte range in the old snapshot. Example: `change.old == (1..4)`.
    pub old: Range<usize>,
    /// Replacement half-open byte range in the new snapshot. Example: `change.new == (1..4)`.
    pub new: Range<usize>,
}
impl SourceChange {
    /// Compare snapshots and isolate changed bytes at UTF-8 boundaries; no side effects.
    ///
    /// # Arguments
    /// - `old` `[in]`: previous source, unchanged.
    /// - `new` `[in]`: replacement source, unchanged.
    ///
    /// # Returns
    /// Envelope excluding a common prefix and nonoverlapping suffix. Equal sources yield empty ranges at EOF.
    ///
    /// # Algorithm
    /// Count equal prefix bytes, back up to scalar boundaries, then count the common
    /// suffix after that prefix and shorten it to scalar boundaries.
    ///
    /// # Flow
    /// ```text
    ///   [old bytes]    [new bytes]
    ///        \           /
    ///         [common prefix]
    ///                |
    ///     [back up to UTF-8 boundary]
    ///                |
    ///     [common suffix after prefix]
    ///                |
    ///     [trim to boundary; envelope]
    /// ```
    ///
    /// # Examples
    /// ```
    /// use cmake_analyzer::model::incremental::SourceChange;
    /// let change = SourceChange::between("a中z", "a文z");
    /// assert_eq!(change.old, 1..4);
    /// ```
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
    /// Map an unchanged old byte offset into the new snapshot; no side effects.
    ///
    /// # Arguments
    /// - `self` `[in]`: replacement envelope; unchanged.
    /// - `old` `[in]`: old byte offset, not checked against a source.
    ///
    /// # Returns
    /// `Some` unchanged prefix offset or shifted suffix offset; `None` inside replaced bytes or on arithmetic overflow. At an insertion boundary, maps after the insertion.
    ///
    /// # Examples
    /// ```
    /// use cmake_analyzer::model::incremental::SourceChange;
    /// let change = SourceChange::between("ab", "a中b");
    /// assert_eq!(change.offset(1), Some(4));
    /// ```
    pub fn offset(&self, old: usize) -> Option<usize> {
        if old < self.old.start {
            Some(old)
        } else if old >= self.old.end {
            old.checked_sub(self.old.end)?.checked_add(self.new.end)
        } else {
            None
        }
    }
    /// Map an entire unchanged byte range, rejecting overlap; no side effects.
    ///
    /// # Arguments
    /// - `self` `[in]`: replacement envelope; unchanged.
    /// - `range` `[in]`: old half-open range; caller supplies ordered valid bounds.
    ///
    /// # Returns
    /// `Some` unchanged prefix range or shifted suffix range; `None` for overlap or mapping overflow. An empty range at the replacement start is kept by the prefix branch.
    ///
    /// # Examples
    /// ```
    /// use cmake_analyzer::model::incremental::SourceChange;
    /// let change = SourceChange::between("ab", "a中b");
    /// assert_eq!(change.range(1..2), Some(4..5));
    /// ```
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

/// Line starts and sparse multibyte index for one source snapshot; no I/O.
/// Incremental layers use [`Self::span`] and [`Self::remap`] to refresh scalar
/// locations rather than adding byte deltas to columns. This is not the LSP mapper.
/// The source is not retained; callers must supply valid boundaries from that source.
///
/// # Flow
/// ```text
///  [source chars]             [old span + change]
///        |                           |
///        v                           v
///  [LF line starts]          [unchanged byte range?]
///  [wide-char excess]                | yes
///        +---------------+-----------+
///                        v
///               [byte range -> scalar span]
/// ```
///
/// # Examples
/// ```
/// use cmake_analyzer::model::incremental::SourcePositions;
/// let positions = SourcePositions::new("中x");
/// assert_eq!(positions.span(3..4).column, 1);
/// ```
pub struct SourcePositions {
    /// Ascending line-start byte offsets, beginning with zero; only LF splits lines.
    /// Example inside this module: `assert_eq!(SourcePositions::new("a\nb").lines, vec![0, 2])`.
    lines: Vec<usize>,
    /// Each pair stores a multibyte scalar start and cumulative excess UTF-8 bytes
    /// over scalar count after that character; ASCII needs no entry.
    /// Example inside this module: `assert_eq!(SourcePositions::new("中").wide, vec![(0, 2)])`.
    wide: Vec<(usize, usize)>,
}
impl SourcePositions {
    /// Scan one source into line and multibyte indexes; no side effects.
    ///
    /// # Arguments
    /// - `source` `[in]`: snapshot whose offsets future queries must use; not retained.
    ///
    /// # Returns
    /// New index; ASCII-only sources have no wide-character entries.
    ///
    /// # Algorithm
    /// Walk scalar starts once. Save byte offsets just after LF and, only for
    /// multibyte scalars, save the accumulated extra bytes over scalar count.
    ///
    /// # Flow
    /// ```text
    ///              [next scalar] <----------------+
    ///                    |                        |
    ///                  <LF?>                      |
    ///                 /    \
    ///               yes    no                     |
    ///                |      |                     |
    ///     [append line start]                     |
    ///                +------+                     |
    ///                    |                        |
    ///              <multibyte?>                   |
    ///               /        \
    ///             yes        no                   |
    ///              |          |                   |
    ///   [append cumulative excess]                |
    ///              +----------+-------------------+
    ///                    | EOF
    ///               [return index]
    /// ```
    ///
    /// # Examples
    /// ```
    /// use cmake_analyzer::model::incremental::SourcePositions;
    /// let positions = SourcePositions::new("中\nx");
    /// assert_eq!(positions.span(4..5).row, 1);
    /// ```
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
    /// Convert a valid byte boundary to the global scalar offset; no side effects.
    ///
    /// # Arguments
    /// - `self` `[in]`: index for this snapshot; unchanged.
    /// - `byte` `[in]`: UTF-8 boundary within the indexed source; not validated.
    ///
    /// # Returns
    /// Number of Unicode scalar values before the byte.
    ///
    /// # Panics
    /// Invalid offsets inside multibyte characters can underflow; only query valid boundaries.
    ///
    /// # Examples
    /// ```text
    /// let positions = SourcePositions::new("中x");
    /// assert_eq!(positions.character_offset(3), 1);
    /// ```
    fn character_offset(&self, byte: usize) -> usize {
        let index = self.wide.partition_point(|(offset, _)| *offset < byte);
        byte - index.checked_sub(1).map_or(0, |i| self.wide[i].1)
    }
    /// Build a span with a recalculated start row and scalar column; no side effects.
    ///
    /// # Arguments
    /// - `self` `[in]`: index for this snapshot; unchanged.
    /// - `range` `[in]`: ordered in-bounds UTF-8 range in the indexed source; not validated.
    ///
    /// # Returns
    /// Span retaining byte bounds and locating the start. End coordinates are not stored.
    ///
    /// # Algorithm
    /// Binary-search the last line start at or before the range start. Subtract
    /// cumulative multibyte excess at the start and line start to obtain the
    /// column in scalars; retain byte bounds without inspecting the end.
    ///
    /// # Flow
    /// ```text
    ///  [range start]           [line starts]
    ///         +---------------------+
    ///                     |
    ///              [find row/line start]
    ///                 /          \
    ///    [scalar offset at byte] [scalar offset at line]
    ///                 \          /
    ///                  [subtract -> column]
    ///                         |
    ///            [span with original byte bounds]
    /// ```
    ///
    /// # Panics
    /// Invalid UTF-8 interior offsets can underflow position arithmetic. Out-of-source
    /// ranges are not reliably rejected; callers must validate them before querying.
    ///
    /// # Examples
    /// ```
    /// use cmake_analyzer::model::incremental::SourcePositions;
    /// let positions = SourcePositions::new("中x");
    /// assert_eq!(positions.span(3..4).column, 1);
    /// ```
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
    /// Relocate an unchanged old span and rebuild its start coordinates; no side effects.
    ///
    /// # Arguments
    /// - `self` `[in]`: index for the new source; unchanged.
    /// - `change` `[in]`: envelope connecting the old and new snapshots.
    /// - `span` `[in]`: valid span from the old source.
    ///
    /// # Returns
    /// `Some` span in the new snapshot for an unchanged range; `None` on overlap or mapping overflow.
    ///
    /// # Panics
    /// Inconsistent manually constructed changes can produce invalid offsets with the
    /// same arithmetic hazards as [`Self::span`]. Use matching source snapshots.
    ///
    /// # Examples
    /// ```
    /// use cmake_analyzer::model::incremental::{SourceChange, SourcePositions};
    /// let change = SourceChange::between("ab", "a中b");
    /// let old = SourcePositions::new("ab").span(1..2);
    /// let mapped = SourcePositions::new("a中b").remap(&change, old).unwrap();
    /// assert_eq!(mapped.range(), 4..5);
    /// ```
    pub fn remap(&self, change: &SourceChange, span: TextSpan) -> Option<TextSpan> {
        change
            .range(span.start_byte..span.end_byte)
            .map(|range| self.span(range))
    }
}
