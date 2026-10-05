use std::ops::Range;

use crate::{
    model::{Spanned, TextSpan},
    parser::nparser::model::NParsedBuffer,
};

pub struct Semanticer {
    pub buffer: NParsedBuffer,
}

impl Semanticer {
    pub fn new(buffer: NParsedBuffer) -> Self {
        Self { buffer }
    }
}

#[derive(Debug)]
pub struct SemanticBuffer {
    pub parsed: NParsedBuffer,
    pub blocks: Vec<Block>,
    pub command_blocks: Vec<Option<usize>>,
    pub scopes: Vec<Scope>,
    pub command_scopes: Vec<usize>,
    pub symbols: Vec<Symbol>,
    pub references: Vec<Reference>,
    pub diagnostics: Vec<Spanned<Diagnostic>>,
}

/// Flat arena: indices refer to this buffer, never to recursively owned nodes.
#[derive(Debug)]
pub struct Block {
    pub kind: BlockKind,
    pub parent: Option<usize>,
    pub opening: usize,
    pub closing: Option<usize>,
    pub branches: Vec<usize>,
    pub scope: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlockKind {
    If,
    Foreach,
    While,
    Function,
    Macro,
    Block,
}

#[derive(Debug)]
pub struct Scope {
    pub kind: ScopeKind,
    pub parent: Option<usize>,
    pub block: Option<usize>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScopeKind {
    Directory,
    Function,
    Macro,
    Variables,
}

/// A slice of an existing command name or argument payload, not a copied string.
#[derive(Debug, Clone)]
pub struct Name {
    pub command: usize,
    pub argument: Option<usize>,
    pub bytes: Range<usize>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Namespace {
    Variable,
    Cache,
    Environment,
    Command,
    Target,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SymbolKind {
    Variable,
    Cache,
    Environment,
    Parameter,
    Function,
    Macro,
    Target,
    Alias,
    Unset,
    ParentWrite,
}

#[derive(Debug)]
pub struct Symbol {
    pub name: Name,
    pub kind: SymbolKind,
    pub namespace: Namespace,
    pub scope: usize,
    pub span: TextSpan,
}

#[derive(Debug)]
pub struct Reference {
    pub name: Name,
    pub namespace: Namespace,
    pub scope: usize,
    pub span: TextSpan,
    pub dynamic: bool,
    /// Static navigation candidates, not a claim about CMake execution.
    pub resolved: Vec<usize>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Severity {
    Error,
    Warning,
}

#[derive(Debug)]
pub struct Diagnostic {
    pub severity: Severity,
    pub kind: DiagnosticKind,
}

#[derive(Debug)]
pub enum DiagnosticKind {
    UnexpectedBranch,
    DuplicateElse,
    ElseIfAfterElse,
    UnmatchedBlockEnd(BlockKind),
    UnclosedBlock(BlockKind),
    EndNameMismatch,
    InvalidArity {
        min: usize,
        max: Option<usize>,
        actual: usize,
    },
    DuplicateTarget,
}

pub(super) struct Structure {
    pub blocks: Vec<Block>,
    pub command_blocks: Vec<Option<usize>>,
    pub scopes: Vec<Scope>,
    pub command_scopes: Vec<usize>,
    pub diagnostics: Vec<Spanned<Diagnostic>>,
}

#[derive(Default)]
pub(super) struct Facts {
    pub symbols: Vec<Symbol>,
    pub references: Vec<Reference>,
    pub diagnostics: Vec<Spanned<Diagnostic>>,
}

pub(super) struct ReferenceFrame {
    pub start: usize,
    pub span: TextSpan,
    pub namespace: Namespace,
    pub dynamic: bool,
}
