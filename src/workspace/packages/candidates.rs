//! Search exact config directories before shallow conventional package-prefix layouts.
//! Filesystem probes accept open overlays, remember missing candidates and avoid recursive scans.
// Contains AI-generated documentation additions.
use super::model::Request;
use crate::workspace::{Workspace, documents::absolute};
use std::{
    collections::HashSet,
    path::{Path, PathBuf},
};

/// List immediate directories whose names begin with a package name, ignoring ASCII case. Reads one
/// directory; no recursion or mutation.
///
/// # Arguments
/// - `base` `[in]`: Directory to enumerate.
/// - `name` `[in]`: Required filename prefix.
///
/// # Returns
/// Sorted child paths; missing/unreadable directories or iteration failures yield the successfully
/// collected prefix of results.
///
/// # Examples
/// ```text
/// let versions = children(Path::new("/usr/lib/cmake"), "Foo").await;
/// ```
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
/// Enumerate conventional package directories and one lib architecture level. Reads shallow
/// directories only; no arbitrary recursive walk.
///
/// # Arguments
/// - `root` `[in]`: Package installation prefix.
/// - `names` `[in]`: Accepted package names used for case-insensitive prefix matching.
///
/// # Returns
/// Ordered conventional candidates, including missing exact package directories and sorted
/// discovered variants.
///
/// # Algorithm
/// Probe root, lib/cmake, lib64/cmake, share/cmake and share; for each name include discovered
/// prefixes and an exact name, plus cmake/CMake children. Enumerate immediate lib architecture
/// directories and their cmake package children.
///
/// # Flow
/// ```text
/// [prefix]
///     |
///  [fixed root/lib/share bases]      [lib architectures]
///     |                                  |
///  [name-prefix children + exact]   [cmake/name children]
///     |                                  |
///  [package/cmake/CMake] ----------------+
///                       |
///               [ordered candidates]
/// ```
///
/// # Examples
/// ```text
/// let dirs = directories(Path::new("/usr"), &["Foo".into()]).await;
/// ```
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
/// Probe suffix/filename combinations at one config directory. Reads file metadata and overlay
/// state, never contents.
///
/// # Arguments
/// - `workspace` `[in]`: Overlay identities accepted as files.
/// - `base` `[in]`: Candidate config directory.
/// - `request` `[in]`: Suffixes and filenames in request order.
/// - `checked` `[in,out]`: Appends normalized paths to the deduplication set.
/// - `candidates` `[in,out]` : Appends each newly checked path, including missing paths; does not
///   clear existing candidates.
///
/// # Returns
/// Some first overlay/existing file; None if all new candidates fail.
///
/// # Examples
/// ```text
/// let entry = find_at(workspace, base, request, &mut checked, &mut candidates).await;
/// ```
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
/// Find the first static config entrypoint, trying exact roots before conventional layouts. Reads
/// shallow directories and metadata; no execution.
///
/// # Arguments
/// - `workspace` `[in]`: Overlay identities and immutable lookup state.
/// - `roots` `[in]`: Ordered owned roots; duplicate normalized roots are skipped.
/// - `source` `[in]`: Source directory used to resolve relative roots.
/// - `request` `[in]`: Package names, suffixes and filenames.
/// - `candidates` `[in,out]` : Appends newly checked paths, including missing higher-priority
///   choices.
///
/// # Returns
/// Some first file/overlay found; None after exhausting supported candidates. Unreadable
/// directories are skipped.
///
/// # Algorithm
/// Normalize/deduplicate roots. Try root, root/cmake and root/CMake without enumerating
/// installations first. Only then enumerate fixed conventional package layouts; suppress duplicate
/// file probes across the search.
///
/// # Flow
/// ```text
/// [ordered roots]
///        |
///  [normalize/deduplicate]
///        |
///  [exact root/cmake/CMake] ---> <found?> -- yes --> [return]
///        | no
///  [conventional layouts]
///        |
///  [suffix/filename probes] ---> <found?> -- no --> [next root]
///                                  | yes
///                               [return]
/// ```
///
/// # Examples
/// ```text
/// let config = find(workspace, roots, source, request, &mut candidates).await;
/// ```
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
