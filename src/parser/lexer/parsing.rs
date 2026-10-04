use crate::{
    model::{Spanned, TextSpan},
    parser::lexer::model::{LexContent, LexError, LexedBuffer, Lexer, RawBuffer},
};

type LexResult<T> = Result<T, LexError>;
impl<'a> Lexer<'a> {
    pub fn new(buffer: RawBuffer<'a>) -> Self {
        Lexer { buffer }
    }
    pub fn parse(&mut self) -> LexedBuffer {
        let mut result = LexedBuffer::default();
        let mut left_parentheses_num: usize = 0;
        let mut last_parentheses_loc: TextSpan = TextSpan::default();
        let mut position = self.buffer.current_byte_to_span();
        loop {
            if let Err(error) = self.skip_whitespace() {
                result.add_error(error);
                break;
            }

            let whitespace_start = position.start_byte;
            advance_position(
                &mut position,
                &self.buffer.text[whitespace_start..self.buffer.cursor.offset],
            );
            let mut span = position;
            let content = match self.peek() {
                Err(LexError::ReachTheEof) => break,
                Err(error) => {
                    result.add_error(error);
                    break;
                }
                Ok('"') => {
                    // Consume the opening quote; the scanner consumes the closing one.
                    self.buffer.cursor.offset += 1;
                    match self.advance_string_literal() {
                        Ok(text) => Some(LexContent::StringLiteral(text)),
                        Err(error) => {
                            result.add_error(error);
                            break;
                        }
                    }
                }
                Ok('(') => {
                    self.buffer.cursor.offset += 1;
                    left_parentheses_num += 1;
                    last_parentheses_loc = TextSpan {
                        end_byte: self.buffer.cursor.offset,
                        ..span
                    };
                    Some(LexContent::LeftParentheses)
                }
                Ok(')') => {
                    self.buffer.cursor.offset += 1;
                    if left_parentheses_num > 0 {
                        left_parentheses_num -= 1;
                        Some(LexContent::RightParentheses)
                    } else {
                        result.add_error(LexError::UnclosedParentheses(TextSpan {
                            end_byte: self.buffer.cursor.offset,
                            ..span
                        }));
                        None
                    }
                }
                Ok(_) => match self.advance_identifier() {
                    Ok(text) => Some(LexContent::Identifier(text)),
                    Err(error) => {
                        result.add_error(error);
                        break;
                    }
                },
            };
            span.end_byte = self.buffer.cursor.offset;
            advance_position(
                &mut position,
                &self.buffer.text[span.start_byte..span.end_byte],
            );
            if let Some(content) = content {
                result.add_lex(Spanned { content, span });
            }
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

    /// Called immediately after consuming the opening quote.
    /// Consumes the closing quote.
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

// Track only newly consumed text instead of rescanning the source for every token.
fn advance_position(position: &mut TextSpan, consumed: &str) {
    for ch in consumed.chars() {
        if ch == '\n' {
            position.row += 1;
            position.column = 0;
        } else {
            position.column += 1;
        }
    }
    position.start_byte += consumed.len();
    position.end_byte = position.start_byte;
}
