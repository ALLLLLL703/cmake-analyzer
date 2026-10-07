use crate::{
    model::{Spanned, TextSpan},
    parser::lexer::model::{LexContent, LexError, LexedBuffer},
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
    pub nodes: Vec<Spanned<NParsedNode>>,
}

#[derive(Clone, Debug)]
pub enum NParsedNode {
    Command(NParsedCommand),
    Error(NParsedError),
}

#[derive(Debug, Clone)]
pub struct NParsedCommand {
    pub name: Spanned<LexContent>,
    pub args: Vec<Spanned<NParsedArgument>>,
    pub closed: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NParsedArgument {
    Unquoted,
    Quoted,
    Bracked,
    LeftParen,
    RightParen,
}

#[derive(Debug, Clone)]
pub enum NParsedError {
    LexError(LexError),
    ShouldBeCommand,
    ShouldHaveLeftParentheses,
    UnclosedLeftParentheses,
    ReachTheEof,
    UnArgStringLiteral,
    CalledInnerError,
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
    pub fn add_node(&mut self, node: NParsedNode, span: TextSpan) {
        self.nodes.push(Spanned {
            content: node,
            span,
        });
    }
}

impl NParsedCommand {
    pub fn new(name: Spanned<LexContent>) -> Self {
        Self {
            name,
            args: Vec::new(),
            closed: true,
        }
    }

    pub fn add_argument(&mut self, argument: NParsedArgument, span: TextSpan) {
        self.args.push(Spanned {
            content: argument,
            span,
        });
    }

    pub fn append_arg(&mut self, args: &mut Vec<Spanned<NParsedArgument>>) {
        self.args.append(args);
    }
}

impl NParsedNode {
    pub fn new_error(error: NParsedError) -> Self {
        NParsedNode::Error(error)
    }

    pub fn new_command(content: NParsedCommand) -> Self {
        NParsedNode::Command(content)
    }
}
