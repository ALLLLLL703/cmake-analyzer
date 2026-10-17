//! Path-warning cache and candidate records used by collection, metadata checks and diagnostics; no I/O
//! on construction.

// AI-authored path diagnostic data, independent of protocol envelopes.
use crate::{model::TextSpan, parser::semanticer::model::PathKind};
use std::{collections::HashSet, path::PathBuf};
#[derive(Default)]
/// Cached filesystem-warning results for one project, consumed during diagnostic publication.
/// Construction performs no I/O. Refresh replaces the whole state after metadata reads; diagnostics
/// also use [`Self::is_generated`] to suppress missing graph inputs that are known outputs.
/// [`PathWarning`] carries source anchors; candidate resolution uses [`Candidate`] before checking disk.
///
/// # Flow
/// ```text
///  [Static/runtime candidates]       [Build directory contexts]
///              |                              |
///        [Declared outputs] ------------------+
///              |                              v
///              +------------------> [Generated exemptions]
///                                             |
///  [Input metadata] --------------------------+
///              |                              |
///              v                              v
///       [Cached warnings]          [Suppress generated read failures]
/// ```
///
/// # Examples
/// ```text
/// let state = PathState::default();
/// assert!(!state.is_generated(std::path::Path::new("/src/input.c")));
/// ```
pub(in crate::server) struct PathState {
    /// Current missing, wrong-kind or unreadable input warnings. Example: `state.warnings.len()` reads
    /// the count after refresh.
    pub warnings: Vec<PathWarning>,
    /// Exact lexically resolved output paths exempt from existence checks. Example:
    /// `state.generated.insert("/build/generated.c".into())` adds an exemption.
    pub generated: HashSet<PathBuf>,
    /// Separate configured/visited binary directories; all descendants are exempt. Example:
    /// `state.build_directories.push("/build".into())` exempts that tree.
    pub build_directories: Vec<PathBuf>,
}
impl PathState {
    /// Test whether a path is an exact output or lies under an exempt build directory; no I/O.
    /// Comparison is lexical and component-based, not canonicalized; symlink identity is not inspected.
    ///
    /// # Arguments
    /// - `self` `[in]`: cached generated paths and build directories.
    /// - `path` `[in]`: resolved candidate or failed graph-input path.
    ///
    /// # Returns
    /// True if an exemption applies; false otherwise.
    ///
    /// # Examples
    /// Local server context:
    /// ```text
    /// let mut state = PathState::default();
    /// state.build_directories.push("/build".into());
    /// assert!(state.is_generated(std::path::Path::new("/build/generated.c")));
    /// ```
    pub(in crate::server) fn is_generated(&self, path: &std::path::Path) -> bool {
        self.generated.contains(path)
            || self
                .build_directories
                .iter()
                .any(|base| path.starts_with(base))
    }
}
/// One cached input-path warning anchored in its owning source file; construction has no side effects.
/// Refresh stores these in [`PathState::warnings`]; diagnostic publication converts [`TextSpan`]
/// from source bytes to negotiated client coordinates.
///
/// # Examples
/// ```text
/// let warning = PathWarning { file: "/src/CMakeLists.txt".into(), span,
///     message: "Input path does not exist".into() };
/// ```
pub(in crate::server) struct PathWarning {
    /// Source path containing the argument, not the failing candidate path. Example:
    /// `warning.file.as_path()` reads the diagnostic owner.
    pub file: PathBuf,
    /// Raw source argument range, including wrappers if present. Example: `warning.span.start_byte`
    /// reads the warning anchor.
    pub span: TextSpan,
    /// Human-readable existence/type/inspection failure. Example: `warning.message.as_str()` reads the
    /// diagnostic text.
    pub message: String,
}
/// One schema-qualified path argument with alternative lexically resolved locations; no I/O on
/// construction.
/// Collection appends these for the metadata refresh. [`PathWarning`] is emitted only if no alternative
/// satisfies the required kind, unless an optional missing module or generated exemption suppresses it.
/// [`PathKind`] distinguishes input expectations from output declarations.
///
/// # Examples
/// ```text
/// let candidate = Candidate { file: "/src/CMakeLists.txt".into(), span,
///     kind: PathKind::File, choices: vec!["/src/main.c".into()],
///     optional: false, dependency: false };
/// ```
pub(super) struct Candidate {
    /// Source document owning this argument. Example: `candidate.file.as_path()` reads where a warning
    /// belongs.
    pub file: PathBuf,
    /// Source argument span used for deduplication and diagnostics. Example: `candidate.span.start_byte`
    /// reads its source anchor.
    pub span: TextSpan,
    /// Expected input kind or declared output exemption. Example: `candidate.kind == PathKind::Output`
    /// tests an output.
    pub kind: PathKind,
    /// Alternative locations; a valid location satisfies the whole argument. May be empty for an
    /// unresolved module search. Example: `candidate.choices.push("/modules/FindX.cmake".into())` adds a
    /// search choice.
    pub choices: Vec<PathBuf>,
    /// Suppress all-not-found failures for OPTIONAL modules, not wrong-kind or permission failures.
    /// Example: `candidate.optional = true` marks an optional module.
    pub optional: bool,
    /// Native include/add_subdirectory operand already checked by an existing dependency graph. Example:
    /// `candidate.dependency = true` delegates that graph-owned check.
    pub dependency: bool,
}
