// AI-generated configured script execution and precedence regression tests.
//! Explicit script profiles, dynamic import observations and configure-flag rejection.
//! Real CMake -P runs are requested through actions; temporary outputs are removed afterward.
use super::{
    actions::{actions, execute},
    hovers::hover,
    navigation::definition,
    session::{Project, Session},
};
use serde_json::json;
#[tokio::test]
/// Check a reloaded nested -P profile executes only on command and installs dynamic import
/// observations.
/// Creates temporary files and an analyzer session; successful completion stops the session and
/// removes fixtures. Execution scenarios may run real CMake or a local stub and write temporary
/// output.
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
/// Initially no actions exist. Reload script mapping/runtime arguments, list runScript with
/// script profile and require no marker until execute. Await success, inspect runtime function
/// comments and dependency definition, expanded CHOICE/script path and cleared unknown-function
/// diagnostics; no build directory/cache is created.
///
/// # Flow
/// ```text
/// [Temporary sources + settings]      [Expected: script observations / no configure files]
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
/// configured_nested_script_reloads_from_startup_settings_and_observes_dynamic_imports(); // #[tokio::test] supplies the runtime
/// ```
async fn configured_nested_script_reloads_from_startup_settings_and_observes_dynamic_imports() {
    let p = Project::new().await;
    let text = "file(WRITE \"${CMAKE_CURRENT_LIST_DIR}/../script-ran\" yes)\nfunction(resolve output)\nset(${output} \"${CMAKE_CURRENT_LIST_DIR}/../modules\" PARENT_SCOPE)\nendfunction()\nresolve(DIR)\ninclude(\"${DIR}/observed.cmake\")\nscript_fn()\nmessage(\"${CHOICE}\")\nmessage(\"${CMAKE_SCRIPT_MODE_FILE}\")\n";
    let root = p.write("tools/check.cmake", text).await;
    p.write(
        "modules/observed.cmake",
        "# Runtime-only function documentation.\nfunction(script_fn)\nendfunction()\n",
    )
    .await;
    let mut client = Session::start(&p.0, vec![], true).await;
    client.open(&root, text).await;
    client.diagnostics(&root, 1).await;
    assert_eq!(
        actions(&mut client, &root, 1, 10, false).await["result"],
        json!([])
    );
    tokio::fs::write(p.0.join(".cmake-analyzer.toml"),"[runtime]\narguments=['-DCHOICE=selected']\nbuild_directory='unused-build'\n[interpreter_commands]\n\"tools/check.cmake\"='cmake -P tools/check.cmake'\n").await.unwrap();
    client
        .send(json!({"jsonrpc":"2.0","id":11,"method":"cmake-analyzer/reloadConfig","params":{}}))
        .await;
    let reloaded = client.response(11).await;
    assert!(reloaded.get("error").is_none(), "{reloaded}");
    let listed = actions(&mut client, &root, 1, 12, false).await;
    let action = &listed["result"][0];
    assert_eq!(action["command"]["command"], "cmake-analyzer.runScript");
    assert_eq!(action["command"]["arguments"][3], "script");
    assert!(!p.0.join("script-ran").exists());
    assert_eq!(
        execute(&mut client, action, 13).await["result"]["started"],
        true
    );
    let completed = client
        .wait(|value| {
            value["method"] == "window/showMessage"
                && (value["params"]["type"] == 1
                    || value["params"]["message"]
                        .as_str()
                        .is_some_and(|message| message.contains("completed")))
        })
        .await;
    assert_eq!(completed["params"]["type"], 3, "{completed}");
    assert!(p.0.join("script-ran").exists());
    let function_line = text.lines().position(|line| line == "script_fn()").unwrap() + 1;
    let function_hover = hover(&mut client, &root, function_line, 0, 30).await;
    assert!(
        function_hover["result"]["contents"]["value"]
            .as_str()
            .unwrap()
            .contains("Runtime-only function documentation."),
        "{function_hover}"
    );
    let import_line = text
        .lines()
        .position(|line| line.starts_with("include("))
        .unwrap();
    let import_definition = definition(&mut client, &root, import_line, 20, 31).await;
    assert!(
        import_definition["result"][0]["uri"]
            .as_str()
            .unwrap()
            .ends_with("/modules/observed.cmake"),
        "{import_definition}"
    );
    assert!(!p.0.join("unused-build").exists());
    assert!(!p.0.join("CMakeCache.txt").exists());
    for (offset, needle, expected) in [
        (0, "message(\"${CHOICE}", "selected".to_owned()),
        (
            1,
            "message(\"${CMAKE_SCRIPT_MODE_FILE}",
            root.display().to_string(),
        ),
    ] {
        let line = text
            .lines()
            .position(|line| line.starts_with(needle))
            .unwrap()
            + 1;
        let listed = actions(&mut client, &root, line, 20 + offset * 2, false).await;
        let view = listed["result"]
            .as_array()
            .unwrap()
            .iter()
            .find(|action| action["command"]["command"] == "cmake-analyzer.expandedArguments")
            .unwrap();
        let result = execute(&mut client, view, 21 + offset * 2).await;
        assert_eq!(
            result["result"]["events"][0]["arguments"][0], expected,
            "{result}"
        );
    }
    let report = client.diagnostics(&root, 1).await;
    assert!(
        !report["params"]["diagnostics"]
            .as_array()
            .unwrap()
            .iter()
            .any(
                |item| item["message"]
                    .as_str()
                    .is_some_and(|message| message.contains("CommandNotFound")
                        && message.contains("script_fn"))
            )
    );
    client.finish().await;
    p.remove().await;
}
#[tokio::test]
/// Check explicit -P mapping overrides CMakeLists.txt inference and rejects configure-only
/// flags.
/// Creates temporary files and an analyzer session; successful completion stops the session and
/// removes fixtures. Execution scenarios may run real CMake or a local stub and write temporary
/// output.
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
/// Map CMakeLists.txt to script mode with -G Ninja and require runScript listing but -32602
/// execution with no marker. Remove flags, reload one workspace, execute successfully and
/// inspect expanded CMAKE_SCRIPT_MODE_FILE; require no cache.
///
/// # Flow
/// ```text
/// [Temporary sources + settings]      [Expected: script precedence / -32602 flags]
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
/// explicit_script_mapping_overrides_cmakelists_filename_and_rejects_configure_flags(); // #[tokio::test] supplies the runtime
/// ```
async fn explicit_script_mapping_overrides_cmakelists_filename_and_rejects_configure_flags() {
    let p = Project::new().await;
    let text = "file(WRITE \"${CMAKE_CURRENT_LIST_DIR}/script-ran\" yes)\nmessage(\"${CMAKE_SCRIPT_MODE_FILE}\")\n";
    let root = p.write("CMakeLists.txt", text).await;
    let config = p.0.join(".cmake-analyzer.toml");
    tokio::fs::write(&config,"[runtime]\narguments=['-G','Ninja']\n[interpreter_commands]\n\"CMakeLists.txt\"='cmake -P CMakeLists.txt'\n").await.unwrap();
    let mut client = Session::start(&p.0, vec![], true).await;
    client.open(&root, text).await;
    client.diagnostics(&root, 1).await;
    let listed = actions(&mut client, &root, 1, 10, false).await;
    assert_eq!(listed["result"].as_array().unwrap().len(), 1);
    assert_eq!(
        listed["result"][0]["command"]["command"],
        "cmake-analyzer.runScript"
    );
    assert_eq!(
        execute(&mut client, &listed["result"][0], 11).await["error"]["code"],
        -32602
    );
    assert!(!p.0.join("script-ran").exists());
    tokio::fs::write(
        &config,
        "[interpreter_commands]\n\"CMakeLists.txt\"='cmake -P CMakeLists.txt'\n",
    )
    .await
    .unwrap();
    client
        .send(json!({"jsonrpc":"2.0","id":12,"method":"cmake-analyzer/reloadConfig","params":{}}))
        .await;
    let reloaded = client.response(12).await;
    assert!(reloaded.get("error").is_none());
    assert_eq!(reloaded["result"]["workspaces"], 1);
    let listed = actions(&mut client, &root, 2, 13, false).await;
    assert_eq!(
        execute(&mut client, &listed["result"][0], 14).await["result"]["started"],
        true
    );
    let completed = client
        .wait(|value| {
            value["method"] == "window/showMessage"
                && (value["params"]["type"] == 1
                    || value["params"]["message"]
                        .as_str()
                        .is_some_and(|message| message.contains("completed")))
        })
        .await;
    assert_eq!(completed["params"]["type"], 3, "{completed}");
    let listed = actions(&mut client, &root, 2, 15, false).await;
    let view = listed["result"]
        .as_array()
        .unwrap()
        .iter()
        .find(|action| action["command"]["command"] == "cmake-analyzer.expandedArguments")
        .unwrap();
    let result = execute(&mut client, view, 16).await;
    assert_eq!(
        result["result"]["events"][0]["arguments"][0],
        root.display().to_string(),
        "{result}"
    );
    assert!(p.0.join("script-ran").exists());
    assert!(!p.0.join("CMakeCache.txt").exists());
    client.finish().await;
    p.remove().await;
}
