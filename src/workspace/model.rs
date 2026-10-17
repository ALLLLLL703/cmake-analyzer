//! Workspace state, immutable snapshots, dependency visits, scope contexts and resource limits.
//! Creating model records does not read files; [`Workspace::flush`] performs deferred analysis.
// Contains AI-generated documentation additions.
// AI-generated implementation.
// review over
use crate::parser::{
    build_struct::model::{BuildStruct, DependencyKind, SourceRef},
    semanticer::model::SemanticedBuffer,
};
use std::{
    collections::{BTreeSet, HashMap, HashSet},
    path::PathBuf,
    sync::Arc,
};

#[derive(Clone, Debug)]
/// Resource ceilings for one workspace's parsing and traversal. Construction has no side effects.
///
/// [`Workspace::with_limits`] installs these limits; supplemental completion and runtime loading
/// reuse them.
///
/// # Examples
/// ```
/// use cmake_analyzer::workspace::model::WorkspaceLimits;
/// let mut limits = WorkspaceLimits::default();
/// limits.files = 128;
/// ```
pub struct WorkspaceLimits {
    /// Maximum simultaneous bounded analysis workers; zero is clamped to one at construction
    /// Example: `limits.workers = 2` .
    pub workers: usize,
    /// Maximum distinct loaded sources in a dependency pass or runtime source set Example:
    /// `limits.files = 256` .
    pub files: usize,
    /// Maximum dependency depth, with root at zero; deeper entries are skipped Example:
    /// `limits.depth = 64` .
    pub depth: usize,
    /// Maximum file-entry attempts, including repeated visits and failures Example:
    /// `limits.invocations = 1024` .
    pub invocations: usize,
}
impl Default for WorkspaceLimits {
    /// Choose default per-workspace resource ceilings without allocating workers or reading files.
    ///
    /// # Arguments
    /// None.
    ///
    /// # Returns
    /// Limits of 2 workers, 256 distinct sources, depth 64 and 1024 file-entry attempts.
    ///
    /// # Examples
    /// ```
    /// use cmake_analyzer::workspace::model::WorkspaceLimits;
    /// let limits = WorkspaceLimits::default();
    /// assert_eq!(limits.workers, 2);
    /// ```
    fn default() -> Self {
        Self {
            workers: 2,
            files: 256,
            depth: 64,
            invocations: 1024,
        }
    }
}
#[derive(Debug)]
/// Saved file results, shared by readers and each visit to the file.
/// Immutable text and shared analysis outputs for one file revision. No I/O on access.
///
/// [`Workspace::snapshot`] returns current results; [`Invocation::snapshot`] retains the result
/// used by each visit.
///
/// # Flow
/// ```text
/// [revision N]             [revision N+1]
///      |                          |
///  [snapshot Arc]             [new snapshot Arc]
///      |                          |
///  [lexed/semantic/build Arcs] [new output Arcs]
///      |                          |
///  [existing readers]         [current readers]
///      |                          |
///      +---- no in-place edits ---+
/// ```
///
/// # Examples
/// ```
/// use cmake_analyzer::workspace::model::*;
/// fn inspect(snapshot: &DocumentSnapshot) {
///     let _ = snapshot.lexed.lex.len();
/// }
/// ```
pub struct DocumentSnapshot {
    /// Shared lexical tokens, comments and checkpoints matching text Example:
    /// `snapshot.lexed.lex.len()` .
    pub lexed: Arc<crate::parser::lexer::model::LexedBuffer>,
    /// Exact overlay or disk text for this revision Example: `snapshot.text.as_ref()`.
    pub text: Arc<str>,
    // AI-authored shared incremental outputs; old snapshots retain their own revision.
    /// Shared file-local commands/scopes/references; not one importer's context Example:
    /// `snapshot.semantics.commands.len()` .
    pub semantics: Arc<SemanticedBuffer>,
    /// Shared structural facts qualified by file and revision Example:
    /// `snapshot.build.analysis.revision` .
    pub build: Arc<BuildStruct>,
    /// Reuse/recompute counts when this snapshot was produced Example: `snapshot.work.build.reused`
    /// .
    pub work: DocumentWork,
}
/// Mutable per-file cache slot retaining stable identity across snapshots. No I/O on construction.
///
/// [`Workspace`] owns these slots; [`DocumentSnapshot`] is the immutable reader-facing output.
///
/// # Examples
/// ```text
/// let _ = document.id;
/// ```
pub(super) struct Document {
    /// Workspace-local file ID retained even when parse data is released Example: `document.id`.
    pub id: u32,
    /// Monotonic clock for accepted overlay or disk analysis changes Example: `document.revision`.
    pub revision: u64,
    /// Editor text overriding disk, or None for disk fallback Example: `document.overlay.is_some()`
    /// .
    pub overlay: Option<Arc<str>>,
    /// Last published result, possibly hidden from callers by a newer revision Example:
    /// `document.snapshot.is_some()` .
    pub snapshot: Option<Arc<DocumentSnapshot>>,
    /// Reusable pipeline state; may be temporarily taken by a parsing worker Example:
    /// `document.analysis.is_some()` .
    pub analysis: Option<DocumentAnalysis>,
}
// AI-authored per-file computation state and visible reuse counters.
/// Reusable per-file pipeline caches, distinct from immutable saved output. No I/O on field access.
///
/// [`DocumentSnapshot`] shares each output Arc; [`Self::update`] replaces cache outputs in pipeline
/// order.
///
/// # Flow
/// ```text
/// [new text] ----> [lexer/change map]
///                          |
///                   [parser commands]
///                          |
///                     [semantics]
///                          |
///                    [build facts]
///                          |
///  [old snapshots]   [new shared outputs]
///        |                 |
///        +---- [independent readers]
/// ```
///
/// # Examples
/// ```text
/// let _ = analysis.lexer.work.reused;
/// ```
pub(super) struct DocumentAnalysis {
    /// Incremental lexical source and output Example: `analysis.lexer.work.reused`.
    pub lexer: crate::parser::lexer::model::IncrementalLexer,
    /// Incremental command parsing state Example: `analysis.parser.work.reused`.
    pub parser: crate::parser::nparser::model::IncrementalNParser,
    /// Incremental schema/command analysis state Example: `analysis.semantics.work.reused`.
    pub semantics: crate::parser::semanticer::model::IncrementalSemanticer,
    /// Incremental structural effects and bindings Example: `analysis.build.work.reused`.
    pub build: crate::parser::build_struct::model::IncrementalBuildStruct,
}
#[derive(Clone, Debug, Default)]
/// Last-operation work counts for each analysis layer. No side effects.
///
/// [`DocumentSnapshot::work`] exposes these counters; they measure command/lexical reuse, not total
/// time.
///
/// # Examples
/// ```
/// use cmake_analyzer::workspace::model::*;
/// fn inspect(work: &DocumentWork) {
///     let _ = work.lexer.reused;
/// }
/// ```
pub struct DocumentWork {
    /// Lexical units recomputed/reused Example: `work.lexer.reused`.
    pub lexer: crate::model::incremental::IncrementalWork,
    /// Parser command units recomputed/reused Example: `work.parser.recomputed`.
    pub parser: crate::model::incremental::IncrementalWork,
    /// Semantic command units recomputed/reused Example: `work.semantics.reused`.
    pub semantics: crate::model::incremental::IncrementalWork,
    /// Build command units recomputed/reused Example: `work.build.recomputed`.
    pub build: crate::model::incremental::IncrementalWork,
}
#[derive(Clone, Debug)]
/// One entry file and its optional project/script directory context. Construction has no I/O.
///
/// [`Workspace::root`] exposes this record; [`DirectoryContext`] records each visit's source/binary
/// paths.
///
/// # Examples
/// ```
/// use cmake_analyzer::workspace::model::*;
/// fn inspect(root: &Root) {
///     let _ = root.script_working_directory.as_ref();
/// }
/// ```
pub struct Root {
    /// Explicit -P working directory; suppresses filename-based project defaults Example:
    /// `root.script_working_directory.as_ref()` .
    pub script_working_directory: Option<PathBuf>,
    /// Lexically normalized absolute entry file, not necessarily existing Example:
    /// `root.file.as_path()` .
    pub file: PathBuf,
    /// Optional configured project build directory; not created here Example:
    /// `root.binary_directory.as_ref()` .
    pub binary_directory: Option<PathBuf>,
}
#[derive(Clone, Debug)]
/// Source and optional binary directories visible at one file visit. No I/O.
///
/// [`Invocation::directory`] retains this context; includes share it and subdirectories derive a
/// child context.
///
/// # Examples
/// ```
/// use cmake_analyzer::workspace::model::*;
/// fn inspect(directory: &DirectoryContext) {
///     let _ = directory.source.as_path();
/// }
/// ```
pub struct DirectoryContext {
    /// Current CMake source directory, not necessarily the list file's parent Example:
    /// `directory.source.as_path()` .
    pub source: PathBuf,
    /// Known current build directory, or None when it cannot be derived Example:
    /// `directory.binary.as_ref()` .
    pub binary: Option<PathBuf>,
}
#[derive(Clone, Debug)]
/// Conservative contextual writes, project name and limited values for a graph scope. No I/O.
///
/// [`RootGraph::scopes`] stores these records; child scopes copy current state instead of following
/// later parent writes.
///
/// # Flow
/// ```text
/// [parent state at child entry]
///             |
///       [shared write set]
///         /          \
///  [parent scope] [child scope]
///         |          |
///  [later write] [own later write]
///         |          |
///  [copy on write: separate sets]
///             |
///  [parent link records ancestry only]
/// ```
///
/// # Examples
/// ```
/// use cmake_analyzer::workspace::model::*;
/// fn inspect(scope: &VariableScope) {
///     let _ = scope.parent.is_none();
/// }
/// ```
pub struct VariableScope {
    /// Parent graph-scope index, not a file-local semantic ScopeId Example:
    /// `scope.parent.is_none()` .
    pub parent: Option<usize>,
    // Share the names until one side changes; later parent writes must not reach the child.
    /// Copy-on-write set of possible local output names Example:
    /// `scope.possible_writes.contains("OUT")` .
    pub possible_writes: Arc<BTreeSet<String>>,
    /// Latest unconditional literal project name in this scope Example: `scope.project.as_deref()`.
    pub project: Option<String>,
    /// Conservative literal/simple-expansion bindings used during traversal Example:
    /// `scope.values.get("CMAKE_MODULE_PATH", None)` .
    pub(super) values: crate::parser::semanticer::model::ValueBindings,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
/// Policy for static imports outside the project boundary. Choosing a policy performs no I/O.
///
/// [`Workspace::set_external_indexing`] configures includes/packages separately;
/// [`Invocation::index`] records the effective mode.
///
/// # Examples
/// ```
/// use cmake_analyzer::workspace::model::ExternalIndex;
/// let policy = ExternalIndex::Functions;
/// ```
pub enum ExternalIndex {
    /// Normal conservative build/reference/value/target analysis Example: `ExternalIndex::Full`.
    Full,
    /// Read static dependencies but export only unconditional top-level function/macro
    /// declarations; descendants inherit this restriction Example: `ExternalIndex::Functions` .
    Functions,
    /// Keep resolved navigation edge, skip loading/visiting and invalidate known values Example:
    /// `ExternalIndex::None` .
    None,
}
#[derive(Clone, Debug)]
/// A single ordered visit to a file, distinct from its reusable parse result. No I/O on access.
///
/// [`RootGraph::invocations`] stores visits; [`DependencyEdge::invocation`] links successful
/// dependency entries.
///
/// # Examples
/// ```
/// use cmake_analyzer::workspace::model::*;
/// fn inspect(visit: &Invocation) {
///     let _ = visit.index == ExternalIndex::Full;
/// }
/// ```
pub struct Invocation {
    /// Effective external policy; a functions-only ancestor restricts descendants Example:
    /// `visit.index == ExternalIndex::Full` .
    pub index: ExternalIndex,
    /// Immutable result shared by repeated visits to the same revision Example:
    /// `visit.snapshot.text.as_ref()` .
    pub snapshot: Arc<DocumentSnapshot>,
    /// Absolute lexical file identity used for loading Example: `visit.file.as_path()`.
    pub file: PathBuf,
    /// Revision identity matching the retained snapshot Example: `visit.analysis.revision`.
    pub analysis: crate::parser::build_struct::model::AnalysisKey,
    /// Calling invocation index, or None for root Example: `visit.parent.is_none()`.
    pub parent: Option<usize>,
    /// Graph variable-scope index active for this visit Example: `visit.scope`.
    pub scope: usize,
    /// Caller-shared or child-derived source/binary context Example:
    /// `visit.directory.source.as_path()` .
    pub directory: DirectoryContext,
}
#[derive(Clone, Debug)]
/// A dependency request, checked paths and optional resolved file/visit. No I/O on access.
///
/// [`Invocation`] distinguishes repeated visits; [`Workspace::invalidate_disk`] uses candidates to
/// trigger refreshed lookup.
///
/// # Examples
/// ```
/// use cmake_analyzer::workspace::model::*;
/// fn inspect(edge: &DependencyEdge) {
///     let _ = edge.caller;
/// }
/// ```
pub struct DependencyEdge {
    /// Index of the invocation containing the request Example: `edge.caller`.
    pub caller: usize,
    /// Revision-qualified dependency command origin Example: `edge.source.command.id`.
    pub source: SourceRef,
    /// Include, package or subdirectory lookup category Example:
    /// `edge.kind == DependencyKind::Package` .
    pub kind: DependencyKind,
    /// Whether absence avoids a required-dependency diagnostic Example: `edge.optional`.
    pub optional: bool,
    // Recheck the root if a missing module appears or an earlier search path gains one.
    /// Checked paths, including missing higher-priority choices Example: `edge.candidates.len()`.
    pub candidates: Vec<PathBuf>,
    /// Chosen path, possibly missing for direct-file requests; not proof of loading Example:
    /// `edge.resolved.as_ref()` .
    pub resolved: Option<PathBuf>,
    /// Entered child visit, or None for missing/skipped/untraversed requests Example:
    /// `edge.invocation.is_none()` .
    pub invocation: Option<usize>,
}
#[derive(Clone, Debug)]
/// File-read or traversal failure from a root dependency pass. No side effects.
///
/// [`RootGraph::diagnostics`] separates these failures from semantic/build diagnostics.
///
/// # Examples
/// ```
/// use cmake_analyzer::workspace::model::*;
/// fn inspect(diagnostic: &WorkspaceDiagnostic) {
///     let _ = diagnostic.kind == WorkspaceDiagnosticKind::Traversal;
/// }
/// ```
pub struct WorkspaceDiagnostic {
    /// Failure category used by diagnostics consumers Example:
    /// `diagnostic.kind == WorkspaceDiagnosticKind::Traversal` .
    pub kind: WorkspaceDiagnosticKind,
    /// Failed file or caller path, depending on failure stage Example: `diagnostic.file.as_path()`.
    pub file: PathBuf,
    /// Dependency caller origin when available; None for root failures Example:
    /// `diagnostic.source.as_ref()` .
    pub source: Option<SourceRef>,
    /// Human-readable reason for the failed read/resolution/traversal Example:
    /// `diagnostic.message.as_str()` .
    pub message: String,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
/// Category of a dependency-walk failure. No side effects.
///
/// [`WorkspaceDiagnostic`] stores this category separately from parser diagnostics.
///
/// # Examples
/// ```
/// use cmake_analyzer::workspace::model::WorkspaceDiagnosticKind;
/// let kind = WorkspaceDiagnosticKind::ReadFailure;
/// ```
pub enum WorkspaceDiagnosticKind {
    /// Failed source read or required literal dependency resolution Example:
    /// `WorkspaceDiagnosticKind::ReadFailure` .
    ReadFailure,
    /// Active dependency cycle or file/depth/attempt limit Example:
    /// `WorkspaceDiagnosticKind::Traversal` .
    Traversal,
}
#[derive(Clone, Debug, Default)]
/// Ordered visits, contextual state and owner-qualified indexes for one project root. No I/O on
/// access.
///
/// [`Workspace::root_graph`] hides dirty/revision-stale inputs; [`DocumentSnapshot`] retains
/// immutable per-file data.
///
/// # Flow
/// ```text
/// [root visit] -----> [dependency edges]
///      |                     |
///  [shared snapshots]   [child visits]
///      |                     |
///      +----> [ordered replay]+
///                       |
///              [references/targets/values]
/// ```
///
/// # Examples
/// ```
/// use cmake_analyzer::workspace::model::*;
/// fn inspect(graph: &RootGraph) {
///     let _ = graph.project_defaults;
/// }
/// ```
pub struct RootGraph {
    /// Whether a CMakeLists.txt project entry seeds default PROJECT_SOURCE_DIR Example:
    /// `graph.project_defaults` .
    pub project_defaults: bool,
    /// Visit order, with root first and callers before children Example: `graph.invocations.len()`.
    pub invocations: Vec<Invocation>,
    /// All dependency candidates, including untraversed conditional/deferred requests Example:
    /// `graph.edges.len()` .
    pub edges: Vec<DependencyEdge>,
    /// Graph-local scope records with forked child state Example: `graph.scopes.len()`.
    pub scopes: Vec<VariableScope>,
    // Each pair selects a file visit, then a fact in that visit's saved BuildStruct.
    /// Pairs of invocation index and fact index in that visit's retained build result Example:
    /// `graph.facts.first().copied()` .
    pub facts: Vec<(usize, usize)>,
    /// Read and traversal failures from the dependency walk Example: `graph.diagnostics.len()`.
    pub diagnostics: Vec<WorkspaceDiagnostic>,
    /// Shared immutable contextual variable/function reference index Example:
    /// `graph.references.variables.len()` .
    pub references: Arc<crate::parser::semanticer::model::ReferenceIndex>,
    /// Shared owner-qualified target declarations and their bound references Example:
    /// `graph.targets.len()` .
    pub targets: Arc<Vec<crate::parser::build_struct::model::TargetDeclaration>>,
    /// Sparse command/visit-qualified possible-value observations Example: `graph.values.len()`.
    pub values: Vec<crate::parser::semanticer::model::VariableValueAt>,
    /// Optional independent installed execution layer Example: `graph.runtime.is_some()`.
    pub runtime: Option<Arc<super::runtime::RuntimeLayer>>,
}

// AI-generated display models.
/// Borrowed terminal view; the graph field can also be printed on its own.
#[allow(non_snake_case)] // Requested public field name.
/// Borrowed workspace overview with a separately printable dependency tree. Creating it does not
/// write output.
///
/// [`Workspace::display`] creates this adapter; [`DependencyGraphDisplay`] renders only
/// graph/scopes/diagnostics.
///
/// # Examples
/// ```
/// use cmake_analyzer::workspace::Workspace;
/// fn render(workspace: &Workspace) -> String {
///     workspace.display(false).to_string()
/// }
/// ```
pub struct WorkspaceDisplay<'a> {
    /// Workspace borrowed for root and cache statistics Example:
    /// `view.workspace.root.file.as_path()` .
    pub(super) workspace: &'a Workspace,
    /// Whether overview labels use terminal styling Example: `view.colored`.
    pub(super) colored: bool,
    /// Public graph-only view; spelling retained by the API Example:
    /// `format!("{}", view.DependcyGraphDisplay)` .
    pub DependcyGraphDisplay: DependencyGraphDisplay<'a>,
}
/// Borrowed dependency tree, scope summary and diagnostics. Creating it has no output side effect.
///
/// [`WorkspaceDisplay::DependcyGraphDisplay`] exposes this adapter; [`Workspace::root_graph`]
/// supplies the captured graph.
///
/// # Flow
/// ```text
/// [captured graph]          [root path context]
///         |                         |
///  [iterative visit tree] ---- [escaped labels]
///         |
///  [scope summaries]       [unique snapshot diagnostics]
///         |                         |
///         +------> [formatter] <----+
/// ```
///
/// # Examples
/// ```
/// use cmake_analyzer::workspace::Workspace;
/// fn render(workspace: &Workspace) -> String {
///     workspace.display(false).DependcyGraphDisplay.to_string()
/// }
/// ```
pub struct DependencyGraphDisplay<'a> {
    /// Graph selected when the view was created; None prints pending/stale Example:
    /// `view.graph.is_some()` .
    pub(super) graph: Option<&'a RootGraph>,
    /// Entry context for relative labels and root diagnostics Example: `view.root.file.as_path()`.
    pub(super) root: &'a Root,
    /// Whether graph labels use terminal styling Example: `view.colored`.
    pub(super) colored: bool,
}

/// One root project and its files. Changes wait for flush().
/// Owner of one root, deferred edits, immutable file results and contextual indexes. Setters queue
/// work; flush performs reads/analysis, never project execution.
///
/// [`Self::set_overlay`] queues editor text; [`Self::flush`] processes it; [`Self::snapshot`] and
/// [`Self::root_graph`] expose results.
///
/// # Flow
/// ```text
/// [editor overlays]        [disk invalidations]
///          |                       |
///          +------> [pending paths]+
///                          |
///                        flush
///                     /         \
///           [bounded parsing] [reuse/rebuild visits]
///                     \         /
///                  [immutable snapshots + indexes]
///                          |
///                   [shared readers]
/// ```
///
/// # Examples
/// ```
/// use cmake_analyzer::workspace::Workspace;
/// let mut workspace = Workspace::new("/demo/CMakeLists.txt", None)?;
/// workspace.set_overlay("/demo/CMakeLists.txt", "cmake_minimum_required(VERSION 3.31)\nproject(Demo)")?;
/// # Ok::<(), std::io::Error>(())
/// ```
pub struct Workspace {
    /// Replaceable module/package completion root and name cache Example:
    /// `workspace.name_cache.modules.is_some()` .
    pub(super) name_cache: super::completion::model::DiscoveryCache,
    /// Latest bounded supplemental target-source retention set Example:
    /// `workspace.completion_files.len()` .
    pub(super) completion_files: HashSet<PathBuf>,
    /// Gate for bare module/package search and name discovery Example:
    /// `workspace.module_lookup_enabled` .
    pub(super) module_lookup_enabled: bool,
    /// Policy for includes outside the canonical root-entry directory Example:
    /// `workspace.external_include == ExternalIndex::Full` .
    pub(super) external_include: ExternalIndex,
    /// Independent policy for external resolved package entrypoints Example:
    /// `workspace.external_find_package == ExternalIndex::Functions` .
    pub(super) external_find_package: ExternalIndex,
    /// Configured package prefixes used when request system paths are enabled Example:
    /// `workspace.package_search_paths.len()` .
    pub(super) package_search_paths: Vec<PathBuf>,
    /// Per-workspace worker/file/depth/attempt ceilings Example: `workspace.limits.files`.
    pub(super) limits: WorkspaceLimits,
    /// Semaphore held by bounded parsing/reference/runtime blocking jobs Example:
    /// `workspace.workers.available_permits()` .
    pub(super) workers: Arc<tokio::sync::Semaphore>,
    /// Stable file IDs and mutable caches keyed by lexical absolute path Example:
    /// `workspace.documents.len()` .
    pub(super) documents: HashMap<PathBuf, Document>,
    /// Single project/script entry and directory context Example: `workspace.root.file.as_path()`.
    pub(super) root: Root,
    /// Last graph, potentially stale until pending changes are flushed Example:
    /// `workspace.graph.is_some()` .
    pub(super) graph: Option<RootGraph>,
    /// Forces traversal rather than snapshot-only graph reuse when rebuild is requested Example:
    /// `workspace.graph_invalidated` .
    pub(super) graph_invalidated: bool,
    /// Whether the last flush performed a new dependency walk Example: `workspace.graph_rebuilt`.
    pub graph_rebuilt: bool,
    /// Visit count refreshed without traversal in the last flush Example:
    /// `workspace.reused_invocations` .
    pub reused_invocations: usize,
    /// the pre-handled document
    /// Coalesced paths awaiting flush; newest overlay wins Example: `workspace.dirty.len()`.
    pub(super) dirty: BTreeSet<PathBuf>,
    // Files and search paths whose changes require rebuilding this project's graph.
    /// Current visited files plus lookup candidates that can trigger traversal Example:
    /// `workspace.dependencies.contains(&workspace.root.file)` .
    pub(super) dependencies: HashSet<PathBuf>,
    /// Next stable file ID; checked before increment Example: `workspace.next_file`.
    pub(super) next_file: u32,
    /// Explicit module search directories, joined to visit source context Example:
    /// `workspace.module_paths.len()` .
    pub(super) module_paths: Vec<PathBuf>,
    /// Optional absolute native module directory supplied by configuration Example:
    /// `workspace.builtin_module_directory.as_ref()` .
    pub(super) builtin_module_directory: Option<PathBuf>,
    /// Independent installed execution layer, retired on known input changes Example:
    /// `workspace.runtime.is_some()` .
    pub(super) runtime: Option<Arc<super::runtime::RuntimeLayer>>,
}
impl Workspace {
    /// Initialize one root with deferred work and a worker semaphore. No files are read.
    ///
    /// # Arguments
    /// - `root` `[in]`: Owned normalized root context.
    /// - `limits` `[in]`: Resource ceilings; worker count is clamped to at least one.
    ///
    /// # Returns
    /// Workspace with full external indexing, module lookup enabled and the root marked dirty.
    ///
    /// # Examples
    /// ```text
    /// let workspace = Workspace::from_root(root, WorkspaceLimits::default());
    /// ```
    pub(super) fn from_root(root: Root, limits: WorkspaceLimits) -> Self {
        let dirty = BTreeSet::from([root.file.clone()]);
        Self {
            name_cache: Default::default(),
            completion_files: HashSet::new(),
            module_lookup_enabled: true,
            external_include: ExternalIndex::Full,
            external_find_package: ExternalIndex::Full,
            package_search_paths: Vec::new(),
            workers: Arc::new(tokio::sync::Semaphore::new(limits.workers.max(1))),
            limits,
            documents: HashMap::new(),
            root,
            graph: None,
            graph_invalidated: true,
            graph_rebuilt: false,
            reused_invocations: 0,
            dirty,
            dependencies: HashSet::new(),
            next_file: 0,
            module_paths: Vec::new(),
            builtin_module_directory: None,
            runtime: None,
        }
    }
}
