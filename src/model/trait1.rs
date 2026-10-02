use crate::model::TextSpan;

pub trait IBuffer {
    fn current_byte_to_span(&self) -> TextSpan;
}
