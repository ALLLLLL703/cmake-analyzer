use super::model::Request;
use crate::workspace::{Workspace, documents::absolute};
use std::{
    collections::HashSet,
    path::{Path, PathBuf},
};

async fn children(base: &Path, name: &str) -> Vec<PathBuf> {
    let mut result = Vec::new();
    let Ok(mut entries) = tokio::fs::read_dir(base).await else {
        return result;
    };
    while let Ok(Some(entry)) = entries.next_entry().await {
        if entry
            .file_name()
            .to_string_lossy()
            .get(..name.len())
            .is_some_and(|prefix| prefix.eq_ignore_ascii_case(name))
            && entry.file_type().await.is_ok_and(|kind| kind.is_dir())
        {
            result.push(entry.path());
        }
    }
    result.sort();
    result
}
async fn directories(root: &Path, names: &[String]) -> Vec<PathBuf> {
    let mut result = Vec::new();
    for name in names {
        for base in [
            root.to_owned(),
            root.join("lib/cmake"),
            root.join("lib64/cmake"),
            root.join("share/cmake"),
            root.join("share"),
        ] {
            let mut packages = children(&base, name).await;
            packages.push(base.join(name));
            for package in packages {
                result.extend([
                    package.clone(),
                    package.join("cmake"),
                    package.join("CMake"),
                ]);
            }
        }
    }
    // Inspect conventional multiarch levels, not an arbitrary recursive filesystem tree.
    let mut architectures = Vec::new();
    if let Ok(mut entries) = tokio::fs::read_dir(root.join("lib")).await {
        while let Ok(Some(entry)) = entries.next_entry().await {
            if entry.file_type().await.is_ok_and(|kind| kind.is_dir()) {
                architectures.push(entry.path());
            }
        }
    }
    architectures.sort();
    for architecture in architectures {
        for name in names {
            result.extend(children(&architecture.join("cmake"), name).await);
        }
    }
    result
}
async fn find_at(
    workspace: &Workspace,
    base: &Path,
    request: &Request,
    checked: &mut HashSet<PathBuf>,
    candidates: &mut Vec<PathBuf>,
) -> Option<PathBuf> {
    for suffix in &request.suffixes {
        for filename in &request.filenames {
            let Ok(path) = absolute(&base.join(suffix).join(filename)) else {
                continue;
            };
            if !checked.insert(path.clone()) {
                continue;
            }
            candidates.push(path.clone());
            if workspace
                .documents
                .get(&path)
                .is_some_and(|document| document.overlay.is_some())
                || tokio::fs::metadata(&path)
                    .await
                    .is_ok_and(|meta| meta.is_file())
            {
                return Some(path);
            }
        }
    }
    None
}
pub(super) async fn find(
    workspace: &Workspace,
    roots: Vec<PathBuf>,
    source: &Path,
    request: &Request,
    candidates: &mut Vec<PathBuf>,
) -> Option<PathBuf> {
    let mut seen = HashSet::new();
    let mut checked = HashSet::new();
    for root in roots {
        let Ok(root) = absolute(&source.join(root)) else {
            continue;
        };
        if !seen.insert(root.clone()) {
            continue;
        }
        // A configured Foo_DIR is normally exact; do not enumerate its tree before trying it.
        for base in [root.clone(), root.join("cmake"), root.join("CMake")] {
            if let Some(path) = find_at(workspace, &base, request, &mut checked, candidates).await {
                return Some(path);
            }
        }
        for base in directories(&root, &request.names).await {
            if let Some(path) = find_at(workspace, &base, request, &mut checked, candidates).await {
                return Some(path);
            }
        }
    }
    None
}
