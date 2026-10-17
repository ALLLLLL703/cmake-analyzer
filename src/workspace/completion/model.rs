use crate::{
    parser::semanticer::model::CommandId,
    workspace::model::{DirectoryContext, DocumentSnapshot},
};
use std::{path::PathBuf, sync::Arc};
#[derive(Default)]
pub(crate) struct Catalog {
    pub modules: Arc<[String]>,
    pub packages: Arc<[String]>,
    pub targets: Vec<TargetSource>,
}
pub(crate) struct TargetSource {
    pub snapshot: Arc<DocumentSnapshot>,
    pub visible: bool,
}
#[derive(Default)]
pub(crate) struct Request {
    pub command: Option<CommandId>,
    pub modules: bool,
    pub packages: bool,
    pub targets: bool,
}
#[derive(Default)]
pub(crate) struct DiscoveryCache {
    pub module_roots: Vec<PathBuf>,
    pub prefix_roots: Vec<PathBuf>,
    pub modules: Option<Arc<[String]>>,
    pub packages: Option<Arc<[String]>>,
}
pub(super) struct Context {
    pub directory: DirectoryContext,
    pub values: crate::parser::semanticer::model::ValueBindings,
}
pub(super) struct Visit {
    pub file: PathBuf,
    pub directory: DirectoryContext,
    pub visible: bool,
    pub depth: usize,
}
