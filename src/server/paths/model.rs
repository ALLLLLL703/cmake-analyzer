// AI-authored path diagnostic data, independent of protocol envelopes.
use crate::{model::TextSpan, parser::semanticer::model::PathKind};
use std::{collections::HashSet, path::PathBuf};
#[derive(Default)]
pub(in crate::server) struct PathState {
    pub warnings: Vec<PathWarning>,
    pub generated: HashSet<PathBuf>,
    pub build_directories: Vec<PathBuf>,
}
impl PathState {
    pub(in crate::server) fn is_generated(&self, path: &std::path::Path) -> bool {
        self.generated.contains(path)
            || self
                .build_directories
                .iter()
                .any(|base| path.starts_with(base))
    }
}
pub(in crate::server) struct PathWarning {
    pub file: PathBuf,
    pub span: TextSpan,
    pub message: String,
}
pub(super) struct Candidate {
    pub file: PathBuf,
    pub span: TextSpan,
    pub kind: PathKind,
    pub choices: Vec<PathBuf>,
    pub optional: bool,
    pub dependency: bool,
}
