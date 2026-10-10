use crate::{model::TextSpan, parser::nparser::model::NParsedBuffer};

pub struct Semanticer {
    pub buffer: NParsedBuffer,
    pub cursor: SemanticCursor,
}

#[derive(Default, Debug)]
pub struct SemanticCursor {
    pub offset: usize,
    pub max_length: usize,
}

#[derive(Clone, Debug)]
pub struct SemanticedBuffer {
    pub symbols: Vec<SemanticSymbol>,
    pub variable: Vec<SemanticVariable>,
    pub id: FileId,
    pub scopes: Vec<SemanticScope>,
    pub blocks: Vec<SemanticBlock>,
    pub commands: Vec<CommandId>,
}

#[derive(Clone, Debug)]
pub struct SemanticVariable {
    pub value: String,
    pub id: VariableId,
    pub block: Option<BlockId>,
    pub scope: Option<ScopeId>,
}

#[derive(Clone, Debug)]
pub struct SemanticSymbol {
    pub kind: SemanticSymbolKind,
    pub block_id: BlockId,
    pub id: SymbolId,
    pub span: TextSpan,
}

#[derive(Clone, Debug)]
pub struct SemanticScope {
    pub id: ScopeId,
    pub span: TextSpan,
    pub file: FileId,
    pub variable: Vec<VariableId>,
    pub child: Vec<ScopeId>,
    pub parent: Option<ScopeId>,
    pub commands: Vec<CommandId>,
}
#[derive(Clone, Debug)]
pub enum SemanticSymbolKind {
    If,
    While,
    Function,
    Foreach,
    TopCommand,
    Variable,
    Option,
    Unset,
}

#[derive(Clone, Debug)]
pub struct SemanticBlock {
    pub id: BlockId,
    pub binding_scope: Option<ScopeId>,
    pub parent: Option<BlockId>,
    pub child: Vec<BlockId>,
    pub commands: Vec<CommandId>,
}

#[derive(Clone, Debug)]
pub struct VariableId {
    pub id: u32,
}

#[derive(Clone, Debug)]
pub struct BlockId {
    pub id: u32,
}

#[derive(Clone, Debug)]
pub struct ScopeId {
    pub id: u32,
}

#[derive(Clone, Debug)]
pub struct FunctionId {
    pub id: u32,
}

#[derive(Clone, Debug)]
pub struct SymbolId {
    pub id: u32,
}
#[derive(Clone, Debug)]
pub struct FileId {
    pub id: u32,
}

#[derive(Clone, Debug)]
pub struct CommandId {
    pub id: u32,
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
