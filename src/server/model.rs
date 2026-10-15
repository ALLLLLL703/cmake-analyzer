// AI-authored server and service models.
use crate::{
    config::{ConfigSources, ConfigStore, Settings},
    workspace::model::{DocumentSnapshot, Workspace},
};
use lspally::server::model::*;
use std::{
    collections::{BTreeMap, BTreeSet},
    path::PathBuf,
    sync::{Arc, Mutex},
};
use tokio::{
    sync::{mpsc, oneshot},
    task::JoinHandle,
};

#[derive(Clone, Debug)]
pub struct ServerOptions {
    pub roots: Vec<PathBuf>,
    pub start_directory: PathBuf,
    pub config: ConfigSources,
}
impl ServerOptions {
    pub fn new(start_directory: PathBuf) -> Self {
        Self {
            roots: Vec::new(),
            config: ConfigSources::new(start_directory.clone()),
            start_directory,
        }
    }
}
/// Clones share one service; callers can stop it after Wrapper returns on EOF.
#[derive(Clone)]
pub struct CmakeAnalyzer {
    pub(super) options: ServerOptions,
    pub(super) state: Arc<Mutex<ServiceState>>,
}
impl CmakeAnalyzer {
    pub fn new(options: ServerOptions) -> Self {
        Self {
            options,
            state: Arc::new(Mutex::new(ServiceState::default())),
        }
    }
}
#[derive(Default)]
pub(super) struct ServiceState {
    pub roots: Vec<PathBuf>,
    pub hierarchical: bool,
    pub config: Option<ConfigStore>,
    pub service: Option<ServiceHandle>,
}
pub(super) struct ServiceHandle {
    pub sender: mpsc::Sender<ServiceCommand>,
    pub stop: Cancellation,
    pub task: JoinHandle<()>,
}
impl Drop for ServiceHandle {
    fn drop(&mut self) {
        self.stop.cancel();
    }
}
pub(super) enum ServiceCommand {
    RuntimeRefresh,
    Open(TextDocumentItem),
    Change(DidChangeTextDocumentParams),
    Close(Uri),
    Save(DidSaveTextDocumentParams),
    Folders(WorkspaceFoldersChangeEvent),
    ReloadConfig {
        path: Option<PathBuf>,
        reply: oneshot::Sender<ServerResult<usize>>,
    },
    Actions {
        uri: Uri,
        line: u64,
        reply: oneshot::Sender<ServerResult<Option<CodeActionResponse>>>,
    },
    RunAction {
        request: super::action::model::ActionRequest,
        reply: oneshot::Sender<ServerResult<Option<Data>>>,
    },
    Symbols {
        uri: Uri,
        reply: oneshot::Sender<ServerResult<Option<DocumentSymbolResponse>>>,
    },
}
#[derive(Clone)]
pub(super) struct OpenDocument {
    pub uri: Uri,
    pub version: i32,
    pub text: Arc<str>,
}
pub(super) struct Project {
    pub workspace: Workspace,
    pub settings: Arc<Settings>,
    pub bindings: BTreeSet<PathBuf>,
    pub standalone: bool,
    pub dirty: bool,
    pub members: BTreeSet<PathBuf>,
    pub overlays: BTreeMap<PathBuf, i32>,
}
pub(super) struct Service {
    pub config: ConfigStore,
    pub roots: BTreeSet<PathBuf>,
    pub projects: Vec<Project>,
    pub documents: BTreeMap<PathBuf, OpenDocument>,
    pub clears: Vec<Uri>,
    pub published: BTreeMap<PathBuf, ReportKey>,
    pub client: ClientHandle,
    pub encoding: CoordinateEncoding,
    pub hierarchical: bool,
    pub jobs: tokio::task::JoinSet<RuntimeCompletion>,
    pub running: BTreeMap<PathBuf, RunningConfigure>,
    pub run_counter: u64,
}
pub(super) struct RunningConfigure {
    pub run: crate::runtime_cmake::model::RunId,
    pub cancel: Cancellation,
    pub build_directory: PathBuf,
}
pub(super) struct RuntimeCompletion {
    pub root: PathBuf,
    pub run: crate::runtime_cmake::model::RunId,
    pub result: std::io::Result<crate::runtime_cmake::model::Trace>,
}
pub(super) struct ReportKey {
    pub version: i32,
    pub enabled: bool,
    // Weak identity does not retain obsolete snapshots just for publication caching.
    pub snapshot: Option<std::sync::Weak<DocumentSnapshot>>,
    pub extra: Vec<(crate::model::TextSpan, u8, String)>,
    pub contextual: bool,
    pub resolved_calls: Vec<crate::parser::semanticer::model::CommandId>,
}
impl ReportKey {
    pub fn same(&self, other: &Self) -> bool {
        self.version == other.version
            && self.enabled == other.enabled
            && self.extra == other.extra
            && self.contextual == other.contextual
            && self.resolved_calls == other.resolved_calls
            && match (&self.snapshot, &other.snapshot) {
                (Some(a), Some(b)) => a.ptr_eq(b),
                (None, None) => true,
                _ => false,
            }
    }
}
pub(super) struct PreparedReport {
    pub path: PathBuf,
    pub key: Option<ReportKey>,
    pub report: DiagnosticReport,
}
pub(super) struct ReportInput {
    pub path: PathBuf,
    pub uri: Uri,
    pub key: ReportKey,
    pub snapshot: Option<Arc<DocumentSnapshot>>,
}
pub(super) struct SymbolInput {
    pub uri: Uri,
    pub snapshot: Arc<DocumentSnapshot>,
    pub encoding: CoordinateEncoding,
    pub hierarchical: bool,
}
