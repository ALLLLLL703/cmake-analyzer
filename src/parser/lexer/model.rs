use crate::model::{
    self,
    trait1::{IBuffer, ICursor},
};

pub struct RawBuffer<'a> {
    pub text: &'a str,
    pub cursor: RawBufferCursor,
}

pub struct RawBufferCursor {
    pub current_byte: u64,
    pub length: u64,
}

impl<'a> IBuffer for RawBuffer<'a> {
    fn current_byte_to_span(&self) -> model::TextSpan {}

    fn buffer_advance_length(&self) -> u64 {
        todo!()
    }
}

impl ICursor for RawBufferCursor {
    fn to_span(&self) -> model::TextSpan {
        todo!()
    }
}
