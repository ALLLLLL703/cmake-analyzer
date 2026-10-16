use crate::config::Runtime;
use std::{collections::BTreeMap, path::PathBuf, sync::Arc};
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(in crate::server::hovers) enum Kind {
    Command,
    Variable,
}
impl Kind {
    pub(super) fn list_flag(self) -> &'static str {
        match self {
            Self::Command => "--help-command-list",
            Self::Variable => "--help-variable-list",
        }
    }
    pub(super) fn help_flag(self) -> &'static str {
        match self {
            Self::Command => "--help-command",
            Self::Variable => "--help-variable",
        }
    }
}
pub(in crate::server) struct LocalHelp {
    pub(super) cwd: PathBuf,
    pub(super) limits: Runtime,
    pub(super) executable: tokio::sync::OnceCell<Option<PathBuf>>,
    pub(super) catalogs: BTreeMap<Kind, Option<Arc<Vec<String>>>>,
    pub(super) documents: BTreeMap<(Kind, String), Option<Arc<str>>>,
    pub(super) cached_bytes: usize,
}
impl LocalHelp {
    pub(in crate::server) fn new(cwd: PathBuf, limits: Runtime) -> Self {
        Self {
            cwd,
            limits,
            executable: tokio::sync::OnceCell::new(),
            catalogs: BTreeMap::new(),
            documents: BTreeMap::new(),
            cached_bytes: 0,
        }
    }
}
