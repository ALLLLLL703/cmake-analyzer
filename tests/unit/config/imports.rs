//! Independent import-indexing and diagnostic-level overrides, including relative package paths and
//! invalid enum values.

// Contains AI-generated documentation edits.
// AI-generated tests.
use super::{ConfigSources, ConfigStore, NEXT};
use cmake_analyzer::{config::DiagnosticLevel, workspace::model::ExternalIndex};
use std::sync::atomic::Ordering;
/// Merge user function-only includes and package prefixes, project disabled package
/// indexing/variable warnings and an explicit function-warning level.
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
/// Check normalized paths, then replace the project layer with invalid modes, types, levels and a
/// misspelled rule.
///
/// # Flow
/// ```text
///   [import/rule layers]
///    |
///   [check modes + path base]----------> [invalid enums / keys]
///                                       |
///                                      [reject reload]
///                                       |
///                                      < checks hold? >
///                                       /         \
///                                     [pass]     [panic]
/// ```
///
/// # Examples
/// Direct local call outside any Tokio runtime (the test attribute creates its runtime):
/// ```text
/// import_modes_paths_and_diagnostic_levels_layer_independently_and_validate();
/// ```
#[tokio::test]
async fn import_modes_paths_and_diagnostic_levels_layer_independently_and_validate() {
    let root = std::env::temp_dir().join(format!(
        "cmake-import-config-{}-{}",
        std::process::id(),
        NEXT.fetch_add(1, Ordering::Relaxed)
    ));
    tokio::fs::create_dir_all(&root).await.unwrap();
    let user = root.join("user.toml");
    let project = root.join(".cmake-analyzer.toml");
    let explicit = root.join("explicit.toml");
    tokio::fs::write(&user,"[modules]\nexternal_include=\"functions\"\npackage_search_paths=[\"prefix\"]\n[diagnostics.rules]\nunknown_function=\"info\"\nunknown_variable=\"hint\"\n").await.unwrap();
    tokio::fs::write(&project,"[modules]\nexternal_find_package=\"none\"\n[diagnostics.rules]\nunknown_variable=\"none\"\n").await.unwrap();
    tokio::fs::write(
        &explicit,
        "[diagnostics.rules]\nunknown_function=\"warn\"\n",
    )
    .await
    .unwrap();
    let sources = ConfigSources {
        cwd: root.clone(),
        user_file: Some(user),
        explicit_file: Some(explicit),
    };
    let mut store = ConfigStore::load(sources.clone()).await.unwrap();
    let settings = store.for_directory(&root).await.unwrap();
    assert_eq!(settings.modules.external_include, ExternalIndex::Functions);
    assert_eq!(settings.modules.external_find_package, ExternalIndex::None);
    assert_eq!(
        settings.modules.package_search_paths,
        vec![root.join("prefix")]
    );
    assert_eq!(
        settings.diagnostics.rules.unknown_function,
        DiagnosticLevel::Warn
    );
    assert_eq!(
        settings.diagnostics.rules.unknown_variable,
        DiagnosticLevel::None
    );
    for invalid in [
        "[modules]\nexternal_include=\"all\"\n",
        "[modules]\nexternal_find_package=false\n",
        "[diagnostics.rules]\nunknown_variable=\"fatal\"\n",
        "[diagnostics.rules]\nunknown_variables=\"none\"\n",
    ] {
        tokio::fs::write(&project, invalid).await.unwrap();
        let mut store = ConfigStore::load(sources.clone()).await.unwrap();
        assert!(store.for_directory(&root).await.is_err(), "{invalid}");
    }
    tokio::fs::remove_dir_all(root).await.unwrap();
}
