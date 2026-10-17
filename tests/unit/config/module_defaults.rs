//! Entry-relative module defaults without creating module directories, and explicit empty-array
//! replacement.

// Contains AI-generated documentation edits.
// AI-generated project module default and override tests.
use cmake_analyzer::config::{ConfigSources, ConfigStore, Modules};
use std::sync::atomic::Ordering;

/// Query two separate entry directories and require each one's three default module paths without
/// creating those paths.
///
/// Writes, reloads and removes temporary configuration files. Enabled module defaults may inspect
/// PATH CMake with --version; no project configure or script execution.
///
/// # Arguments
/// None. The test builds its own fixture and expected results.
///
/// # Returns
/// `()` after all checks pass; no data or output parameter is returned.
///
/// # Panics
/// If a stated invariant fails or an unwrapped fixture/setup operation fails.
///
/// # Algorithm
/// Reload after writing an empty search array and relative builtin directory; defaults must be
/// replaced, not appended.
///
/// # Flow
/// ```text
///   [entries a and b]
///    |
///   [resolve default paths]----------> [explicit empty array]
///                                     |
///                                    [replace defaults on reload]
///                                     |
///                                    < checks hold? >
///                                     /         \
///                                   [pass]     [panic]
/// ```
///
/// # Examples
/// Direct local call outside any Tokio runtime (the test attribute creates its runtime):
/// ```text
/// project_paths_follow_each_entry_and_explicit_empty_arrays_replace_defaults();
/// ```
#[tokio::test]
async fn project_paths_follow_each_entry_and_explicit_empty_arrays_replace_defaults() {
    let root = std::env::temp_dir().join(format!(
        "cmake-module-defaults-{}-{}",
        std::process::id(),
        super::NEXT.fetch_add(1, Ordering::Relaxed)
    ));
    let a = root.join("a");
    let b = root.join("b");
    tokio::fs::create_dir_all(&a).await.unwrap();
    tokio::fs::create_dir_all(&b).await.unwrap();
    let sources = ConfigSources {
        cwd: root.clone(),
        user_file: None,
        explicit_file: None,
    };
    let mut store = ConfigStore::load(sources.clone()).await.unwrap();
    for entry in [&a, &b] {
        let settings = store.for_directory(entry).await.unwrap();
        assert_eq!(
            settings.modules.search_paths,
            ["cmake", "cmake/Modules", "cmake/modules"].map(|path| entry.join(path))
        );
        assert!(!entry.join("cmake").exists());
    }
    tokio::fs::write(
        a.join(".cmake-analyzer.toml"),
        "[modules]\nsearch_paths=[]\nbuiltin_directory='builtin'\n",
    )
    .await
    .unwrap();
    let mut reloaded = ConfigStore::load(sources).await.unwrap();
    let settings = reloaded.for_directory(&a).await.unwrap();
    assert!(settings.modules.search_paths.is_empty());
    assert_eq!(settings.modules.builtin_directory, Some(a.join("builtin")));
    assert_eq!(
        Modules::default().search_paths,
        ["cmake", "cmake/Modules", "cmake/modules"].map(std::path::PathBuf::from)
    );
    tokio::fs::remove_dir_all(root).await.unwrap();
}
