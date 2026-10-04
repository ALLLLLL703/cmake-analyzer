use crate::{
    model::{TextSpan, trait1::IBuffer},
    parser::lexer::model::{LexContent, LexError, LexedBuffer, Lexer, RawBuffer},
};

type LexResult<T> = Result<T, LexError>;
impl<'a> Lexer<'a> {
    pub fn new(buffer: RawBuffer<'a>) -> Self {
        Lexer { buffer }
    }
    pub fn parse(&mut self) -> LexedBuffer {
        let mut result = LexedBuffer::default();
        if let Err(LexError::ReachTheEof) = self.skip_whitespace() {
            result.add_error(LexError::ReachTheEof);
            return result;
        }
        let mut advance_result = match self
            .buffer
            .text
            .get(self.buffer.cursor.offset..)
            .and_then(|rest| rest.chars().next())
        {
            Some(c) => {
                self.buffer.cursor.offset += c.len_utf8();
                Ok(c)
            }
            None => Err(LexError::ReachTheEof),
        };

        let mut left_parentheses_num: usize = 0;
        let mut last_parentheses_loc: TextSpan = TextSpan::default();
        loop {
            match advance_result {
                Err(LexError::ReachTheEof) => {
                    result.add_error(LexError::ReachTheEof);
                    break;
                }
                Ok('"') => match self.advance_string_literal() {
                    Err(LexError::ReachTheEof) => {
                        result.add_error(LexError::ReachTheEof);
                        break;
                    }
                    Ok(ok) => result.add_lex(LexContent::StringLiteral(ok)),

                    _ => {}
                },

                Ok('(') => {
                    left_parentheses_num += 1;
                    last_parentheses_loc = self.buffer.current_byte_to_span();
                    result.add_lex(LexContent::LeftParentheses);
                }
                Ok(')') => {
                    if left_parentheses_num > 0 {
                        left_parentheses_num -= 1;
                        result.add_lex(LexContent::RightParentheses);
                    } else {
                        result.add_error(LexError::UnclosedParentheses(
                            self.buffer.current_byte_to_span(),
                        ));
                    }
                }
                Ok(_) => match self.advance_identifier() {
                    Err(LexError::ReachTheEof) => {
                        result.add_error(LexError::ReachTheEof);
                        break;
                    }
                    Err(e) => result.add_error(e),
                    Ok(ok) => result.add_lex(LexContent::Identifier(ok)),
                },
                _ => {}
            }
            if let Err(LexError::ReachTheEof) = self.skip_whitespace() {
                result.add_error(LexError::ReachTheEof);
                return result;
            }

            advance_result = self.advance();
        }

        if left_parentheses_num > 0 {
            result.add_error(LexError::UnclosedParentheses(last_parentheses_loc));
        }

        result
    }

    /// offset will stay at the first non-whitespace char
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

    /// return the current char(before advance)
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

    pub fn peek(&self) -> LexResult<char> {
        self.buffer
            .text
            .get(self.buffer.cursor.offset..)
            .and_then(|rest| rest.chars().next())
            .ok_or(LexError::ReachTheEof)
    }

    /// not skip the last char of the Identifier
    pub fn advance_identifier(&mut self) -> LexResult<String> {
        let rest = self
            .buffer
            .text
            .get(self.buffer.cursor.offset..)
            .filter(|rest| !rest.is_empty())
            .ok_or(LexError::ReachTheEof)?;

        let length = rest
            .bytes()
            .take_while(|byte| !matches!(byte, b' ' | b'\t' | b'\n' | b'\r' | b'(' | b')'))
            .count();
        self.buffer.cursor.offset += length;
        Ok(rest[..length].to_owned())
    }

    /// should be call when offset is at "
    ///  skip the last "
    pub fn advance_string_literal(&mut self) -> LexResult<String> {
        let start = self.buffer.cursor.offset;
        let rest = self
            .buffer
            .text
            .get(self.buffer.cursor.offset..)
            .ok_or(LexError::ReachTheEof)?;

        let mut escaped = false;
        for (index, ch) in rest.char_indices() {
            if escaped {
                escaped = false;
                continue;
            }

            match ch {
                '\\' => escaped = true,
                '"' => {
                    let end = start + index;
                    self.buffer.cursor.offset = end + 1;
                    return Ok(self.buffer.text[start..end].to_owned());
                }
                _ => {}
            }
        }

        self.buffer.cursor.offset = self.buffer.text.len();
        Err(LexError::UnclosedStringLiteral(
            self.buffer.current_byte_to_span(),
        ))
    }
}
