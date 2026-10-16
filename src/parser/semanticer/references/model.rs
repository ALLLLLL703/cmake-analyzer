// AI-authored internal reference-analysis state.
use crate::parser::semanticer::model::*;
use std::collections::{HashMap, HashSet};

// Aggregate and document-local arenas have distinct identity domains.
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub(super) struct IndexedVariableId(usize);
impl IndexedVariableId {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn index(self) -> usize {
        self.0
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub(super) struct IndexedFunctionId(usize);
impl IndexedFunctionId {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn index(self) -> usize {
        self.0
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub(super) struct SnapshotFunctionId(usize);
impl SnapshotFunctionId {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn index(self) -> usize {
        self.0
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub(super) struct SnapshotVariableId(usize);
impl SnapshotVariableId {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn index(self) -> usize {
        self.0
    }
}
#[derive(Clone, Copy, PartialEq, Eq)]
pub(super) enum Value {
    Unknown,
    MaybeUnset,
    Unset,
    Empty,
    NonEmpty,
}
#[derive(Clone, Copy)]
pub(super) struct Binding {
    pub variable: IndexedVariableId,
    pub value: Value,
    // Numeric call/write clocks are counters, not entity identities.
    pub written_at: usize,
}
#[derive(Default)]
pub(super) struct Environment {
    pub parent: Option<ReferenceEnvironmentId>,
    pub owner: ReferenceEnvironmentId,
    pub deferred: bool,
    pub invalidate_after: usize,
    pub variables: HashMap<String, Binding>,
    pub functions: HashMap<String, Vec<IndexedFunctionId>>,
}
pub(crate) struct FileContext {
    pub(super) root: ReferenceEnvironmentId,
    pub(super) frames: HashMap<BlockId, Frame>,
    pub(super) local_blocks: HashSet<BlockId>,
    pub(super) definitions: HashMap<CommandId, SnapshotFunctionId>,
    pub(super) function_blocks: HashMap<BlockId, SnapshotFunctionId>,
}
pub(super) struct Frame {
    pub environment: ReferenceEnvironmentId,
    pub parent: ReferenceEnvironmentId,
    pub conditional: bool,
    pub writes: HashMap<String, Binding>,
    pub functions: HashMap<String, Vec<IndexedFunctionId>>,
}
#[derive(Default)]
pub(crate) struct ReferenceCollector {
    pub(super) index: ReferenceIndex,
    pub(super) environments: Vec<Environment>,
    pub(super) variables: HashMap<(ReferenceEnvironmentId, String), IndexedVariableId>,
    pub(super) functions: HashMap<(FileId, CommandId), IndexedFunctionId>,
    pub(super) builtins: HashSet<String>,
    pub(super) builtin_functions: HashMap<String, IndexedFunctionId>,
    pub(super) reference_seen: HashSet<(
        IndexedVariableId,
        FileId,
        CommandId,
        usize,
        usize,
        ReferenceKind,
    )>,
    pub(super) call_seen: HashSet<(IndexedFunctionId, FileId, CommandId)>,
    pub(super) resolved_seen: HashSet<(FileId, CommandId)>,
    pub(super) deferred_calls: HashSet<(FileId, CommandId)>,
    pub(super) clock: usize,
}
