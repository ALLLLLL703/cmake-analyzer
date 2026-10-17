// AI-generated tests.
use super::session::{HEADER, Project, Session, uri};
use serde_json::{Value, json};
use std::path::Path;

#[tokio::test]
async fn format_cli_writes_only_resolved_project_files_and_check_never_writes() {
    let p = Project::new().await;
    let body = "if( TRUE )\nset( X   yes )\nendif( )\ninclude( cmake/a.cmake )\nadd_subdirectory( src )\ninclude( ../outside.cmake )\ninclude( build/generated.cmake )\nexecute_process(COMMAND touch should-not-exist)\n";
    let root = p.write("project/CMakeLists.txt", body).await;
    let child = p
        .write(
            "project/cmake/a.cmake",
            "function(F ARG)\nmessage( STATUS \"x  y\" )\nendfunction( )\n",
        )
        .await;
    let sub = p
        .write("project/src/CMakeLists.txt", "set( SOURCE   value )\n")
        .await;
    let untouched = p
        .write("project/unused.cmake", "set( UNUSED   yes )\n")
        .await;
    let outside = p.write("outside.cmake", "set( OUTSIDE   yes )\n").await;
    let generated = p
        .write("project/build/generated.cmake", "set( GENERATED   yes )\n")
        .await;
    tokio::fs::write(p.0.join("project/build/CMakeCache.txt"), "")
        .await
        .unwrap();
    tokio::fs::write(
        p.0.join("project/.cmake-analyzer.toml"),
        "[modules]\nenable=false\n[format]\nline_width=60\n",
    )
    .await
    .unwrap();
    // An in-source cache does not turn the source project itself into generated output.
    tokio::fs::write(p.0.join("project/CMakeCache.txt"), "")
        .await
        .unwrap();
    tokio::fs::write(p.0.join("project/src/CMakeCache.txt"), "")
        .await
        .unwrap();
    let before = tokio::fs::read_to_string(&root).await.unwrap();
    let run = |check: bool| {
        let mut command = tokio::process::Command::new(env!("CARGO_BIN_EXE_cmake-analyzer"));
        command
            .current_dir(&p.0)
            .env("HOME", &p.0)
            .arg("format")
            .arg(root.parent().unwrap());
        if check {
            command.arg("--check");
        }
        command
    };
    let result = run(true).output().await.unwrap();
    assert_eq!(
        result.status.code(),
        Some(1),
        "{}",
        String::from_utf8_lossy(&result.stderr)
    );
    assert_eq!(tokio::fs::read_to_string(&root).await.unwrap(), before);
    let result = run(false).output().await.unwrap();
    assert!(
        result.status.success(),
        "{}",
        String::from_utf8_lossy(&result.stderr)
    );
    assert!(
        tokio::fs::read_to_string(&root)
            .await
            .unwrap()
            .contains("if(TRUE)\n  set(X yes)\nendif()")
    );
    assert!(
        tokio::fs::read_to_string(child)
            .await
            .unwrap()
            .contains("\n  message(STATUS \"x  y\")\n")
    );
    assert!(
        tokio::fs::read_to_string(sub)
            .await
            .unwrap()
            .contains("set(SOURCE value)")
    );
    for (file, text) in [
        (untouched, "set( UNUSED   yes )\n"),
        (outside, "set( OUTSIDE   yes )\n"),
        (generated, "set( GENERATED   yes )\n"),
    ] {
        assert_eq!(
            tokio::fs::read_to_string(file).await.unwrap(),
            format!("{HEADER}{text}")
        );
    }
    assert!(!p.0.join("should-not-exist").exists());
    let result = run(true).output().await.unwrap();
    assert!(
        result.status.success(),
        "{}",
        String::from_utf8_lossy(&result.stderr)
    );
    p.remove().await;
}
#[tokio::test]
async fn invalid_dependency_syntax_leaves_every_project_file_unchanged() {
    let p = Project::new().await;
    let body = "include(bad.cmake)\nset( VALUE   yes )\n";
    let root = p.write("CMakeLists.txt", body).await;
    p.write("bad.cmake", "set(BAD \"unfinished").await;
    let result = tokio::process::Command::new(env!("CARGO_BIN_EXE_cmake-analyzer"))
        .current_dir(&p.0)
        .env("HOME", &p.0)
        .args(["format", "."])
        .output()
        .await
        .unwrap();
    assert!(!result.status.success());
    assert_eq!(
        tokio::fs::read_to_string(root).await.unwrap(),
        format!("{HEADER}{body}")
    );
    p.remove().await;
}
#[tokio::test]
async fn shared_documents_do_not_choose_an_arbitrary_owner_style() {
    let p = Project::new().await;
    let body = "if( TRUE )\nmessage( STATUS x )\nendif()\n";
    let shared = p.write("shared.cmake", body).await;
    for (name, width) in [("a", 80), ("b", 100)] {
        p.write(
            &format!("{name}/CMakeLists.txt"),
            "include(../shared.cmake)\n",
        )
        .await;
        tokio::fs::write(
            p.0.join(name).join(".cmake-analyzer.toml"),
            format!("[format]\nline_width={width}\n"),
        )
        .await
        .unwrap();
    }
    let folders = vec![
        json!({"uri":uri(&p.0.join("a")),"name":"a"}),
        json!({"uri":uri(&p.0.join("b")),"name":"b"}),
    ];
    let mut client = Session::start(&p.0, folders, true).await;
    client.open(&shared, body).await;
    client.diagnostics(&shared, 1).await;
    let result = format(&mut client, &shared, 10, true).await;
    assert_eq!(result["error"]["code"], -32602, "{result}");
    assert_eq!(
        tokio::fs::read_to_string(&shared).await.unwrap(),
        format!("{HEADER}{body}")
    );
    client.finish().await;
    p.remove().await;
}
async fn format(client: &mut Session, path: &Path, id: i32, save: bool) -> Value {
    let (method, params) = if save {
        (
            "textDocument/willSaveWaitUntil",
            json!({"textDocument":{"uri":uri(path)},"reason":1}),
        )
    } else {
        (
            "textDocument/formatting",
            json!({"textDocument":{"uri":uri(path)},"options":{"tabSize":4,"insertSpaces":true}}),
        )
    };
    client
        .send(json!({"jsonrpc":"2.0","id":id,"method":method,"params":params}))
        .await;
    client.response(id).await
}
#[tokio::test]
async fn manual_and_default_save_formatting_are_client_edits_with_transactional_reload() {
    let p = Project::new().await;
    let body = "if( TRUE )\nmessage( STATUS \"😀\" )\nendif( )\n";
    let root = p.write("CMakeLists.txt", body).await;
    let mut client = Session::start(&p.0, vec![], true).await;
    client.open(&root, body).await;
    client.diagnostics(&root, 1).await;
    let manual = format(&mut client, &root, 10, false).await;
    assert!(
        manual["result"][0]["newText"]
            .as_str()
            .unwrap()
            .contains("\n    message(STATUS \"😀\")\n"),
        "{manual}"
    );
    let save = format(&mut client, &root, 11, true).await;
    assert!(
        save["result"][0]["newText"]
            .as_str()
            .unwrap()
            .contains("\n  message(STATUS \"😀\")\n"),
        "{save}"
    );
    assert_eq!(
        tokio::fs::read_to_string(&root).await.unwrap(),
        format!("{HEADER}{body}")
    );
    let config = p.0.join(".cmake-analyzer.toml");
    tokio::fs::write(&config, "[format]\non_save=false\nindent_width=3\n")
        .await
        .unwrap();
    client
        .send(json!({"jsonrpc":"2.0","id":12,"method":"cmake-analyzer/reloadConfig","params":null}))
        .await;
    assert!(client.response(12).await.get("error").is_none());
    assert_eq!(
        format(&mut client, &root, 13, true).await["result"],
        json!([])
    );
    assert_eq!(
        format(&mut client, &root, 14, false).await["result"],
        manual["result"]
    );
    tokio::fs::write(&config, "[format]\nline_width=0\n")
        .await
        .unwrap();
    client
        .send(json!({"jsonrpc":"2.0","id":15,"method":"cmake-analyzer/reloadConfig","params":null}))
        .await;
    assert!(client.response(15).await.get("error").is_some());
    assert_eq!(
        format(&mut client, &root, 16, true).await["result"],
        json!([])
    );
    client.send(json!({"jsonrpc":"2.0","method":"textDocument/didChange","params":{"textDocument":{"uri":uri(&root),"version":2},"contentChanges":[{"text":format!("{HEADER}set(NEW \"unfinished")}]}})).await;
    assert_eq!(
        format(&mut client, &root, 17, false).await["result"],
        json!([])
    );
    client.finish().await;
    p.remove().await;
}
