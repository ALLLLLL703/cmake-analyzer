//! Execution-qualified values, function calls and optional source mappings for a runtime layer.
//! Records preserve repeated events and frames independently of static file/visit indexes; no I/O.
// Contains AI-generated documentation additions.
// AI-authored runtime layer models; event order and frames retain repeated executions.
use super::super::model::DocumentSnapshot;
use crate::{
    parser::semanticer::model::{FunctionKind, PossibleValue, ReferenceLocation},
    runtime_cmake::model::{EventId, RuntimeFrame, Trace},
};
use std::{collections::BTreeMap, path::PathBuf, sync::Arc};
#[repr(transparent)]
#[derive(Clone, Copy)]
/// Private position in one runtime layer's function list. No side effects.
///
/// [`RuntimeFunction`] holds the corresponding observed declaration and calls.
///
/// # Examples
/// ```text
/// let index = RuntimeFunctionId::new(0).index();
/// ```
pub(super) struct RuntimeFunctionId(/** Position. Example: `RuntimeFunctionId(0)`. */ usize);
impl RuntimeFunctionId {
    /// Wrap a layer-local function position without bounds checks. No side effects.
    ///
    /// # Arguments
    /// - `index` `[in]`: Position in the runtime function vector.
    ///
    /// # Returns
    /// Typed index local to the current layer.
    ///
    /// # Examples
    /// ```text
    /// let id = RuntimeFunctionId::new(0);
    /// ```
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    /// Read the layer-local function position. No side effects.
    ///
    /// # Arguments
    /// - `self` `[in]`: Copied function identifier.
    ///
    /// # Returns
    /// Underlying usize; no bounds validation occurs.
    ///
    /// # Examples
    /// ```text
    /// let index = RuntimeFunctionId::new(0).index();
    /// ```
    pub fn index(self) -> usize {
        self.0
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
/// Assignment namespace or uncertainty category observed in a trace. No side effects.
///
/// [`RuntimeValueAt::namespace`] distinguishes ordinary, cache and parent-scope values without
/// inventing cross-scope effects.
///
/// # Examples
/// ```
/// use cmake_analyzer::workspace::runtime::ValueNamespace;
/// let namespace = ValueNamespace::Ordinary;
/// ```
pub enum ValueNamespace {
    /// Trace arguments may have split/vanished; namespace cannot be trusted Example:
    /// `ValueNamespace::Unknown` .
    Unknown,
    /// Ordinary variable assignment/read in the observed frame Example: `ValueNamespace::Ordinary`.
    Ordinary,
    /// CACHE assignment/unset; values are confirmed only for forced set operations Example:
    /// `ValueNamespace::Cache` .
    Cache,
    /// PARENT_SCOPE write; destination values are not modeled Example:
    /// `ValueNamespace::ParentScope` .
    ParentScope,
}
#[derive(Clone, Debug)]
/// One execution-qualified assignment/read observation; not a complete runtime variable store. No
/// I/O on access.
///
/// [`RuntimeLayer::values`] preserves repeated executions; [`ValueNamespace`] explains unconfirmed
/// destinations.
///
/// # Examples
/// ```
/// use cmake_analyzer::workspace::runtime::*;
/// fn inspect(value: &RuntimeValueAt) {
///     let _ = value.event.index();
/// }
/// ```
pub struct RuntimeValueAt {
    /// Trace event that produced this observation Example: `value.event.index()`.
    pub event: EventId,
    /// Trace frame depth retained without guessing static caller scopes Example:
    /// `value.frame.depth()` .
    pub frame: RuntimeFrame,
    /// Observed variable name Example: `value.name.as_str()`.
    pub name: String,
    /// Source command span for assignments, or a name span for reads.
    /// Optional source command span for writes or variable-name span for reads; absent for unmapped
    /// events Example: `value.location.as_ref()` .
    pub location: Option<ReferenceLocation>,
    /// Ordinary/cache/parent/unknown destination category Example:
    /// `value.namespace == ValueNamespace::Ordinary` .
    pub namespace: ValueNamespace,
    /// Observed value only when assignment/read mapping is sufficiently unambiguous Example:
    /// `value.possible_value.as_ref().map(|value| value.text())` .
    pub possible_value: Option<PossibleValue>,
}
#[derive(Clone, Debug)]
/// Observed function/macro declaration and ordered calls. No I/O on access.
///
/// [`RuntimeLayer::functions`] owns these execution-qualified records, separate from static
/// function binding.
///
/// # Examples
/// ```
/// use cmake_analyzer::workspace::runtime::*;
/// fn inspect(function: &RuntimeFunction) {
///     let _ = function.name.as_str();
/// }
/// ```
pub struct RuntimeFunction {
    /// Declaration's first expanded argument, retaining its spelling Example:
    /// `function.name.as_str()` .
    pub name: String,
    /// Function or Macro declaration category Example: `function.kind`.
    pub kind: FunctionKind,
    /// Event ID of the observed declaration Example: `function.declaration.index()`.
    pub declaration: EventId,
    /// Event IDs of later calls matched by case-insensitive active name Example:
    /// `function.calls.len()` .
    pub calls: Vec<EventId>,
}
#[derive(Clone, Debug)]
/// Immutable execution overlay with repeated events, optional source mappings and observed
/// values/functions. Reading it does not execute CMake.
///
/// [`super::super::model::Workspace::install_runtime`] builds it from an existing trace; static
/// [`DocumentSnapshot`] data is not rewritten.
///
/// # Flow
/// ```text
/// [executed trace]          [saved source snapshots]
///          |                         |
///          +----> [optional mapping]+
///                         |
///                  [event-order replay]
///                   /             \
///           [values/frames]   [declarations/calls]
///                   \             /
///                  [independent shared layer]
/// ```
///
/// # Examples
/// ```
/// use cmake_analyzer::workspace::runtime::*;
/// fn inspect(layer: &RuntimeLayer) {
///     let _ = layer.source_commands.len();
/// }
/// ```
pub struct RuntimeLayer {
    /// EventId indexes this map; ambiguous/missing source commands remain None.
    /// Event-indexed unique same-line/name source command mapping; None for absent or ambiguous
    /// mapping Example: `layer.source_commands.len()` .
    pub source_commands: Vec<Option<crate::parser::semanticer::model::CommandId>>,
    /// Original executed event/dependency sequence Example: `layer.trace.events.len()`.
    pub trace: Trace,
    /// Shared source snapshots successfully loaded for observed files Example:
    /// `layer.documents.len()` .
    pub documents: BTreeMap<PathBuf, Arc<DocumentSnapshot>>,
    /// Execution-qualified sparse reads/writes with possible observed values Example:
    /// `layer.values.len()` .
    pub values: Vec<RuntimeValueAt>,
    /// Observed declarations with call event lists Example: `layer.functions.len()`.
    pub functions: Vec<RuntimeFunction>,
    /// Event-indexed half-open ranges into values Example: `layer.event_values.first().cloned()`.
    pub event_values: Vec<std::ops::Range<usize>>,
    /// Mapped user-call command IDs per file, for supplementing diagnostics Example:
    /// `layer.resolved_calls.len()` .
    pub resolved_calls: BTreeMap<PathBuf, Vec<crate::parser::semanticer::model::CommandId>>,
}
