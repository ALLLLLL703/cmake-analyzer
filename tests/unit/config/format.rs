// AI-generated tests.
use super::{ConfigSources, ConfigStore, NEXT};
use std::sync::atomic::Ordering;
#[tokio::test]
async fn formatting_defaults_layer_and_reject_invalid_sizes_or_fields() {
    let root = std::env::temp_dir().join(format!(
        "cmake-format-config-{}-{}",
        std::process::id(),
        NEXT.fetch_add(1, Ordering::Relaxed)
    ));
    tokio::fs::create_dir_all(&root).await.unwrap();
    let project = root.join(".cmake-analyzer.toml");
    tokio::fs::write(&project, "[modules]\nenable=false\n")
        .await
        .unwrap();
    let mut sources = ConfigSources {
        cwd: root.clone(),
        user_file: None,
        explicit_file: None,
    };
    let mut store = ConfigStore::load(sources.clone()).await.unwrap();
    let settings = store.for_directory(&root).await.unwrap();
    assert!(settings.format.on_save);
    assert!(settings.format.insert_spaces);
    assert_eq!(settings.format.indent_width, 2);
    assert_eq!(settings.format.line_width, 100);
    let user = root.join("user.toml");
    tokio::fs::write(
        &user,
        "[format]\non_save=false\ninsert_spaces=false\nindent_width=4\nline_width=80\n",
    )
    .await
    .unwrap();
    sources.user_file = Some(user);
    tokio::fs::write(&project, "[format]\non_save=true\nline_width=60\n")
        .await
        .unwrap();
    let override_file = root.join("override.toml");
    tokio::fs::write(&override_file, "[format]\nindent_width=3\n")
        .await
        .unwrap();
    sources.explicit_file = Some(override_file);
    let mut store = ConfigStore::load(sources.clone()).await.unwrap();
    let settings = store.for_directory(&root).await.unwrap();
    assert!(settings.format.on_save);
    assert_eq!(settings.format.indent_width, 3);
    assert!(!settings.format.insert_spaces);
    assert_eq!(settings.format.line_width, 60);
    for invalid in [
        "[format]\nline_width=0\n",
        "[format]\nindent_width=0\n",
        "[format]\nindent_width=256\n",
        "[format]\non_save=1\n",
        "[format]\nunknown=true\n",
    ] {
        tokio::fs::write(&project, invalid).await.unwrap();
        let mut store = ConfigStore::load(sources.clone()).await.unwrap();
        assert!(store.for_directory(&root).await.is_err(), "{invalid}");
    }
    tokio::fs::remove_dir_all(root).await.unwrap();
}
