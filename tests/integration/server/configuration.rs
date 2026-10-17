// AI-generated configuration RPC and module discovery tests.
//! Configuration reload RPC, module search paths and interpreter-profile transitions.
//! Temporary TOML rewrites test transactional success/failure while source versions stay fixed.
use super::session::{Project, Session, uri};
use serde_json::json;

/// Send cmake-analyzer/reloadConfig with caller-selected parameters.
/// The server may rebuild workspaces or reject configuration transactionally.
///
/// # Arguments
/// - `client` `[in,out]`: writes the request and consumes its reply; unrelated messages append
///   to pending.
/// - `id` `[in]`: request/response correlation ID.
/// - `params` `[in]`: consumed JSON object/null, including optional override path or deliberate
///   invalid input.
///
/// # Returns
/// The full reply; this helper leaves success/error assertions to each scenario.
///
/// # Panics
/// Panics on session transport/deadline failures.
///
/// # Examples
/// ```text
/// let reply = reload(&mut client, 10, json!({"path":"global.toml"})).await;
/// ```
async fn reload(client: &mut Session, id: i32, params: serde_json::Value) -> serde_json::Value {
    client
        .send(
            json!({"jsonrpc":"2.0","id":id,"method":"cmake-analyzer/reloadConfig","params":params}),
        )
        .await;
    client.response(id).await
}

#[tokio::test]
/// Check default cmake, cmake/Modules and cmake/modules search directories resolve.
/// Creates temporary files and an analyzer session; successful completion stops the session and
/// removes fixtures.
///
/// # Arguments
/// None.
///
/// # Returns
/// `()` if every assertion passes; no output parameter.
///
/// # Panics
/// Panics on any failed assertion or unwrapped fixture/transport/process failure.
/// Cleanup is explicit on the success path; a panic may leave a fixture tree.
///
/// # Algorithm
/// Write one module in each default directory and include all three from the root without TOML.
/// Open the root and require an empty version-1 diagnostic publication.
///
/// # Flow
/// ```text
/// [Temporary sources + settings]      [Expected: empty root diagnostics]
///              |                              |
///              v                              |
/// [LSP scenario messages]                 |
///              |                              |
///              +-------------+----------------+
///                            v
///                  [Compare assertions]
///                            |
///                            v
///             [Finish session + remove tree]
/// ```
///
/// # Examples
/// ```text
/// default_project_module_directories_work_without_configuration(); // #[tokio::test] supplies the runtime
/// ```
async fn default_project_module_directories_work_without_configuration() {
    let project = Project::new().await;
    let text = "cmake_minimum_required(VERSION 3.31)\ninclude(DirectDefault)\ninclude(UpperDefault)\ninclude(LowerDefault)\n";
    let root = project.write("CMakeLists.txt", text).await;
    for (path, name) in [
        ("cmake/DirectDefault.cmake", "D"),
        ("cmake/Modules/UpperDefault.cmake", "U"),
        ("cmake/modules/LowerDefault.cmake", "L"),
    ] {
        project
            .write(
                path,
                &format!("cmake_minimum_required(VERSION 3.31)\nset({name} value)\n"),
            )
            .await;
    }
    let mut client = Session::start(&project.0, vec![], true).await;
    client.open(&root, text).await;
    assert_eq!(
        client.diagnostics(&root, 1).await["params"]["diagnostics"],
        json!([])
    );
    client.finish().await;
    project.remove().await;
}

#[tokio::test]
/// Check module-search reload discovers nested includes and disabling modules restores
/// failures.
/// Creates temporary files and an analyzer session; successful completion stops the session and
/// removes fixtures.
///
/// # Arguments
/// None.
///
/// # Returns
/// `()` if every assertion passes; no output parameter.
///
/// # Panics
/// Panics on any failed assertion or unwrapped fixture/transport/process failure.
/// Cleanup is explicit on the success path; a panic may leave a fixture tree.
///
/// # Algorithm
/// Start with unresolved Outer/BuiltOnly plus a valid child project. Write search_paths and
/// builtin_directory, reload and require empty root diagnostics. Open Inner with an unknown
/// command, then disable modules and require root diagnostics again.
///
/// # Flow
/// ```text
/// [Temporary sources + settings]      [Expected: discovery / disabled diagnostics]
///              |                              |
///              v                              |
/// [LSP scenario messages]                 |
///              |                              |
///              +-------------+----------------+
///                            v
///                  [Compare assertions]
///                            |
///                            v
///             [Finish session + remove tree]
/// ```
///
/// # Examples
/// ```text
/// reload_search_paths_discovers_nested_modules_and_subdirectories(); // #[tokio::test] supplies the runtime
/// ```
async fn reload_search_paths_discovers_nested_modules_and_subdirectories() {
    let project = Project::new().await;
    let root = project
        .write(
            "CMakeLists.txt",
            "include(Outer)\ninclude(BuiltOnly)\nadd_subdirectory(child)\n",
        )
        .await;
    project
        .write("child/CMakeLists.txt", "set(CHILD x)\n")
        .await;
    project
        .write("modules/Outer.cmake", "include(Inner)\n")
        .await;
    project.write("modules/Inner.cmake", "set(INNER x)\n").await;
    project
        .write("builtin/BuiltOnly.cmake", "set(BUILTIN x)\n")
        .await;
    let mut client = Session::start(
        &project.0,
        vec![json!({"uri":uri(&project.0),"name":"project"})],
        true,
    )
    .await;
    client
        .open(
            &root,
            "include(Outer)\ninclude(BuiltOnly)\nadd_subdirectory(child)\n",
        )
        .await;
    assert!(
        !client.diagnostics(&root, 1).await["params"]["diagnostics"]
            .as_array()
            .unwrap()
            .is_empty()
    );
    tokio::fs::write(
        project.0.join(".cmake-analyzer.toml"),
        "[modules]\nenable=true\nsearch_paths=['modules']\nbuiltin_directory='builtin'\n",
    )
    .await
    .unwrap();
    assert_eq!(
        reload(&mut client, 10, json!({})).await["result"]["reloaded"],
        true
    );
    assert_eq!(
        client.diagnostics(&root, 1).await["params"]["diagnostics"],
        json!([])
    );
    let child = project.0.join("modules/Inner.cmake");
    client.open(&child, "unknown_command()\n").await;
    assert!(
        !client.diagnostics(&child, 1).await["params"]["diagnostics"]
            .as_array()
            .unwrap()
            .is_empty()
    );
    tokio::fs::write(
        project.0.join(".cmake-analyzer.toml"),
        "[modules]\nenable=false\nsearch_paths=['modules']\nbuiltin_directory='builtin'\n",
    )
    .await
    .unwrap();
    assert_eq!(
        reload(&mut client, 11, json!({})).await["result"]["reloaded"],
        true
    );
    assert!(
        !client.diagnostics(&root, 1).await["params"]["diagnostics"]
            .as_array()
            .unwrap()
            .is_empty()
    );
    client.finish().await;
    project.remove().await;
}

#[tokio::test]
/// Check explicit override switching is transactional for diagnostics and symbols.
/// Creates temporary files and an analyzer session; successful completion stops the session and
/// removes fixtures.
///
/// # Arguments
/// None.
///
/// # Returns
/// `()` if every assertion passes; no output parameter.
///
/// # Panics
/// Panics on any failed assertion or unwrapped fixture/transport/process failure.
/// Cleanup is explicit on the success path; a panic may leave a fixture tree.
///
/// # Algorithm
/// Disable both features with a relative override path and require cleared diagnostics/null
/// symbols. Invalid TOML and a missing new path fail without enabling symbols. Repair the
/// retained override, reload and require diagnostics/symbols again; numeric path input is
/// -32602.
///
/// # Flow
/// ```text
/// [Temporary sources + settings]      [Expected: feature state + reload errors]
///              |                              |
///              v                              |
/// [LSP scenario messages]                 |
///              |                              |
///              +-------------+----------------+
///                            v
///                  [Compare assertions]
///                            |
///                            v
///             [Finish session + remove tree]
/// ```
///
/// # Examples
/// ```text
/// reload_switches_global_overrides_and_preserves_old_settings_on_error(); // #[tokio::test] supplies the runtime
/// ```
async fn reload_switches_global_overrides_and_preserves_old_settings_on_error() {
    let project = Project::new().await;
    let root = project.write("CMakeLists.txt", "unknown_command()\n").await;
    let mut client = Session::start(&project.0, vec![], true).await;
    client.open(&root, "unknown_command()\n").await;
    assert!(
        !client.diagnostics(&root, 1).await["params"]["diagnostics"]
            .as_array()
            .unwrap()
            .is_empty()
    );
    let override_path = project.0.join("override.toml");
    tokio::fs::write(
        &override_path,
        "[diagnostics]\nenable=false\n[document_symbols]\nenable=false\n",
    )
    .await
    .unwrap();
    assert_eq!(
        reload(&mut client, 20, json!({"path":"override.toml"})).await["result"]["reloaded"],
        true
    );
    assert_eq!(
        client.diagnostics(&root, 1).await["params"]["diagnostics"],
        json!([])
    );
    assert!(client.symbols(&root, 21).await["result"].is_null());
    tokio::fs::write(&override_path, "[diagnostics]\nenable='invalid'\n")
        .await
        .unwrap();
    assert!(
        reload(&mut client, 22, json!({}))
            .await
            .get("error")
            .is_some()
    );
    assert!(client.symbols(&root, 23).await["result"].is_null());
    assert!(
        reload(&mut client, 24, json!({"path":"missing.toml"}))
            .await
            .get("error")
            .is_some()
    );
    tokio::fs::write(
        &override_path,
        "[diagnostics]\nenable=true\n[document_symbols]\nenable=true\n",
    )
    .await
    .unwrap();
    assert_eq!(
        reload(&mut client, 25, json!({})).await["result"]["reloaded"],
        true
    );
    assert!(
        !client.diagnostics(&root, 1).await["params"]["diagnostics"]
            .as_array()
            .unwrap()
            .is_empty()
    );
    assert!(client.symbols(&root, 26).await["result"].is_array());
    assert_eq!(
        reload(&mut client, 27, json!({"path":42})).await["error"]["code"],
        -32602
    );
    client.finish().await;
    project.remove().await;
}

#[tokio::test]
/// Check one explicit module override rebuilds both independent workspaces.
/// Creates temporary files and an analyzer session; successful completion stops the session and
/// removes fixtures.
///
/// # Arguments
/// None.
///
/// # Returns
/// `()` if every assertion passes; no output parameter.
///
/// # Panics
/// Panics on any failed assertion or unwrapped fixture/transport/process failure.
/// Cleanup is explicit on the success path; a panic may leave a fixture tree.
///
/// # Algorithm
/// Open two roots that cannot resolve Shared, then reload global.toml containing its relative
/// search path. Require two rebuilt workspaces and empty diagnostics for both unchanged
/// version-1 documents.
///
/// # Flow
/// ```text
/// [Temporary sources + settings]      [Expected: two rebuilt workspaces]
///              |                              |
///              v                              |
/// [LSP scenario messages]                 |
///              |                              |
///              +-------------+----------------+
///                            v
///                  [Compare assertions]
///                            |
///                            v
///             [Finish session + remove tree]
/// ```
///
/// # Examples
/// ```text
/// explicit_module_override_applies_to_all_workspaces(); // #[tokio::test] supplies the runtime
/// ```
async fn explicit_module_override_applies_to_all_workspaces() {
    let project = Project::new().await;
    let left = project
        .write("left/CMakeLists.txt", "include(Shared)\n")
        .await;
    let right = project
        .write("right/CMakeLists.txt", "include(Shared)\n")
        .await;
    project
        .write("shared/Shared.cmake", "set(SHARED x)\n")
        .await;
    let folders = vec![
        json!({"uri":uri(&project.0.join("left")),"name":"left"}),
        json!({"uri":uri(&project.0.join("right")),"name":"right"}),
    ];
    let mut client = Session::start(&project.0, folders, true).await;
    for file in [&left, &right] {
        client.open(file, "include(Shared)\n").await;
        assert!(
            !client.diagnostics(file, 1).await["params"]["diagnostics"]
                .as_array()
                .unwrap()
                .is_empty()
        );
    }
    tokio::fs::write(
        project.0.join("global.toml"),
        "[modules]\nsearch_paths=['shared']\n",
    )
    .await
    .unwrap();
    assert_eq!(
        reload(&mut client, 40, json!({"path":"global.toml"})).await["result"]["workspaces"],
        2
    );
    for file in [&left, &right] {
        assert_eq!(
            client.diagnostics(file, 1).await["params"]["diagnostics"],
            json!([])
        );
    }
    client.finish().await;
    project.remove().await;
}

#[tokio::test]
/// Check removing a script execution profile retains file-local missing-path warnings.
/// Creates temporary files and an analyzer session; successful completion stops the session and
/// removes fixtures.
///
/// # Arguments
/// None.
///
/// # Returns
/// `()` if every assertion passes; no output parameter.
///
/// # Panics
/// Panics on any failed assertion or unwrapped fixture/transport/process failure.
/// Cleanup is explicit on the success path; a panic may leave a fixture tree.
///
/// # Algorithm
/// Map loose.cmake to cmake -P and require its missing include warning. Empty the configuration
/// and reload; require another severity-2 missing.cmake warning even without an execution
/// profile.
///
/// # Flow
/// ```text
/// [Temporary sources + settings]      [Expected: missing-path warning after reload]
///              |                              |
///              v                              |
/// [LSP scenario messages]                 |
///              |                              |
///              +-------------+----------------+
///                            v
///                  [Compare assertions]
///                            |
///                            v
///             [Finish session + remove tree]
/// ```
///
/// # Examples
/// ```text
/// interpreter_script_mode_traverses_dependencies_and_reload_can_remove_it(); // #[tokio::test] supplies the runtime
/// ```
async fn interpreter_script_mode_traverses_dependencies_and_reload_can_remove_it() {
    let project = Project::new().await;
    let script = project
        .write("loose.cmake", "include(missing.cmake)\n")
        .await;
    let config = project.0.join(".cmake-analyzer.toml");
    tokio::fs::write(
        &config,
        "[interpreter_commands]\n'loose.cmake'='cmake -P loose.cmake'\n",
    )
    .await
    .unwrap();
    let mut client = Session::start(&project.0, vec![], true).await;
    client.open(&script, "include(missing.cmake)\n").await;
    let diagnostics = client.diagnostics(&script, 1).await;
    assert!(
        diagnostics["params"]["diagnostics"]
            .as_array()
            .unwrap()
            .iter()
            .any(|item| item["severity"] == 2 && item["range"]["start"]["line"] == 1),
        "{diagnostics}"
    );
    tokio::fs::write(&config, "").await.unwrap();
    assert_eq!(
        reload(&mut client, 30, json!({})).await["result"]["reloaded"],
        true
    );
    // Path checks remain available without an execution profile.
    let diagnostics = client.diagnostics(&script, 1).await;
    assert!(
        diagnostics["params"]["diagnostics"]
            .as_array()
            .unwrap()
            .iter()
            .any(|item| item["severity"] == 2
                && item["message"].as_str().unwrap().contains("missing.cmake")),
        "{diagnostics}"
    );
    client.finish().await;
    project.remove().await;
}
