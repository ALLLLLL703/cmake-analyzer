// AI-authored snapshot-qualified reference and warning models.
use super::model::{BlockId, CommandId, FileId, ScopeId};
use crate::model::{Spanned, TextSpan};

/// Identifies an environment within one reference collector/index generation.
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct ReferenceEnvironmentId(usize);
impl ReferenceEnvironmentId {
    pub(crate) fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn index(self) -> usize {
        self.0
    }
}
/// Identifies a visit in the RootGraph associated with a reference index.
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ReferenceInvocationId(usize);
impl ReferenceInvocationId {
    pub(crate) fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn index(self) -> usize {
        self.0
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ReferenceKind {
    Definition,
    Read,
    Write,
    Call,
}

/// IDs and spans refer to the owning immutable analysis snapshot.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ReferenceLocation {
    pub possible_value: Option<super::model::PossibleValue>,
    pub file: FileId,
    pub command: CommandId,
    pub span: TextSpan,
    pub kind: ReferenceKind,
}
#[derive(Clone, Debug)]
pub struct ArgumentReference {
    pub name: String,
    pub argument: usize,
    pub location: ReferenceLocation,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FunctionKind {
    Function,
    Macro,
}
#[derive(Clone, Debug)]
pub struct FunctionParameter {
    pub name: String,
    pub definition: ReferenceLocation,
}
#[derive(Clone, Debug)]
pub struct SemanticFunction {
    pub name: String,
    pub kind: FunctionKind,
    pub block: BlockId,
    pub scope: Option<ScopeId>,
    pub definition: ReferenceLocation,
    pub parameters: Vec<FunctionParameter>,
    pub references: Vec<ReferenceLocation>,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SemanticWarnKind {
    PossiblyUnset,
    Unset,
    Empty,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SemanticWarn {
    pub variable: String,
    pub kind: SemanticWarnKind,
}
impl SemanticWarn {
    pub fn message(&self) -> String {
        let state = match self.kind {
            SemanticWarnKind::PossiblyUnset => "may be unset",
            SemanticWarnKind::Unset => "is unset",
            SemanticWarnKind::Empty => "is empty",
        };
        format!("Variable {:?} {state}", self.variable)
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ReferenceTarget {
    Variable,
    Function,
}
#[derive(Clone, Debug)]
pub struct UnresolvedReference {
    pub name: String,
    pub target: ReferenceTarget,
    pub location: ReferenceLocation,
}
#[derive(Clone, Debug)]
pub struct IndexedVariable {
    pub name: String,
    pub scope: ReferenceEnvironmentId,
    pub local_scope: Option<ScopeId>,
    pub possible_value: Option<super::model::PossibleValue>,
    pub definition: Option<ReferenceLocation>,
    pub references: Vec<ReferenceLocation>,
}
#[derive(Clone, Debug)]
pub struct ReferenceWarning {
    pub invocation: ReferenceInvocationId,
    pub location: ReferenceLocation,
    pub warning: Spanned<SemanticWarn>,
}
/// Workspace-owned aggregates; referenced document snapshots remain immutable.
#[derive(Clone, Debug, Default)]
pub struct ReferenceIndex {
    pub variables: Vec<IndexedVariable>,
    pub functions: Vec<SemanticFunction>,
    pub unresolved: Vec<UnresolvedReference>,
    pub warnings: Vec<ReferenceWarning>,
    pub resolved_calls: Vec<ReferenceLocation>,
}
