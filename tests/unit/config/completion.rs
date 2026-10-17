//! Completion setting inheritance through temporary user, project and explicit TOML files,
//! including explicit false and invalid fields.

// Contains AI-generated documentation edits.
// AI-generated completion configuration defaults, layering, and validation tests.
use super::{ConfigSources, ConfigStore, NEXT};
use std::sync::atomic::Ordering;
/// Start without configuration, inherit user `reserve_arg=true` through an unrelated project
/// setting, then override it with project false and explicit true.
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
/// Rewrite the project file with a numeric value and a misspelled key to require validation errors.
///
/// # Flow
/// ```text
///   [default/user/project/explicit]
///    |
///   [read reserve_arg]-----------------------> [invalid type / key]
///                                             |
///                                            [reject reload]
///                                             |
///                                            < checks hold? >
///                                             /         \
///                                           [pass]     [panic]
/// ```
///
/// # Examples
/// Direct local call outside any Tokio runtime (the test attribute creates its runtime):
/// ```text
/// reserve_arg_defaults_false_inherits_and_accepts_explicit_false_overrides();
/// ```
#[tokio::test]
async fn reserve_arg_defaults_false_inherits_and_accepts_explicit_false_overrides() {
    let root = std::env::temp_dir().join(format!(
        "cmake-completion-config-{}-{}",
        std::process::id(),
        NEXT.fetch_add(1, Ordering::Relaxed)
    ));
    tokio::fs::create_dir_all(&root).await.unwrap();
    let user = root.join("user.toml");
    let explicit = root.join("explicit.toml");
    let project = root.join(".cmake-analyzer.toml");
    let mut sources = ConfigSources {
        cwd: root.clone(),
        user_file: None,
        explicit_file: None,
    };
    let mut store = ConfigStore::load(sources.clone()).await.unwrap();
    assert!(
        !store
            .for_directory(&root)
            .await
            .unwrap()
            .completion
            .reserve_arg
    );
    tokio::fs::write(&user, "[completion]\nreserve_arg=true\n")
        .await
        .unwrap();
    sources.user_file = Some(user);
    tokio::fs::write(&project, "[modules]\nenable=false\n")
        .await
        .unwrap();
    let mut store = ConfigStore::load(sources.clone()).await.unwrap();
    assert!(
        store
            .for_directory(&root)
            .await
            .unwrap()
            .completion
            .reserve_arg
    );
    tokio::fs::write(&project, "[completion]\nreserve_arg=false\n")
        .await
        .unwrap();
    let mut store = ConfigStore::load(sources.clone()).await.unwrap();
    assert!(
        !store
            .for_directory(&root)
            .await
            .unwrap()
            .completion
            .reserve_arg
    );
    tokio::fs::write(&explicit, "[completion]\nreserve_arg=true\n")
        .await
        .unwrap();
    sources.explicit_file = Some(explicit);
    let mut store = ConfigStore::load(sources.clone()).await.unwrap();
    assert!(
        store
            .for_directory(&root)
            .await
            .unwrap()
            .completion
            .reserve_arg
    );
    for invalid in [
        "[completion]\nreserve_arg=1\n",
        "[completion]\nreserve_args=true\n",
    ] {
        tokio::fs::write(&project, invalid).await.unwrap();
        let mut store = ConfigStore::load(sources.clone()).await.unwrap();
        assert!(store.for_directory(&root).await.is_err());
    }
    tokio::fs::remove_dir_all(root).await.unwrap();
}
