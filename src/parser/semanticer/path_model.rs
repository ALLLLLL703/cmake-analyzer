//! Schema-selected filesystem argument roles. Classification does not access paths.
// Contains AI-generated documentation additions.
// AI-authored filesystem roles; generic schema Path remains unspecified.
/// Filesystem role selected by schema matching; no path access or validation occurs.
///
/// [`PathArgument`] pairs the role with a source argument slot.
///
/// # Examples
/// ```
/// use cmake_analyzer::parser::semanticer::model::*;
/// let value = PathKind::File;
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PathKind {
    /// Input explicitly expected to be a file.
    /// Example: `PathKind::File`.
    File,
    /// Input explicitly expected to be a directory.
    /// Example: `PathKind::Directory`.
    Directory,
    /// Input path with unspecified filesystem type.
    /// Example: `PathKind::Input`.
    Input,
    /// Module-name or module-path input.
    /// Example: `PathKind::Module`.
    Module,
    /// Output path; not proof that a file will be written.
    /// Example: `PathKind::Output`.
    Output,
}
/// A schema-selected filesystem role at one command argument slot. Path completion and dependency discovery consume it; classification itself performs no path access.
///
/// [`PathKind`] distinguishes file, directory and unresolved path roles.
///
/// # Examples
/// ```
/// use cmake_analyzer::parser::semanticer::model::{PathArgument, PathKind};
/// let path = PathArgument { argument: 0, kind: PathKind::Input };
/// ```
#[derive(Clone, Debug)]
pub struct PathArgument {
    /// Zero-based source argument index, not an expanded list index.
    /// Example: `path.argument == 0`.
    pub argument: usize,
    /// Schema path role; no existence or type check is implied.
    /// Example: `path.kind == PathKind::Input`.
    pub kind: PathKind,
}
