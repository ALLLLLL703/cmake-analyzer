//! Default module paths and best-effort installed CMake discovery. Reads PATH and
//! filesystem metadata; builtin discovery runs only `cmake --version` with no project input.
//! The caller caches success and failure once per configuration store.
//!
//! Contains AI-generated documentation additions.
// AI-authored default module directories and best-effort installed CMake discovery.
use crate::parser::{interpreter_commands::resolve_path, semanticer::model::CMakeVersion};
use std::{
    ffi::OsStr,
    path::{Path, PathBuf},
};

pub(super) static PROJECT_SEARCH_PATHS: &[&str] = &["cmake", "cmake/Modules", "cmake/modules"];

/// Discover conventional builtin modules using startup PATH. Reads environment and metadata;
/// may run the selected executable with --version, never configure/project arguments.
///
/// # Arguments
/// - `cwd` `[in]`: base for relative PATH entries and process working directory.
///
/// # Returns
/// `Some(directory)` for the first recognized installation layout; `None` for absent PATH, executable, version or directory.
///
/// # Examples
/// ```text
/// let modules = builtin_directory(std::path::Path::new("/work")).await;
/// ```
pub(super) async fn builtin_directory(cwd: &Path) -> Option<PathBuf> {
    discover_from_path(std::env::var_os("PATH").as_deref()?, cwd).await
}
/// Find the first executable CMake, query --version and check its canonical prefix share layouts.
/// Reads metadata, spawns a version-only child and captures output; no project code is passed.
/// The child is kill-on-drop; this probe has no explicit timeout/output budget.
///
/// # Arguments
/// - `path` `[in]`: PATH-style search list.
/// - `cwd` `[in]`: base for relative search entries and child cwd.
///
/// # Returns
/// `Some(path)` for share/cmake-major.minor/Modules or share/cmake/Modules; `None` for any failed probe or unrecognized installation.
///
/// # Algorithm
/// Locate/canonicalize the first executable. Parse successful UTF-8 version output, then test the versioned and generic Modules directories in order.
///
/// # Flow
/// ```text
///  [PATH entries] ---> [first executable]
///                            |
///                      [cmake --version]
///                            |
///                      <valid version?> --no--> [None]
///                            | yes
///                   [versioned Modules dir]
///                      /           \
///                   found          missing
///                     |              |
///                   [Some]    [generic Modules dir]
///                                   /     \
///                                [Some] [None]
/// ```
///
/// # Examples
/// ```text
/// let modules = discover_from_path(std::ffi::OsStr::new("/usr/bin"), std::path::Path::new("/work")).await;
/// ```
async fn discover_from_path(path: &OsStr, cwd: &Path) -> Option<PathBuf> {
    let executable = executable(path, cwd).await?;
    let output = tokio::process::Command::new(&executable)
        .arg("--version")
        .current_dir(cwd)
        .kill_on_drop(true)
        .output()
        .await
        .ok()?;
    if !output.status.success() {
        return None;
    }
    let text = std::str::from_utf8(&output.stdout).ok()?;
    let version = text
        .lines()
        .find_map(|line| line.strip_prefix("cmake version "))?
        .split_whitespace()
        .next()?;
    let version = CMakeVersion::parse(version.split('-').next().unwrap_or(version)).ok()?;
    let share = executable.parent()?.parent()?.join("share");
    for directory in [
        share
            .join(format!("cmake-{}.{}", version.major, version.minor))
            .join("Modules"),
        share.join("cmake/Modules"),
    ] {
        if tokio::fs::metadata(&directory)
            .await
            .is_ok_and(|metadata| metadata.is_dir())
        {
            return Some(directory);
        }
    }
    None
}
/// Read PATH and locate/canonicalize the first executable CMake; no process is launched.
///
/// # Arguments
/// - `cwd` `[in]`: base for relative PATH entries.
///
/// # Returns
/// `Some(path)` for the first canonical executable, `None` if PATH is absent or no candidate succeeds.
///
/// # Examples
/// ```text
/// let cmake = cmake_executable(std::path::Path::new("/work")).await;
/// ```
pub(crate) async fn cmake_executable(cwd: &Path) -> Option<PathBuf> {
    executable(std::env::var_os("PATH").as_deref()?, cwd).await
}
/// Search PATH entries in order with asynchronous metadata/canonicalization.
/// Requires a regular file and Unix execute bits where applicable; does not launch or write it.
///
/// # Arguments
/// - `path` `[in]`: platform-separated search list.
/// - `cwd` `[in]`: base for resolving relative search entries.
///
/// # Returns
/// `Some(path)` for the first canonical candidate, `None` when exhausted or path normalization fails.
/// Metadata/canonicalization failures skip candidates.
///
/// # Algorithm
/// Choose cmake.exe on Windows or cmake elsewhere. Resolve each PATH directory against
/// cwd, require a regular file and Unix execute bits, then canonicalize the first
/// successful candidate. Only path normalization failure stops the search early.
///
/// # Flow
/// ```text
///  [PATH entries] ---> [next candidate] <--------------+
///                            |                        |
///                     <regular/executable?> --no------+
///                            | yes                    |
///                       <canonicalize?> --failure-----+
///                            | success
///                        [Some(path)]
///  [entries exhausted] ---> [None]
/// ```
///
/// # Examples
/// ```text
/// let cmake = executable(std::ffi::OsStr::new("/usr/bin"), std::path::Path::new("/work")).await;
/// ```
async fn executable(path: &OsStr, cwd: &Path) -> Option<PathBuf> {
    let name = if cfg!(windows) { "cmake.exe" } else { "cmake" };
    for directory in std::env::split_paths(path) {
        let candidate = resolve_path(cwd, &directory.join(name)).ok()?;
        let Ok(metadata) = tokio::fs::metadata(&candidate).await else {
            continue;
        };
        if !metadata.is_file() {
            continue;
        }
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            if metadata.permissions().mode() & 0o111 == 0 {
                continue;
            }
        }
        if let Ok(executable) = tokio::fs::canonicalize(&candidate).await {
            return Some(executable);
        }
    }
    None
}

#[cfg(all(test, unix))]
#[path = "../../tests/unit/config/module_discovery.rs"]
/// Installed-CMake discovery fixtures loaded only for Unix tests; test implementations live in the external test tree.
mod tests;
