// AI-generated missing-candidate and ownership-transition regressions.
use super::session::*;
use serde_json::json;

#[tokio::test]
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
