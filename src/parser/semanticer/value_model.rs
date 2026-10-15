// AI-authored conservative value facts and sparse command observations.
use super::model::{ReferenceInvocationId, ReferenceLocation, ScopeId};
use std::{collections::BTreeMap, path::PathBuf, sync::Arc};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PossibleValue {
    Static(Arc<str>),
    Observed(Arc<str>),
}
impl PossibleValue {
    pub fn text(&self) -> &str {
        match self {
            Self::Static(value) | Self::Observed(value) => value,
        }
    }
}
#[derive(Clone, Debug)]
pub struct VariableValueAt {
    pub name: String,
    /// Reads use name spans; writes use the raw output-argument span.
    pub location: ReferenceLocation,
    pub scope: Option<ScopeId>,
    pub invocation: Option<ReferenceInvocationId>,
    pub possible_value: Option<PossibleValue>,
}
#[derive(Clone, Debug)]
pub struct ValueDirectories {
    pub source_root: PathBuf,
    pub source: PathBuf,
    pub binary_root: Option<PathBuf>,
    pub binary: Option<PathBuf>,
    pub file: PathBuf,
}
#[derive(Clone, Debug, Default)]
pub struct ValueBindings {
    pub(super) entries: Arc<BTreeMap<String, ValueBinding>>,
    pub(super) epoch: usize,
}
#[derive(Clone, Debug)]
pub(super) struct ValueBinding {
    pub value: Option<PossibleValue>,
    pub epoch: usize,
}
