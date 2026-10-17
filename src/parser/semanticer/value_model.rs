//! Optional text facts and directory inputs for conservative value tracking.
//!
//! [`PossibleValue::Static`] is inferred text, not proof that a project executed.
//! [`PossibleValue::Observed`] is externally supplied text. Neither creates runtime effects.
// Contains AI-generated documentation additions.
// AI-authored conservative value facts and sparse command observations.
use super::model::{ReferenceInvocationId, ReferenceLocation, ScopeId};
use std::{collections::BTreeMap, path::PathBuf, sync::Arc};

/// Optional text fact labeled by its origin, not a guarantee of runtime truth. Constructing a fact has no side effects.
///
/// [`ValueBindings`] stores conservative facts by variable name.
///
/// # Examples
/// ```
/// use cmake_analyzer::parser::semanticer::model::*;
/// let value = PossibleValue::Static("yes".into());
/// ```
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PossibleValue {
    /// Text inferred from supported literals/substitutions without execution.
    /// Example: `PossibleValue::Static("yes".into())`.
    Static(
        /// Inferred text; e.g. `PossibleValue::Static("yes".into())`.
        Arc<str>,
    ),
    /// Text supplied from external observations; this layer does not collect it by executing projects.
    /// Example: `PossibleValue::Observed("yes".into())`.
    Observed(
        /// Externally supplied text; e.g. `PossibleValue::Observed("yes".into())`.
        Arc<str>,
    ),
}
impl PossibleValue {
    /// Borrow text without changing its Static/Observed origin tag. No side effects.
    ///
    /// # Arguments
    /// - `self` `[in]`: value fact borrowed without mutation.
    ///
    /// # Returns
    /// Shared string contents for either origin variant.
    ///
    /// # Examples
    /// ```
    /// use cmake_analyzer::parser::semanticer::model::*;
    /// let value = PossibleValue::Observed("yes".into());
    /// assert_eq!(value.text(), "yes");
    /// ```
    pub fn text(&self) -> &str {
        match self {
            Self::Static(value) | Self::Observed(value) => value,
        }
    }
}
/// Optional text fact at a source read or write, optionally qualified by a dependency visit. Hover and target analysis use sparse observations; creating one neither runs nor observes a project.
///
/// [`PossibleValue`] labels the fact origin.
///
/// # Examples
/// ```
/// use cmake_analyzer::parser::semanticer::model::*;
/// use cmake_analyzer::model::TextSpan;
/// let location = ReferenceLocation {
///     possible_value: None, file: FileId { id: 1 }, command: CommandId { id: 0 },
///     span: TextSpan::default(), kind: ReferenceKind::Definition,
/// };
/// let fact = VariableValueAt { name: "x".into(), location, scope: None, invocation: None, possible_value: None };
/// assert!(fact.possible_value.is_none());
/// ```
#[derive(Clone, Debug)]
pub struct VariableValueAt {
    /// Case-sensitive variable name.
    /// Example: `fact.name.as_str()`.
    pub name: String,
    /// Source-qualified read or write; reads use name spans, writes use raw output spans.
    /// Example: `fact.location.span.start_byte`.
    pub location: ReferenceLocation,
    /// File-local lexical variable scope.
    /// Example: `fact.scope.is_none()`.
    pub scope: Option<ScopeId>,
    /// Dependency visit when workspace replay produced the fact; None for a local pass.
    /// Example: `fact.invocation.is_some()`.
    pub invocation: Option<ReferenceInvocationId>,
    /// Known text if conservative tracking succeeded; None does not mean empty.
    /// Example: `fact.possible_value.is_none()`.
    pub possible_value: Option<PossibleValue>,
}
/// Directory context supplied by workspace analysis for builtin path variables. These are inputs to text inference, not verified filesystem state; construction has no I/O.
///
/// [`ValueBindings::get`] uses these paths for builtin fallback.
///
/// # Examples
/// ```
/// use cmake_analyzer::parser::semanticer::model::ValueDirectories;
/// let dirs = ValueDirectories { source_root: "/src".into(), source: "/src/sub".into(), binary_root: None, binary: None, file: "/src/sub/CMakeLists.txt".into() };
/// ```
#[derive(Clone, Debug)]
pub struct ValueDirectories {
    /// Top-level source directory of the analysis entry.
    /// Example: `dirs.source_root.as_path()`.
    pub source_root: PathBuf,
    /// Source directory active for this dependency visit.
    /// Example: `dirs.source.as_path()`.
    pub source: PathBuf,
    /// Top-level binary directory if known.
    /// Example: `dirs.binary_root.is_none()`.
    pub binary_root: Option<PathBuf>,
    /// Binary directory active for this visit if known.
    /// Example: `dirs.binary.is_none()`.
    pub binary: Option<PathBuf>,
    /// Current list file, also used to infer its parent directory.
    /// Example: `dirs.file.parent()`.
    pub file: PathBuf,
}
/// Copy-on-write map of limited value facts with lazy epoch invalidation. Local analysis and workspace replay clone bindings cheaply; edits affect only the modified map, not shared clones. No I/O.
///
/// [`Self::apply`] updates facts from a source command without executing it.
///
/// # Flow
/// ```text
///  [clone A]             [clone B]
///      |                     |
///      +---------+-----------+
///                v
///        [shared name map]
///                |
///          <mutation?>
///           /       \
///  [copy if shared] [read epoch]
///           |          |
///  [new entry stamp] <stamp current?>
///                    /         \
///                [fact]     [unknown]
/// ```
/// # Examples
/// ```
/// use cmake_analyzer::parser::semanticer::model::{ValueBindings, PossibleValue};
/// let mut values = ValueBindings::default();
/// values.assign("x", Some(PossibleValue::Static("yes".into())));
/// assert_eq!(values.get("x", None).unwrap().text(), "yes");
/// ```
#[derive(Clone, Debug, Default)]
pub struct ValueBindings {
    /// Shared ordered name map; mutation uses Arc::make_mut.
    /// Example: `values.entries.len()`.
    pub(super) entries: Arc<BTreeMap<String, ValueBinding>>,
    /// Current validity generation; older entries are unknown, not removed.
    /// Example: `values.epoch`.
    pub(super) epoch: usize,
}
/// Stored text fact stamped with the assignment epoch. Lookup treats older epochs as unknown without scanning the map; construction has no I/O.
///
/// [`super::model::ValueBindings`] owns the map and current epoch.
///
/// # Examples
/// ```text
/// let binding = ValueBinding { value: None, epoch: 0 };
/// ```
#[derive(Clone, Debug)]
pub(super) struct ValueBinding {
    /// Known text, or an explicit unknown that masks directory fallback.
    /// Example: `binding.value.is_none()`.
    pub value: Option<PossibleValue>,
    /// Validity generation when assigned.
    /// Example: `binding.epoch == 0`.
    pub epoch: usize,
}
