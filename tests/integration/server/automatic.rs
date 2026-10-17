// AI-generated authorized on-save configure/script execution and invalidation tests.
//! Globally authorized didSave execution, cancellation and observation invalidation.
//! Tests run temporary CMake projects or scripts only after save notifications,
//! then join the server and remove all output files.
use super::{
    hovers::hover,
    session::{HEADER, Project, Session, uri},
};
use cmake_analyzer::server::ServerOptions;
use serde_json::{Value, json};
/// Send didSave without text; may start authorized on-save execution or refresh analysis.
///
/// # Arguments
/// - `client` `[in,out]`: appends the notification to input without draining output.
/// - `path` `[in]`: document path converted to a fixture URI.
///
/// # Returns
/// `()` after the frame is sent; no completion is awaited.
///
/// # Panics
/// Panics if sending fails.
///
/// # Examples
/// ```text
/// save(&mut client, &root).await;
/// ```
async fn save(client: &mut Session, path: &std::path::Path) {
    client.send(json!({"jsonrpc":"2.0","method":"textDocument/didSave","params":{"textDocument":{"uri":uri(path)}}})).await;
}
/// Poll a fixture file until its entire text equals the expected marker.
/// Reads disk every ten milliseconds; missing/read-error states are retried.
///
/// # Arguments
/// - `path` `[in]`: filesystem marker file to poll; read errors are retried.
/// - `expected` `[in]`: exact desired file contents, not a substring.
///
/// # Returns
/// `()` on an exact read match within five seconds.
///
/// # Panics
/// Panics if no matching read occurs within five seconds.
///
/// # Algorithm
/// Read the file, compare successful contents, otherwise sleep ten milliseconds and retry under
/// a single five-second deadline. The file is never created or changed by this helper.
///
/// # Flow
/// ```text
/// [Five-second deadline]
///           |
///           v
///   [Read marker file] <--------------+
///           |                        |
///           v                        |
///  <Exact contents?> -- no --> [Sleep 10 ms]
///           |
///          yes
///           v
///    [Return ()]        [Deadline expires: panic]
/// ```
///
/// # Examples
/// ```text
/// wait_file(&p.0.join("runs"), "xx").await;
/// ```
async fn wait_file(path: &std::path::Path, expected: &str) {
    tokio::time::timeout(std::time::Duration::from_secs(5), async {
        loop {
            if tokio::fs::read_to_string(path).await.ok().as_deref() == Some(expected) {
                break;
            }
            tokio::time::sleep(std::time::Duration::from_millis(10)).await;
        }
    })
    .await
    .unwrap();
}
#[tokio::test]
/// Check a second save replaces a running script even with one workspace job slot.
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
/// Authorize on_save in user settings. Start a script blocked on a resume file, edit old to
/// new, save again and require two run markers. Release the script and hover the reference;
/// only the new observed value may remain.
///
/// # Flow
/// ```text
/// [Temporary sources + settings]      [Expected: two runs + new observation]
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
/// a_second_save_cancels_and_restarts_a_script_with_one_job_slot(); // #[tokio::test] supplies the runtime
/// ```
async fn a_second_save_cancels_and_restarts_a_script_with_one_job_slot() {
    let p = Project::new().await;
    let text = "file(APPEND \"${CMAKE_CURRENT_LIST_DIR}/../runs\" x)\nwhile(NOT EXISTS \"${CMAKE_CURRENT_LIST_DIR}/../resume\")\nexecute_process(COMMAND \"${CMAKE_COMMAND}\" -E sleep 0.01)\nendwhile()\nset(A old)\nmessage(\"${A}\")\n";
    let root = p.write("tools/check.cmake", text).await;
    tokio::fs::write(
        p.0.join(".cmake-analyzer.toml"),
        "[interpreter_commands]\n\"tools/check.cmake\"='cmake -P tools/check.cmake'\n",
    )
    .await
    .unwrap();
    let user = p.0.join("user.toml");
    tokio::fs::write(
        &user,
        "[runtime]\nmode='on_save'\n[limits]\nmax_workspaces=1\n",
    )
    .await
    .unwrap();
    let mut options = ServerOptions::new(p.0.clone());
    options.config.user_file = Some(user);
    let mut client = Session::start_options(options, vec![], true).await;
    client.open(&root, text).await;
    client.diagnostics(&root, 1).await;
    save(&mut client, &root).await;
    wait_file(&p.0.join("runs"), "x").await;
    let changed = text.replace("set(A old)", "set(A new)");
    p.write("tools/check.cmake", &changed).await;
    client.send(json!({"jsonrpc":"2.0","method":"textDocument/didChange","params":{"textDocument":{"uri":uri(&root),"version":2},"contentChanges":[{"text":format!("{HEADER}{changed}")}]}})).await;
    save(&mut client, &root).await;
    wait_file(&p.0.join("runs"), "xx").await;
    tokio::fs::write(p.0.join("resume"), "").await.unwrap();
    completed(&mut client).await;
    let result = hover(&mut client, &root, 6, 11, 10).await;
    let value = result["result"]["contents"]["value"].as_str().unwrap();
    assert!(
        value.contains("Observed") && value.contains("\"new\""),
        "{result}"
    );
    assert!(!value.contains("\"old\""), "{result}");
    client.finish().await;
    p.remove().await;
}
/// Wait for a completion/error showMessage and require successful informational completion.
///
/// # Arguments
/// - `client` `[in,out]`: consumes a matching window/showMessage; unrelated traffic is
///   buffered.
///
/// # Returns
/// The full informational notification (type 3); an error notification fails the assertion.
///
/// # Panics
/// Panics on transport/deadline failures or if the selected message is not type 3.
///
/// # Examples
/// ```text
/// let notice = completed(&mut client).await;
/// ```
async fn completed(client: &mut Session) -> Value {
    let result = client
        .wait(|value| {
            value["method"] == "window/showMessage"
                && (value["params"]["type"] == 1
                    || value["params"]["message"]
                        .as_str()
                        .is_some_and(|message| message.contains("completed")))
        })
        .await;
    assert_eq!(result["params"]["type"], 3, "{result}");
    result
}
#[tokio::test]
/// Check authorized saves configure, invalidate observations on edits and rerun only after
/// saving.
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
/// Opening and symbol queries must not create runs. Save, await configure success and cache
/// creation, suppress generated-source warnings and expose cached/observed values. An unsaved
/// edit removes Observed hover data without another run; writing and saving produces the second
/// run.
///
/// # Flow
/// ```text
/// [Temporary sources + settings]      [Expected: configure outputs + hover state]
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
/// user_authorized_save_runs_configure_and_later_saved_edits_rerun(); // #[tokio::test] supplies the runtime
/// ```
async fn user_authorized_save_runs_configure_and_later_saved_edits_rerun() {
    let p = Project::new().await;
    let text = "project(save_test LANGUAGES NONE)\nfile(APPEND \"${CMAKE_CURRENT_LIST_DIR}/runs\" x)\nset(A original)\nmessage(\"${A}\")\nadd_library(iface INTERFACE)\ntarget_sources(iface INTERFACE \"$<BUILD_INTERFACE:generated_at_build.c>\")\nset(CACHED value CACHE STRING doc FORCE)\n";
    let root = p.write("CMakeLists.txt", text).await;
    let user = p.0.join("user.toml");
    tokio::fs::write(&user, "[runtime]\nmode='on_save'\n")
        .await
        .unwrap();
    let mut options = ServerOptions::new(p.0.clone());
    options.config.user_file = Some(user);
    let mut client = Session::start_options(options, vec![], true).await;
    client.open(&root, text).await;
    client.diagnostics(&root, 1).await;
    client.symbols(&root, 10).await;
    assert!(!p.0.join("runs").exists());
    save(&mut client, &root).await;
    completed(&mut client).await;
    assert_eq!(
        tokio::fs::read_to_string(p.0.join("runs")).await.unwrap(),
        "x"
    );
    assert!(p.0.join(".cmake-analyzer/build/CMakeCache.txt").exists());
    let report = client.diagnostics(&root, 1).await;
    assert!(
        !report["params"]["diagnostics"]
            .as_array()
            .unwrap()
            .iter()
            .any(|item| item["message"]
                .as_str()
                .unwrap()
                .contains("generated_at_build.c")),
        "{report}"
    );
    let cached = hover(&mut client, &root, 7, 4, 14).await;
    assert!(
        cached["result"]["contents"]["value"]
            .as_str()
            .unwrap()
            .contains("\"value\""),
        "{cached}"
    );
    let before = hover(&mut client, &root, 4, 11, 11).await;
    assert!(
        before["result"]["contents"]["value"]
            .as_str()
            .unwrap()
            .contains("Observed"),
        "{before}"
    );
    let changed = text.replace("original", "changed");
    client.send(json!({"jsonrpc":"2.0","method":"textDocument/didChange","params":{"textDocument":{"uri":uri(&root),"version":2},"contentChanges":[{"text":format!("{HEADER}{changed}")}]}})).await;
    let after = hover(&mut client, &root, 4, 11, 12).await;
    let value = after["result"]["contents"]["value"].as_str().unwrap();
    assert!(!value.contains("Observed"), "{after}");
    assert!(value.contains("changed"), "{after}");
    assert_eq!(
        tokio::fs::read_to_string(p.0.join("runs")).await.unwrap(),
        "x"
    );
    p.write("CMakeLists.txt", &changed).await;
    save(&mut client, &root).await;
    completed(&mut client).await;
    assert_eq!(
        tokio::fs::read_to_string(p.0.join("runs")).await.unwrap(),
        "xx"
    );
    client.finish().await;
    p.remove().await;
}
#[tokio::test]
/// Check enabling on_save by reload is authority, not an immediate script execution.
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
/// An action-mode save creates no marker. Reload explicit on_save settings and query symbols;
/// still require no run. Only the next didSave runs -P, records CHOICE=observed and creates
/// neither build directory nor cache.
///
/// # Flow
/// ```text
/// [Temporary sources + settings]      [Expected: save-only script run]
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
/// explicit_script_save_authority_is_not_execution_until_a_save_notification(); // #[tokio::test] supplies the runtime
/// ```
async fn explicit_script_save_authority_is_not_execution_until_a_save_notification() {
    let p = Project::new().await;
    let text = "file(APPEND \"${CMAKE_CURRENT_LIST_DIR}/../runs\" x)\nmessage(\"${CHOICE}\")\n";
    let root = p.write("tools/check.cmake", text).await;
    tokio::fs::write(
        p.0.join(".cmake-analyzer.toml"),
        "[interpreter_commands]\n\"tools/check.cmake\"='cmake -P tools/check.cmake'\n",
    )
    .await
    .unwrap();
    let config = p.0.join("explicit.toml");
    tokio::fs::write(
        &config,
        "[runtime]\nmode='action'\narguments=['-DCHOICE=observed']\n",
    )
    .await
    .unwrap();
    let mut options = ServerOptions::new(p.0.clone());
    options.config.user_file = None;
    options.config.explicit_file = Some(config.clone());
    let mut client = Session::start_options(options, vec![], true).await;
    client.open(&root, text).await;
    client.diagnostics(&root, 1).await;
    save(&mut client, &root).await;
    client.symbols(&root, 10).await;
    assert!(!p.0.join("runs").exists());
    tokio::fs::write(
        config,
        "[runtime]\nmode='on_save'\narguments=['-DCHOICE=observed']\n",
    )
    .await
    .unwrap();
    client
        .send(json!({"jsonrpc":"2.0","id":11,"method":"cmake-analyzer/reloadConfig","params":{}}))
        .await;
    assert!(client.response(11).await.get("error").is_none());
    client.symbols(&root, 12).await;
    assert!(!p.0.join("runs").exists());
    save(&mut client, &root).await;
    completed(&mut client).await;
    assert_eq!(
        tokio::fs::read_to_string(p.0.join("runs")).await.unwrap(),
        "x"
    );
    assert!(!p.0.join(".cmake-analyzer/build").exists());
    assert!(!p.0.join("CMakeCache.txt").exists());
    let result = hover(&mut client, &root, 2, 11, 13).await;
    let value = result["result"]["contents"]["value"].as_str().unwrap();
    assert!(
        value.contains("Observed") && value.contains("\"observed\""),
        "{result}"
    );
    client.finish().await;
    p.remove().await;
}
