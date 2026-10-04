use crate::{
    model::{Spanned, TextSpan},
    parser::lexer::model::{LexError, LexedBuffer},
};

pub struct NParser {
    pub buffer: LexedBuffer,
    pub cursor: NParseCursor,
}

#[derive(Default, Debug)]
pub struct NParseCursor {
    pub offset: usize,
    pub max_length: usize,
}

#[derive(Default, Debug, Clone)]
pub struct NParsedBuffer {
    pub commands: Vec<NParsedCommand>,
    pub error: Vec<Spanned<NParsedError>>,
}

#[derive(Debug, Clone)]
pub struct NParsedCommand {
    pub name: Spanned<String>,
    pub args: Vec<Spanned<NParsedArgument>>,
}

#[derive(Debug, Clone)]
pub enum NParsedArgument {
    Unquoted(String),
    Quoted(String),
    Bracked(String),
    LeftParen,
    RightParen,
}

#[derive(Debug, Clone)]
pub enum NParsedError {
    LexError(LexError),
    ReachTheEof,
}

impl NParser {
    pub fn new(buffer: LexedBuffer) -> Self {
        let cursor = NParseCursor {
            max_length: buffer.lex.len(),
            ..Default::default()
        };
        NParser { buffer, cursor }
    }
}

impl NParsedBuffer {
    pub fn add_command(&mut self, command: NParsedCommand) {
        self.commands.push(command);
    }

    pub fn add_error(&mut self, error: NParsedError, span: TextSpan) {
        self.error.push(Spanned {
            span,
            content: error,
        });
    }
}
