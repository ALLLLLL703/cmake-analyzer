//! Immutable snapshot inputs and staged edits for the project formatter.
//! These records allocate/retain data only; [`super::write::apply`] owns disk replacement.
//!
//! Contains AI-generated documentation additions.
use crate::{format::FormatOptions, workspace::model::DocumentSnapshot};
use std::{path::PathBuf, sync::Arc};
/// One eligible saved file passed to [`super::prepare`]. Carries a retained analysis
/// snapshot and its effective style; constructing it does not read disk.
///
/// # Examples
/// ```text
/// let input = Input { path: "CMakeLists.txt".into(), snapshot, options };
/// ```
pub(super) struct Input {
    /// Canonical eligible destination path. Example: `input.path.display()`.
    pub path: PathBuf,
    /// Retained source and matching lexical/semantic analysis. Example: `input.snapshot.text.as_ref()`.
    pub snapshot: Arc<DocumentSnapshot>,
    /// Effective project style copied into the render job. Example: `input.options.line_width = 100`.
    pub options: FormatOptions,
}
/// A changed file staged by [`super::prepare`] for [`super::write::apply`].
/// Contains old text for conflict checking and replacement text; no I/O on construction.
///
/// # Examples
/// ```text
/// let edit = Edit { path: "CMakeLists.txt".into(), before: "set(A 1)".into(), after: "set(A 1)\n".into() };
/// ```
pub(super) struct Edit {
    /// File to replace. Example: `edit.path.display()`.
    pub path: PathBuf,
    /// Expected saved text compared immediately before writing. Example: `edit.before.as_ref()`.
    pub before: Arc<str>,
    /// Complete replacement, not a patch. Example: `edit.after.ends_with("\n")`.
    pub after: String,
}
