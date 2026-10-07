// AI-generated tests.

use cmake_analyzer::model::TextSpan;

fn span(start_byte: usize, end_byte: usize) -> TextSpan {
    TextSpan {
        start_byte,
        end_byte,
        ..TextSpan::default()
    }
}

#[test]
fn spans_iterate_half_open_byte_offsets_by_value_and_reference() {
    let span = span(2, 5);
    assert_eq!(span.iter().collect::<Vec<_>>(), [2, 3, 4]);
    assert_eq!(span.into_iter().collect::<Vec<_>>(), [2, 3, 4]);
    assert_eq!((&span).into_iter().rev().collect::<Vec<_>>(), [4, 3, 2]);
    assert_eq!(span.iter().len(), 3);
}

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
