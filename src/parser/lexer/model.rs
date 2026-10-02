use crate::model::{TextSpan, trait1::IBuffer};

/// A borrowed source and a UTF-8 byte cursor.
pub struct PreBuffer<'src> {
    pub text: &'src str,
    pub current_byte: usize,
}

impl<'src> PreBuffer<'src> {
    pub fn new(text: &'src str) -> Self {
        Self {
            text,
            current_byte: 0,
        }
    }
}

impl IBuffer for PreBuffer<'_> {
    fn current_byte_to_span(&self) -> TextSpan {
        // A caller-provided invalid cursor is diagnosed by the lexer.
        let mut byte = self.current_byte.min(self.text.len());
        while !self.text.is_char_boundary(byte) {
            byte -= 1;
        }
        let mut row = 0;
        let mut column = 0;
        for (offset, ch) in self.text[..byte].char_indices() {
            match ch {
                '\n' => {
                    row += 1;
                    column = 0;
                }
                '\r' if self.text.as_bytes().get(offset + 1) == Some(&b'\n') => {}
                '\u{feff}' if offset == 0 => {}
                _ => column += 1,
            }
        }
        TextSpan {
            row,
            column,
            start_byte: byte,
            end_byte: byte,
        }
    }
}

/// Lossless lexical output. Source text is borrowed, never copied per token.
#[derive(Debug)]
pub struct LexBuffer<'src> {
    pub text: &'src str,
    pub parts: Vec<LexPart>,
    pub errors: Vec<LexError>,
}

impl<'src> LexBuffer<'src> {
    pub fn part_text(&self, part: &LexPart) -> Option<&'src str> {
        self.text.get(part.span.start_byte..part.span.end_byte)
    }
}

#[derive(Debug)]
pub struct LexPart {
    pub kind: LexKind,
    pub span: TextSpan,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LexKind {
    Bom,
    Space,
    Newline,
    Identifier,
    ParenLeft,
    ParenRight,
    ArgumentUnquoted,
    ArgumentQuoted,
    ArgumentBracket,
    CommentLine,
    CommentBracket,
    Invalid,
}

#[derive(Debug)]
pub struct LexError {
    pub kind: LexErrorKind,
    pub span: TextSpan,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LexErrorKind {
    InvalidOffset,
    UnexpectedCharacter,
    UnterminatedQuotedArgument,
    UnterminatedBracketArgument,
    UnterminatedBracketComment,
}

#[derive(Default)]
pub struct Lexer;

impl Lexer {
    pub fn new() -> Self {
        Self
    }

    pub fn lex(text: &str) -> LexBuffer<'_> {
        Self::parse_buffer(PreBuffer::new(text))
    }

    pub fn parse_buffer(buffer: PreBuffer<'_>) -> LexBuffer<'_> {
        super::lex_main::scan(buffer)
    }
}
