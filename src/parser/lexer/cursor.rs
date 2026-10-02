use crate::model::{TextSpan, trait1::IBuffer};

use super::model::PreBuffer;

pub(super) struct Cursor<'src> {
    pub text: &'src str,
    pub offset: usize,
    row: u64,
    column: u64,
}

impl<'src> Cursor<'src> {
    pub fn new(buffer: PreBuffer<'src>) -> Self {
        let position = buffer.current_byte_to_span();
        Self {
            text: buffer.text,
            offset: buffer.current_byte,
            row: position.row,
            column: position.column,
        }
    }

    pub fn span(&self, end: usize) -> TextSpan {
        TextSpan {
            row: self.row,
            column: self.column,
            start_byte: self.offset,
            end_byte: end,
        }
    }

    pub fn advance_to(&mut self, end: usize) {
        for (relative, ch) in self.text[self.offset..end].char_indices() {
            let byte = self.offset + relative;
            match ch {
                '\n' => {
                    self.row += 1;
                    self.column = 0;
                }
                '\r' if self.text.as_bytes().get(byte + 1) == Some(&b'\n') => {}
                '\u{feff}' if byte == 0 => {}
                _ => self.column += 1,
            }
        }
        self.offset = end;
    }
}
