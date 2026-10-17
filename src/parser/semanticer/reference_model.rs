//! Source-qualified reference records and workspace aggregates.
//!
//! [`ReferenceEnvironmentId`] belongs to one collector generation; [`ScopeId`] belongs
//! to one file snapshot. [`ReferenceInvocationId`] distinguishes dependency visits,
//! not calls that were actually executed. Spans use the owning immutable source.
// Contains AI-generated documentation additions.
// AI-authored snapshot-qualified reference and warning models.
use super::model::{BlockId, CommandId, FileId, ScopeId};
use crate::model::{Spanned, TextSpan};

/// Typed reference environment index; construction performs no lookup or I/O.
///
/// [`IndexedVariable::scope`] uses this domain. The number is valid only in the owning
/// snapshot or collector/match generation, not across rebuilt arenas.
///
/// # Examples
/// ```
/// use cmake_analyzer::parser::semanticer::model::ReferenceEnvironmentId;
/// let id = ReferenceEnvironmentId::default();
/// assert_eq!(id.index(), 0);
/// ```
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct ReferenceEnvironmentId(
    /// Zero-based reference environment arena slot; not range-checked.
    /// Example: `id.0` reads the stored slot inside this module.
    usize,
);
impl ReferenceEnvironmentId {
    /// Wrap a reference environment arena slot without checking its bounds. No side effects.
    ///
    /// # Arguments
    /// - `index` `[in]`: zero-based slot in the owning arena.
    ///
    /// # Returns
    /// Typed index; no allocation or lookup.
    ///
    /// # Examples
    /// ```text
    /// let id = ReferenceEnvironmentId::new(0);
    /// ```
    pub(crate) fn new(index: usize) -> Self {
        Self(index)
    }
    /// Read the stored reference environment arena slot. No side effects.
    ///
    /// # Arguments
    /// - `self` `[in]`: typed index copied by value.
    ///
    /// # Returns
    /// The original zero-based slot; no bounds validation.
    ///
    /// # Examples
    /// ```
    /// use cmake_analyzer::parser::semanticer::model::ReferenceEnvironmentId;
    /// let id = ReferenceEnvironmentId::default();
    /// assert_eq!(id.index(), 0);
    /// ```
    pub fn index(self) -> usize {
        self.0
    }
}
/// Typed dependency visit index; construction performs no lookup or I/O.
///
/// [`ReferenceWarning::invocation`] uses this domain. The number is valid only in the owning
/// snapshot or collector/match generation, not across rebuilt arenas.
///
/// # Examples
/// ```
/// use cmake_analyzer::parser::semanticer::model::*;
/// // Workspace replay supplies visit IDs; callers can inspect, but not construct them.
/// fn visit_number(visit: ReferenceInvocationId) -> usize { visit.index() }
/// let read_visit: fn(ReferenceInvocationId) -> usize = visit_number;
/// ```
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ReferenceInvocationId(
    /// Zero-based dependency visit arena slot; not range-checked.
    /// Example: `id.0` reads the stored slot inside this module.
    usize,
);
impl ReferenceInvocationId {
    /// Wrap a dependency visit arena slot without checking its bounds. No side effects.
    ///
    /// # Arguments
    /// - `index` `[in]`: zero-based slot in the owning arena.
    ///
    /// # Returns
    /// Typed index; no allocation or lookup.
    ///
    /// # Examples
    /// ```text
    /// let id = ReferenceInvocationId::new(0);
    /// ```
    pub(crate) fn new(index: usize) -> Self {
        Self(index)
    }
    /// Read the stored dependency visit arena slot. No side effects.
    ///
    /// # Arguments
    /// - `self` `[in]`: typed index copied by value.
    ///
    /// # Returns
    /// The original zero-based slot; no bounds validation.
    ///
    /// # Examples
    /// ```
    /// use cmake_analyzer::parser::semanticer::model::ReferenceInvocationId;
    /// fn slot(visit: ReferenceInvocationId) -> usize { visit.index() }
    /// let _ = slot;
    /// ```
    pub fn index(self) -> usize {
        self.0
    }
}

/// Source-use category for navigation and value positions, not a runtime event. No side effects.
///
/// [`ReferenceLocation`] qualifies its span and identity.
///
/// # Examples
/// ```
/// use cmake_analyzer::parser::semanticer::model::*;
/// let value = ReferenceKind::Definition;
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ReferenceKind {
    /// First established source definition or signature parameter.
    /// Example: `ReferenceKind::Definition`.
    Definition,
    /// Variable read extracted from an argument.
    /// Example: `ReferenceKind::Read`.
    Read,
    /// Source write to an already known or nonlocal variable.
    /// Example: `ReferenceKind::Write`.
    Write,
    /// Command-name use at a candidate call site.
    /// Example: `ReferenceKind::Call`.
    Call,
}

/// Source-qualified definition, read, write or call in an immutable analysis snapshot. Navigation consumers must pair file/command IDs and spans with that snapshot. Construction has no side effects.
///
/// [`ReferenceKind`] describes the source use.
///
/// # Examples
/// ```
/// use cmake_analyzer::{model::TextSpan, parser::semanticer::model::{ReferenceLocation, ReferenceKind, FileId, CommandId}};
/// let location = ReferenceLocation { possible_value: None, file: FileId { id: 1 }, command: CommandId { id: 0 }, span: TextSpan::default(), kind: ReferenceKind::Read };
/// ```
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ReferenceLocation {
    /// Optional text fact attached to this position; None is unknown, not empty.
    /// Example: `location.possible_value.is_none()`.
    pub possible_value: Option<super::model::PossibleValue>,
    /// Workspace file identity, meaningful with the owning snapshot generation.
    /// Example: `location.file.id`.
    pub file: FileId,
    /// Command index within that file snapshot.
    /// Example: `location.command.id`.
    pub command: CommandId,
    /// Half-open UTF-8 byte range plus source coordinates; argument reads use name spans.
    /// Example: `location.span.start_byte`.
    pub span: TextSpan,
    /// Definition/read/write/call classification.
    /// Example: `location.kind == ReferenceKind::Read`.
    pub kind: ReferenceKind,
}
/// A simple variable read extracted from one source argument before binding. The collector resolves its name later; construction performs no I/O.
///
/// [`ReferenceLocation`] qualifies the exact read position.
///
/// # Examples
/// ```
/// use cmake_analyzer::parser::semanticer::model::*;
/// use cmake_analyzer::model::TextSpan;
/// let location = ReferenceLocation {
///     possible_value: None, file: FileId { id: 1 }, command: CommandId { id: 0 },
///     span: TextSpan::default(), kind: ReferenceKind::Definition,
/// };
/// let reference = ArgumentReference { name: "x".into(), argument: 0, location };
/// assert_eq!(reference.argument, 0);
/// ```
#[derive(Clone, Debug)]
pub struct ArgumentReference {
    /// Literal case-sensitive variable name, without ${} delimiters.
    /// Example: `reference.name.as_str()`.
    pub name: String,
    /// Zero-based source argument slot containing the read.
    /// Example: `reference.argument`.
    pub argument: usize,
    /// Read position in the owning file snapshot.
    /// Example: `reference.location.kind == ReferenceKind::Read`.
    pub location: ReferenceLocation,
}
/// Source definition family or builtin identity. Creating it does not execute a body.
///
/// [`SemanticFunction`] stores definition and candidate references.
///
/// # Examples
/// ```
/// use cmake_analyzer::parser::semanticer::model::*;
/// let value = FunctionKind::Builtin;
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FunctionKind {
    /// Encountered native command without a source definition.
    /// Example: `FunctionKind::Builtin`.
    Builtin,
    /// Source function declaration.
    /// Example: `FunctionKind::Function`.
    Function,
    /// Source macro declaration.
    /// Example: `FunctionKind::Macro`.
    Macro,
}
/// Literal parameter declaration in a function or macro signature. Static body binding records it as an unknown-valued variable, without running a call.
///
/// [`SemanticFunction`] owns ordered parameters.
///
/// # Examples
/// ```
/// use cmake_analyzer::parser::semanticer::model::*;
/// use cmake_analyzer::model::TextSpan;
/// let location = ReferenceLocation {
///     possible_value: None, file: FileId { id: 1 }, command: CommandId { id: 0 },
///     span: TextSpan::default(), kind: ReferenceKind::Definition,
/// };
/// let parameter = FunctionParameter { name: "input".into(), definition: location };
/// assert_eq!(parameter.name, "input");
/// ```
#[derive(Clone, Debug)]
pub struct FunctionParameter {
    /// Literal parameter spelling.
    /// Example: `parameter.name.as_str()`.
    pub name: String,
    /// Source name range in the declaration command.
    /// Example: `parameter.definition.span.start_byte`.
    pub definition: ReferenceLocation,
}
/// User function/macro definition or encountered builtin command in a reference index. Navigation uses candidate calls, not executed calls; builtin records have no source definition or body. Construction performs no I/O.
///
/// [`FunctionParameter`] records literal signature parameters.
///
/// # Flow
/// ```text
///  [declaration source]         [builtin command]
///           |                         |
///     [name + body block]       [no source definition]
///           |                         |
///     [literal parameters]            |
///           +------------+------------+
///                        v
///               [candidate references]
///                        |
///                        v
///              [navigation, not execution]
/// ```
/// # Examples
/// ```
/// use cmake_analyzer::parser::semanticer::model::*;
/// let function = SemanticFunction {
///     documentation: None, name: "set".into(), kind: FunctionKind::Builtin,
///     block: None, scope: None, definition: None, parameters: vec![], references: vec![],
/// };
/// assert!(function.definition.is_none());
/// ```
#[derive(Clone, Debug)]
pub struct SemanticFunction {
    /// Leading declaration comments, absent for builtins.
    /// Example: `function.documentation.is_some()`.
    pub documentation: Option<std::sync::Arc<super::model::Documentation>>,
    /// Source spelling for user definitions; normalized name for builtin records.
    /// Example: `function.name.as_str()`.
    pub name: String,
    /// Builtin, function or macro identity.
    /// Example: `function.kind == FunctionKind::Function`.
    pub kind: FunctionKind,
    /// Definition body block; None for builtins.
    /// Example: `function.block.is_none()`.
    pub block: Option<BlockId>,
    /// Scope of the declaration opener, not the function body scope.
    /// Example: `function.scope.is_none()`.
    pub scope: Option<ScopeId>,
    /// Source name declaration; builtin commands have neither a definition nor body block.
    /// Example: `function.definition.is_none()`.
    pub definition: Option<ReferenceLocation>,
    /// Literal nonempty parameter names in source order.
    /// Example: `function.parameters.len()`.
    pub parameters: Vec<FunctionParameter>,
    /// Definition and statically resolved candidate call locations.
    /// Example: `function.references.len()`.
    pub references: Vec<ReferenceLocation>,
}
/// Conservative variable state at a read position, not runtime verification. No side effects.
///
/// [`SemanticWarn::message`] renders the state.
///
/// # Examples
/// ```
/// use cmake_analyzer::parser::semanticer::model::*;
/// let value = SemanticWarnKind::PossiblyUnset;
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SemanticWarnKind {
    /// No definite binding or only a possible branch assignment.
    /// Example: `SemanticWarnKind::PossiblyUnset`.
    PossiblyUnset,
    /// Source-order state explicitly unset.
    /// Example: `SemanticWarnKind::Unset`.
    Unset,
    /// Supported literal assignment established an empty string.
    /// Example: `SemanticWarnKind::Empty`.
    Empty,
}
/// A variable-state warning computed from conservative source-order binding. Reporting it does not prove runtime failure or run code.
///
/// [`Self::message`] renders the user-facing wording.
///
/// # Examples
/// ```
/// use cmake_analyzer::parser::semanticer::model::{SemanticWarn, SemanticWarnKind};
/// let warning = SemanticWarn { variable: "x".into(), kind: SemanticWarnKind::PossiblyUnset };
/// assert!(warning.message().contains("may be unset"));
/// ```
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SemanticWarn {
    /// Variable spelling quoted in the diagnostic message.
    /// Example: `warning.variable.as_str()`.
    pub variable: String,
    /// Degree of unset/empty knowledge at this read.
    /// Example: `warning.kind == SemanticWarnKind::PossiblyUnset`.
    pub kind: SemanticWarnKind,
}
impl SemanticWarn {
    /// Render the variable spelling and conservative unset/empty state into a message. No reporting or I/O occurs.
    ///
    /// # Arguments
    /// - `self` `[in]`: warning payload, unchanged.
    ///
    /// # Returns
    /// Owned message using debug-quoted variable spelling and the state wording.
    ///
    /// # Examples
    /// ```
    /// use cmake_analyzer::parser::semanticer::model::*;
    /// let warning = SemanticWarn { variable: "x".into(), kind: SemanticWarnKind::Unset };
    /// assert_eq!(warning.message(), "Variable \"x\" is unset");
    /// ```
    pub fn message(&self) -> String {
        let state = match self.kind {
            SemanticWarnKind::PossiblyUnset => "may be unset",
            SemanticWarnKind::Unset => "is unset",
            SemanticWarnKind::Empty => "is empty",
        };
        format!("Variable {:?} {state}", self.variable)
    }
}
/// Namespace requested by an unresolved source use. Construction has no side effects.
///
/// [`UnresolvedReference`] stores name and use location.
///
/// # Examples
/// ```
/// use cmake_analyzer::parser::semanticer::model::*;
/// let value = ReferenceTarget::Variable;
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ReferenceTarget {
    /// Case-sensitive variable namespace.
    /// Example: `ReferenceTarget::Variable`.
    Variable,
    /// Case-insensitive command/function namespace.
    /// Example: `ReferenceTarget::Function`.
    Function,
}
/// A source use with no binding established by this analysis. Dynamic names, nonlocal writes and unknown calls can remain unresolved without proving an invalid project. No I/O.
///
/// [`ReferenceTarget`] distinguishes variable and function lookup.
///
/// # Examples
/// ```
/// use cmake_analyzer::parser::semanticer::model::*;
/// use cmake_analyzer::model::TextSpan;
/// let location = ReferenceLocation {
///     possible_value: None, file: FileId { id: 1 }, command: CommandId { id: 0 },
///     span: TextSpan::default(), kind: ReferenceKind::Definition,
/// };
/// let unresolved = UnresolvedReference { name: "x".into(), target: ReferenceTarget::Variable, location };
/// assert_eq!(unresolved.target, ReferenceTarget::Variable);
/// ```
#[derive(Clone, Debug)]
pub struct UnresolvedReference {
    /// Unbound source spelling.
    /// Example: `unresolved.name.as_str()`.
    pub name: String,
    /// Which namespace could not be bound.
    /// Example: `unresolved.target == ReferenceTarget::Variable`.
    pub target: ReferenceTarget,
    /// Exact source use, including its reference kind.
    /// Example: `unresolved.location.span.start_byte`.
    pub location: ReferenceLocation,
}
/// Workspace aggregate for a variable name owned by one reference environment. Repeated file visits can contribute locations; this is not a runtime variable instance. No I/O.
///
/// [`ReferenceIndex`] owns the aggregate vector.
///
/// # Examples
/// ```
/// use cmake_analyzer::parser::semanticer::model::*;
/// let variable = IndexedVariable {
///     name: "x".into(), scope: ReferenceEnvironmentId::default(), local_scope: None,
///     possible_value: None, definition: None, references: vec![],
/// };
/// assert!(variable.possible_value.is_none());
/// ```
#[derive(Clone, Debug)]
pub struct IndexedVariable {
    /// Case-sensitive variable spelling.
    /// Example: `variable.name.as_str()`.
    pub name: String,
    /// Collector-generation owner, not a file-local ScopeId.
    /// Example: `variable.scope.index()`.
    pub scope: ReferenceEnvironmentId,
    /// Lexical scope in the originating file snapshot, when available.
    /// Example: `variable.local_scope.is_none()`.
    pub local_scope: Option<ScopeId>,
    /// Aggregate text fact only when replay observations agree.
    /// Example: `variable.possible_value.is_none()`.
    pub possible_value: Option<super::model::PossibleValue>,
    /// First statically established definition if available.
    /// Example: `variable.definition.is_some()`.
    pub definition: Option<ReferenceLocation>,
    /// Deduplicated source definitions, reads and writes across visits.
    /// Example: `variable.references.len()`.
    pub references: Vec<ReferenceLocation>,
}
/// Variable warning qualified by the dependency visit that produced it. Workspace aggregation keeps this separate from deduplicated navigation locations. No I/O.
///
/// [`SemanticWarn`] contains message data.
///
/// # Examples
/// ```
/// use cmake_analyzer::parser::semanticer::model::*;
/// // A visit-qualified warning is supplied by the workspace reference index.
/// fn message(warning: &ReferenceWarning) -> String {
///     let visit = warning.invocation.index();
///     format!("visit {visit}: {}", warning.warning.content.message())
/// }
/// let render_warning: fn(&ReferenceWarning) -> String = message;
/// ```
#[derive(Clone, Debug)]
pub struct ReferenceWarning {
    /// RootGraph visit identity in the owning index generation.
    /// Example: `warning.invocation.index()`.
    pub invocation: ReferenceInvocationId,
    /// Read location that triggered the warning.
    /// Example: `warning.location.span.start_byte`.
    pub location: ReferenceLocation,
    /// Source-spanned warning payload.
    /// Example: `warning.warning.content.message()`.
    pub warning: Spanned<SemanticWarn>,
}
/// Workspace-owned reference aggregates over immutable document snapshots. Navigation and warnings use source IDs qualified by the index generation; building this structure does not execute a project.
///
/// [`IndexedVariable`] stores environment-qualified variable aggregates.
///
/// # Flow
/// ```text
///  [immutable file A]       [immutable file B]
///           |                         |
///           +------------+------------+
///                        v
///             [ordered dependency visits]
///                 /            \
///        [variable owners] [function candidates]
///                 |            |
///                 +------+-----+
///                        v
///          [references / warnings / unresolved]
/// ```
/// # Examples
/// ```
/// use cmake_analyzer::parser::semanticer::model::ReferenceIndex;
/// let index = ReferenceIndex::default();
/// assert!(index.variables.is_empty());
/// ```
#[derive(Clone, Debug, Default)]
pub struct ReferenceIndex {
    /// Variables grouped by owner environment and exact name.
    /// Example: `index.variables.len()`.
    pub variables: Vec<IndexedVariable>,
    /// User definitions and encountered builtin command identities.
    /// Example: `index.functions.len()`.
    pub functions: Vec<SemanticFunction>,
    /// Source uses not statically bound, including deferred calls with no candidates.
    /// Example: `index.unresolved.len()`.
    pub unresolved: Vec<UnresolvedReference>,
    /// Visit-qualified unset/empty read warnings.
    /// Example: `index.warnings.len()`.
    pub warnings: Vec<ReferenceWarning>,
    /// Deduplicated calls bound to user-definition candidates.
    /// Example: `index.resolved_calls.len()`.
    pub resolved_calls: Vec<ReferenceLocation>,
}
