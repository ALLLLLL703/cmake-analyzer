use crate::{
    parser::semanticer::model::{FileId, ReferenceIndex},
    workspace::model::DocumentSnapshot,
};
use std::{collections::HashMap, sync::Arc};
#[derive(Clone, Copy)]
pub(in crate::server) enum Query {
    Definition,
    References { include_declaration: bool },
}
pub(super) struct Input {
    pub snapshot: Arc<DocumentSnapshot>,
    pub index: Option<Arc<ReferenceIndex>>,
    // File IDs are local to this owner's index.
    pub files: HashMap<FileId, (String, Arc<DocumentSnapshot>)>,
}
