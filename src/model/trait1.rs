use crate::model::TextSpan;

pub trait IBuffer {
    fn current_byte_to_span(&self) -> TextSpan;
    fn buffer_advance_length(&self) -> usize;
    fn span_to_text(&self, span: TextSpan) -> Option<&str>;
}

pub trait ICursor {
    fn to_span(&self, content: &str) -> TextSpan;
}
