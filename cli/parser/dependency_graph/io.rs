//! Load a root overlay and static Workspace graph for dependency inspection.
//! Reads stdin/files and dependency sources; never configures or runs project code.
//!
//! Contains AI-generated documentation additions.
// AI-generated implementation.
use super::args::Args;
use cmake_analyzer::workspace::Workspace;
use std::{
    io,
    path::{Path, PathBuf},
};
use tokio::io::AsyncReadExt;

/// Read the root source, install it as an overlay, configure search paths and flush a static Workspace.
/// Stdin/text uses cwd/CMakeLists.txt as a path base; disk inputs keep their selected root.
/// Dependency reads and semantic version probing may occur; no project configure is run.
///
/// # Arguments
/// - `args` `[in]`: consumed source selection, binary context and module directories.
///
/// # Returns
/// `Ok(workspace)` has a flushed root graph/diagnostics; `Err` reports setup/read/worker errors.
///
/// # Errors
/// Input/cwd errors, invalid Workspace paths, and flush failures. Graph diagnostics may instead be retained for display.
///
/// # Algorithm
/// Choose input first, create the root overlay, resolve extra module paths against cwd, set builtin paths, then await graph traversal.
///
/// # Flow
/// ```text
///   [file]   [text/stdin]
///      \       /
///       [root overlay] <--- [cwd path base]
///             |
///      [module + binary settings]
///             |
///       [flush dependencies] ---> [read sources]
///             |
///        [Workspace]
/// ```
///
/// # Examples
/// ```text
/// let workspace = load_workspace(Args::parse_from(["dependency-graph", "--text", "set(A 1)"])).await?;
/// ```
pub(crate) async fn load_workspace(args: Args) -> io::Result<Workspace> {
    let file = args.file.as_deref().filter(|p| *p != Path::new("-"));
    // Stdin/text needs a real path base for relative includes and subdirectories.
    let root = file.unwrap_or(Path::new("CMakeLists.txt"));
    let source = if let Some(text) = args.text {
        text
    } else if let Some(file) = file {
        tokio::fs::read_to_string(file).await?
    } else {
        let mut text = String::new();
        tokio::io::stdin().read_to_string(&mut text).await?;
        text
    };
    let mut workspace = Workspace::new(root, args.binary_directory)?;
    workspace.set_overlay(root, source)?;
    let cwd = std::env::current_dir()?;
    let paths: Vec<PathBuf> = args
        .module_path
        .into_iter()
        .map(|path| cwd.join(path))
        .collect();
    workspace.set_module_paths(paths);
    workspace.set_builtin_module_directory(args.builtin_module_directory)?;
    workspace.flush().await?;
    Ok(workspace)
}
