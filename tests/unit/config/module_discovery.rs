// AI-generated private tests for installed CMake module discovery.
use super::discover_from_path;
use crate::config::{ConfigSources, ConfigStore};
use std::{
    path::PathBuf,
    sync::atomic::{AtomicU64, Ordering},
};
static NEXT: AtomicU64 = AtomicU64::new(0);
struct Installation {
    root: PathBuf,
    bin: PathBuf,
}
impl Installation {
    async fn new(output: &str, status: u8) -> Self {
        use std::os::unix::fs::PermissionsExt;
        let root = std::env::temp_dir().join(format!(
            "cmake-module-discovery-{}-{}",
            std::process::id(),
            NEXT.fetch_add(1, Ordering::Relaxed)
        ));
        let bin = root.join("prefix space/bin");
        tokio::fs::create_dir_all(&bin).await.unwrap();
        let program = bin.join("cmake");
        tokio::fs::write(&program,format!("#!/bin/sh\n[ \"$#\" -eq 1 ] && [ \"$1\" = --version ] || exit 99\nprintf '%s\\n' '{output}'\nexit {status}\n")).await.unwrap();
        tokio::fs::set_permissions(&program, std::fs::Permissions::from_mode(0o700))
            .await
            .unwrap();
        Self { root, bin }
    }
    async fn modules(&self, name: &str) -> PathBuf {
        let path = self
            .bin
            .parent()
            .unwrap()
            .join("share")
            .join(name)
            .join("Modules");
        tokio::fs::create_dir_all(&path).await.unwrap();
        path
    }
    async fn remove(self) {
        tokio::fs::remove_dir_all(self.root).await.unwrap();
    }
}
#[tokio::test]
async fn discovers_matching_version_and_resolves_executable_symlinks() {
    let install = Installation::new("cmake version 3.31.9", 0).await;
    install.modules("cmake-3.30").await;
    let versioned = install.modules("cmake-3.31").await;
    let unversioned = install.modules("cmake").await;
    let links = install.root.join("links");
    tokio::fs::create_dir_all(&links).await.unwrap();
    std::os::unix::fs::symlink(install.bin.join("cmake"), links.join("cmake")).unwrap();
    assert_eq!(
        discover_from_path(links.as_os_str(), &install.root).await,
        Some(versioned.clone())
    );
    tokio::fs::remove_dir_all(versioned).await.unwrap();
    assert_eq!(
        discover_from_path(links.as_os_str(), &install.root).await,
        Some(unversioned)
    );
    install.remove().await;
}
#[tokio::test]
async fn missing_failed_and_invalid_installations_are_optional() {
    for (output, status) in [
        ("cmake version 3.31.9", 1),
        ("not a version", 0),
        ("cmake version invalid", 0),
        ("cmake version 3.31.9", 0),
    ] {
        let install = Installation::new(output, status).await;
        assert_eq!(
            discover_from_path(install.bin.as_os_str(), &install.root).await,
            None
        );
        tokio::fs::remove_file(install.bin.join("cmake"))
            .await
            .unwrap();
        assert_eq!(
            discover_from_path(install.bin.as_os_str(), &install.root).await,
            None
        );
        install.remove().await;
    }
}
#[tokio::test]
async fn overrides_and_disabled_modules_skip_discovery_and_cache_resets_on_reload() {
    let install = Installation::new("cmake version 3.31.9", 0).await;
    let sources = ConfigSources {
        cwd: install.root.clone(),
        user_file: None,
        explicit_file: None,
    };
    let project = install.root.join(".cmake-analyzer.toml");
    tokio::fs::write(&project, "[modules]\nbuiltin_directory='custom'\n")
        .await
        .unwrap();
    let mut store = ConfigStore::load(sources.clone()).await.unwrap();
    assert_eq!(
        store
            .for_directory(&install.root)
            .await
            .unwrap()
            .modules
            .builtin_directory,
        Some(install.root.join("custom"))
    );
    assert!(store.builtin_directory.get().is_none());
    tokio::fs::write(&project, "[modules]\nenable=false\n")
        .await
        .unwrap();
    let mut store = ConfigStore::load(sources.clone()).await.unwrap();
    assert!(
        store
            .for_directory(&install.root)
            .await
            .unwrap()
            .modules
            .builtin_directory
            .is_none()
    );
    assert!(store.builtin_directory.get().is_none());
    tokio::fs::remove_file(project).await.unwrap();
    let discovered = install.modules("cmake-3.31").await;
    store
        .builtin_directory
        .set(Some(discovered.clone()))
        .unwrap();
    let a = install.root.join("a");
    let b = install.root.join("b");
    assert_eq!(
        store
            .for_directory(&a)
            .await
            .unwrap()
            .modules
            .builtin_directory,
        Some(discovered.clone())
    );
    assert_eq!(
        store
            .for_directory(&b)
            .await
            .unwrap()
            .modules
            .builtin_directory,
        Some(discovered)
    );
    assert!(
        ConfigStore::load(sources)
            .await
            .unwrap()
            .builtin_directory
            .get()
            .is_none()
    );
    install.remove().await;
}
