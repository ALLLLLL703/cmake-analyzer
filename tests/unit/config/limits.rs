//! Process-wide document/workspace limits: global precedence, project rejection and positive
//! integer validation.

// Contains AI-generated documentation edits.
// AI-generated process-limit layering and validation tests.
use cmake_analyzer::config::{ConfigSources, ConfigStore, Limits};
use std::sync::atomic::Ordering;

/// Merge user limits with an explicit document cap and check 256-document/32-workspace defaults.
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
/// Reject process limits discovered as project settings, accept the same file as an explicit global
/// source, and reject zero, negative, fractional, string and boolean counts.
///
/// # Flow
/// ```text
///   [user + explicit caps]
///    |
///   [merge global limits]-----------> [project / invalid counts]
///                                    |
///                                   [reject local or invalid limits]
///                                    |
///                                   < checks hold? >
///                                    /         \
///                                  [pass]     [panic]
/// ```
///
/// # Examples
/// Direct local call outside any Tokio runtime (the test attribute creates its runtime):
/// ```text
/// limits_merge_only_global_sources_and_require_positive_counts();
/// ```
#[tokio::test]
async fn limits_merge_only_global_sources_and_require_positive_counts() {
    let directory = std::env::temp_dir().join(format!(
        "cmake-config-limits-{}-{}",
        std::process::id(),
        super::NEXT.fetch_add(1, Ordering::Relaxed)
    ));
    tokio::fs::create_dir_all(&directory).await.unwrap();
    let user = directory.join("user.toml");
    let explicit = directory.join("explicit.toml");
    tokio::fs::write(&user, "[limits]\nmax_open_documents=3\nmax_workspaces=4\n")
        .await
        .unwrap();
    tokio::fs::write(&explicit, "[limits]\nmax_open_documents=5\n")
        .await
        .unwrap();
    let sources = ConfigSources {
        cwd: directory.clone(),
        user_file: Some(user),
        explicit_file: Some(explicit.clone()),
    };
    let mut store = ConfigStore::load(sources.clone()).await.unwrap();
    assert_eq!(store.limits.max_open_documents, 5);
    assert_eq!(store.limits.max_workspaces, 4);
    assert_eq!(Limits::default().max_open_documents, 256);
    assert_eq!(Limits::default().max_workspaces, 32);
    tokio::fs::write(
        directory.join(".cmake-analyzer.toml"),
        "[limits]\nmax_workspaces=2\n",
    )
    .await
    .unwrap();
    let error = store.for_directory(&directory).await.unwrap_err();
    assert!(error.to_string().contains("process-wide"));
    let mut explicit_project = ConfigStore::load(ConfigSources {
        cwd: directory.clone(),
        user_file: None,
        explicit_file: Some(directory.join(".cmake-analyzer.toml")),
    })
    .await
    .unwrap();
    assert_eq!(explicit_project.limits.max_workspaces, 2);
    explicit_project.for_directory(&directory).await.unwrap();
    for value in ["0", "-1", "1.5", "'wrong'", "true"] {
        tokio::fs::write(&explicit, format!("[limits]\nmax_open_documents={value}\n"))
            .await
            .unwrap();
        assert!(ConfigStore::load(sources.clone()).await.is_err(), "{value}");
    }
    tokio::fs::write(&explicit, "[limits]\nmax_workspaces=0\n")
        .await
        .unwrap();
    assert!(ConfigStore::load(sources).await.is_err());
    tokio::fs::remove_dir_all(directory).await.unwrap();
}
