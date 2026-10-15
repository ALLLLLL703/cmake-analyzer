// AI-generated import change.
pub use super::path_model::*;
pub use super::reference_model::*;
pub use super::value_model::*;
use std::{collections::HashMap, sync::Arc};

use crate::{
    model::{Spanned, TextSpan},
    parser::{
        nparser::model::{NParsedBuffer, NParsedError},
        semanticer::schema::CommandSchema,
    },
};

// AI-authored incremental analysis cache.
pub struct IncrementalSemanticer {
    pub buffer: Arc<SemanticedBuffer>,
    pub work: crate::model::incremental::IncrementalWork,
    pub structure_rebuilt: bool,
    pub templates_rebuilt: bool,
    pub(super) analyzer: Semanticer,
    pub(super) source: std::sync::Arc<str>,
    pub(super) variable_seed: Vec<SemanticVariable>,
}

pub struct Semanticer {
    pub buffer: NParsedBuffer,
    pub cursor: SemanticCursor,
    // AI-generated change.
    pub schemas: HashMap<String, Arc<CommandSchema>>,
    // AI-generated additions.
    pub version: Option<CMakeVersion>,
    pub(super) injected_schemas: HashMap<String, Arc<CommandSchema>>,
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
    pub functions: Vec<SemanticFunction>,
    pub values: Vec<VariableValueAt>,
    pub warnings: Vec<Spanned<SemanticWarn>>,
    pub unresolved_references: Vec<UnresolvedReference>,
    pub diagnostics: Vec<Spanned<SemanticError>>,
    // AI-generated addition.
    pub version: Option<CMakeVersion>,
    // AI-generated additions.
    pub blocks: Vec<SemanticBlock>,
    pub scopes: Vec<SemanticScope>,
    pub symbols: Vec<SemanticSymbol>,
}

// AI-generated implementation.
pub struct SemanticDisplay<'a> {
    pub(super) buffer: &'a SemanticedBuffer,
    pub(super) colored: bool,
    pub(super) all_variables: bool,
}

#[derive(Clone, Debug)]
pub enum SemanticError {
    NParsedError(NParsedError),
    CommandNotFound(String),
    // AI-generated additions.
    Version(String),
    UnsupportedVersion(CMakeVersion),
    // AI-generated additions.
    UnclosedBlock(BlockKind),
    UnexpectedBlockEnd(String),
    UnexpectedBranch(String),
    InvalidIfBranch(String),
}

#[derive(Clone, Debug)]
pub struct SemanticVariable {
    // AI-generated addition.
    pub name: String,
    pub builtin: bool,
    /// First possible source write, not proof of runtime initialization.
    /// Raw variable-name argument range in the owning SemanticedBuffer; absent for unwritten builtins.
    pub first_write: Option<TextSpan>,
    // AI-generated change.
    pub scope: Option<ScopeId>,
    pub changing: Vec<VariableChanging>,
    pub references: Vec<ReferenceLocation>,
}

#[derive(Clone, Debug)]
pub struct VariableChanging {
    pub command: CommandId,
    /// Raw Out argument range, including any quote/bracket delimiters.
    pub span: TextSpan,
}
#[derive(Clone, Debug)]
pub struct SemanticCommandRepresent {
    pub id: CommandId,
    pub kind: CommandKind,
    pub span: TextSpan,
    pub args: Vec<SemanticArg>,
    pub variable_references: Vec<ArgumentReference>,
    pub paths: Vec<PathArgument>,
    // AI-generated additions.
    pub name: String,
    pub closed: bool,
    pub schema: Option<Arc<CommandSchema>>,
    // AI-generated additions.
    pub full_span: TextSpan,
    pub block: Option<BlockId>,
    pub scope: Option<ScopeId>,
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
    // AI-generated additions.
    pub span: TextSpan,
    pub form: SemanticArgForm,
}

// AI-generated implementation.
impl SemanticArg {
    /// Return raw text only when expansion, escaping, or list splitting is unnecessary.
    pub fn literal_text(&self) -> Option<&str> {
        match self.form {
            // Bracket arguments suppress expansion and escape processing.
            SemanticArgForm::Bracket => Some(&self.content),
            SemanticArgForm::Quoted | SemanticArgForm::Unquoted => {
                (!self.content.contains(['$', '\\'])
                    && (self.form != SemanticArgForm::Unquoted || !self.content.contains(';')))
                .then_some(self.content.as_str())
            }
            _ => None,
        }
    }
}

// AI-generated implementation.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SemanticArgForm {
    Unquoted,
    Quoted,
    Bracket,
    LeftParen,
    RightParen,
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

// AI-generated implementation.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct BlockId {
    pub id: u32,
}

// AI-generated implementation.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ScopeId {
    pub id: u32,
}

// AI-generated implementation.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BlockKind {
    If,
    While,
    Foreach,
    Function,
    Macro,
    Block,
}

// AI-generated implementation.
#[derive(Clone, Debug)]
pub struct SemanticBlock {
    pub id: BlockId,
    pub kind: BlockKind,
    pub contains: Vec<CommandId>,
    pub children: Vec<BlockId>,
    pub parent: Option<BlockId>,
    pub opening: CommandId,
    pub closing: Option<CommandId>,
    pub closed: bool,
    pub span: TextSpan,
}

// AI-generated implementation.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ScopeKind {
    Function,
    VariablesBlock,
}

// AI-generated implementation.
#[derive(Clone, Debug)]
pub struct SemanticScope {
    pub id: ScopeId,
    pub kind: ScopeKind,
    pub block: BlockId,
    pub parent: Option<ScopeId>,
}

// AI-generated implementation.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SemanticSymbolKind {
    Block(BlockId),
    Command(CommandId),
}

// AI-generated implementation.
#[derive(Clone, Debug)]
pub struct SemanticSymbol {
    pub kind: SemanticSymbolKind,
    pub span: TextSpan,
}

// AI-generated implementation.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CMakeVersion {
    pub major: u32,
    pub minor: u32,
    pub patch: u32,
}

#[derive(Clone)]
pub(super) struct OutputMatch {
    /// the position of the arg
    pub position: usize,
    /// the possible collect of out arg
    pub indices: Vec<usize>,
    /// the keywords sum
    pub keywords: usize,
}

#[derive(Default, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct FileId {
    pub id: u32,
}

// AI-generated derive change.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
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
        // AI-generated addition.
        let schemas: HashMap<_, _> = schemas
            .into_iter()
            .map(|(name, schema)| (name.to_ascii_lowercase(), Arc::new(schema)))
            .collect();
        Self {
            buffer,
            cursor: SemanticCursor::new(max_length),
            // AI-generated additions.
            injected_schemas: schemas.clone(),
            schemas,
            version: None,
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
            variable_references: Vec::new(),
            paths: Vec::new(),
            kind,
            span,
            // AI-generated additions.
            name: command_name,
            closed: true,
            schema: None,
            full_span: span,
            block: None,
            scope: None,
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
