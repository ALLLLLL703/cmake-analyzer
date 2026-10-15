// AI-authored action command identities and validated request payloads.
use lspally::server::model::{Cancellation, Uri};
use std::path::PathBuf;
#[derive(Clone, Copy, Debug)]
pub(in crate::server) enum ActionKind {
    Configure,
    ExpandedArguments,
    CancelConfigure,
}
impl ActionKind {
    pub fn command(self) -> &'static str {
        match self {
            Self::Configure => "cmake-analyzer.configure",
            Self::ExpandedArguments => "cmake-analyzer.expandedArguments",
            Self::CancelConfigure => "cmake-analyzer.cancelConfigure",
        }
    }
    pub fn parse(value: &str) -> Option<Self> {
        [
            Self::Configure,
            Self::ExpandedArguments,
            Self::CancelConfigure,
        ]
        .into_iter()
        .find(|kind| kind.command() == value)
    }
}
pub(in crate::server) struct ActionRequest {
    pub kind: ActionKind,
    pub uri: Uri,
    pub root: PathBuf,
    pub line: u64,
    pub cancellation: Cancellation,
}
