// AI-generated missing-candidate and ownership-transition regressions.
//! Missing dependency overlays and transitions from imported to standalone documents.
//! Open text must survive ownership changes; fixture trees are explicitly removed.
use super::session::*;
use serde_json::json;

#[tokio::test]
/// Check opening an unsaved missing dependency clears its importer diagnostics at the same
/// version.
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
/// Include a nonexistent external path and require a nonempty root report. didOpen that target
/// with a valid set command; require empty reports for both root and dependency, each still
/// version 1.
///
/// # Flow
/// ```text
/// [Temporary sources + settings]      [Expected: unchanged-version clear reports]
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
/// missing_external_dependency_is_owned_and_overlay_clears_parent_report(); // #[tokio::test] supplies the runtime
/// ```
async fn missing_external_dependency_is_owned_and_overlay_clears_parent_report() {
    let project = Project::new().await;
    let missing = project.0.join("external/new.cmake");
    let root = project
        .write(
            "project/CMakeLists.txt",
            &format!("include(\"{}\")", missing.display()),
        )
        .await;
    let mut session = Session::start(root.parent().unwrap(), vec![], true).await;
    session
        .open(&root, &format!("include(\"{}\")", missing.display()))
        .await;
    assert!(
        !session.diagnostics(&root, 1).await["params"]["diagnostics"]
            .as_array()
            .unwrap()
            .is_empty()
    );
    session.open(&missing, "set(NEW x)").await;
    // The parent text/version did not change, only dependency resolution did.
    assert!(
        session.diagnostics(&root, 1).await["params"]["diagnostics"]
            .as_array()
            .unwrap()
            .is_empty()
    );
    assert!(
        session.diagnostics(&missing, 1).await["params"]["diagnostics"]
            .as_array()
            .unwrap()
            .is_empty()
    );
    session.finish().await;
    project.remove().await;
}

#[tokio::test]
/// Check dependency detachment preserves unsaved module text and subsequent edits.
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
/// Open an external module with UNSAVED instead of disk content, then open its root without the
/// include. Symbols must retain UNSAVED; a later module didChange must expose LATER despite
/// losing the importer.
///
/// # Flow
/// ```text
/// [Temporary sources + settings]      [Expected: UNSAVED then LATER symbols]
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
/// detached_external_module_becomes_standalone_without_losing_text(); // #[tokio::test] supplies the runtime
/// ```
async fn detached_external_module_becomes_standalone_without_losing_text() {
    let project = Project::new().await;
    let module = project.write("external/module.cmake", "set(DISK x)").await;
    let root = project
        .write(
            "project/CMakeLists.txt",
            &format!("include(\"{}\")", module.display()),
        )
        .await;
    let mut session = Session::start(root.parent().unwrap(), vec![], true).await;
    session.open(&module, "set(UNSAVED x)").await;
    session.diagnostics(&module, 1).await;
    session.open(&root, "message(no_dependencies)").await;
    session.diagnostics(&root, 1).await;
    assert!(
        session.symbols(&module, 2).await["result"]
            .as_array()
            .unwrap()
            .iter()
            .any(|symbol| symbol["name"] == "UNSAVED")
    );
    session.send(json!({"jsonrpc":"2.0","method":"textDocument/didChange","params":{"textDocument":{"uri":uri(&module),"version":2},"contentChanges":[{"text":format!("{HEADER}set(LATER x)")}]}})).await;
    assert!(
        session.symbols(&module, 3).await["result"]
            .as_array()
            .unwrap()
            .iter()
            .any(|symbol| symbol["name"] == "LATER")
    );
    session.finish().await;
    project.remove().await;
}
