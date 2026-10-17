//! Execution-qualified identities, observations and child-process options. Values are
//! in-memory records only; [`super::execute`] owns process launch and [`crate::workspace`]
//! may later install observations without rewriting static snapshots.
//!
//! Contains AI-generated documentation additions.
// AI-authored runtime observations, distinct from static snapshots.
use crate::parser::build_struct::model::DependencyKind;
use std::{path::PathBuf, sync::Arc, time::Duration};
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
/// Execution identity chosen by the caller; not a timestamp or globally allocated ID.
/// [`Trace`] uses it to distinguish observations across runs. Constructors/accessors have no side effects.
///
/// # Examples
/// ```
/// # use cmake_analyzer::runtime_cmake::model::RunId;
/// assert_eq!(RunId::new(7).value(), 7);
/// ```
pub struct RunId(
    /// Raw caller-selected execution number. Example: `RunId::new(7).value()`.
    u64,
);
impl RunId {
    /// Wrap a supplied execution number without allocation, validation or I/O.
    ///
    /// # Arguments
    /// - `value` `[in]`: raw caller-owned run number; uniqueness is the caller's responsibility.
    ///
    /// # Returns
    /// A typed identity containing the unchanged value.
    ///
    /// # Examples
    /// ```
    /// # use cmake_analyzer::runtime_cmake::model::RunId;
    /// let id = RunId::new(7);
    /// ```
    pub fn new(value: u64) -> Self {
        Self(value)
    }
    /// Read the underlying run number. No mutation or I/O.
    ///
    /// # Arguments
    /// - `self` `[in]`: copied typed value; the caller can keep using the original.
    ///
    /// # Returns
    /// The unchanged stored u64 run number.
    ///
    /// # Examples
    /// ```
    /// # use cmake_analyzer::runtime_cmake::model::RunId;
    /// assert_eq!(RunId::new(7).value(), 7);
    /// ```
    pub fn value(self) -> u64 {
        self.0
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
/// Zero-based command-event identity within one [`Trace`], not a persistent source ID.
/// The decoder creates it in execution order; consumers use [`Self::index`] for lookup. No I/O.
///
/// # Examples
/// ```
/// # use cmake_analyzer::runtime_cmake::model::{TraceEvent, EventId};
/// fn offset(event: &TraceEvent) -> usize { event.id.index() }
/// ```
pub struct EventId(
    /// Index into the owning trace events vector. Example: `EventId::new(0).index()`.
    usize,
);
impl EventId {
    /// Wrap a supplied event index without allocation, validation or I/O.
    ///
    /// # Arguments
    /// - `value` `[in]`: raw zero-based event index, valid only for the owning trace.
    ///
    /// # Returns
    /// A typed identity containing the unchanged value.
    ///
    /// # Examples
    /// ```text
    /// let id = EventId::new(0);
    /// ```
    pub(super) fn new(value: usize) -> Self {
        Self(value)
    }
    /// Read the underlying trace-local event index. No mutation or I/O.
    ///
    /// # Arguments
    /// - `self` `[in]`: copied typed value; the caller can keep using the original.
    ///
    /// # Returns
    /// The unchanged stored usize event offset.
    ///
    /// # Examples
    /// ```
    /// # use cmake_analyzer::runtime_cmake::model::TraceEvent;
    /// fn index(event: &TraceEvent) -> usize { event.id.index() }
    /// ```
    pub fn index(self) -> usize {
        self.0
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
/// CMake global_frame nesting depth, not an event ID or lexical scope.
/// [`TraceEvent`] carries it and [`Self::depth`] exposes the numeric level. No I/O.
///
/// # Examples
/// ```
/// # use cmake_analyzer::runtime_cmake::model::TraceEvent;
/// fn depth(event: &TraceEvent) -> u64 { event.frame.depth() }
/// ```
pub struct RuntimeFrame(
    /// Raw global_frame depth from the JSON trace. Example: `RuntimeFrame::new(2).depth()`.
    u64,
);
impl RuntimeFrame {
    /// Wrap a supplied frame depth without allocation, validation or I/O.
    ///
    /// # Arguments
    /// - `value` `[in]`: raw global_frame depth from a recognized event.
    ///
    /// # Returns
    /// A typed identity containing the unchanged value.
    ///
    /// # Examples
    /// ```text
    /// let id = RuntimeFrame::new(2);
    /// ```
    pub(super) fn new(value: u64) -> Self {
        Self(value)
    }
    /// Read the underlying execution nesting depth. No mutation or I/O.
    ///
    /// # Arguments
    /// - `self` `[in]`: copied typed value; the caller can keep using the original.
    ///
    /// # Returns
    /// The unchanged stored u64 global_frame depth.
    ///
    /// # Examples
    /// ```
    /// # use cmake_analyzer::runtime_cmake::model::TraceEvent;
    /// fn depth(event: &TraceEvent) -> u64 { event.frame.depth() }
    /// ```
    pub fn depth(self) -> u64 {
        self.0
    }
}
#[derive(Clone, Debug)]
/// One expanded command invocation observed during execution, possibly repeated for
/// the same source location. [`Trace::events`] owns it; [`EventId`] indexes that vector.
/// Arguments reflect trace expansion, not always final unquoted list splitting. No I/O on construction.
///
/// # Examples
/// ```
/// # use cmake_analyzer::runtime_cmake::model::TraceEvent;
/// fn location(event: &TraceEvent) -> (&std::path::Path, u32) { (&event.file, event.line) }
/// ```
pub struct TraceEvent {
    /// Shared source-directory context tracked across add_subdirectory frames. Example: `event.source_directory.as_path()`.
    pub source_directory: Arc<PathBuf>,
    /// Position within the owning trace events vector. Example: `event.id.index()`.
    pub id: EventId,
    /// Normalized file path named by CMake; may no longer exist. Example: `event.file.display()`.
    pub file: PathBuf,
    /// Positive one-based source line from CMake. Example: `event.line == 1`.
    pub line: u32,
    /// Lowercase command name. Example: `event.command == "set"`.
    pub command: String,
    /// Expanded source arguments before any unquoted list/empty-item reinterpretation. Example: `event.arguments.first()`.
    pub arguments: Vec<String>,
    /// Global execution nesting depth, not lexical scope. Example: `event.frame.depth()`.
    pub frame: RuntimeFrame,
    /// False after a same-name function/macro declaration has been observed; heuristic, not policy evaluation. Example: `event.builtin && event.command == "include"`.
    pub builtin: bool,
}
#[derive(Clone, Debug)]
/// Dependency observation tied to a builtin include/find_package/add_subdirectory event.
/// [`Trace::dependencies`] retains unresolved entries too; [`TraceEvent`] contains raw arguments.
/// Constructing the record neither loads nor checks its target.
///
/// # Examples
/// ```
/// # use cmake_analyzer::runtime_cmake::model::{Trace, ObservedDependency};
/// fn targets(trace: &Trace) -> usize { trace.dependencies.iter().filter(|d| d.resolved.is_some()).count() }
/// ```
pub struct ObservedDependency {
    /// Caller event index in the same trace. Example: `dependency.event.index()`.
    pub event: EventId,
    /// Include, package or subdirectory dependency category. Example: `matches!(dependency.kind, cmake_analyzer::parser::build_struct::model::DependencyKind::Include)`.
    pub kind: DependencyKind,
    /// Observed or conservatively inferred path; None when trace evidence/path fallback is insufficient. Not existence-checked. Example: `dependency.resolved.as_deref()`.
    pub resolved: Option<PathBuf>,
}
#[derive(Clone, Debug)]
/// Successful execution observations before Workspace source mapping. [`super::execute`]
/// returns this only after successful child exit and decoding; [`TraceEvent`] IDs remain
/// execution-local. No source text or static snapshot is mutated by holding a Trace.
///
/// # Flow
/// ```text
///        [Trace: one run]
///         /           \
///  [events in order] [dependency observations]
///        ^                     |
///        +----- EventId -------+
///         |
///  [file/line + expanded args]
///         |
///  [Workspace may map source later]
/// ```
///
/// # Examples
/// ```
/// # use cmake_analyzer::runtime_cmake::model::{Trace, RunId};
/// let trace = Trace { run: RunId::new(1), build_directory: None, events: vec![], dependencies: vec![] };
/// ```
pub struct Trace {
    /// Caller-assigned execution identity. Example: `trace.run.value()`.
    pub run: RunId,
    /// None for script runs; no build directory is created or exempted from input checks.
    /// Configure output context; None for scripts. This record itself creates nothing. Example: `trace.build_directory.is_none()`.
    pub build_directory: Option<PathBuf>,
    /// All recognized command events in order, retaining repeated executions. Example: `trace.events.get(event_id.index())`.
    pub events: Vec<TraceEvent>,
    /// Observed dependency calls, including unresolved ones. Example: `trace.dependencies.iter().filter(|d| d.resolved.is_none()).count()`.
    pub dependencies: Vec<ObservedDependency>,
}
#[derive(Clone, Debug, PartialEq, Eq)]
/// Select actual process mode for [`ConfigureOptions`] and [`super::execute`].
/// Configure creates/updates build state; scripts may also have arbitrary effects, but
/// choosing the enum alone performs no I/O.
///
/// # Examples
/// ```
/// # use cmake_analyzer::runtime_cmake::model::ExecutionMode;
/// let mode = ExecutionMode::Script { file: "/work/check.cmake".into() };
/// ```
pub enum ExecutionMode {
    /// Execute configure using -S and -B; this does not build targets. Example: `let mode = ExecutionMode::Configure`.
    Configure,
    /// Execute the selected script using -P. Example: `ExecutionMode::Script { file: "check.cmake".into() }`.
    Script {
        /// Script entry; server callers pass an absolute path. Example: `ExecutionMode::Script { file: "/work/check.cmake".into() }`.
        file: PathBuf,
    },
}
#[derive(Clone, Debug)]
/// Shared execution options. Script mode uses source_directory as cwd and ignores build_directory.
/// One explicit child-process invocation used by [`super::execute`], despite the configure-oriented
/// name also supporting scripts. Server admission owns permission, saved-input checks and
/// argument validation; execute does not repeat them. Constructing options performs no I/O.
/// Project code is not sandboxed and can write/spawn outside the configured directory.
///
/// # Flow
/// ```text
///  [server authorization + saved input checks]
///                    |
///           [ConfigureOptions]
///               /        \
///          Configure     Script
///             |             |
///        [-S/-B paths]   [-P file; cwd]
///             \             /
///              [execute owns child]
///                     |
///           [bounded stderr / Trace]
/// ```
///
/// # Examples
/// ```
/// # use cmake_analyzer::runtime_cmake::model::{ConfigureOptions, ExecutionMode};
/// # use std::time::Duration;
/// let options = ConfigureOptions {
///     mode: ExecutionMode::Configure, executable: "cmake".into(),
///     source_directory: "/work/project".into(), build_directory: "/work/build".into(),
///     arguments: vec!["-DFLAG=ON".into()], timeout: Duration::from_secs(30),
///     max_output_bytes: 1_048_576, max_error_bytes: 4096,
/// };
/// ```
pub struct ConfigureOptions {
    /// Configure or script execution mode. Example: `options.mode = ExecutionMode::Configure`.
    pub mode: ExecutionMode,
    /// Program name/path passed directly to Tokio Command, without a shell. Example: `options.executable = "cmake".into()`.
    pub executable: PathBuf,
    /// Child cwd in both modes; also -S for configure. Scripts use startup cwd from server admission. Example: `options.source_directory.display()`.
    pub source_directory: PathBuf,
    /// -B target for configure, ignored in script mode. No sandbox guarantee. Example: `options.build_directory = "/work/build".into()`.
    pub build_directory: PathBuf,
    /// Extra child arguments before owned trace/mode flags; caller must validate them. Example: `options.arguments.push("-DFLAG=ON".into())`.
    pub arguments: Vec<String>,
    /// Deadline for reading stderr and waiting on the child, not decoding time. Example: `options.timeout = Duration::from_secs(5)`.
    pub timeout: Duration,
    /// Maximum captured stderr bytes including trace/warnings; overflow kills/reaps the direct child. Example: `options.max_output_bytes = 4096`.
    pub max_output_bytes: usize,
    /// Input-byte budget for non-JSON failure-detail suffixes; lossy UTF-8 can expand output. Example: `options.max_error_bytes = 1024`.
    pub max_error_bytes: usize,
}
