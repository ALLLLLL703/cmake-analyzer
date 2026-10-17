//! Local help categories and service-lifetime cache state; construction performs no process or
//! filesystem I/O.

use crate::config::Runtime;
use std::{collections::BTreeMap, path::PathBuf, sync::Arc};
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
/// Native help namespace used by [`LocalHelp`] to select fixed help flags.
/// Used during hover lookup; choosing a namespace has no side effects.
/// [`Self::list_flag`] requests canonical names; [`Self::help_flag`] requests one body.
///
/// # Examples
/// ```text
/// let kind = Kind::Variable;
/// assert_eq!(kind.help_flag(), "--help-variable");
/// ```
pub(in crate::server::hovers) enum Kind {
    /// Command-name help namespace. Example: `Kind::Command.list_flag()` requests --help-command-list.
    Command,
    /// Case-sensitive variable help namespace. Example: `Kind::Variable.help_flag()` requests
    /// --help-variable.
    Variable,
}
impl Kind {
    /// Choose the fixed catalog flag without side effects.
    ///
    /// # Arguments
    /// - `self` `[in]`: help namespace copied by value.
    ///
    /// # Returns
    /// The command-list or variable-list flag, never project arguments.
    ///
    /// # Examples
    /// Local server context:
    /// ```text
    /// assert_eq!(Kind::Command.list_flag(), "--help-command-list");
    /// ```
    pub(super) fn list_flag(self) -> &'static str {
        match self {
            Self::Command => "--help-command-list",
            Self::Variable => "--help-variable-list",
        }
    }
    /// Choose the fixed single-document help flag without side effects.
    ///
    /// # Arguments
    /// - `self` `[in]`: help namespace copied by value.
    ///
    /// # Returns
    /// The command or variable help flag; the caller supplies a canonical name separately.
    ///
    /// # Examples
    /// Local server context:
    /// ```text
    /// assert_eq!(Kind::Variable.help_flag(), "--help-variable");
    /// ```
    pub(super) fn help_flag(self) -> &'static str {
        match self {
            Self::Command => "--help-command",
            Self::Variable => "--help-variable",
        }
    }
}
/// Lazy service-local native help cache, used by hover dispatch instead of project execution settings.
/// [`Self::new`] only allocates state; lookup may inspect PATH and start help-only children.
/// [`Kind`] separates command and variable catalogs. Missing catalogs/bodies are cached too.
/// Successful reload replaces this instance; project-local runtime executable/arguments/enable/mode
/// are not used. There is no embedded/manual-network fallback.
///
/// # Flow
/// ```text
///  [Hover name]                 [Global inspection budgets]
///       |                                  |
///  [Catalog + canonical key]                |
///       |                                  v
///  [Cached body?] --no--> [Startup-PATH help child]
///       | yes                       |
///       |                  [Fits aggregate body budget?]
///       |                       /          \
///       |                     yes           no
///       |                      |             |
///       |                 [Cache body] [Return uncached]
///       +----------------------+-------------+
///                              v
///                        [Hover text]
/// ```
///
/// # Examples
/// ```text
/// let help = LocalHelp::new(startup_cwd, crate::config::Runtime::default());
/// assert_eq!(help.cached_bytes, 0);
/// ```
pub(in crate::server) struct LocalHelp {
    /// Startup working directory for PATH resolution and help children. Example: `let cwd = &help.cwd`
    /// reads it.
    pub(super) cwd: PathBuf,
    /// User/explicit runtime settings used only for timeout_seconds and max_output_bytes inspection
    /// budgets. Example: `help.limits.max_output_bytes` reads the per-child and aggregate-body cap.
    pub(super) limits: Runtime,
    /// Once-only startup-PATH executable result, including absence. Example: `help.executable.get()`
    /// checks whether discovery ran.
    pub(super) executable: tokio::sync::OnceCell<Option<PathBuf>>,
    /// Sorted deduplicated names per namespace; None remembers unavailable help. Example:
    /// `help.catalogs.get(&Kind::Variable)` reads a cached result.
    pub(super) catalogs: BTreeMap<Kind, Option<Arc<Vec<String>>>>,
    /// Canonical (namespace, name) bodies or remembered failures. Example:
    /// `help.documents.get(&(Kind::Command, "set".into()))` reads one entry.
    pub(super) documents: BTreeMap<(Kind, String), Option<Arc<str>>>,
    /// Sum of cached successful body UTF-8 bytes; catalog storage is not included. Example:
    /// `assert_eq!(help.cached_bytes, 0)` checks an empty cache.
    pub(super) cached_bytes: usize,
}
impl LocalHelp {
    /// Create an empty lazy help cache; no filesystem access or process execution occurs.
    ///
    /// # Arguments
    /// - `cwd` `[in]`: startup directory, moved into the cache.
    /// - `limits` `[in]`: global inspection settings, not an execution authorization.
    ///
    /// # Returns
    /// A cache with empty catalogs/bodies, zero cached bytes and unresolved executable.
    ///
    /// # Examples
    /// Local server context:
    /// ```text
    /// let help = LocalHelp::new(startup_cwd, crate::config::Runtime::default());
    /// ```
    pub(in crate::server) fn new(cwd: PathBuf, limits: Runtime) -> Self {
        Self {
            cwd,
            limits,
            executable: tokio::sync::OnceCell::new(),
            catalogs: BTreeMap::new(),
            documents: BTreeMap::new(),
            cached_bytes: 0,
        }
    }
}
