//! Snapshot-qualified hover selection records shared by lookup and rendering; no side effects.

use crate::parser::semanticer::model::ReferenceLocation;
use lspally::server::model::Range;
/// A hover site qualified by a reference in one immutable snapshot; constructing it has no side effects.
/// Lookup in the sibling targets module produces it; variable/command rendering consumes it.
/// [`TargetKind`] chooses source-value versus native-help treatment, while [`ReferenceLocation`]
/// keeps command and byte-span identity and [`Range`] supplies client coordinates.
///
/// # Examples
/// ```text
/// let target = Target { name: "VALUE".into(), location, range, kind: TargetKind::Variable };
/// ```
pub(super) struct Target {
    /// Source spelling to resolve across owners or the native catalog. Example: `target.name.as_str()`
    /// reads the identifier.
    pub name: String,
    /// Snapshot-local file/command/span and reference role. Example: `target.location.command` reads its
    /// command identity.
    pub location: ReferenceLocation,
    /// Name-only range in negotiated client coordinates. Example: `let start = &target.range.start`
    /// reads the hover start.
    pub range: Range,
    /// Lookup route chosen at the cursor. Example: `target.kind = TargetKind::Command` selects command
    /// documentation.
    pub kind: TargetKind,
}
/// Hover routing choice produced by source lookup; selecting it has no side effects.
/// Used with [`Target`]; ordinary variables can combine values/comments/help, while literal fallbacks
/// request help only. Commands prefer known user-function documentation over native help.
///
/// # Examples
/// ```text
/// let kind = TargetKind::BuiltinVariable;
/// assert!(matches!(kind, TargetKind::BuiltinVariable));
/// ```
pub(super) enum TargetKind {
    /// A bound, unresolved or output variable site. Example: `matches!(target.kind,
    /// TargetKind::Variable)` tests the value/comment route.
    Variable,
    /// A literal argument fallback, not proof of a builtin variable. Example:
    /// `TargetKind::BuiltinVariable` requests catalog-only lookup.
    BuiltinVariable,
    /// A command name or function declaration site. Example: `TargetKind::Command` selects source
    /// documentation before native help.
    Command,
}
