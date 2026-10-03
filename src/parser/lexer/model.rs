use std::fmt::LowerExp;

use crate::model::{
    self, TextSpan,
    trait1::{IBuffer, ICursor},
};

#[derive(Debug)]
pub struct RawBuffer<'a> {
    pub text: &'a str,
    pub cursor: RawBufferCursor,
}

#[derive(Default, Debug)]
pub struct RawBufferCursor {
    pub offset: usize,
    pub length: usize,
}

#[derive(Debug)]
pub struct Lexer<'a> {
    pub buffer: RawBuffer<'a>,
}

#[derive(Debug)]
pub struct LexedBuffer {
    pub lex: Vec<LexContent>,
}

#[derive(Debug)]
pub enum LexContent {
    Identifier(String),
    LeftParentheses,
    RightParentheses,
    /// with out double quote
    StringLiteral(String),
}

#[derive(Debug)]
pub enum LexError {
    ReachTheEof,
}

impl<'a> IBuffer for RawBuffer<'a> {
    fn current_byte_to_span(&self) -> model::TextSpan {
        self.cursor.to_span(self.text)
    }

    fn buffer_advance_length(&self) -> usize {
        self.text.len()
    }
}

impl<'a> RawBuffer<'a> {
    pub fn new(content: &'a str) -> Self {
        RawBuffer {
            text: content,
            cursor: RawBufferCursor::new_with_len(content.len()),
        }
    }
}

impl ICursor for RawBufferCursor {
    fn to_span(&self, content: &str) -> model::TextSpan {
        let mut span = TextSpan::default();
        for ch in content.chars() {
            if ch == '\n' {
                span.row += 1;
                continue;
            }
            span.column += 1;
        }
        span.start_byte = self.offset;
        span.end_byte = self.offset;

        span
    }
}

impl RawBufferCursor {
    pub fn new_with_len(len: usize) -> Self {
        RawBufferCursor {
            offset: 0,
            length: len,
        }
    }
}
