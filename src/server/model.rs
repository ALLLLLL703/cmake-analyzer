// AI-authored server and service models.
use crate::workspace::model::{DocumentSnapshot, Workspace};
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
}
impl ServerOptions {
    pub fn new(start_directory: PathBuf) -> Self {
        Self {
            roots: Vec::new(),
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
    Open(TextDocumentItem),
    Change(DidChangeTextDocumentParams),
    Close(Uri),
    Save(DidSaveTextDocumentParams),
    Folders(WorkspaceFoldersChangeEvent),
    Symbols {
        uri: Uri,
        reply: oneshot::Sender<ServerResult<Option<DocumentSymbolResponse>>>,
    },
}
pub(super) struct OpenDocument {
    pub uri: Uri,
    pub version: i32,
    pub text: Arc<str>,
}
pub(super) struct Project {
    pub workspace: Workspace,
    pub standalone: bool,
    pub dirty: bool,
    pub members: BTreeSet<PathBuf>,
    pub overlays: BTreeMap<PathBuf, i32>,
}
pub(super) struct Service {
    pub projects: Vec<Project>,
    pub documents: BTreeMap<PathBuf, OpenDocument>,
    pub clears: Vec<Uri>,
    pub published: BTreeMap<PathBuf, ReportKey>,
    pub client: ClientHandle,
    pub encoding: CoordinateEncoding,
    pub hierarchical: bool,
}
pub(super) struct ReportKey {
    pub version: i32,
    // Weak identity does not retain obsolete snapshots just for publication caching.
    pub snapshot: Option<std::sync::Weak<DocumentSnapshot>>,
    pub extra: Vec<(crate::model::TextSpan, String)>,
}
impl ReportKey {
    pub fn same(&self, other: &Self) -> bool {
        self.version == other.version
            && self.extra == other.extra
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
