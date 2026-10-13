// AI-generated implementation.
use crate::{
    model::{Spanned, TextSpan},
    parser::semanticer::model::{BlockId, CommandId, ScopeId, SemanticError, SemanticedBuffer},
};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
/// Qualifies IDs that are valid only within one semantic snapshot.
pub struct AnalysisKey {
    pub file: u32,
    pub revision: u64,
}
#[derive(Clone, Debug)]
pub struct SourceRef {
    pub analysis: AnalysisKey,
    pub command: CommandId,
    pub span: TextSpan,
    pub block: Option<BlockId>,
    pub scope: Option<ScopeId>,
    pub conditional: bool,
    pub deferred: bool,
}
#[derive(Clone, Debug)]
pub struct Argument {
    pub text: String,
    pub literal: bool,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DependencyKind {
    Include,
    Subdirectory,
}
#[derive(Clone, Debug)]
pub struct Dependency {
    pub kind: DependencyKind,
    pub path: Argument,
    pub binary: Option<Argument>,
    pub optional: bool,
    pub no_policy_scope: bool,
}
#[derive(Clone, Debug)]
pub struct ProjectDeclaration {
    pub name: Argument,
    pub version: Option<Argument>,
    pub description: Option<Argument>,
    pub homepage: Option<Argument>,
    pub languages: Vec<Argument>,
    pub arguments: Vec<Argument>,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TargetKind {
    Executable,
    Library,
    StaticLibrary,
    SharedLibrary,
    ModuleLibrary,
    ObjectLibrary,
    InterfaceLibrary,
    UnknownLibrary,
    Custom,
    Alias,
}
#[derive(Clone, Debug)]
pub struct TargetDeclaration {
    pub name: Argument,
    pub kind: TargetKind,
    pub imported: bool,
    pub global: bool,
    pub alias: Option<Argument>,
    pub sources: Vec<Argument>,
    pub arguments: Vec<Argument>,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Visibility {
    Private,
    Public,
    Interface,
}
#[derive(Clone, Debug)]
pub struct FileSet {
    pub name: Argument,
    pub kind: Option<Argument>,
    pub base_dirs: Vec<Argument>,
    pub files: Vec<Argument>,
}
#[derive(Clone, Debug)]
pub struct SourceSection {
    pub visibility: Option<Visibility>,
    pub files: Vec<Argument>,
    pub file_sets: Vec<FileSet>,
}
#[derive(Clone, Debug)]
pub struct SourceDeclaration {
    pub target: Argument,
    pub sections: Vec<SourceSection>,
    pub arguments: Vec<Argument>,
}
#[derive(Clone, Debug)]
pub enum BuildEffect {
    VariableWrites {
        names: Vec<String>,
        // False retains nonlocal writes without guessing their destination.
        local: bool,
    },
    Project(ProjectDeclaration),
    Minimum {
        arguments: Vec<Argument>,
    },
    Languages {
        arguments: Vec<Argument>,
    },
    Target(TargetDeclaration),
    Sources(SourceDeclaration),
    Links {
        target: Argument,
        arguments: Vec<Argument>,
    },
    Dependency(Dependency),
}
#[derive(Clone, Debug)]
pub struct BuildFact {
    pub source: SourceRef,
    pub effect: BuildEffect,
}
#[derive(Clone, Debug)]
pub struct BuildDiagnostic {
    pub source: SourceRef,
    pub message: String,
}
#[derive(Clone, Debug)]
/// File-local facts; child files belong to the Workspace graph.
pub struct BuildStruct {
    pub analysis: AnalysisKey,
    pub facts: Vec<BuildFact>,
    pub diagnostics: Vec<BuildDiagnostic>,
}
// AI-generated display model.
pub struct BuildDisplay<'a> {
    pub(super) buffer: &'a BuildStruct,
    pub(super) colored: bool,
    pub(super) semantic_diagnostics: &'a [Spanned<SemanticError>],
}

/// Temporary borrow; the returned BuildStruct owns its payloads.
pub struct BuildStructAnalyzer<'a> {
    pub buffer: &'a SemanticedBuffer,
    pub origin: AnalysisKey,
}
impl<'a> BuildStructAnalyzer<'a> {
    pub fn new(buffer: &'a SemanticedBuffer, origin: AnalysisKey) -> Self {
        Self { buffer, origin }
    }
}
