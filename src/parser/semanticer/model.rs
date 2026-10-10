use std::collections::HashMap;

use crate::{
    model::TextSpan,
    parser::{
        nparser::model::{NParsedBuffer, NParsedError},
        semanticer::schema::CommandSchema,
    },
};

pub struct Semanticer {
    pub buffer: NParsedBuffer,
    pub cursor: SemanticCursor,
    pub schemas: HashMap<String, CommandSchema>,
}

#[derive(Default, Debug)]
pub struct SemanticCursor {
    pub offset: usize,
    pub max_length: usize,
}

#[derive(Default, Clone, Debug)]
pub struct SemanticedBuffer {
    pub id: FileId,
    pub commands: Vec<SemanticCommandRepresent>,
    pub variables: Vec<SemanticVariable>,
}

#[derive(Clone, Debug)]
pub enum SemanticError {
    NParsedError(NParsedError),
    CommandNotFound(String),
}

#[derive(Clone, Debug)]
pub struct SemanticVariable {
    pub init_value: String,
    pub scope: Option<SemanticScope>,
    pub changing: Vec<VariableChanging>,
}

#[derive(Clone, Debug)]
pub struct VariableChanging {
    pub command: CommandId,
}
#[derive(Clone, Debug)]
pub struct SemanticCommandRepresent {
    pub id: CommandId,
    pub kind: CommandKind,
    pub span: TextSpan,
    pub args: Vec<SemanticArg>,
}

#[derive(Clone, Debug)]
pub enum ArgKind {
    ReferencingVar,
    Out,
    Pure,

    Path,
}

#[derive(Clone, Debug)]
pub struct SemanticArg {
    pub kind: Vec<ArgKind>,

    pub content: String,
}

#[derive(Clone, Debug)]
pub enum CommandKind {
    Builtin(BuiltinCommand),
    Other(String),
}

#[derive(Clone, Debug)]
pub enum BuiltinCommand {
    KeyWord,
    Other,
}

#[derive(Clone, Debug)]
pub enum KeyWordCommand {
    If,
    EndIf,
    Function,
    EndFunction,
}

#[derive(Clone, Debug)]
pub struct SemanticScope {
    pub conatains: Vec<CommandId>,
    pub span: TextSpan,
    pub closed: bool,
}

#[derive(Default, Clone, Debug)]
pub struct FileId {
    pub id: u32,
}

#[derive(Clone, Debug)]
pub struct CommandId {
    pub id: u32,
}

impl SemanticCursor {
    pub fn new(max_length: usize) -> Self {
        Self {
            offset: 0,
            max_length,
        }
    }
}

impl Semanticer {
    pub fn new(buffer: NParsedBuffer, schemas: HashMap<String, CommandSchema>) -> Self {
        let max_length = buffer.nodes.len();
        Self {
            buffer,
            cursor: SemanticCursor::new(max_length),
            schemas,
        }
    }
}

impl SemanticedBuffer {
    pub fn new_with_file_id(id: FileId) -> Self {
        Self {
            id,
            ..Default::default()
        }
    }

    /// discard
    pub fn new_command<'cmd>(
        &'cmd mut self,
        command_name: String,
        span: TextSpan,
        kind: CommandKind,
    ) -> &'cmd mut SemanticCommandRepresent {
        let result = SemanticCommandRepresent {
            id: CommandId {
                id: self.commands.len() as u32,
            },
            args: Vec::new(),
            kind,
            span,
        };

        self.commands.push(result);

        todo!()
    }
}

// 需要干什么?
//首先需要虑出所有symbol
// and then extract all variable and its !Span and where it might be changed bind the scope with id
// then extract all function and its !Span(also should know the builtin function sign)
// then find all blocks and its Span
// then find all out args function and mark the variable changes
// need a scope struct to represent the scope of a variable (can be invert!)
// scope should not own the variable struct
// need a struct to represent the sign of a command(can be inject via lua script)
//
//
// writing in the paper is extremely important,,, i see
