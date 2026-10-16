// AI-authored default module directories and best-effort installed CMake discovery.
use crate::parser::{interpreter_commands::resolve_path, semanticer::model::CMakeVersion};
use std::{
    ffi::OsStr,
    path::{Path, PathBuf},
};

pub(super) static PROJECT_SEARCH_PATHS: &[&str] = &["cmake", "cmake/Modules", "cmake/modules"];

pub(super) async fn builtin_directory(cwd: &Path) -> Option<PathBuf> {
    discover_from_path(std::env::var_os("PATH").as_deref()?, cwd).await
}
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
pub(crate) async fn cmake_executable(cwd: &Path) -> Option<PathBuf> {
    executable(std::env::var_os("PATH").as_deref()?, cwd).await
}
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
mod tests;
