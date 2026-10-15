// AI-generated multi-root and dependency ownership tests.
use super::session::*;
use serde_json::json;

#[tokio::test]
async fn roots_and_shared_dependency_receive_live_overlays() {
    let project = Project::new().await;
    let shared = project.write("shared.cmake", "set(SHARED disk)").await;
    let a = project
        .write(
            "a/CMakeLists.txt",
            &format!("include(\"{}\")", shared.display()),
        )
        .await;
    let b = project
        .write(
            "b/CMakeLists.txt",
            &format!("include(\"{}\")", shared.display()),
        )
        .await;
    let folders = vec![
        json!({"uri":uri(a.parent().unwrap()),"name":"a"}),
        json!({"uri":uri(b.parent().unwrap()),"name":"b"}),
    ];
    let mut session = Session::start(&project.0, folders, true).await;
    session
        .open(&shared, "include(missing.cmake)\nbogus()")
        .await;
    let report = session.diagnostics(&shared, 1).await;
    let errors = report["params"]["diagnostics"].as_array().unwrap();
    assert!(errors.len() >= 2);
    assert!(
        errors
            .iter()
            .any(|error| error["severity"] == 2 && error["range"]["start"]["line"] == 1)
    );
    assert!(errors.iter().all(|error| {
        !error["message"].as_str().unwrap().contains("missing.cmake")
            && !error["message"]
                .as_str()
                .unwrap()
                .contains(&shared.display().to_string())
    }));
    session.send(json!({"jsonrpc":"2.0","method":"textDocument/didChange","params":{"textDocument":{"uri":uri(&shared),"version":2},"contentChanges":[{"text":format!("{HEADER}set(LIVE x)")}]}})).await;
    assert!(
        session.diagnostics(&shared, 2).await["params"]["diagnostics"]
            .as_array()
            .unwrap()
            .is_empty()
    );
    assert!(
        session.symbols(&shared, 2).await["result"]
            .as_array()
            .unwrap()
            .iter()
            .any(|symbol| symbol["name"] == "LIVE")
    );
    // A separate project is discovered when an unrelated entry is opened.
    let c = project.write("c/CMakeLists.txt", "set(THIRD x)").await;
    session.open(&c, "set(THIRD x)").await;
    assert!(
        session.symbols(&c, 3).await["result"]
            .as_array()
            .unwrap()
            .iter()
            .any(|symbol| symbol["name"] == "THIRD")
    );
    session.finish().await;
    project.remove().await;
}

#[tokio::test]
async fn dependency_resolution_diagnostics_clear_on_root_edit() {
    let project = Project::new().await;
    let root = project
        .write("CMakeLists.txt", "include(missing.cmake)")
        .await;
    let mut session = Session::start(&project.0, vec![], true).await;
    session.open(&root, "include(missing.cmake)").await;
    let diagnostics = session.diagnostics(&root, 1).await;
    assert!(
        diagnostics["params"]["diagnostics"]
            .as_array()
            .unwrap()
            .iter()
            .any(|error| error["severity"] == 2 && error["range"]["start"]["line"] == 1)
    );
    assert!(
        diagnostics["params"]["diagnostics"]
            .as_array()
            .unwrap()
            .iter()
            .all(
                |error| !error["message"].as_str().unwrap().contains("missing.cmake")
                    && !error["message"]
                        .as_str()
                        .unwrap()
                        .contains(&root.display().to_string())
            )
    );
    session.send(json!({"jsonrpc":"2.0","method":"textDocument/didChange","params":{"textDocument":{"uri":uri(&root),"version":2},"contentChanges":[{"text":format!("{HEADER}message(fixed)")}]}})).await;
    assert!(
        session.diagnostics(&root, 2).await["params"]["diagnostics"]
            .as_array()
            .unwrap()
            .is_empty()
    );
    session.finish().await;
    project.remove().await;
}
