//! Copy comments and normalize intervening whitespace in memory. Output is appended
//! to the caller-owned writer; no filesystem/process effects or syntax-tree changes.
//!
//! Contains AI-generated documentation additions.
use super::model::{FormatError, FormatWriter};
use crate::parser::lexer::model::LexedBuffer;
use std::ops::Range;

/// Append comments and surrounding whitespace for one source gap. Preserves raw comment bodies
/// and blank-line counts, normalizing inserted line endings. No I/O.
///
/// # Arguments
/// - `source` `[in]`: matching UTF-8 source.
/// - `lexed` `[in]`: ordered comment spans for this source.
/// - `range` `[in]`: half-open byte gap containing only comments/whitespace.
/// - `level` `[in]`: indentation for standalone comments.
/// - `out` `[in,out]`: receives appended comments/normalized breaks and bookkeeping updates, without clearing earlier text.
/// - `stop` `[in]`: cancellation predicate checked per comment.
///
/// # Returns
/// `Ok(())` after the gap is emitted; `Err` can leave partial appended text for the outer renderer to discard.
///
/// # Errors
/// InvalidSyntax for overlapping/out-of-gap comments or non-whitespace gaps; Cancelled for a stop request.
///
/// # Panics
/// Source gaps and comment offsets must be valid UTF-8 boundaries within source.
///
/// # Algorithm
/// Binary-search the first relevant comment, validate each span, render whitespace and attach
/// comments either at current line or block indentation. Strip the line-comment CR only when
/// followed by LF, then emit trailing whitespace.
///
/// # Flow
/// ```text
///  [gap + ordered comments]
///             |
///     [first relevant comment] <-------+
///             |                       |
///        [validate span] --bad--> [Err]|
///             |                       |
///  [whitespace + attached comment] ---+
///             | done
///      [trailing whitespace]
/// ```
///
/// # Examples
/// ```rust
/// render(source, &lexed, 0..command.full_span.start_byte, 0, &mut out, &|| false)?;
/// ```
pub(super) fn render(
    source: &str,
    lexed: &LexedBuffer,
    range: Range<usize>,
    level: usize,
    out: &mut FormatWriter<'_>,
    stop: &impl Fn() -> bool,
) -> Result<(), FormatError> {
    let mut cursor = range.start;
    let first = lexed
        .comments
        .partition_point(|comment| comment.end_byte <= cursor);
    for comment in lexed.comments[first..]
        .iter()
        .take_while(|comment| comment.start_byte < range.end)
    {
        if stop() {
            return Err(FormatError::Cancelled);
        }
        if comment.start_byte < cursor || comment.end_byte > range.end {
            return Err(FormatError::InvalidSyntax);
        }
        whitespace(&source[cursor..comment.start_byte], out)?;
        if out.column == 0 {
            out.indent(level);
        } else {
            out.push(" ");
        }
        let mut raw = &source[comment.start_byte..comment.end_byte];
        // The lexer includes CR in line comments, but not the following LF.
        if source.as_bytes().get(comment.end_byte) == Some(&b'\n') {
            raw = raw.strip_suffix('\r').unwrap_or(raw);
        }
        out.push(raw);
        cursor = comment.end_byte;
    }
    whitespace(&source[cursor..range.end], out)
}
/// Translate whitespace-only gaps to at least their existing LF count in the output.
/// No I/O; does not copy horizontal whitespace.
///
/// # Arguments
/// - `raw` `[in]`: source gap expected to contain only Unicode whitespace.
/// - `out` `[in,out]`: appends missing line breaks and resets column when breaks are present; never clears text.
///
/// # Returns
/// `Ok(())` for whitespace-only input, `Err(InvalidSyntax)` otherwise; invalid input does not change out.
///
/// # Errors
/// InvalidSyntax for any non-whitespace character.
///
/// # Examples
/// ```text
/// whitespace("\n\n  ", &mut out)?;
/// ```
fn whitespace(raw: &str, out: &mut FormatWriter<'_>) -> Result<(), FormatError> {
    if !raw.chars().all(char::is_whitespace) {
        return Err(FormatError::InvalidSyntax);
    }
    let count = raw.bytes().filter(|byte| *byte == b'\n').count();
    if count > 0 {
        out.breaks(count);
    }
    Ok(())
}
