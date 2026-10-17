//! Private Unix discovery fixtures loaded by `config::defaults`. Fake CMake executables answer only
//! `--version`; tests create/remove installation trees and symlinks, never configure projects.

// Contains AI-generated documentation edits.
// AI-generated private tests for installed CMake module discovery.
use super::discover_from_path;
use crate::config::{ConfigSources, ConfigStore};
use std::{
    path::PathBuf,
    sync::atomic::{AtomicU64, Ordering},
};
/// Process-local suffix counter for collision-free temporary fixture names.
/// Example: `NEXT.fetch_add(1, Ordering::Relaxed)` reserves the next local suffix.
static NEXT: AtomicU64 = AtomicU64::new(0);
/// Temporary fake CMake prefix used by the private Unix discovery tests.
/// [`Self::new`] writes its executable, [`Self::modules`] supplies candidate module trees and
/// [`Self::remove`] deletes the owned root. Construction does not launch a process; discovery does.
///
/// # Examples
/// Local async test context:
/// ```text
/// let install = Installation::new("cmake version 3.31.9", 0).await;
/// let modules = install.modules("cmake-3.31").await;
/// assert!(modules.starts_with(&install.root));
/// install.remove().await;
/// ```
struct Installation {
    /// Unique cleanup root containing the fake prefix and optional symlink directory.
    /// Example: `let links = install.root.join("links");` locates a separate PATH directory.
    root: PathBuf,
    /// Executable directory at `root/prefix space/bin`, deliberately testing spaces in paths.
    /// Example: `install.bin.join("cmake")` selects the fake version executable.
    bin: PathBuf,
}
impl Installation {
    /// Create a unique temporary fake installation with a Unix executable that accepts only
    /// `--version`. Writes a shell fixture and sets mode 0700; does not launch it. Used by the
    /// discovery tests through [`Installation`].
    ///
    /// # Arguments
    /// - `output` `[in]`: Text printed by the fixture, embedded verbatim inside shell single quotes;
    ///   callers use fixed quote-free version strings.
    /// - `status` `[in]`: Exit status of the fake version command; tests use zero for success and one
    ///   for failure.
    ///
    /// # Returns
    /// An installation owning its temporary root and executable directory; callers must invoke
    /// [`Installation::remove`] to clean it up.
    ///
    /// # Panics
    /// If temporary-directory creation, executable writing or permission changes fail.
    ///
    /// # Examples
    /// Local test-module context (await inside an async test):
    /// ```text
    /// let install = Installation::new("cmake version 3.31.9", 0).await;
    /// let modules = install.modules("cmake-3.31").await;
    /// install.remove().await;
    /// ```
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
    /// Create a named share-directory module tree beneath the fake prefix. Mutates the filesystem, not
    /// the installation record; does not execute CMake.
    ///
    /// # Arguments
    /// - `self` `[in]`: Installation whose bin parent supplies the fake prefix.
    /// - `name` `[in]`: Share child, such as `cmake-3.31` or `cmake`.
    ///
    /// # Returns
    /// The created `prefix/share/<name>/Modules` path.
    ///
    /// # Panics
    /// If bin has no parent or directory creation fails; constructor-created instances always have a
    /// parent.
    ///
    /// # Examples
    /// Local test-module context (await inside an async test):
    /// ```text
    /// let install = Installation::new("cmake version 3.31.9", 0).await;
    /// let path = install.modules("cmake-3.31").await;
    /// assert!(path.ends_with("cmake-3.31/Modules"));
    /// install.remove().await;
    /// ```
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
    /// Consume a fake installation and recursively delete its temporary root. Performs filesystem
    /// cleanup, with no process execution.
    ///
    /// # Arguments
    /// - `self` `[in]`: Owned installation to consume; its root and all fixtures below it are deleted.
    ///
    /// # Returns
    /// `()` on successful removal; no record is returned.
    ///
    /// # Panics
    /// If recursive directory removal fails.
    ///
    /// # Examples
    /// Local test-module context (await inside an async test):
    /// ```text
    /// let install = Installation::new("cmake version 3.31.9", 0).await;
    /// install.remove().await;
    /// ```
    async fn remove(self) {
        tokio::fs::remove_dir_all(self.root).await.unwrap();
    }
}
/// Create a fake 3.31.9 installation with mismatched, matching and unversioned module trees.
///
/// Creates and removes temporary Unix installation fixtures; discovery launches only fake --version
/// processes where requested, never project commands.
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
/// Resolve CMake through a symlink in a separate directory; prefer 3.31, then remove it and require
/// the unversioned fallback.
///
/// # Flow
/// ```text
///   [symlink + fake 3.31.9]
///    |
///   [discover matching tree]----------> [remove 3.31 tree]
///                                      |
///                                     [fall back to unversioned]
///                                      |
///                                     < checks hold? >
///                                      /         \
///                                    [pass]     [panic]
/// ```
///
/// # Examples
/// Direct local call outside any Tokio runtime (the test attribute creates its runtime):
/// ```text
/// discovers_matching_version_and_resolves_executable_symlinks();
/// ```
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
/// For failed status, malformed version text and a valid version without module directories,
/// require no discovery result.
///
/// Creates and removes temporary Unix installation fixtures; discovery launches only fake --version
/// processes where requested, never project commands.
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
/// Remove each fake executable and require absence again rather than an error.
///
/// # Flow
/// ```text
///   [status/version/tree cases]
///    |
///   [discover via fixture]---------------> [remove executable]
///                                         |
///                                        [still return None]
///                                         |
///                                        < checks hold? >
///                                         /         \
///                                       [pass]     [panic]
/// ```
///
/// # Examples
/// Direct local call outside any Tokio runtime (the test attribute creates its runtime):
/// ```text
/// missing_failed_and_invalid_installations_are_optional();
/// ```
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
/// Write an explicit builtin path, then disable modules, checking that neither lookup initializes
/// the lazy discovery cell.
///
/// Creates and removes temporary Unix installation fixtures; discovery launches only fake --version
/// processes where requested, never project commands.
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
/// Remove the project file, seed the cell with a fake module tree and require both entry
/// directories to reuse it; a new store must start uncached.
///
/// # Flow
/// ```text
///   [override / disabled modules]
///    |
///   [leave discovery cell empty]-----------> [seed cell for two entries]
///                                           |
///                                          [share until store reload]
///                                           |
///                                          < checks hold? >
///                                           /         \
///                                         [pass]     [panic]
/// ```
///
/// # Examples
/// Direct local call outside any Tokio runtime (the test attribute creates its runtime):
/// ```text
/// overrides_and_disabled_modules_skip_discovery_and_cache_resets_on_reload();
/// ```
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
