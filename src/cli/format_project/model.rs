use crate::{format::Options, workspace::model::DocumentSnapshot};
use std::{path::PathBuf, sync::Arc};
pub(super) struct Input {
    pub path: PathBuf,
    pub snapshot: Arc<DocumentSnapshot>,
    pub options: Options,
}
pub(super) struct Edit {
    pub path: PathBuf,
    pub before: Arc<str>,
    pub after: String,
}
