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
pub struct WorkspaceLimits {
    pub workers: usize,
    pub files: usize,
    pub depth: usize,
    pub invocations: usize,
}
impl Default for WorkspaceLimits {
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
pub struct DocumentSnapshot {
    pub lexed: Arc<crate::parser::lexer::model::LexedBuffer>,
    pub text: Arc<str>,
    // AI-authored shared incremental outputs; old snapshots retain their own revision.
    pub semantics: Arc<SemanticedBuffer>,
    pub build: Arc<BuildStruct>,
    pub work: DocumentWork,
}
pub(super) struct Document {
    pub id: u32,
    pub revision: u64,
    pub overlay: Option<Arc<str>>,
    pub snapshot: Option<Arc<DocumentSnapshot>>,
    pub analysis: Option<DocumentAnalysis>,
}
// AI-authored per-file computation state and visible reuse counters.
pub(super) struct DocumentAnalysis {
    pub lexer: crate::parser::lexer::model::IncrementalLexer,
    pub parser: crate::parser::nparser::model::IncrementalNParser,
    pub semantics: crate::parser::semanticer::model::IncrementalSemanticer,
    pub build: crate::parser::build_struct::model::IncrementalBuildStruct,
}
#[derive(Clone, Debug, Default)]
pub struct DocumentWork {
    pub lexer: crate::model::incremental::IncrementalWork,
    pub parser: crate::model::incremental::IncrementalWork,
    pub semantics: crate::model::incremental::IncrementalWork,
    pub build: crate::model::incremental::IncrementalWork,
}
#[derive(Clone, Debug)]
pub struct Root {
    pub script_working_directory: Option<PathBuf>,
    pub file: PathBuf,
    pub binary_directory: Option<PathBuf>,
}
#[derive(Clone, Debug)]
pub struct DirectoryContext {
    pub source: PathBuf,
    pub binary: Option<PathBuf>,
}
#[derive(Clone, Debug)]
pub struct VariableScope {
    pub parent: Option<usize>,
    // Share the names until one side changes; later parent writes must not reach the child.
    pub possible_writes: Arc<BTreeSet<String>>,
    pub project: Option<String>,
    pub(super) values: crate::parser::semanticer::model::ValueBindings,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ExternalIndex {
    Full,
    Functions,
    None,
}
#[derive(Clone, Debug)]
pub struct Invocation {
    pub index: ExternalIndex,
    pub snapshot: Arc<DocumentSnapshot>,
    pub file: PathBuf,
    pub analysis: crate::parser::build_struct::model::AnalysisKey,
    pub parent: Option<usize>,
    pub scope: usize,
    pub directory: DirectoryContext,
}
#[derive(Clone, Debug)]
pub struct DependencyEdge {
    pub caller: usize,
    pub source: SourceRef,
    pub kind: DependencyKind,
    pub optional: bool,
    // Recheck the root if a missing module appears or an earlier search path gains one.
    pub candidates: Vec<PathBuf>,
    pub resolved: Option<PathBuf>,
    pub invocation: Option<usize>,
}
#[derive(Clone, Debug)]
pub struct WorkspaceDiagnostic {
    pub kind: WorkspaceDiagnosticKind,
    pub file: PathBuf,
    pub source: Option<SourceRef>,
    pub message: String,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WorkspaceDiagnosticKind {
    ReadFailure,
    Traversal,
}
#[derive(Clone, Debug, Default)]
pub struct RootGraph {
    pub project_defaults: bool,
    pub invocations: Vec<Invocation>,
    pub edges: Vec<DependencyEdge>,
    pub scopes: Vec<VariableScope>,
    // Each pair selects a file visit, then a fact in that visit's saved BuildStruct.
    pub facts: Vec<(usize, usize)>,
    pub diagnostics: Vec<WorkspaceDiagnostic>,
    pub references: Arc<crate::parser::semanticer::model::ReferenceIndex>,
    pub targets: Arc<Vec<crate::parser::build_struct::model::TargetDeclaration>>,
    pub values: Vec<crate::parser::semanticer::model::VariableValueAt>,
    pub runtime: Option<Arc<super::runtime::RuntimeLayer>>,
}

// AI-generated display models.
/// Borrowed terminal view; the graph field can also be printed on its own.
#[allow(non_snake_case)] // Requested public field name.
pub struct WorkspaceDisplay<'a> {
    pub(super) workspace: &'a Workspace,
    pub(super) colored: bool,
    pub DependcyGraphDisplay: DependencyGraphDisplay<'a>,
}
pub struct DependencyGraphDisplay<'a> {
    pub(super) graph: Option<&'a RootGraph>,
    pub(super) root: &'a Root,
    pub(super) colored: bool,
}

/// One root project and its files. Changes wait for flush().
pub struct Workspace {
    pub(super) name_cache: super::completion::model::DiscoveryCache,
    pub(super) completion_files: HashSet<PathBuf>,
    pub(super) module_lookup_enabled: bool,
    pub(super) external_include: ExternalIndex,
    pub(super) external_find_package: ExternalIndex,
    pub(super) package_search_paths: Vec<PathBuf>,
    pub(super) limits: WorkspaceLimits,
    pub(super) workers: Arc<tokio::sync::Semaphore>,
    pub(super) documents: HashMap<PathBuf, Document>,
    pub(super) root: Root,
    pub(super) graph: Option<RootGraph>,
    pub(super) graph_invalidated: bool,
    pub graph_rebuilt: bool,
    pub reused_invocations: usize,
    /// the pre-handled document
    pub(super) dirty: BTreeSet<PathBuf>,
    // Files and search paths whose changes require rebuilding this project's graph.
    pub(super) dependencies: HashSet<PathBuf>,
    pub(super) next_file: u32,
    pub(super) module_paths: Vec<PathBuf>,
    pub(super) builtin_module_directory: Option<PathBuf>,
    pub(super) runtime: Option<Arc<super::runtime::RuntimeLayer>>,
}
impl Workspace {
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
