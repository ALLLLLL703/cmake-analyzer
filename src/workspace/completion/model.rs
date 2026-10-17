//! Completion-only catalogs, invalidatable name caches and directory-qualified source visits.
//! These records have no I/O and do not install ordinary graph/reference/value bindings.
// Contains AI-generated documentation additions.
use crate::{
    parser::semanticer::model::CommandId,
    workspace::model::{DirectoryContext, DocumentSnapshot},
};
use std::{path::PathBuf, sync::Arc};
#[derive(Default)]
/// Requested module/package labels and supplemental target sources. No side effects.
///
/// [`super::super::Workspace::completion_catalog`] fills this record; consumers do not install
/// graph bindings.
///
/// # Examples
/// ```text
/// let _ = catalog.modules.len();
/// ```
pub(crate) struct Catalog {
    /// Shared sorted bare module names, without .cmake Example: `catalog.modules.len()`.
    pub modules: Arc<[String]>,
    /// Shared sorted config/Find-module package names Example: `catalog.packages.len()`.
    pub packages: Arc<[String]>,
    /// Supplemental parsed sources for literal target names only Example: `catalog.targets.len()`.
    pub targets: Vec<TargetSource>,
}
/// Supplemental immutable source with query-directory visibility. No I/O on access.
///
/// [`Catalog::targets`] carries these to completion; [`DocumentSnapshot`] remains unmodified.
///
/// # Examples
/// ```text
/// let _ = source.snapshot.build.facts.len();
/// ```
pub(crate) struct TargetSource {
    /// Parsed package/include source shared from workspace cache Example:
    /// `source.snapshot.build.facts.len()` .
    pub snapshot: Arc<DocumentSnapshot>,
    /// Whether directory-local imported targets are visible to the requesting scope Example:
    /// `source.visible` .
    pub visible: bool,
}
#[derive(Default)]
/// Schema-selected discovery needs for one completion site. Constructing it performs no discovery.
///
/// [`super::super::Workspace::completion_catalog`] skips unrelated catalogs.
///
/// # Examples
/// ```text
/// let _ = request.command.is_none();
/// ```
pub(crate) struct Request {
    /// Active command excluded when restoring prior local hints; None replays all commands Example:
    /// `request.command.is_none()` .
    pub command: Option<CommandId>,
    /// Request bare module labels Example: `request.modules = true`.
    pub modules: bool,
    /// Request package labels Example: `request.packages = true`.
    pub packages: bool,
    /// Request supplemental package target sources Example: `request.targets = true`.
    pub targets: bool,
}
#[derive(Default)]
/// One replaceable root-keyed module/package name cache per workspace. No I/O on construction.
///
/// [`super::discovery::collect`] scans on cache miss; workspace input/path invalidation clears it.
/// Root/name cardinality is not independently capped by this record.
///
/// # Examples
/// ```text
/// let _ = cache.module_roots.len();
/// ```
pub(crate) struct DiscoveryCache {
    /// Sorted normalized module roots used as a cache key Example: `cache.module_roots.len()`.
    pub module_roots: Vec<PathBuf>,
    /// Sorted normalized package prefixes used as a cache key Example: `cache.prefix_roots.len()`.
    pub prefix_roots: Vec<PathBuf>,
    /// Cached sorted module names, or None before scan/after invalidation Example:
    /// `cache.modules.is_some()` .
    pub modules: Option<Arc<[String]>>,
    /// Cached sorted package names, or None before scan/after invalidation Example:
    /// `cache.packages.is_some()` .
    pub packages: Option<Arc<[String]>>,
}
/// One visit's directory context and conservative completion search hints. No side effects.
///
/// [`super::context::capture`] restores local hints before scanning names.
///
/// # Examples
/// ```text
/// let _ = context.directory.source.as_path();
/// ```
pub(super) struct Context {
    /// Source/binary directories used to normalize hint paths Example:
    /// `context.directory.source.as_path()` .
    pub directory: DirectoryContext,
    /// Copy-on-write known values restored before the active command Example:
    /// `context.values.get("CMAKE_PREFIX_PATH", None)` .
    pub values: crate::parser::semanticer::model::ValueBindings,
}
/// Pending supplemental include visit retaining directory and visibility identity. No side effects.
///
/// [`super::targets::collect`] deduplicates this context and applies workspace traversal budgets.
///
/// # Examples
/// ```text
/// let _ = visit.file.as_path();
/// ```
pub(super) struct Visit {
    /// Lexical absolute source identity Example: `visit.file.as_path()`.
    pub file: PathBuf,
    /// Inherited package source/binary context Example: `visit.directory.source.as_path()`.
    pub directory: DirectoryContext,
    /// Whether directory-local imported declarations may be offered Example: `visit.visible`.
    pub visible: bool,
    /// Dependency nesting level, including original graph ancestry Example: `visit.depth`.
    pub depth: usize,
}
