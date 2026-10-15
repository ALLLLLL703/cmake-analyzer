// AI-generated manual runtime settings and argument validation tests.
use cmake_analyzer::config::{ConfigSources, ConfigStore};
use std::sync::atomic::Ordering;
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
