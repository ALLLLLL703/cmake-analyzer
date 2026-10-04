use crate::{
    model::Spanned,
    parser::{
        lexer::model::LexContent,
        nparser::model::{NParsedBuffer, NParsedError, NParser},
    },
};

type NParseResult<T> = Result<T, NParsedError>;

impl NParser {
    pub fn parse(&mut self) -> NParsedBuffer {
        let mut result = NParsedBuffer::default();
        if self.cursor.max_length == 0 {
            return result;
        }

        while let Ok(lex) = self.peek() {}

        result
    }

    pub fn peek(&self) -> NParseResult<&Spanned<LexContent>> {
        self.buffer
            .lex
            .get(self.cursor.offset)
            .ok_or(NParsedError::ReachTheEof)
    }
}
