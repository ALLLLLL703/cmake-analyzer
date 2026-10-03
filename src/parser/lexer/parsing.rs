use crate::parser::lexer::model::{LexError, LexedBuffer, Lexer, RawBuffer};

type LexResult<T> = Result<T, LexError>;
impl<'a> Lexer<'a> {
    pub fn parse(&mut self) -> LexedBuffer {
        loop {
            match self.advance() {
                Err(LexError::ReachTheEof) => {
                    break;
                }
                Ok('"') => match self.advance_string_literal() {
                    Err(LexError::ReachTheEof) => {
                        break;
                    }
                    _ => {}
                },
                _ => {}
            }
        }

        todo!()
    }

    pub fn skip_whitespace(&mut self) -> LexResult<()> {
        let rest = self
            .buffer
            .text
            .get(self.buffer.cursor.offset..)
            .ok_or(LexError::ReachTheEof)?;

        let skipped = rest
            .bytes()
            .take_while(|byte| matches!(byte, b' ' | b'\t' | b'\n' | b'\r'))
            .count();
        self.buffer.cursor.offset += skipped;
        Ok(())
    }

    pub fn advance(&mut self) -> LexResult<char> {
        let ch = self
            .buffer
            .text
            .get(self.buffer.cursor.offset..)
            .and_then(|rest| rest.chars().next())
            .ok_or(LexError::ReachTheEof)?;
        self.buffer.cursor.offset += ch.len_utf8();

        Ok(ch)
    }

    pub fn advance_identifier(&mut self) -> LexResult<String> {
        todo!()
    }

    /// should be call when offset is at "
    /// skip the last "
    pub fn advance_string_literal(&mut self) -> LexResult<String> {
        let rest = self
            .buffer
            .text
            .get(self.buffer.cursor.offset..)
            .ok_or(LexError::ReachTheEof)?;

        let mut skipped: usize = 0;
        let mut chars = rest.chars();

        while let Some(ch) = chars.next() {}

        todo!()
    }
}
