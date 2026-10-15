// AI-authored runtime layer models; event order and frames retain repeated executions.
use super::super::model::DocumentSnapshot;
use crate::{
    parser::semanticer::model::{FunctionKind, PossibleValue, ReferenceLocation},
    runtime_cmake::model::{EventId, RuntimeFrame, Trace},
};
use std::{collections::BTreeMap, path::PathBuf, sync::Arc};
#[repr(transparent)]
#[derive(Clone, Copy)]
pub(super) struct RuntimeFunctionId(usize);
impl RuntimeFunctionId {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn index(self) -> usize {
        self.0
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ValueNamespace {
    Unknown,
    Ordinary,
    Cache,
    ParentScope,
}
#[derive(Clone, Debug)]
pub struct RuntimeValueAt {
    pub event: EventId,
    pub frame: RuntimeFrame,
    pub name: String,
    /// Source command span for assignments, or a name span for reads.
    pub location: Option<ReferenceLocation>,
    pub namespace: ValueNamespace,
    pub possible_value: Option<PossibleValue>,
}
#[derive(Clone, Debug)]
pub struct RuntimeFunction {
    pub name: String,
    pub kind: FunctionKind,
    pub declaration: EventId,
    pub calls: Vec<EventId>,
}
#[derive(Clone, Debug)]
pub struct RuntimeLayer {
    /// EventId indexes this map; ambiguous/missing source commands remain None.
    pub source_commands: Vec<Option<crate::parser::semanticer::model::CommandId>>,
    pub trace: Trace,
    pub documents: BTreeMap<PathBuf, Arc<DocumentSnapshot>>,
    pub values: Vec<RuntimeValueAt>,
    pub functions: Vec<RuntimeFunction>,
    pub event_values: Vec<std::ops::Range<usize>>,
    pub resolved_calls: BTreeMap<PathBuf, Vec<crate::parser::semanticer::model::CommandId>>,
}
