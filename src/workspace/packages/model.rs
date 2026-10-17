//! Parsed static package-search flags, candidate names and paths; construction has no I/O.
// Contains AI-generated documentation additions.
use std::path::PathBuf;
/// Supported literal package-search plan, not version/component validation. No I/O on construction.
///
/// [`super::request::parse`] builds it; [`super::candidates::find`] consumes the config search
/// plan.
///
/// # Examples
/// ```text
/// let filenames = request.filenames.as_slice();
/// ```
pub(super) struct Request {
    /// Package names for conventional prefix directory matching Example: `request.names.as_slice()`
    /// .
    pub names: Vec<String>,
    /// Ordered HINTS/PATHS directories preceding default roots Example: `request.hints.len()`.
    pub hints: Vec<PathBuf>,
    /// Config path suffixes; begins with empty suffix Example: `request.suffixes.len()`.
    pub suffixes: Vec<PathBuf>,
    /// Explicit CONFIGS names or generated NameConfig/name-config filenames Example:
    /// `request.filenames.as_slice()` .
    pub filenames: Vec<String>,
    /// Skip Find-module fallback Example: `request.config_only`.
    pub config_only: bool,
    /// Skip config entrypoint search Example: `request.module_only`.
    pub module_only: bool,
    /// Whether default root families are enabled Example: `request.defaults`.
    pub defaults: bool,
    /// Whether known/environment Package_ROOT contributes roots Example: `request.package_roots`.
    pub package_roots: bool,
    /// Whether known CMAKE_PREFIX_PATH contributes roots Example: `request.cmake_paths`.
    pub cmake_paths: bool,
    /// Whether environment prefix/package hints are read Example: `request.environment`.
    pub environment: bool,
    /// Whether configured package prefixes contribute default roots Example: `request.system_paths`
    /// .
    pub system_paths: bool,
}
