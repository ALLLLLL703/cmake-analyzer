//! Immutable completion inputs and edit plans; constructing these records has no side effects.
//! Candidates describe possible spellings, not proven bindings or runtime visibility.

use crate::{parser::semanticer::model::ReferenceIndex, workspace::model::DocumentSnapshot};
use std::{ops::Range, sync::Arc};

/// Cursor classification used by [`super::query`] before optional catalogue reads.
/// Creation performs no I/O; [`Input`] keeps the source used to interpret byte ranges.
///
/// # Flow
/// ```text
///          [Cursor classification]
///                    |
///          +---------+---------+
///          v                   v
///     [Commands]          [Arguments]
///          |                   |
///   [schema names]     [role-gated discovery]
///          +---------+---------+
///                    v
///             [Candidate rendering]
/// ```
/// # Examples
/// ```text
/// let plan = Prepared::Commands(site);
/// ```
pub(super) enum Prepared {
    /// Command-name replacement. Example: `Prepared::Commands(site)` skips discovery.
    Commands(
        /// Command spelling span and prefix. Example: `Prepared::Commands(site)` stores the edit site.
        Site,
    ),
    /// Argument replacement. Example: `Prepared::Arguments { site, expansion: true, requests: vec![] }`.
    Arguments {
        /// Payload or variable-name edit. Example: `Prepared::Arguments { site, expansion: false, requests }`.
        site: Site,
        /// Ordinary `${...}` name mode.
        /// Example: `Prepared::Arguments { site, expansion: true, requests }` restricts choices to variables.
        expansion: bool,
        /// Discovery flags aligned with captured owners.
        /// Example: `if let Prepared::Arguments { requests, .. } = plan { assert_eq!(requests.len(), inputs.len()); }`.
        requests: Vec<crate::workspace::completion::model::Request>,
    },
}
/// One owner's retained analysis for [`super::names::collect`] and command collection.
/// [`super::inputs::capture`] clones shared snapshots, not mutable analysis arenas.
/// File and command IDs are meaningful only with this owner's [`ReferenceIndex`].
/// Construction performs no I/O; later catalogue loading may cache additional snapshots.
///
/// # Flow
/// ```text
///    [Workspace owner]               [Other owner]
///           |                             |
///           v                             v
///    [Input snapshot/index]         [separate Input]
///           |
///      +----+-----+
///      v          v
///  [sources]  [catalogue]
///      +----+-----+
///           v
///   [possible names only]
/// ```
/// # Examples
/// ```text
/// let input = super::inputs::capture(workspace, path)?;
/// let analysis = input.snapshot.build.analysis;
/// ```
pub(super) struct Input {
    /// Discovered module/package names and supplemental target sources.
    /// Example: `input.catalog.modules.iter()` reads module spellings without binding them.
    pub catalog: crate::workspace::completion::model::Catalog,
    /// Queried text and immutable analysis. Example: `input.snapshot.text.len()` reads its byte length.
    pub snapshot: Arc<DocumentSnapshot>,
    /// Owner-local bindings when a graph exists. Example: `input.index.as_ref().map(|i| i.functions.len())`.
    pub index: Option<Arc<ReferenceIndex>>,
    /// Unique full-indexed dependency snapshots. Example: `input.sources.iter().filter(|s| s.variables_visible)`.
    pub sources: Vec<Source>,
}
/// Retained dependency analysis used by [`super::names::collect`]; creation has no side effects.
/// [`Input`] owns these records after directory-scope ancestry is checked.
///
/// # Examples
/// ```text
/// let source = Source { snapshot: snapshot.clone(), variables_visible: true };
/// ```
pub(super) struct Source {
    /// Shared source facts; never edited by completion. Example: `source.snapshot.build.facts.len()`.
    pub snapshot: Arc<DocumentSnapshot>,
    /// Whether this visit's directory scope is current or ancestral.
    /// Example: `assert!(source.variables_visible)` checks that directory visibility was retained.
    pub variables_visible: bool,
}
/// Borrowed argument slot found by [`super::argument_context::argument`]; no side effects.
/// [`super::roles::choices`] uses its command and ordinal to probe the schema.
///
/// # Examples
/// ```text
/// let slot = ArgumentSite { command, argument: 0, payload: 8..11, form: SemanticArgForm::Unquoted };
/// ```
pub(super) struct ArgumentSite<'a> {
    /// Owning command in this snapshot. Example: `slot.command.args.get(slot.argument)` reads a parsed operand.
    pub command: &'a crate::parser::semanticer::model::SemanticCommandRepresent,
    /// Zero-based slot, possibly a gap or unfinished final operand.
    /// Example: `slot.argument == slot.command.args.len()` identifies an appended slot.
    pub argument: usize,
    /// Half-open UTF-8 byte range excluding quote/bracket wrappers.
    /// Example: `snapshot.text.get(slot.payload.clone())` reads the editable payload.
    pub payload: Range<usize>,
    /// Original quoting form. Example: `slot.form == SemanticArgForm::Bracket` disables expansion completion.
    pub form: crate::parser::semanticer::model::SemanticArgForm,
}
/// Schema-supported active-slot choices from [`super::roles::choices`]; no side effects.
/// [`super::names::collect`] expands roles into possible names, not reference bindings.
///
/// # Examples
/// ```text
/// let choices = ArgumentChoices::default();
/// assert!(choices.keywords.is_empty());
/// ```
#[derive(Default)]
pub(super) struct ArgumentChoices<'a> {
    /// Name categories allowed at this slot. Example: `choices.roles.contains(&NameRole::Variable)`.
    pub roles: Vec<crate::parser::semanticer::schema::NameRole>,
    /// Borrowed reachable schema keywords. Example: `choices.keywords.iter().any(|word| *word == "PRIVATE")`.
    pub keywords: Vec<&'a str>,
}
/// Byte-based edit plan consumed by [`super::render::item`]; creation has no side effects.
/// [`super::context::site`] and argument context preserve surrounding syntax.
///
/// # Examples
/// ```text
/// let site = Site { replace: 0..3, prefix: "mes".into(), has_parentheses: false };
/// ```
pub(super) struct Site {
    /// Entire spelling to replace, including text after the cursor but excluding wrappers.
    /// Example: `site.replace = 2..7` replaces five UTF-8 bytes.
    pub replace: Range<usize>,
    /// Text before the cursor; lowercase for command names, case-preserving for arguments.
    /// Example: `site.prefix.as_str() == "mes"` filters `message`.
    pub prefix: String,
    /// Existing opening parenthesis after a command name.
    /// Example: `site.has_parentheses = true` prevents inserting a second call.
    pub has_parentheses: bool,
}
/// Presentation category converted by [`super::render::item`] into protocol numeric kinds.
/// Used in [`Candidate`] and name deduplication; selecting a category has no side effects.
///
/// # Examples
/// ```text
/// let category = Kind::Variable;
/// ```
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(super) enum Kind {
    /// Control word or argument keyword. Example: `Kind::Keyword` renders protocol kind 14.
    Keyword,
    /// Build command or module/package spelling. Example: `Kind::Module` renders protocol kind 9.
    Module,
    /// Other command or source function. Example: `Kind::Function` renders protocol kind 3.
    Function,
    /// Variable or formal parameter name. Example: `Kind::Variable` renders protocol kind 6.
    Variable,
    /// Target or language name. Example: `Kind::EnumMember` renders protocol kind 20.
    EnumMember,
}
/// Required call fragment emitted by [`super::arguments::snippet`]; creation has no side effects.
/// [`super::arguments::parameters`] keeps only common required schema prefixes.
///
/// # Examples
/// ```text
/// let parameter = Parameter::Placeholder("variable".into());
/// ```
#[derive(Clone, PartialEq, Eq)]
pub(super) enum Parameter {
    /// Fixed word, not a tab stop. Example: `Parameter::Literal("VERSION".into())`.
    Literal(
        /// Unescaped keyword text. Example: `Parameter::Literal("VERSION".into())` stores `VERSION`.
        String,
    ),
    /// Editable snippet tab stop. Example: `Parameter::Placeholder("path".into())`.
    Placeholder(
        /// Unescaped default label. Example: `Parameter::Placeholder("path".into())` renders `${1:path}`.
        String,
    ),
}
/// Possible insertion from [`super::candidates::collect`] or [`super::names::collect`].
/// This is not a bound reference or proof that a name exists at runtime.
/// [`super::render::item`] makes the client edit; construction has no side effects.
///
/// # Examples
/// ```text
/// let candidate = Candidate { name: "message".into(), kind: Kind::Function,
///     parameters: None, source_defined: false };
/// ```
pub(super) struct Candidate {
    /// Display and insertion spelling. Example: `candidate.name.as_str() == "message"`.
    pub name: String,
    /// Presentation and deduplication category. Example: `candidate.kind = Kind::Keyword`.
    pub kind: Kind,
    /// Required call pieces when snippets are authorized; `None` means name-only insertion.
    /// Example: `candidate.parameters = Some(vec![])` enables an empty `name()$0` snippet.
    pub parameters: Option<Vec<Parameter>>,
    /// Source declaration precedence over native command names.
    /// Example: `candidate.source_defined = true` marks a user shadow.
    pub source_defined: bool,
}
