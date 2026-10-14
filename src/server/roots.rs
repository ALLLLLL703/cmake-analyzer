// AI-authored project root discovery.
use std::{
    io,
    path::{Path, PathBuf},
};

pub(super) fn normalize(path: &Path) -> io::Result<PathBuf> {
    crate::workspace::absolute(path)
}
/// Find the nearest project entry, including start itself when it is a directory.
/// No recursive child scan: a module directory is not automatically a project.
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
/// Explicit roots must name an entry file or its directory, not an arbitrary module.
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
