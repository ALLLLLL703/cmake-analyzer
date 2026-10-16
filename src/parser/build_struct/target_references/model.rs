use crate::parser::{
    build_struct::model::{AnalysisKey, TargetDeclaration, TargetKind, TargetReference},
    semanticer::{model::CommandId, schema::NameRole},
};
use std::collections::HashMap;

/// IDs belong to one TargetCollector; they are not Workspace variable-scope IDs.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub(crate) struct TargetDirectoryId(usize);
impl TargetDirectoryId {
    pub(super) fn new(index: usize) -> Self {
        Self(index)
    }
    pub(super) fn index(self) -> usize {
        self.0
    }
}
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub(super) struct TargetEntryId(usize);
impl TargetEntryId {
    pub(super) fn new(index: usize) -> Self {
        Self(index)
    }
    pub(super) fn index(self) -> usize {
        self.0
    }
}
pub(super) struct Entry {
    pub declaration: TargetDeclaration,
    pub directory: TargetDirectoryId,
    pub local: bool,
    pub class: TargetKind,
}
pub(super) struct Use {
    pub name: String,
    pub role: NameRole,
    pub directory: TargetDirectoryId,
    pub reference: TargetReference,
}
#[derive(Default)]
pub(crate) struct TargetCollector {
    pub(super) directories: Vec<Option<TargetDirectoryId>>,
    pub(super) entries: Vec<Entry>,
    pub(super) names: HashMap<String, Vec<TargetEntryId>>,
    pub(super) definitions:
        HashMap<(AnalysisKey, CommandId, Option<TargetDirectoryId>), TargetEntryId>,
    pub(super) uses: Vec<Use>,
}
