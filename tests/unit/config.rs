//! Configuration and literal interpreter-command regressions. Temporary TOML layers exercise
//! precedence, path bases, validation and cache reuse; interpreter commands are parsed, not
//! executed.

// Contains AI-generated documentation edits.
// AI-generated configuration and interpreter tests.
mod completion;
mod format;
mod imports;
mod limits;
mod module_defaults;
mod runtime;
use cmake_analyzer::{
    config::{ConfigSources, ConfigStore},
    parser::interpreter_commands::{InterpreterServices, parse},
};
use std::{
    path::PathBuf,
    sync::atomic::{AtomicU64, Ordering},
};
/// Process-local suffix counter for collision-free temporary fixture names.
/// Example: `NEXT.fetch_add(1, Ordering::Relaxed)` reserves the next local suffix.
static NEXT: AtomicU64 = AtomicU64::new(0);

/// Parse quoted project source/build paths and a script path relative to `/workspace`, then reject
/// missing operands, mixed modes, unsupported programs and shell separators. This proves service
/// selection is literal parsing, not execution.
///
/// Only local values and analysis buffers are created or changed; no file, network or process side
/// effects.
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
/// # Examples
/// Local test-module call:
/// ```text
/// interpreter_commands_select_services_without_execution();
/// ```
#[test]
fn interpreter_commands_select_services_without_execution() {
    let cwd = PathBuf::from("/workspace");
    assert_eq!(
        parse("/usr/bin/cmake -S 'source dir' -Bbuild", &cwd).unwrap(),
        InterpreterServices::Project {
            source_directory: cwd.join("source dir"),
            binary_directory: Some(cwd.join("build"))
        }
    );
    assert_eq!(
        parse("cmake -P \"scripts/my script.cmake\"", &cwd).unwrap(),
        InterpreterServices::Script {
            file: cwd.join("scripts/my script.cmake")
        }
    );
    for command in [
        "",
        "other -P script.cmake",
        "cmake -S",
        "cmake -S a -P b",
        "cmake -B a",
        "cmake -P a -P b",
        "cmake --build build",
        "cmake -P a ; touch marker",
        "cmake -P 'unterminated",
    ] {
        assert!(parse(command, &cwd).is_err(), "{command}");
    }
}
/// Write user, project and explicit TOML layers; check field-wise precedence, each layer's
/// module-path base and startup-relative interpreter replacement.
///
/// Creates, rewrites and removes temporary TOML files/directories; enabled module discovery may
/// inspect CMake --version, never execute a project.
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
/// Repeated lookup must share its settings allocation. Rewrite the project layer with a wrong type
/// and a misspelled table and reload to require errors.
///
/// # Flow
/// ```text
///   [three TOML layers]
///    |
///   [merge + cached lookup]----------> [wrong type / table name]
///                                     |
///                                    [reject on reload]
///                                     |
///                                    < checks hold? >
///                                     /         \
///                                   [pass]     [panic]
/// ```
///
/// # Examples
/// Direct local call outside any Tokio runtime (the test attribute creates its runtime):
/// ```text
/// configuration_layers_inherit_fields_and_normalize_paths();
/// ```
#[tokio::test]
async fn configuration_layers_inherit_fields_and_normalize_paths() {
    let directory = std::env::temp_dir().join(format!(
        "cmake-config-{}-{}",
        std::process::id(),
        NEXT.fetch_add(1, Ordering::Relaxed)
    ));
    let project = directory.join("project");
    let overrides = directory.join("overrides");
    tokio::fs::create_dir_all(&project).await.unwrap();
    tokio::fs::create_dir_all(&overrides).await.unwrap();
    let user = directory.join("user.toml");
    tokio::fs::write(&user,"[diagnostics]\nenable=false\n[modules]\nsearch_paths=['user-modules']\nbuiltin_directory='builtin'\n[interpreter_commands]\n'one.cmake'='cmake -P one.cmake'\n").await.unwrap();
    tokio::fs::write(project.join(".cmake-analyzer.toml"),"[diagnostics]\nenable=true\n[modules]\nsearch_paths=['local-modules']\n[interpreter_commands]\n'./one.cmake'='cmake -P replacement.cmake'\n").await.unwrap();
    let explicit = overrides.join("extra.toml");
    tokio::fs::write(
        &explicit,
        "[document_symbols]\nenable=false\n[modules]\nsearch_paths=['override-modules']\n",
    )
    .await
    .unwrap();
    let mut store = ConfigStore::load(ConfigSources {
        cwd: directory.clone(),
        user_file: Some(user),
        explicit_file: Some(explicit),
    })
    .await
    .unwrap();
    let settings = store.for_directory(&project).await.unwrap();
    assert!(settings.diagnostics.enable);
    assert!(!settings.document_symbols.enable);
    assert!(settings.modules.enable);
    assert_eq!(
        settings.modules.search_paths,
        vec![overrides.join("override-modules")]
    );
    assert_eq!(
        settings.modules.builtin_directory,
        Some(directory.join("builtin"))
    );
    assert_eq!(
        settings.interpreter_commands[&directory.join("one.cmake")],
        InterpreterServices::Script {
            file: directory.join("replacement.cmake")
        }
    );
    assert!(std::sync::Arc::ptr_eq(
        &settings,
        &store.for_directory(&project).await.unwrap()
    ));
    tokio::fs::write(
        project.join(".cmake-analyzer.toml"),
        "[diagnostics]\nenable='wrong'\n",
    )
    .await
    .unwrap();
    let sources = store.sources.clone();
    assert!(
        ConfigStore::load(sources.clone())
            .await
            .unwrap()
            .for_directory(&project)
            .await
            .is_err()
    );
    tokio::fs::write(
        project.join(".cmake-analyzer.toml"),
        "[diagnostic]\nenable=false\n",
    )
    .await
    .unwrap();
    assert!(
        ConfigStore::load(sources)
            .await
            .unwrap()
            .for_directory(&project)
            .await
            .is_err()
    );
    tokio::fs::remove_dir_all(directory).await.unwrap();
}
