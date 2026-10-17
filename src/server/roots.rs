//! Find CMake project entries using explicit roots or ancestor search.
//! Paths are made absolute and normalized without resolving symlink aliases.
//! Metadata reads do not scan descendants, read CMake contents or execute projects.
// AI-authored project root discovery.
use std::{
    io,
    path::{Path, PathBuf},
};

/// Make a path absolute and collapse lexical dot/parent components without resolving symlinks.
/// Uses current process directory for relative input; does not inspect the target file.
///
/// # Arguments
/// - `path` `[in]`: Borrowed relative or absolute path.
///
/// # Returns
/// `Ok(PathBuf)` with normalized absolute components; `Err` if a needed current directory cannot be obtained.
///
/// # Errors
/// Current-directory lookup failure from workspace::absolute.
///
/// # Examples
/// ```text
/// let path = normalize(std::path::Path::new("/work/demo/./CMakeLists.txt"))?;
/// ```
pub(super) fn normalize(path: &Path) -> io::Result<PathBuf> {
    crate::workspace::absolute(path)
}
/// Search the starting directory and its ancestors for the nearest regular CMakeLists.txt.
/// Reads metadata only; no recursive child scan, project parse or execution.
///
/// # Arguments
/// - `start` `[in]`: File/directory path. A missing path with an extension is treated as a file; otherwise as a directory.
///
/// # Returns
/// `Ok(Some(entry))` for the first ancestor regular entry file, `Ok(None)` if none exists, or `Err` for metadata/normalization failures other than missing files.
///
/// # Errors
/// Current-directory lookup or metadata access errors; NotFound candidates are skipped.
///
/// # Algorithm
/// Normalize, choose the start directory from metadata (or the missing-path extension heuristic), then check CMakeLists.txt in each ancestor, stopping at the first regular file.
///
/// # Flow
/// ```text
///  [Start path metadata]
///             |
///        <Directory?>
///         /        \
///       yes        no/file-like missing
///        |          |
///  [Use path]   [Use parent]
///        +----------+
///             v
///  [Check entry in each ancestor]
///             |
///  <Regular entry?> -- yes --> [Some(entry)]
///             | no
///     [Next ancestor / None]
/// ```
///
/// # Examples
/// ```no_run
/// use cmake_analyzer::server::roots::find_project;
/// use std::path::Path;
/// # async fn inspect() -> std::io::Result<()> {
/// // Supply a local checkout; this example only inspects metadata.
/// let entry = find_project(Path::new("/work/demo/cmake/helpers.cmake")).await?;
/// # Ok(())
/// # }
/// ```
pub async fn find_project(start: &Path) -> io::Result<Option<PathBuf>> {
    let path = normalize(start)?;
    let directory = match tokio::fs::metadata(&path).await {
        Ok(meta) if meta.is_dir() => path.as_path(),
        Ok(_) => path.parent().unwrap_or(&path),
        Err(error) if error.kind() == io::ErrorKind::NotFound => {
            if path.extension().is_some() {
                path.parent().unwrap_or(&path)
            } else {
                path.as_path()
            }
        }
        Err(error) => return Err(error),
    };
    for directory in directory.ancestors() {
        let entry = directory.join("CMakeLists.txt");
        match tokio::fs::metadata(&entry).await {
            Ok(meta) if meta.is_file() => return Ok(Some(entry)),
            Ok(_) => {}
            Err(error) if error.kind() == io::ErrorKind::NotFound => {}
            Err(error) => return Err(error),
        }
    }
    Ok(None)
}
/// Validate that an explicit path is CMakeLists.txt or a directory containing that regular file.
/// Reads metadata but never searches ancestors, parses content or executes CMake.
///
/// # Arguments
/// - `path` `[in]`: Borrowed entry file or its containing directory; normalized first.
///
/// # Returns
/// `Ok(entry)` with a normalized absolute CMakeLists.txt path, or `Err` if it cannot be validated.
///
/// # Errors
/// Current-directory/metadata errors, missing entry, or InvalidInput when the selected file has the wrong name or is not regular.
///
/// # Examples
/// ```no_run
/// use cmake_analyzer::server::roots::explicit_root;
/// use std::path::Path;
/// # async fn inspect() -> std::io::Result<()> {
/// // Requires an existing checkout containing CMakeLists.txt.
/// let entry = explicit_root(Path::new("/work/demo")).await?;
/// # Ok(())
/// # }
/// ```
pub async fn explicit_root(path: &Path) -> io::Result<PathBuf> {
    let path = normalize(path)?;
    let entry = if tokio::fs::metadata(&path).await?.is_dir() {
        path.join("CMakeLists.txt")
    } else {
        path
    };
    if entry
        .file_name()
        .is_some_and(|name| name == "CMakeLists.txt")
        && tokio::fs::metadata(&entry).await?.is_file()
    {
        Ok(entry)
    } else {
        Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "A project root must contain CMakeLists.txt",
        ))
    }
}
