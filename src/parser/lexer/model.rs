use std::fmt::LowerExp;

use crate::model::{
    self, Spanned, TextSpan,
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

#[derive(Debug, Default)]
pub struct LexedBuffer {
    pub lex: Vec<Spanned<LexContent>>,
    pub errors: Vec<LexError>,
}

#[derive(Debug)]
pub enum LexContent {
    Identifier(String),
    LeftParentheses,
    RightParentheses,
    /// with out double quote
    StringLiteral(String),
    /// Raw bracket content without delimiters or the optional initial newline.
    BracketArgument(String),
}

#[derive(Debug, Clone)]
pub enum LexError {
    ReachTheEof,
    UnclosedParentheses(TextSpan),
    UnclosedStringLiteral(TextSpan),
    UnclosedBracketArgument(TextSpan),
}

impl<'a> RawBuffer<'a> {
    pub fn current_byte_to_span(&self) -> model::TextSpan {
        self.cursor.to_span(self.text)
    }

    pub fn buffer_advance_length(&self) -> usize {
        self.text.len()
    }

    pub fn span_to_text(&self, span: TextSpan) -> Option<String> {
        self.text
            .get(span.start_byte..span.end_byte)
            .map(|s| s.to_owned())
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
        for (offset, ch) in content.char_indices() {
            if offset >= self.offset {
                break;
            }
            if ch == '\n' {
                span.row += 1;
                span.column = 0;
            } else {
                span.column += 1;
            }
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

impl LexedBuffer {
    pub fn add_error(&mut self, error: LexError) {
        self.errors.push(error)
    }

    pub fn add_lex(&mut self, lex: Spanned<LexContent>) {
        self.lex.push(lex);
    }
}
