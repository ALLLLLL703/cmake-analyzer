//! Half-open byte-span iteration and borrowed Unicode text extraction, including invalid boundaries
//! and empty end-of-file spans. No external side effects.

// Contains AI-generated documentation edits.
// AI-generated tests.

use cmake_analyzer::model::TextSpan;

/// Construct a byte-range fixture with default zero row/column coordinates. Does not validate
/// bounds or UTF-8 boundaries and has no side effects.
///
/// # Arguments
/// - `start_byte` `[in]`: Inclusive byte offset, stored unchanged.
/// - `end_byte` `[in]`: Exclusive byte offset, stored unchanged even for reversed ranges.
///
/// # Returns
/// A TextSpan suitable for testing slicing or iteration; positions other than endpoints keep
/// defaults.
///
/// # Examples
/// Local test-module context:
/// ```text
/// let range = span(1, 4);
/// assert_eq!(range.text("x中"), Some("中"));
/// ```
fn span(start_byte: usize, end_byte: usize) -> TextSpan {
    TextSpan {
        start_byte,
        end_byte,
        ..TextSpan::default()
    }
}

/// Construct byte span 2..5 and compare direct, owned and reversed borrowed iteration with exact
/// offsets and length.
///
/// Only local values and analysis buffers are created or changed; no file, network or process side
/// effects.
///
/// # Arguments
/// None. The test builds its own fixture and expected results.
///
/// # Returns
/// `()` after all checks pass; no data or output parameter is returned.
///
/// # Panics
/// If a stated invariant fails.
///
/// # Examples
/// Local test-module call:
/// ```text
/// spans_iterate_half_open_byte_offsets_by_value_and_reference();
/// ```
#[test]
fn spans_iterate_half_open_byte_offsets_by_value_and_reference() {
    let span = span(2, 5);
    assert_eq!(span.iter().collect::<Vec<_>>(), [2, 3, 4]);
    assert_eq!(span.into_iter().collect::<Vec<_>>(), [2, 3, 4]);
    assert_eq!((&span).into_iter().rev().collect::<Vec<_>>(), [4, 3, 2]);
    assert_eq!(span.iter().len(), 3);
}

/// Slice the Chinese/crab payload from `x中🦀y`; compare pointers to prove borrowing, and distinguish
/// two Unicode scalars from seven byte offsets.
///
/// Only local values and analysis buffers are created or changed; no file, network or process side
/// effects.
///
/// # Arguments
/// None. The test builds its own fixture and expected results.
///
/// # Returns
/// `()` after all checks pass; no data or output parameter is returned.
///
/// # Panics
/// If a stated invariant fails or an unwrapped fixture/setup operation fails.
///
/// # Examples
/// Local test-module call:
/// ```text
/// text_iteration_uses_unicode_scalars_without_allocating_text();
/// ```
#[test]
fn text_iteration_uses_unicode_scalars_without_allocating_text() {
    let source = "x中🦀y";
    let span = span(1, 8);
    assert_eq!(span.text(source), Some("中🦀"));
    assert_eq!(span.text(source).unwrap().as_ptr(), source[1..].as_ptr());
    assert_eq!(
        span.iter_text(source).unwrap().collect::<Vec<_>>(),
        ['中', '🦀']
    );
    assert_eq!(span.iter().count(), 7);
}

/// Try split UTF-8 boundaries, out-of-bounds and reversed spans against `中x`, requiring absent
/// text/character iterators. A zero-width end-of-file span must instead yield valid empty text and
/// iteration.
///
/// Only local values and analysis buffers are created or changed; no file, network or process side
/// effects.
///
/// # Arguments
/// None. The test builds its own fixture and expected results.
///
/// # Returns
/// `()` after all checks pass; no data or output parameter is returned.
///
/// # Panics
/// If a stated invariant fails or an unwrapped fixture/setup operation fails.
///
/// # Examples
/// Local test-module call:
/// ```text
/// invalid_ranges_are_rejected_and_empty_eof_is_valid();
/// ```
#[test]
fn invalid_ranges_are_rejected_and_empty_eof_is_valid() {
    let source = "中x";
    for (start, end) in [(1, 3), (0, 2), (0, 5), (4, 3)] {
        assert!(span(start, end).text(source).is_none());
        assert!(span(start, end).iter_text(source).is_none());
    }
    let span = span(source.len(), source.len());
    assert_eq!(span.text(source), Some(""));
    assert_eq!(span.iter_text(source).unwrap().count(), 0);
    assert_eq!(span.iter().count(), 0);
}
