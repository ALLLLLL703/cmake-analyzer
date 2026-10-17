//! Private arenas and branch state for static reference binding.
//!
//! Snapshot IDs and aggregate IDs have different index domains. Environments inherit
//! bindings through parents; branch environments can share an identity owner without
//! sharing their mutable maps. No project code is executed.
// Contains AI-generated documentation additions.
// AI-authored internal reference-analysis state.
use crate::parser::semanticer::model::*;
use std::collections::{HashMap, HashSet};

// Aggregate and document-local arenas have distinct identity domains.
/// Typed aggregate variable index; construction performs no lookup or I/O.
///
/// [`ReferenceCollector`] uses this domain. The number is valid only in the owning
/// snapshot or collector/match generation, not across rebuilt arenas.
///
/// # Examples
/// ```text
/// let id = IndexedVariableId::new(0);
/// assert_eq!(id.index(), 0);
/// ```
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub(super) struct IndexedVariableId(
    /// Zero-based aggregate variable arena slot; not range-checked.
    /// Example: `id.0` reads the stored slot inside this module.
    usize,
);
impl IndexedVariableId {
    /// Wrap a aggregate variable arena slot without checking its bounds. No side effects.
    ///
    /// # Arguments
    /// - `index` `[in]`: zero-based slot in the owning arena.
    ///
    /// # Returns
    /// Typed index; no allocation or lookup.
    ///
    /// # Examples
    /// ```text
    /// let id = IndexedVariableId::new(0);
    /// ```
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    /// Read the stored aggregate variable arena slot. No side effects.
    ///
    /// # Arguments
    /// - `self` `[in]`: typed index copied by value.
    ///
    /// # Returns
    /// The original zero-based slot; no bounds validation.
    ///
    /// # Examples
    /// ```text
    /// let id = IndexedVariableId::new(0);
    /// assert_eq!(id.index(), 0);
    /// ```
    pub fn index(self) -> usize {
        self.0
    }
}
/// Typed aggregate function index; construction performs no lookup or I/O.
///
/// [`ReferenceCollector`] uses this domain. The number is valid only in the owning
/// snapshot or collector/match generation, not across rebuilt arenas.
///
/// # Examples
/// ```text
/// let id = IndexedFunctionId::new(0);
/// assert_eq!(id.index(), 0);
/// ```
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub(super) struct IndexedFunctionId(
    /// Zero-based aggregate function arena slot; not range-checked.
    /// Example: `id.0` reads the stored slot inside this module.
    usize,
);
impl IndexedFunctionId {
    /// Wrap a aggregate function arena slot without checking its bounds. No side effects.
    ///
    /// # Arguments
    /// - `index` `[in]`: zero-based slot in the owning arena.
    ///
    /// # Returns
    /// Typed index; no allocation or lookup.
    ///
    /// # Examples
    /// ```text
    /// let id = IndexedFunctionId::new(0);
    /// ```
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    /// Read the stored aggregate function arena slot. No side effects.
    ///
    /// # Arguments
    /// - `self` `[in]`: typed index copied by value.
    ///
    /// # Returns
    /// The original zero-based slot; no bounds validation.
    ///
    /// # Examples
    /// ```text
    /// let id = IndexedFunctionId::new(0);
    /// assert_eq!(id.index(), 0);
    /// ```
    pub fn index(self) -> usize {
        self.0
    }
}
/// Typed file-local function index; construction performs no lookup or I/O.
///
/// [`FileContext`] uses this domain. The number is valid only in the owning
/// snapshot or collector/match generation, not across rebuilt arenas.
///
/// # Examples
/// ```text
/// let id = SnapshotFunctionId::new(0);
/// assert_eq!(id.index(), 0);
/// ```
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub(super) struct SnapshotFunctionId(
    /// Zero-based file-local function arena slot; not range-checked.
    /// Example: `id.0` reads the stored slot inside this module.
    usize,
);
impl SnapshotFunctionId {
    /// Wrap a file-local function arena slot without checking its bounds. No side effects.
    ///
    /// # Arguments
    /// - `index` `[in]`: zero-based slot in the owning arena.
    ///
    /// # Returns
    /// Typed index; no allocation or lookup.
    ///
    /// # Examples
    /// ```text
    /// let id = SnapshotFunctionId::new(0);
    /// ```
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    /// Read the stored file-local function arena slot. No side effects.
    ///
    /// # Arguments
    /// - `self` `[in]`: typed index copied by value.
    ///
    /// # Returns
    /// The original zero-based slot; no bounds validation.
    ///
    /// # Examples
    /// ```text
    /// let id = SnapshotFunctionId::new(0);
    /// assert_eq!(id.index(), 0);
    /// ```
    pub fn index(self) -> usize {
        self.0
    }
}
/// Typed file-local variable index; construction performs no lookup or I/O.
///
/// [`ReferenceCollector`] uses this domain. The number is valid only in the owning
/// snapshot or collector/match generation, not across rebuilt arenas.
///
/// # Examples
/// ```text
/// let id = SnapshotVariableId::new(0);
/// assert_eq!(id.index(), 0);
/// ```
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub(super) struct SnapshotVariableId(
    /// Zero-based file-local variable arena slot; not range-checked.
    /// Example: `id.0` reads the stored slot inside this module.
    usize,
);
impl SnapshotVariableId {
    /// Wrap a file-local variable arena slot without checking its bounds. No side effects.
    ///
    /// # Arguments
    /// - `index` `[in]`: zero-based slot in the owning arena.
    ///
    /// # Returns
    /// Typed index; no allocation or lookup.
    ///
    /// # Examples
    /// ```text
    /// let id = SnapshotVariableId::new(0);
    /// ```
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    /// Read the stored file-local variable arena slot. No side effects.
    ///
    /// # Arguments
    /// - `self` `[in]`: typed index copied by value.
    ///
    /// # Returns
    /// The original zero-based slot; no bounds validation.
    ///
    /// # Examples
    /// ```text
    /// let id = SnapshotVariableId::new(0);
    /// assert_eq!(id.index(), 0);
    /// ```
    pub fn index(self) -> usize {
        self.0
    }
}
/// Small state lattice for warning inference: it tracks emptiness/unset status, not text values. No I/O.
///
/// [`Binding`] carries this state with an aggregate variable ID.
///
/// # Examples
/// ```text
/// let state = Value::Unknown;
/// ```
#[derive(Clone, Copy, PartialEq, Eq)]
pub(super) enum Value {
    /// No reliable unset/empty fact.
    /// Example: `Value::Unknown`.
    Unknown,
    /// Conditional definition may not run.
    /// Example: `Value::MaybeUnset`.
    MaybeUnset,
    /// Source command removes the local binding.
    /// Example: `Value::Unset`.
    Unset,
    /// Literal assignment is the empty string.
    /// Example: `Value::Empty`.
    Empty,
    /// Literal assignment is nonempty.
    /// Example: `Value::NonEmpty`.
    NonEmpty,
}
/// One visible variable identity and its warning state at a write clock. The collector copies bindings during lookup and branch merge; construction performs no I/O.
///
/// [`Environment`] stores visible bindings.
///
/// # Examples
/// ```text
/// let binding = Binding { variable: IndexedVariableId::new(0), value: Value::Unknown, written_at: 0 };
/// ```
#[derive(Clone, Copy)]
pub(super) struct Binding {
    /// Aggregate variable identity, never a snapshot variable index.
    /// Example: `binding.variable.index()`.
    pub variable: IndexedVariableId,
    /// Unset/empty warning state, not the possible text value.
    /// Example: `binding.value == Value::Unknown`.
    pub value: Value,
    // Numeric call/write clocks are counters, not entity identities.
    /// Write clock for call invalidation; a counter, not an entity ID.
    /// Example: `binding.written_at == 0`.
    pub written_at: usize,
}
/// One mutable visibility frame in a collector generation. Conditional frames can share an owner with a parent while retaining independent branch maps; deferred frames suppress unsafe outer-value warnings. No I/O.
///
/// [`ReferenceCollector`] owns the arena.
///
/// # Flow
/// ```text
///  [parent visibility frame]
///           |
///           v
///    [local environment] -----> [identity owner]
///       /          \                |
///  [variable map] [function map]     v
///       |          |        [aggregate variable arena]
///       +-----+----+
///             v
///   [branch save / call invalidation]
/// ```
/// # Examples
/// ```text
/// let environment = Environment::default();
/// assert!(environment.variables.is_empty());
/// ```
#[derive(Default)]
pub(super) struct Environment {
    /// Next visibility frame, or None at the analysis root.
    /// Example: `environment.parent.is_none()`.
    pub parent: Option<ReferenceEnvironmentId>,
    /// Identity domain for variables created in this frame.
    /// Example: `environment.owner.index()`.
    pub owner: ReferenceEnvironmentId,
    /// Whether source belongs to a function/macro body, including nested frames.
    /// Example: `environment.deferred`.
    pub deferred: bool,
    /// Latest user-call clock that makes older warning-state facts unknown.
    /// Example: `environment.invalidate_after`.
    pub invalidate_after: usize,
    /// Locally visible variable bindings, drained between branches.
    /// Example: `environment.variables.len()`.
    pub variables: HashMap<String, Binding>,
    /// Lowercase names mapped to candidate aggregate function identities.
    /// Example: `environment.functions.len()`.
    pub functions: HashMap<String, Vec<IndexedFunctionId>>,
}
/// Per-visit file structure and lazily created reference frames. Workspace replay gives each dependency visit its own context even for a shared immutable snapshot. No I/O on construction.
///
/// [`ReferenceCollector::file`] initializes these lookup maps.
///
/// # Flow
/// ```text
///  [one dependency visit]      [immutable file snapshot]
///            |                           |
///            v                           v
///      [root environment]       [definition/block maps]
///            |                           |
///            +-------------+-------------+
///                          v
///                   [lazy body frames]
///                          |
///                          v
///                    [ordered commands]
/// ```
/// # Examples
/// ```text
/// let context = collector.file(buffer, root);
/// assert!(context.frames.is_empty());
/// ```
pub(crate) struct FileContext {
    /// Visibility environment at file entry, shared for includes or forked for subdirectories.
    /// Example: `context.root.index()`.
    pub(super) root: ReferenceEnvironmentId,
    /// Active recovered-block frames, built lazily and removed at conditional closers.
    /// Example: `context.frames.len()`.
    pub(super) frames: HashMap<BlockId, Frame>,
    /// Blocks known to create a lexical variable scope.
    /// Example: `context.local_blocks.contains(&block)`.
    pub(super) local_blocks: HashSet<BlockId>,
    /// Declaration command to snapshot-local function index.
    /// Example: `context.definitions.get(&command.id)`.
    pub(super) definitions: HashMap<CommandId, SnapshotFunctionId>,
    /// Function/macro body block to snapshot-local function index.
    /// Example: `context.function_blocks.get(&block)`.
    pub(super) function_blocks: HashMap<BlockId, SnapshotFunctionId>,
}
/// Recovered-block reference state and accumulated conditional alternatives. Merge uses possible branch writes, not condition evaluation; no I/O.
///
/// [`ReferenceCollector::environment`] creates frames lazily.
///
/// # Flow
/// ```text
///  [parent environment]
///           |
///           v
///   [branch environment]
///       /          \
///  [branch one] [branch two]
///       |          |
///       +-----+----+
///             v
///  [accumulated writes/functions]
///             |
///             v
///  [conservative parent merge]
/// ```
/// # Examples
/// ```text
/// let frame = context.frames.get(&block_id);
/// ```
pub(super) struct Frame {
    /// Arena frame used while indexing this body.
    /// Example: `frame.environment.index()`.
    pub environment: ReferenceEnvironmentId,
    /// Surrounding frame receiving conditional candidates at merge.
    /// Example: `frame.parent.index()`.
    pub parent: ReferenceEnvironmentId,
    /// Whether branch writes/functions must be merged on close.
    /// Example: `frame.conditional`.
    pub conditional: bool,
    /// Accumulated branch variable writes, with conflicting states reduced to Unknown.
    /// Example: `frame.writes.len()`.
    pub writes: HashMap<String, Binding>,
    /// Candidate declarations accumulated across branches.
    /// Example: `frame.functions.len()`.
    pub functions: HashMap<String, Vec<IndexedFunctionId>>,
}
/// Mutable source-order reference builder for local analysis and workspace dependency replay. It records candidate bindings and warnings without executing conditions or user bodies. No filesystem or process access.
///
/// [`Self::finish`] resolves remaining deferred candidate calls and returns the index.
///
/// # Flow
/// ```text
///  [file contexts]       [environment arena]
///          |                      |
///          +-----------+----------+
///                      v
///             [source-order command]
///                /      |       \
///           [reads]  [writes]  [calls]
///                |      |       |
///                +------+-------+
///                       v
///           [deduplicated aggregate index]
///                       |
///                       v
///        [finish deferred candidate calls]
/// ```
/// # Examples
/// ```text
/// let mut collector = ReferenceCollector::default();
/// let root = collector.root();
/// let index = collector.finish();
/// ```
#[derive(Default)]
pub(crate) struct ReferenceCollector {
    /// Aggregate records being built; document snapshots are not mutated.
    /// Example: `collector.index.variables.len()`.
    pub(super) index: ReferenceIndex,
    /// Visibility arena indexed only by ReferenceEnvironmentId.
    /// Example: `collector.environments.len()`.
    pub(super) environments: Vec<Environment>,
    /// Owner/name identity map into aggregate variables.
    /// Example: `collector.variables.len()`.
    pub(super) variables: HashMap<(ReferenceEnvironmentId, String), IndexedVariableId>,
    /// Source declaration identity map into aggregate functions.
    /// Example: `collector.functions.len()`.
    pub(super) functions: HashMap<(FileId, CommandId), IndexedFunctionId>,
    /// Known builtin variable names eligible for unknown-valued root bindings.
    /// Example: `collector.builtins.contains("CMAKE_SOURCE_DIR")`.
    pub(super) builtins: HashSet<String>,
    /// Normalized builtin command names mapped to aggregate function IDs.
    /// Example: `collector.builtin_functions.len()`.
    pub(super) builtin_functions: HashMap<String, IndexedFunctionId>,
    /// Deduplication keys for variable/source-range/reference-kind combinations.
    /// Example: `collector.reference_seen.len()`.
    pub(super) reference_seen: HashSet<(
        IndexedVariableId,
        FileId,
        CommandId,
        usize,
        usize,
        ReferenceKind,
    )>,
    /// Deduplication keys per candidate function and file command.
    /// Example: `collector.call_seen.len()`.
    pub(super) call_seen: HashSet<(IndexedFunctionId, FileId, CommandId)>,
    /// Deduplication keys for resolved user call locations.
    /// Example: `collector.resolved_seen.len()`.
    pub(super) resolved_seen: HashSet<(FileId, CommandId)>,
    /// Unresolved calls seen in deferred bodies, eligible for later static candidates.
    /// Example: `collector.deferred_calls.len()`.
    pub(super) deferred_calls: HashSet<(FileId, CommandId)>,
    /// Monotonic user-call invalidation counter, not an entity identity.
    /// Example: `collector.clock`.
    pub(super) clock: usize,
}
