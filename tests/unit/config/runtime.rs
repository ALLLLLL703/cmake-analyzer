//! Runtime configuration authority and safe argument validation. Temporary TOML files select
//! settings; no project configure or script is executed.

// Contains AI-generated documentation edits.
// AI-generated manual runtime settings and argument validation tests.
use cmake_analyzer::config::{ConfigSources, ConfigStore};
use std::sync::atomic::Ordering;
/// Write relative build/executable paths and bounded runtime arguments, then check normalized
/// settings.
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
/// Replace the file with build/script/trace-redirection overrides, a dangling define and zero
/// budgets; each lookup must reject the unsafe or invalid configuration.
///
/// # Flow
/// ```text
///   [runtime TOML]
///    |
///   [normalize paths + budgets]----------> [unsafe args / zero budgets]
///                                         |
///                                        [reject replacement layer]
///                                         |
///                                        < checks hold? >
///                                         /         \
///                                       [pass]     [panic]
/// ```
///
/// # Examples
/// Direct local call outside any Tokio runtime (the test attribute creates its runtime):
/// ```text
/// runtime_settings_layer_paths_and_reject_execution_mode_overrides();
/// ```
#[tokio::test]
async fn runtime_settings_layer_paths_and_reject_execution_mode_overrides() {
    let root = std::env::temp_dir().join(format!(
        "cmake-runtime-config-{}-{}",
        std::process::id(),
        super::NEXT.fetch_add(1, Ordering::Relaxed)
    ));
    tokio::fs::create_dir_all(&root).await.unwrap();
    let config = root.join(".cmake-analyzer.toml");
    let sources = ConfigSources {
        cwd: root.clone(),
        user_file: None,
        explicit_file: None,
    };
    tokio::fs::write(&config,"[modules]\nenable=false\n[runtime]\nbuild_directory='separate'\nexecutable='./tools/cmake'\narguments=['-DX=one','-G','Ninja']\ntimeout_seconds=5\nmax_output_bytes=1024\nmax_view_bytes=512\n").await.unwrap();
    let settings = ConfigStore::load(sources.clone())
        .await
        .unwrap()
        .for_directory(&root)
        .await
        .unwrap();
    assert!(settings.runtime.enable);
    assert_eq!(
        settings.runtime.build_directory,
        Some(root.join("separate"))
    );
    assert_eq!(settings.runtime.executable, root.join("tools/cmake"));
    assert_eq!(settings.runtime.timeout_seconds, 5);
    assert_eq!(settings.runtime.max_view_bytes, 512);
    for invalid in [
        "arguments=['-B','other']",
        "arguments=['--trace-redirect=secret']",
        "arguments=['-P','script.cmake']",
        "arguments=['--build','build']",
        "arguments=['-D']",
        "timeout_seconds=0",
        "max_output_bytes=0",
        "max_view_bytes=0",
    ] {
        tokio::fs::write(&config, format!("[runtime]\n{invalid}\n"))
            .await
            .unwrap();
        assert!(
            ConfigStore::load(sources.clone())
                .await
                .unwrap()
                .for_directory(&root)
                .await
                .is_err(),
            "{invalid}"
        );
    }
    tokio::fs::remove_dir_all(root).await.unwrap();
}
/// Reject project-only on-save execution, then authorize it in user configuration.
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
/// Rewrite the project layer to opt into action mode or disable runtime; a disabled user runtime
/// must not grant save authority. Finally load that project-named file explicitly as a global
/// source and accept its on-save mode.
///
/// # Flow
/// ```text
///   [project on_save]
///    |
///   [reject without user authority]----------> [global grant + project opt-outs]
///                                             |
///                                            [check effective runtime mode]
///                                             |
///                                            < checks hold? >
///                                             /         \
///                                           [pass]     [panic]
/// ```
///
/// # Examples
/// Direct local call outside any Tokio runtime (the test attribute creates its runtime):
/// ```text
/// save_execution_requires_global_authority_and_project_settings_can_opt_out();
/// ```
#[tokio::test]
async fn save_execution_requires_global_authority_and_project_settings_can_opt_out() {
    use cmake_analyzer::config::RuntimeMode;
    let root = std::env::temp_dir().join(format!(
        "cmake-save-config-{}-{}",
        std::process::id(),
        super::NEXT.fetch_add(1, Ordering::Relaxed)
    ));
    tokio::fs::create_dir_all(&root).await.unwrap();
    let project = root.join(".cmake-analyzer.toml");
    let user = root.join("user.toml");
    let mut sources = ConfigSources {
        cwd: root.clone(),
        user_file: Some(user.clone()),
        explicit_file: None,
    };
    tokio::fs::write(&user, "[modules]\nenable=false\n")
        .await
        .unwrap();
    tokio::fs::write(&project, "[runtime]\nmode='on_save'\n")
        .await
        .unwrap();
    assert!(
        ConfigStore::load(sources.clone())
            .await
            .unwrap()
            .for_directory(&root)
            .await
            .unwrap_err()
            .to_string()
            .contains("requires user configuration")
    );
    tokio::fs::write(
        &user,
        "[modules]\nenable=false\n[runtime]\nmode='on_save'\n",
    )
    .await
    .unwrap();
    tokio::fs::write(&project, "").await.unwrap();
    assert_eq!(
        ConfigStore::load(sources.clone())
            .await
            .unwrap()
            .for_directory(&root)
            .await
            .unwrap()
            .runtime
            .mode,
        RuntimeMode::OnSave
    );
    tokio::fs::write(&project, "[runtime]\nmode='action'\n")
        .await
        .unwrap();
    assert_eq!(
        ConfigStore::load(sources.clone())
            .await
            .unwrap()
            .for_directory(&root)
            .await
            .unwrap()
            .runtime
            .mode,
        RuntimeMode::Action
    );
    tokio::fs::write(&project, "[runtime]\nenable=false\n")
        .await
        .unwrap();
    assert!(
        !ConfigStore::load(sources.clone())
            .await
            .unwrap()
            .for_directory(&root)
            .await
            .unwrap()
            .runtime
            .enable
    );
    tokio::fs::write(
        &user,
        "[modules]\nenable=false\n[runtime]\nenable=false\nmode='on_save'\n",
    )
    .await
    .unwrap();
    tokio::fs::write(&project, "[runtime]\nenable=true\n")
        .await
        .unwrap();
    assert_eq!(
        ConfigStore::load(sources.clone())
            .await
            .unwrap()
            .for_directory(&root)
            .await
            .unwrap()
            .runtime
            .mode,
        RuntimeMode::Action
    );
    sources.user_file = None;
    sources.explicit_file = Some(project.clone());
    tokio::fs::write(
        &project,
        "[modules]\nenable=false\n[runtime]\nmode='on_save'\n",
    )
    .await
    .unwrap();
    assert_eq!(
        ConfigStore::load(sources)
            .await
            .unwrap()
            .for_directory(&root)
            .await
            .unwrap()
            .runtime
            .mode,
        RuntimeMode::OnSave
    );
    tokio::fs::remove_dir_all(root).await.unwrap();
}
