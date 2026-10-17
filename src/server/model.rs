//! State and messages shared by server adapters and the single service task.
//! [`CmakeAnalyzer`] clones share lifecycle state, not independent Workspaces.
//! `Service` owns analysis and job bookkeeping after startup; `ServiceCommand`
//! transfers notification payloads or reply channels to that owner. This single
//! message-processing task is called the actor. An overlay is open editor text
//! used instead of disk contents. Constructing these records alone performs no
//! I/O, except that dropping `ServiceHandle` signals stop.
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

/// Startup inputs used by [`CmakeAnalyzer::new`] and initialization.
/// CLI callers can set explicit roots or an override configuration source. Creating
/// options does not inspect disk; discovery happens during initialization.
///
/// # Examples
/// ```
/// use cmake_analyzer::server::ServerOptions;
/// let options = ServerOptions::new(std::path::PathBuf::from("/work/demo"));
/// assert!(options.roots.is_empty());
/// ```
#[derive(Clone, Debug)]
pub struct ServerOptions {
    /// Explicit CMakeLists.txt entries or directories; a nonempty list overrides client root discovery.
    /// Example: `options.roots.push("/work/demo/CMakeLists.txt".into())`.
    pub roots: Vec<PathBuf>,
    /// Startup directory used as the final discovery fallback and configuration context.
    /// Example: `let cwd = &options.start_directory`.
    pub start_directory: PathBuf,
    /// Configuration source locations loaded during initialization.
    /// Example: `options.config.explicit_file = Some("/work/settings.toml".into())`.
    pub config: ConfigSources,
}
impl ServerOptions {
    /// Construct root-discovery options without reading files or starting tasks.
    ///
    /// # Arguments
    /// - `start_directory` `[in]`: Startup directory used for fallback discovery and configuration sources; consumed.
    ///
    /// # Returns
    /// New options with no explicit roots and sources based on the supplied directory.
    ///
    /// # Examples
    /// ```rust
    /// use cmake_analyzer::server::ServerOptions;
    /// let options = ServerOptions::new("/work/demo".into());
    /// assert!(options.roots.is_empty());
    /// ```
    pub fn new(start_directory: PathBuf) -> Self {
        Self {
            roots: Vec::new(),
            config: ConfigSources::new(start_directory.clone()),
            start_directory,
        }
    }
}
/// Cloneable protocol adapter sharing one service and lifecycle mutex.
/// Transport wrappers call its language-server implementation. [`ServerOptions`]
/// selects startup inputs; [`Self::stop`] joins the service after shutdown or EOF.
/// Construction performs no I/O. Clones copy options but share mutable lifecycle
/// state; they do not create independent project owners.
///
/// # Flow
/// ```text
///  [Wrapper]                [EOF cleanup clone]
///      |                            |
///      +-------------+--------------+
///                    v
///            [Shared ServiceState]
///                    |
///                    v
///             [ServiceHandle] -----> [Single Service task]
/// ```
/// # Examples
/// ```
/// use cmake_analyzer::server::{CmakeAnalyzer, ServerOptions};
/// let server = CmakeAnalyzer::new(ServerOptions::new("/work/demo".into()));
/// let cleanup = server.clone();
/// ```
#[derive(Clone)]
pub struct CmakeAnalyzer {
    /// Copied startup inputs; cloning an adapter does not update another clone's options.
    /// Example: `let start = &server.options.start_directory`.
    pub(super) options: ServerOptions,
    /// Shared short-held lifecycle mutex; project state lives in the service task after startup.
    /// Example: `let guard = server.state.lock().unwrap()`.
    pub(super) state: Arc<Mutex<ServiceState>>,
}
impl CmakeAnalyzer {
    /// Construct an adapter with fresh, uninitialized shared lifecycle state.
    /// No files are read and no service task is spawned.
    ///
    /// # Arguments
    /// - `options` `[in]`: consumed startup inputs retained by the adapter.
    ///
    /// # Returns
    /// A new adapter; initialization and the initialized notification must precede queries.
    ///
    /// # Examples
    /// ```
    /// use cmake_analyzer::server::{CmakeAnalyzer, ServerOptions};
    /// let server = CmakeAnalyzer::new(ServerOptions::new("/work/demo".into()));
    /// ```
    pub fn new(options: ServerOptions) -> Self {
        Self {
            options,
            state: Arc::new(Mutex::new(ServiceState::default())),
        }
    }
}
/// Lifecycle handoff between initialization and service startup.
/// [`CmakeAnalyzer`] protects this with a short-held mutex. [`ServiceHandle`] is
/// installed at startup; roots and configuration are moved into [`Service`].
/// Capability booleans remain available to request adapters. No I/O occurs here.
///
/// # Flow
/// ```text
///  [initialize] ----> [roots + config + client flags]
///                                  |
///                               [start]
///                              /       \
///                             v         v
///                      [Actor owns    [Handle stored;
///                       roots/config]  flags retained]
///                                           |
///                                        [stop]
///                                           |
///                                 [Take handle + cancel/join]
/// ```
///
/// # Examples
/// ```text
/// let state = ServiceState::default();
/// assert!(state.service.is_none());
/// ```
#[derive(Default)]
pub(super) struct ServiceState {
    /// Whether the client advertised completion-item snippet support; defaults false.
    /// Example: `state.completion_snippets = true`.
    pub completion_snippets: bool,
    /// Whether the client's first hover format preference is Markdown, not merely whether Markdown appears anywhere.
    /// Example: `state.hover_markdown = true`.
    pub hover_markdown: bool,
    /// Sorted, deduplicated entry paths prepared by initialize and drained by start.
    /// Example: `let entries = std::mem::take(&mut state.roots)`.
    pub roots: Vec<PathBuf>,
    /// Client support for nested document symbols; defaults false for flat compatibility.
    /// Example: `state.hierarchical = true`.
    pub hierarchical: bool,
    /// Loaded configuration awaiting transfer to the actor; None before initialization or after startup.
    /// Example: `let config = state.config.take()`.
    pub config: Option<ConfigStore>,
    /// Active queue/task handle, installed once on start and removed on stop.
    /// Example: `assert!(state.service.is_none())`.
    pub service: Option<ServiceHandle>,
}
/// Queue endpoint, stop signal and join handle for one [`Service`].
/// [`CmakeAnalyzer::stop`] takes this record out of [`ServiceState`] to await exit.
/// Dropping it signals cancellation but does not itself await the task.
///
/// # Examples
/// ```text
/// handle.stop.cancel();
/// // stop() also awaits handle.task before returning.
/// ```
pub(super) struct ServiceHandle {
    /// Bounded message sender cloned by request/notification adapters.
    /// Example: `handle.sender.send(ServiceCommand::RuntimeRefresh).await?`.
    pub sender: mpsc::Sender<ServiceCommand>,
    /// Service-wide cooperative exit signal; Drop cancels it.
    /// Example: `handle.stop.cancel()`.
    pub stop: Cancellation,
    /// Actor join handle; awaiting it also waits for tracked runtime jobs to be stopped.
    /// Example: `(&mut handle.task).await?`.
    pub task: JoinHandle<()>,
}
impl Drop for ServiceHandle {
    /// Signal service cancellation when its lifecycle handle is released. Does not join the task.
    ///
    /// # Arguments
    /// - `self` `[in,out]`: Handle being dropped; its shared stop signal is set. No project state is synchronously changed.
    ///
    /// # Returns
    /// `()` after signalling cancellation; task completion is asynchronous.
    ///
    /// # Examples
    /// ```text
    /// drop(handle); // The service observes its stop signal.
    /// ```
    fn drop(&mut self) {
        self.stop.cancel();
    }
}
/// Owned messages processed in receive order by the service actor.
/// Generated notification adapters enqueue edits; query adapters attach a one-use
/// reply sender. [`Service`] applies edit batches, flushes analysis, then answers the
/// next query before processing later messages. Constructing a message has no I/O.
/// Closed reply receivers allow queued queries to be skipped; cancellation carried
/// by a query is cooperative, not a rollback of earlier edits.
///
/// # Flow
/// ```text
///  [Notification]            [Query + reply channel]
///        |                           |
///        +-------------+-------------+
///                      v
///               [Bounded FIFO queue]
///                      |
///              +-------+-------+
///              v               v
///        [Apply edits]     [Query barrier]
///              |               |
///              +----> [Flush] -+
///                              v
///                        [Reply once]
/// ```
/// # Examples
/// ```text
/// service.apply(ServiceCommand::Open(document)).await?;
/// let command = ServiceCommand::Symbols { uri, reply };
/// ```
pub(super) enum ServiceCommand {
    /// Revisit analysis/report scheduling after a runtime completion; no direct edit payload. Example: `ServiceCommand::RuntimeRefresh`.
    RuntimeRefresh,
    /// Admit an editor buffer and discover owners before installing overlays. Example: `ServiceCommand::Open(document)`.
    Open(
        /// Initial URI, version and editor text to admit.
        /// Example: `ServiceCommand::Open(document)`.
        TextDocumentItem,
    ),
    /// Apply strictly newer sequential editor changes and invalidate affected runtime data. Example: `ServiceCommand::Change(params)`.
    Change(
        /// New version and ordered content changes for an already open file.
        /// Example: `ServiceCommand::Change(params)`.
        DidChangeTextDocumentParams,
    ),
    /// Release an open buffer and queue diagnostic clearing when needed. Example: `ServiceCommand::Close(uri)`.
    Close(
        /// Original local file URI of the buffer to release.
        /// Example: `ServiceCommand::Close(uri)`.
        Uri,
    ),
    /// Invalidate disk caches and coalesce authorized on-save runs; ignores optional save text. Example: `ServiceCommand::Save(params)`.
    Save(
        /// Saved document identity; optional text is not authoritative.
        /// Example: `ServiceCommand::Save(params)`.
        DidSaveTextDocumentParams,
    ),
    /// Remove/add discovered root projects, invalidate runtime and reassign open buffers. Example: `ServiceCommand::Folders(event)`.
    Folders(
        /// Removed and added client workspace folders.
        /// Example: `ServiceCommand::Folders(event)`.
        WorkspaceFoldersChangeEvent,
    ),
    /// Barrier rebuilding configuration/projects transactionally before replying with the project count. Example: `ServiceCommand::ReloadConfig { path: None, reply }`.
    ReloadConfig {
        /// Optional replacement explicit configuration path; None keeps existing sources.
        /// Example: `ServiceCommand::ReloadConfig { path: None, .. }` in a match inspects this payload.
        path: Option<PathBuf>,
        /// One-use reply with rebuilt project count or reload error; closed receivers let the actor skip this barrier.
        /// Example: `ServiceCommand::ReloadConfig { reply, .. }` in a match inspects this payload.
        reply: oneshot::Sender<ServerResult<usize>>,
    },
    /// Barrier listing manual actions for a document line using current owners. Example: `ServiceCommand::Actions { uri, line: 0, reply }`.
    Actions {
        /// Target document URI, decoded to a normalized local path by the handler.
        /// Example: `ServiceCommand::Actions { uri, .. }` in a match inspects this payload.
        uri: Uri,
        /// Zero-based line used when selecting actions.
        /// Example: `ServiceCommand::Actions { line: 0, .. }` in a match inspects this payload.
        line: u64,
        /// One-use reply with the available code-action list (possibly empty), or an ownership/URI error.
        /// Example: `ServiceCommand::Actions { reply, .. }` in a match inspects this payload.
        reply: oneshot::Sender<ServerResult<Option<CodeActionResponse>>>,
    },
    /// Barrier validating and dispatching a selected runtime/view/cancel action. Example: `ServiceCommand::RunAction { request, reply }`.
    RunAction {
        /// Decoded action data and cancellation; the actor revalidates project ownership.
        /// Example: `ServiceCommand::RunAction { request, .. }` in a match inspects this payload.
        request: super::action::model::ActionRequest,
        /// One-use reply with launch metadata, expanded-argument data or cancellation status, or an action error.
        /// Example: `ServiceCommand::RunAction { reply, .. }` in a match inspects this payload.
        reply: oneshot::Sender<ServerResult<Option<Data>>>,
    },
    /// Barrier rendering whole-document edits without applying them. Example: `ServiceCommand::Format { uri, options: None, save: true, cancel, reply }`.
    Format {
        /// Target document URI, decoded to a normalized local path by the handler.
        /// Example: `ServiceCommand::Format { uri, .. }` in a match inspects this payload.
        uri: Uri,
        /// Client indentation overrides for manual formatting; None uses configured style.
        /// Example: `ServiceCommand::Format { options: None, .. }` in a match inspects this payload.
        options: Option<FormattingOptions>,
        /// Whether save-format opt-out rules apply.
        /// Example: `ServiceCommand::Format { save: true, .. }` in a match inspects this payload.
        save: bool,
        /// Cooperative request cancellation checked by the query implementation.
        /// Example: `ServiceCommand::Format { cancel, .. }` in a match binds this signal.
        cancel: Cancellation,
        /// One-use reply with client-applied edits (possibly empty) or formatting/cancellation errors.
        /// Example: `ServiceCommand::Format { reply, .. }` in a match inspects this payload.
        reply: oneshot::Sender<ServerResult<Option<Vec<TextEdit>>>>,
    },
    /// Barrier computing completion candidates at a protocol position. Example: `ServiceCommand::Complete { uri, position, snippets: false, cancel, reply }`.
    Complete {
        /// Target document URI, decoded to a normalized local path by the handler.
        /// Example: `ServiceCommand::Complete { uri, .. }` in a match inspects this payload.
        uri: Uri,
        /// Zero-based position in the negotiated encoding.
        /// Example: `ServiceCommand::Complete { position, .. }` in a match inspects this payload.
        position: Position,
        /// Client snippet support, further constrained by owner completion settings.
        /// Example: `ServiceCommand::Complete { snippets: false, .. }` in a match inspects this payload.
        snippets: bool,
        /// Cooperative request cancellation checked by the query implementation.
        /// Example: `ServiceCommand::Complete { cancel, .. }` in a match binds this signal.
        cancel: Cancellation,
        /// One-use reply with completion candidates, None when unavailable/disabled, or a query error.
        /// Example: `ServiceCommand::Complete { reply, .. }` in a match inspects this payload.
        reply: oneshot::Sender<ServerResult<Option<CompletionResponse>>>,
    },
    /// Barrier resolving definition/reference locations in selected owners. Example: `ServiceCommand::Navigate { uri, position, query, cancel, reply }`.
    Navigate {
        /// Target document URI, decoded to a normalized local path by the handler.
        /// Example: `ServiceCommand::Navigate { uri, .. }` in a match inspects this payload.
        uri: Uri,
        /// Zero-based position in the negotiated encoding.
        /// Example: `ServiceCommand::Navigate { position, .. }` in a match inspects this payload.
        position: Position,
        /// Definition or references selection, including declaration policy where applicable.
        /// Example: `ServiceCommand::Navigate { query, .. }` in a match inspects this payload.
        query: super::navigation::model::Query,
        /// Cooperative request cancellation checked by the query implementation.
        /// Example: `ServiceCommand::Navigate { cancel, .. }` in a match binds this signal.
        cancel: Cancellation,
        /// One-use reply with resolved locations (possibly empty), or a navigation/cancellation error.
        /// Example: `ServiceCommand::Navigate { reply, .. }` in a match inspects this payload.
        reply: oneshot::Sender<ServerResult<Vec<Location>>>,
    },
    /// Barrier building variable/command hover text and optional local native help. Example: `ServiceCommand::Hover { cancel, uri, position, reply }`.
    Hover {
        /// Cooperative request cancellation checked by the query implementation.
        /// Example: `ServiceCommand::Hover { cancel, .. }` in a match binds this signal.
        cancel: Cancellation,
        /// Target document URI, decoded to a normalized local path by the handler.
        /// Example: `ServiceCommand::Hover { uri, .. }` in a match inspects this payload.
        uri: Uri,
        /// Zero-based position in the negotiated encoding.
        /// Example: `ServiceCommand::Hover { position, .. }` in a match inspects this payload.
        position: Position,
        /// One-use reply with hover contents, None when no hover is available, or a query/cancellation error.
        /// Example: `ServiceCommand::Hover { reply, .. }` in a match inspects this payload.
        reply: oneshot::Sender<ServerResult<Option<Hover>>>,
    },
    /// Barrier aggregating symbols across cached participating projects. Example: `ServiceCommand::WorkspaceSymbols { cancel, reply }`.
    WorkspaceSymbols {
        /// Cooperative request cancellation checked by the query implementation.
        /// Example: `ServiceCommand::WorkspaceSymbols { cancel, .. }` in a match binds this signal.
        cancel: Cancellation,
        /// One-use reply with flat workspace symbols (possibly empty), or a collection/cancellation error.
        /// Example: `ServiceCommand::WorkspaceSymbols { reply, .. }` in a match inspects this payload.
        reply: oneshot::Sender<ServerResult<Option<WorkspaceSymbolResponse>>>,
    },
    /// Barrier ensuring ownership and returning one document's symbols. Example: `ServiceCommand::Symbols { uri, reply }`.
    Symbols {
        /// Target document URI, decoded to a normalized local path by the handler.
        /// Example: `ServiceCommand::Symbols { uri, .. }` in a match inspects this payload.
        uri: Uri,
        /// One-use reply with document symbols, None when the first owner disables them, or a query error.
        /// Example: `ServiceCommand::Symbols { reply, .. }` in a match inspects this payload.
        reply: oneshot::Sender<ServerResult<Option<DocumentSymbolResponse>>>,
    },
}
/// Authoritative editor buffer shared with project overlays by [`Service`].
/// [`Project::overlays`] tracks which version has been copied into each Workspace.
/// Cloning shares text bytes and preserves client URI spelling. No disk writes occur.
///
/// # Examples
/// ```text
/// let open = OpenDocument { uri, version: 1, text: "project(demo)".into() };
/// ```
#[derive(Clone)]
pub(super) struct OpenDocument {
    /// Original client URI spelling reused in diagnostics and navigation/symbol replies.
    /// Example: `let uri = &open.uri`.
    pub uri: Uri,
    /// Last accepted editor version; changes must strictly increase it.
    /// Example: `open.version = 2`.
    pub version: i32,
    /// Authoritative editor contents, shared with all owning Workspace overlays.
    /// Example: `let text = open.text.clone()`.
    pub text: Arc<str>,
}
/// One analysis Workspace and its ownership/execution profile.
/// [`Service`] can keep several profiles for the same entry file when binary directory,
/// standalone mode or execution mode differ. [`OpenDocument`] overlays feed every
/// selected owner; settings are shared immutable data until a profile override.
/// Constructing the record alone has no side effects.
///
/// # Flow
/// ```text
/// [Explicit bindings]  [Dependency members]  [Directory fallback]
///          |                    |                    |
///          +--------------------+--------------------+
///                               v
///                           [Project]
///                               |
///              +----------------+----------------+
///              |                |                |
///              v                v                v
///         [Workspace]       [Settings]       [Execution]
/// ```
/// # Examples
/// ```text
/// let owners = service.owners(&path);
/// let project = &service.projects[owners[0]];
/// assert!(project.members.contains(&path) || project.bindings.contains(&path)
///         || path.starts_with(project.workspace.root().file.parent().unwrap()));
/// ```
pub(super) struct Project {
    /// Cached missing-input warnings and generated/build-path exclusions from the last path refresh.
    /// Example: `let warnings = &project.path_state.warnings`.
    pub path_state: super::paths::model::PathState,
    /// Whether input-path checks must be refreshed after analysis/runtime invalidation.
    /// Example: `project.paths_dirty = true`.
    pub paths_dirty: bool,
    /// Independent parser caches, root graph and optional runtime layer for this profile.
    /// Example: `let entry = &project.workspace.root().file`.
    pub workspace: Workspace,
    /// Directory-layered effective configuration; script runtime settings may come from a selecting startup mapping.
    /// Example: `let enabled = project.settings.diagnostics.enable`.
    pub settings: Arc<Settings>,
    /// Files explicitly assigned to this execution profile by interpreter mappings; strongest ownership priority.
    /// Example: `project.bindings.insert(path.clone())`.
    pub bindings: BTreeSet<PathBuf>,
    /// File-only fallback: flush documents without traversing a project graph and never infer execution.
    /// Example: `if project.standalone { /* file-local analysis */ }`.
    pub standalone: bool,
    /// Optional configure/script identity; unmapped standalone files have no executable profile.
    /// Example: `if let Some(mode) = &project.execution { /* action may be offered */ }`.
    pub execution: Option<crate::runtime_cmake::model::ExecutionMode>,
    /// Whether Workspace analysis needs flushing after overlay or disk changes; cleared before a flush attempt.
    /// Example: `project.dirty = true`.
    pub dirty: bool,
    /// Known dependency visit/candidate paths plus observed runtime documents; includes missing dependency candidates.
    /// Example: `let imports = project.members.contains(&path)`.
    pub members: BTreeSet<PathBuf>,
    /// Editor versions currently installed in this Workspace, not all open documents or disk revisions.
    /// Example: `assert_eq!(project.overlays.get(&path), Some(&2))`.
    pub overlays: BTreeMap<PathBuf, i32>,
}
/// Single-task owner of mutable projects, open buffers and runtime jobs.
/// The runtime loop creates this from initialization inputs, then processes
/// [`ServiceCommand`] messages. [`Project`] holds per-profile analysis, while
/// [`ReportKey`] caches successful publications. Fields are not independent locks:
/// only the actor mutates them. Methods may read disk, notify the client or launch
/// explicitly authorized CMake jobs; constructing this record does none of those.
///
/// # Flow
/// ```text
///  [ServiceCommand queue]           [Background job completions]
///            |                                 |
///            +----------------+----------------+
///                             v
///                         [Service]
///                             |
///                +------------+---------------+
///                |            |               |
///                v            v               v
///           [Documents]   [Projects]     [Running IDs]
///                |            |               |
///                +------------+---------------+
///                             v
///                      [Prepared reports] ----> [Client]
/// ```
/// # Examples
/// ```text
/// service.apply(ServiceCommand::Open(document)).await?;
/// service.flush().await?;
/// let reports = service.reports().await?;
/// ```
pub(super) struct Service {
    /// Lazy native CMake help/catalogue cache using startup directory and global inspection budgets; replaced on reload.
    /// Example: `let help = &mut service.local_help`.
    pub local_help: super::hovers::LocalHelp,
    /// Negotiated hover presentation preference copied from initialization.
    /// Example: `let markdown = service.hover_markdown`.
    pub hover_markdown: bool,
    /// Live layered configuration store and process-wide admission limits.
    /// Example: `let limit = service.config.limits.max_workspaces`.
    pub config: ConfigStore,
    /// Registered entry paths used to rebuild ordinary projects on reload; not every mapped or standalone project.
    /// Example: `service.roots.insert(entry.clone())`.
    pub roots: BTreeSet<PathBuf>,
    /// Ordered independent profiles; owner indices are temporary positions and can change after pruning.
    /// Example: `let project = &service.projects[owner]`.
    pub projects: Vec<Project>,
    /// All currently open editor buffers keyed by normalized absolute path.
    /// Example: `let open = service.documents.get(&path)`.
    pub documents: BTreeMap<PathBuf, OpenDocument>,
    /// Client URIs waiting for empty close diagnostics; retained until an uninterrupted idle publication pass.
    /// Example: `service.clears.push(uri)`.
    pub clears: Vec<Uri>,
    /// Keys of reports successfully sent to the client, used to suppress unchanged diagnostics.
    /// Example: `service.published.remove(&path)`.
    pub published: BTreeMap<PathBuf, ReportKey>,
    /// Transport-owned outgoing handle for diagnostics, logs and action messages.
    /// Example: `service.client.publish_diagnostics(report).await?`.
    pub client: ClientHandle,
    /// Negotiated coordinate encoding used for every protocol position/range conversion.
    /// Example: `let encoding = service.encoding`.
    pub encoding: CoordinateEncoding,
    /// Whether document symbols should be nested; workspace symbols remain flat.
    /// Example: `let nested = service.hierarchical`.
    pub hierarchical: bool,
    /// Join set of outstanding or completed-unreaped runtime tasks; it can exceed the live-marker count.
    /// Example: `let count = service.jobs.len()`.
    pub jobs: tokio::task::JoinSet<RuntimeCompletion>,
    /// Entry-keyed accepted job markers; removing one makes its later completion stale.
    /// Example: `service.running.remove(&root)`.
    pub running: BTreeMap<PathBuf, RunningConfigure>,
    /// Monotonically checked run-ID clock, retained across reload; overflow rejects a launch.
    /// Example: `let last = service.run_counter`.
    pub run_counter: u64,
    /// Latest pending authorized save per entry path; busy jobs defer draining and profile identity is rechecked.
    /// Example: `service.saved_runs.clear()`.
    pub saved_runs: BTreeMap<PathBuf, SavedRun>,
}
/// Latest save-triggered runtime request for an entry in [`Service::saved_runs`].
/// Scheduling revalidates the initiating file and [`Project::execution`]. Creating
/// this pending record does not launch a process.
///
/// # Examples
/// ```text
/// let saved = SavedRun { file: path, mode: ExecutionMode::Configure };
/// ```
pub(super) struct SavedRun {
    /// Open file whose save requested execution; closing it removes this pending request.
    /// Example: `let initiating_file = &saved.file`.
    pub file: PathBuf,
    /// Requested execution identity checked again against the selected owner before launch.
    /// Example: `let mode = saved.mode.clone()`.
    pub mode: crate::runtime_cmake::model::ExecutionMode,
}
/// Live acceptance marker for a background job in [`Service::running`].
/// The actual task remains in [`Service::jobs`] after this marker is removed.
/// [`RuntimeCompletion`] must match the run ID to be installed. Construction has no
/// side effects; the stored cancellation signal can stop the associated process.
///
/// # Examples
/// ```text
/// if let Some(job) = service.running.remove(&root) { job.cancel.cancel(); }
/// ```
pub(super) struct RunningConfigure {
    /// Execution identity retained for action cancellation/profile checks.
    /// Example: `let mode = &job.mode`.
    pub mode: crate::runtime_cmake::model::ExecutionMode,
    /// Unique acceptance ID compared to a completion before installing results.
    /// Example: `assert_eq!(job.run, completion.run)`.
    pub run: crate::runtime_cmake::model::RunId,
    /// Process cancellation signal, separate from request cancellation after launch.
    /// Example: `job.cancel.cancel()`.
    pub cancel: Cancellation,
    /// Reserved configure output directory; scripts use None and reserve no build directory.
    /// Example: `if let Some(build) = &job.build_directory { /* reject concurrent reuse */ }`.
    pub build_directory: Option<PathBuf>,
}
/// Owned output of a joined CMake task, checked against [`RunningConfigure`].
/// The actor rejects stale IDs and profiles before installing a trace into a
/// [`Project`]. Carrying this result does not itself mutate analysis or publish.
///
/// # Examples
/// ```text
/// service.configure_finished(Ok(completion)).await;
/// ```
pub(super) struct RuntimeCompletion {
    /// Execution identity used to locate the still-matching project profile.
    /// Example: `let mode = &completion.mode`.
    pub mode: crate::runtime_cmake::model::ExecutionMode,
    /// Analysis entry key used to match the running marker.
    /// Example: `let active = service.running.get(&completion.root)`.
    pub root: PathBuf,
    /// Launch ID that distinguishes stale work from a later run at the same entry.
    /// Example: `let current = active.is_some_and(|job| job.run == completion.run)`.
    pub run: crate::runtime_cmake::model::RunId,
    /// Trace on successful execution or I/O/process/trace failure; installation still checks saved sources.
    /// Example: `let trace = completion.result.map_err(internal)?`.
    pub result: std::io::Result<crate::runtime_cmake::model::Trace>,
}
/// Inputs that determine whether open-file diagnostics need republishing.
/// [`Service::published`] retains these only after successful client publication.
/// [`Self::same`] compares weak snapshot identity, not snapshot contents, so old
/// snapshots are not kept alive by the publication cache. No I/O occurs here.
///
/// # Flow
/// ```text
///  [Previous successful publication]    [Current captured inputs]
///                |                              |
///                +---------------+--------------+
///                                v
///                       [same: fields + weak identity]
///                                |
///                         <All equal?>
///                          /        \
///                        yes        no
///                         |          |
///                       [Skip]   [Convert + send]
///                                    |
///                           [Replace cached key on success]
/// ```
///
/// # Examples
/// ```text
/// if service.published.get(&path).is_some_and(|old| old.same(&key)) {
///     // No diagnostic conversion is needed for these inputs.
/// }
/// ```
pub(super) struct ReportKey {
    /// Strongest configured diagnostic levels among enabled owners.
    /// Example: `let level = key.rules.unknown_function`.
    pub rules: crate::config::DiagnosticRules,
    /// Editor version attached to the outgoing report, including empty disabled-feature reports.
    /// Example: `let version = key.version`.
    pub version: i32,
    /// Whether at least one owner enables diagnostics; false converts to an empty report.
    /// Example: `if !key.enabled { /* clear old diagnostics */ }`.
    pub enabled: bool,
    // Weak identity does not retain obsolete snapshots just for publication caching.
    /// Weak identity of the first enabled owner snapshot, or None if analysis is unavailable.
    /// Example: `let retained = key.snapshot.as_ref().and_then(std::sync::Weak::upgrade)`.
    pub snapshot: Option<std::sync::Weak<DocumentSnapshot>>,
    /// Owner-dependent byte spans, LSP severity numbers and messages: path, contextual variable and dependency findings.
    /// Example: `key.extra.push((span, 2, "Missing input".into()))`.
    pub extra: Vec<(crate::model::TextSpan, u8, String)>,
    /// Whether a root-graph visit supplies variable warnings, suppressing file-local variable warnings.
    /// Example: `if key.contextual { /* use graph warnings */ }`.
    pub contextual: bool,
    /// Sorted unique snapshot-local command IDs resolved by contextual/static or runtime observations.
    /// Example: `let calls = &key.resolved_calls`.
    pub resolved_calls: Vec<crate::parser::semanticer::model::CommandId>,
}
impl ReportKey {
    /// Compare all diagnostic inputs and weak snapshot pointer identity. No I/O or mutation.
    ///
    /// # Arguments
    /// - `self` `[in]`: Previously published or candidate key.
    /// - `other` `[in]`: Key to compare, including ordered extra findings and resolved-call IDs.
    ///
    /// # Returns
    /// `true` when every field matches, including two absent snapshots or equal weak pointers; `false` otherwise.
    ///
    /// # Examples
    /// ```text
    /// if previous.same(&candidate) { /* skip conversion and publication */ }
    /// ```
    pub fn same(&self, other: &Self) -> bool {
        self.rules == other.rules
            && self.version == other.version
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
/// Converted push report awaiting the runtime loop's freshness check.
/// [`ReportInput`] becomes this record on a blocking worker. A missing [`Self::key`]
/// means a close-clear report rather than an ordinary versioned publication. Creating
/// or converting the record does not send it to the client.
///
/// # Examples
/// ```text
/// client.publish_diagnostics(prepared.report).await?;
/// ```
pub(super) struct PreparedReport {
    /// Normalized path whose publication cache is updated after a successful send.
    /// Example: `service.published.remove(&prepared.path)`.
    pub path: PathBuf,
    /// Some for a versioned open-file report; None removes the cache entry after a close-clear.
    /// Example: `if let Some(key) = prepared.key { service.published.insert(prepared.path, key); }`.
    pub key: Option<ReportKey>,
    /// Fully converted client payload; preparing it does not publish.
    /// Example: `client.publish_diagnostics(prepared.report).await?`.
    pub report: DiagnosticReport,
}
/// Immutable diagnostic inputs moved into the conversion worker.
/// [`ReportKey`] captures owner-dependent policy; [`PreparedReport`] is the output.
/// The strong snapshot here keeps analysis alive only while conversion needs it.
/// No I/O occurs when constructing this record.
///
/// # Examples
/// ```text
/// let input = ReportInput { path, uri, key, snapshot };
/// ```
pub(super) struct ReportInput {
    /// Normalized document identity carried through to publication bookkeeping.
    /// Example: `let path = &input.path`.
    pub path: PathBuf,
    /// Client URI spelling used in the converted report.
    /// Example: `let uri = &input.uri`.
    pub uri: Uri,
    /// Complete policy/version/identity inputs tested against the last successful publication.
    /// Example: `let enabled = input.key.enabled`.
    pub key: ReportKey,
    /// Strong snapshot retained while conversion runs; None yields an unavailable-analysis diagnostic when enabled.
    /// Example: `if let Some(snapshot) = &input.snapshot { let text = &snapshot.text; }`.
    pub snapshot: Option<Arc<DocumentSnapshot>>,
}
/// Snapshot and presentation policy for document/workspace symbol conversion.
/// The symbol converter reads this without mutating analysis. Workspace queries set
/// [`Self::hierarchical`] false; restricted external visits set [`Self::functions_only`].
/// [`OpenDocument`] supplies the original client URI when available. No I/O occurs.
///
/// # Examples
/// ```text
/// let input = SymbolInput { functions_only: false, uri, snapshot, encoding,
///                           hierarchical: true };
/// ```
pub(super) struct SymbolInput {
    /// Restrict output to top-level defined functions/macros for limited external indexing; forces flat output.
    /// Example: `input.functions_only = true`.
    pub functions_only: bool,
    /// Document location URI for flat symbols; nested symbols carry ranges only.
    /// Example: `let uri = &input.uri`.
    pub uri: Uri,
    /// Immutable lexical/semantic/build data; symbol conversion reads semantic commands and blocks.
    /// Example: `let commands = &input.snapshot.semantics.commands`.
    pub snapshot: Arc<DocumentSnapshot>,
    /// Coordinate units used when converting byte spans to protocol ranges.
    /// Example: `let encoding = input.encoding`.
    pub encoding: CoordinateEncoding,
    /// Return Nested when true unless functions_only; false flattens parent names into container_name.
    /// Example: `input.hierarchical = false`.
    pub hierarchical: bool,
}
