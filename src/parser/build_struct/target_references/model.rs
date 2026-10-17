//! Collector-local target identities, declarations, pending uses and directory ancestry.
//! These owned staging records have no I/O and do not use variable-scope IDs.
// Contains AI-generated documentation additions.
use crate::parser::{
    build_struct::model::{AnalysisKey, TargetDeclaration, TargetKind, TargetReference},
    semanticer::{model::CommandId, schema::NameRole},
};
use std::collections::HashMap;

/// IDs belong to one TargetCollector; they are not Workspace variable-scope IDs.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
/// Index into one target collector directory tree; not a workspace variable scope. No side effects.
///
/// [`TargetCollector::directory`] allocates it; [`TargetCollector::file`] consumes it.
///
/// # Examples
/// ```text
/// let directory = collector.directory(None);
/// ```
pub(crate) struct TargetDirectoryId(/** Position. Example: `TargetDirectoryId(0)`. */ usize);
impl TargetDirectoryId {
    /// Wrap a collector-local position without checking vector bounds. No side effects.
    ///
    /// # Arguments
    /// - `index` `[in]`: Position in this collector's corresponding vector.
    ///
    /// # Returns
    /// Typed position; not interchangeable with another collector's IDs.
    ///
    /// # Examples
    /// ```text
    /// let id = TargetDirectoryId::new(0);
    /// ```
    pub(super) fn new(index: usize) -> Self {
        Self(index)
    }
    /// Read the collector-local numeric position. No side effects.
    ///
    /// # Arguments
    /// - `self` `[in]`: Copied typed position.
    ///
    /// # Returns
    /// Underlying vector index; no bounds check is performed.
    ///
    /// # Examples
    /// ```text
    /// let index = TargetDirectoryId::new(0).index();
    /// ```
    pub(super) fn index(self) -> usize {
        self.0
    }
}
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
/// Index into one collector declaration list. No side effects.
///
/// [`TargetCollector`] stores entries; [`Entry`] stores declaration visibility.
///
/// # Examples
/// ```text
/// let id = TargetEntryId::new(0); let index = id.index();
/// ```
pub(super) struct TargetEntryId(/** Position. Example: `TargetEntryId(0)`. */ usize);
impl TargetEntryId {
    /// Wrap a collector-local position without checking vector bounds. No side effects.
    ///
    /// # Arguments
    /// - `index` `[in]`: Position in this collector's corresponding vector.
    ///
    /// # Returns
    /// Typed position; not interchangeable with another collector's IDs.
    ///
    /// # Examples
    /// ```text
    /// let id = TargetEntryId::new(0);
    /// ```
    pub(super) fn new(index: usize) -> Self {
        Self(index)
    }
    /// Read the collector-local numeric position. No side effects.
    ///
    /// # Arguments
    /// - `self` `[in]`: Copied typed position.
    ///
    /// # Returns
    /// Underlying vector index; no bounds check is performed.
    ///
    /// # Examples
    /// ```text
    /// let index = TargetEntryId::new(0).index();
    /// ```
    pub(super) fn index(self) -> usize {
        self.0
    }
}
/// Staged target declaration with directory visibility and constructor class. No side effects.
///
/// [`TargetCollector::finish`] resolves uses against these entries.
///
/// # Examples
/// ```text
/// let local = entry.local;
/// ```
pub(super) struct Entry {
    /// Owned metadata and collected reference list Example: `entry.declaration.name.text.as_str()`.
    pub declaration: TargetDeclaration,
    /// Directory in which the declaration appeared Example: `entry.directory.index()`.
    pub directory: TargetDirectoryId,
    /// Whether visibility is restricted to this directory and descendants Example: `entry.local`.
    pub local: bool,
    /// Constructor class for typed uses; aliases use their constructor class Example:
    /// `entry.class == TargetKind::Executable` .
    pub class: TargetKind,
}
/// Literal schema-identified target operand awaiting visibility lookup. No side effects.
///
/// [`TargetCollector::file`] queues these; [`TargetCollector::finish`] binds unique names.
///
/// # Examples
/// ```text
/// let name = usage.name.as_str();
/// ```
pub(super) struct Use {
    /// Literal nonempty operand text Example: `usage.name.as_str()`.
    pub name: String,
    /// Target class required by the matched schema slot Example: `usage.role == NameRole::Target`.
    pub role: NameRole,
    /// Use-site directory controlling local-import visibility Example: `usage.directory.index()`.
    pub directory: TargetDirectoryId,
    /// Revision-qualified raw argument location Example: `usage.reference.span.start_byte`.
    pub reference: TargetReference,
}
#[derive(Default)]
/// Directory-aware staging area for local or workspace target binding. No I/O.
///
/// [`Self::directory`] establishes ancestry; [`Self::file`] gathers definitions/uses;
/// [`Self::finish`] binds them.
///
/// # Flow
/// ```text
/// [directory ancestry]      [literal definitions]
///           |                        |
///           +----> [visibility lookup]+
///                          ^
///                    [pending uses]
///                          |
///                     [bound targets]
/// ```
///
/// # Examples
/// ```text
/// let mut collector = TargetCollector::default();
/// let directory = collector.directory(None);
/// let targets = collector.finish();
/// ```
pub(crate) struct TargetCollector {
    /// Parent directory per allocated ID; None marks a root Example: `collector.directories.len()`.
    pub(super) directories: Vec<Option<TargetDirectoryId>>,
    /// Staged declarations, including duplicate visits until finish Example:
    /// `collector.entries.len()` .
    pub(super) entries: Vec<Entry>,
    /// Case-sensitive name to candidate declaration IDs Example: `collector.names.get("core")`.
    pub(super) names: HashMap<String, Vec<TargetEntryId>>,
    /// Analysis/command/directory deduplication keys; global declarations omit directory Example:
    /// `collector.definitions.len()` .
    pub(super) definitions:
        HashMap<(AnalysisKey, CommandId, Option<TargetDirectoryId>), TargetEntryId>,
    /// Pending reads appended by file collection Example: `collector.uses.len()`.
    pub(super) uses: Vec<Use>,
}
