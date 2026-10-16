// AI-generated command completion integration tests.
#[path = "completion/keywords.rs"]
mod keywords;
#[path = "completion/names.rs"]
mod names;
use super::session::{HEADER, Project, Session, uri};
use cmake_analyzer::server::ServerOptions;
use serde_json::{Value, json};
use std::path::Path;
async fn complete(
    client: &mut Session,
    path: &Path,
    line: usize,
    character: usize,
    id: i32,
) -> Value {
    client.send(json!({"jsonrpc":"2.0","id":id,"method":"textDocument/completion","params":{"textDocument":{"uri":uri(path)},"position":{"line":line+1,"character":character}}})).await;
    let response = client.response(id).await;
    assert!(response.get("error").is_none(), "{response}");
    response["result"].clone()
}
fn item<'a>(items: &'a Value, name: &str) -> &'a Value {
    items
        .as_array()
        .unwrap()
        .iter()
        .find(|item| item["label"] == name)
        .unwrap_or_else(|| panic!("Missing {name}: {items}"))
}
async fn start(p: &Project, snippets: bool) -> Session {
    let mut options = ServerOptions::new(p.0.clone());
    options.config.user_file = None;
    Session::start_completion(options, vec![], snippets).await
}

#[tokio::test]
async fn outer_names_are_sorted_deduplicated_versioned_and_classified_without_touching_arguments() {
    let p = Project::new().await;
    p.write(
        "cmake/Helpers.cmake",
        "function(imported_command input)\nendfunction()\n",
    )
    .await;
    let text = "include(Helpers)\nfunction(MyFunction input output)\nendfunction()\nmacro(MyMacro arg)\nendmacro()\n\nADd_liBRARY(existing)\nif(TRUE)\n  en\nendif()\nmessage(\"add_li\")\n#[=[ add_li ]=]\n";
    let root = p.write("CMakeLists.txt", text).await;
    let mut client = start(&p, true).await;
    client.open(&root, text).await;
    client.diagnostics(&root, 1).await;
    let all = complete(&mut client, &root, 5, 0, 10).await;
    for (label, kind) in [
        ("if", 14),
        ("endif", 14),
        ("add_library", 9),
        ("target_link_libraries", 9),
        ("include", 9),
        ("message", 3),
        ("set", 3),
        ("MyFunction", 3),
        ("MyMacro", 3),
        ("imported_command", 3),
    ] {
        assert_eq!(item(&all, label)["kind"], kind, "{label}");
    }
    let labels: Vec<_> = all
        .as_array()
        .unwrap()
        .iter()
        .map(|item| item["label"].as_str().unwrap().to_ascii_lowercase())
        .collect();
    assert!(labels.windows(2).all(|pair| pair[0] < pair[1]));
    assert!(!labels.contains(&"cmake_diagnostic".into()));
    let existing = complete(&mut client, &root, 6, 6, 11).await;
    let library = item(&existing, "add_library");
    assert_eq!(
        library["textEdit"],
        json!({"range":{"start":{"line":7,"character":0},"end":{"line":7,"character":11}},"newText":"add_library"})
    );
    assert_eq!(library["insertTextFormat"], 1);
    assert!(
        item(&complete(&mut client, &root, 8, 4, 12).await, "endif")["textEdit"]["newText"]
            == "endif"
    );
    assert!(
        complete(&mut client, &root, 10, 15, 13)
            .await
            .as_array()
            .unwrap()
            .is_empty()
    );
    assert!(
        complete(&mut client, &root, 11, 10, 14)
            .await
            .as_array()
            .unwrap()
            .is_empty()
    );
    client.finish().await;
    p.remove().await;
}
#[tokio::test]
async fn reserve_arg_uses_snippets_only_for_capable_clients_and_preserves_existing_parentheses() {
    let p = Project::new().await;
    tokio::fs::write(
        p.0.join(".cmake-analyzer.toml"),
        "[completion]\nreserve_arg=true\n",
    )
    .await
    .unwrap();
    let text =
        "function(MyFunction input output)\nendfunction()\n\ncmake_minimum_r\nadd_lib(existing)\n";
    let root = p.write("CMakeLists.txt", text).await;
    let mut client = start(&p, true).await;
    client.open(&root, text).await;
    client.diagnostics(&root, 1).await;
    let all = complete(&mut client, &root, 2, 0, 10).await;
    for (name, expected) in [
        ("if", "if(${1:condition})$0"),
        ("endif", "endif()$0"),
        ("MyFunction", "MyFunction(${1:input} ${2:output})$0"),
    ] {
        assert_eq!(item(&all, name)["insertTextFormat"], 2);
        assert_eq!(item(&all, name)["textEdit"]["newText"], expected);
    }
    let version = complete(&mut client, &root, 3, 15, 11).await;
    assert_eq!(
        item(&version, "cmake_minimum_required")["textEdit"]["newText"],
        "cmake_minimum_required(VERSION ${1:version})$0"
    );
    let existing = complete(&mut client, &root, 4, 7, 12).await;
    assert_eq!(
        item(&existing, "add_library")["textEdit"]["newText"],
        "add_library"
    );
    assert_eq!(item(&existing, "add_library")["insertTextFormat"], 1);
    client.finish().await;
    let mut client = start(&p, false).await;
    client.open(&root, text).await;
    client.diagnostics(&root, 1).await;
    let all = complete(&mut client, &root, 2, 0, 20).await;
    assert_eq!(
        item(&all, "MyFunction")["textEdit"]["newText"],
        "MyFunction"
    );
    assert_eq!(item(&all, "if")["insertTextFormat"], 1);
    client.finish().await;
    p.remove().await;
}
#[tokio::test]
async fn completion_observes_pending_edits_and_transactional_configuration_reload() {
    let p = Project::new().await;
    let text = "# 😀\nadd_li";
    let root = p.write("CMakeLists.txt", text).await;
    let mut client = start(&p, true).await;
    client.open(&root, text).await;
    client.diagnostics(&root, 1).await;
    let initial = complete(&mut client, &root, 1, 6, 10).await;
    assert_eq!(
        item(&initial, "add_library")["textEdit"]["newText"],
        "add_library"
    );
    let config = p.0.join(".cmake-analyzer.toml");
    tokio::fs::write(&config, "[completion]\nreserve_arg=true\n")
        .await
        .unwrap();
    client
        .send(json!({"jsonrpc":"2.0","id":11,"method":"cmake-analyzer/reloadConfig","params":{}}))
        .await;
    assert!(client.response(11).await.get("error").is_none());
    assert_eq!(
        item(&complete(&mut client, &root, 1, 6, 12).await, "add_library")["insertTextFormat"],
        2
    );
    tokio::fs::write(&config, "[completion]\nreserve_arg='invalid'\n")
        .await
        .unwrap();
    client
        .send(json!({"jsonrpc":"2.0","id":13,"method":"cmake-analyzer/reloadConfig","params":{}}))
        .await;
    assert!(client.response(13).await.get("error").is_some());
    assert_eq!(
        item(&complete(&mut client, &root, 1, 6, 14).await, "add_library")["insertTextFormat"],
        2
    );
    client.send(json!({"jsonrpc":"2.0","method":"textDocument/didChange","params":{"textDocument":{"uri":uri(&root),"version":2},"contentChanges":[{"text":format!("{HEADER}message(\"😀\") se")} ]}})).await;
    let changed = complete(&mut client, &root, 0, 16, 15).await;
    let set = item(&changed, "set");
    assert_eq!(
        set["textEdit"]["range"],
        json!({"start":{"line":1,"character":14},"end":{"line":1,"character":16}})
    );
    client.finish().await;
    p.remove().await;
}
#[tokio::test]
async fn known_user_shadows_keep_function_icons_and_formal_parameter_placeholders() {
    let p = Project::new().await;
    tokio::fs::write(
        p.0.join(".cmake-analyzer.toml"),
        "[completion]\nreserve_arg=true\n",
    )
    .await
    .unwrap();
    let text = "function(add_library special)\nendfunction()\nadd_li";
    let loose = p.write("loose.cmake", text).await;
    let mut client = start(&p, true).await;
    client.open(&loose, text).await;
    client.diagnostics(&loose, 1).await;
    let matches = complete(&mut client, &loose, 2, 6, 10).await;
    let library = item(&matches, "add_library");
    assert_eq!(library["kind"], 3);
    assert_eq!(
        library["textEdit"]["newText"],
        "add_library(${1:special})$0"
    );
    client.finish().await;
    p.remove().await;
}

#[tokio::test]
async fn shared_owners_require_agreement_for_snippets_and_keep_conflicting_signatures_plain() {
    let p = Project::new().await;
    let a = p
        .write(
            "a/CMakeLists.txt",
            "function(Choose left)\nendfunction()\ninclude(../shared.cmake)\n",
        )
        .await;
    let b = p
        .write(
            "b/CMakeLists.txt",
            "function(Choose right)\nendfunction()\ninclude(../shared.cmake)\n",
        )
        .await;
    let shared = p.write("shared.cmake", "Cho").await;
    tokio::fs::write(
        p.0.join("a/.cmake-analyzer.toml"),
        "[completion]\nreserve_arg=true\n",
    )
    .await
    .unwrap();
    let config = p.0.join("b/.cmake-analyzer.toml");
    tokio::fs::write(&config, "[completion]\nreserve_arg=false\n")
        .await
        .unwrap();
    let mut options = ServerOptions::new(p.0.clone());
    options.config.user_file = None;
    let folders = vec![
        json!({"uri":uri(a.parent().unwrap()),"name":"a"}),
        json!({"uri":uri(b.parent().unwrap()),"name":"b"}),
    ];
    let mut client = Session::start_completion(options, folders, true).await;
    client.open(&shared, "Cho").await;
    client.diagnostics(&shared, 1).await;
    let initial = complete(&mut client, &shared, 0, 3, 10).await;
    assert_eq!(initial.as_array().unwrap().len(), 1);
    assert_eq!(item(&initial, "Choose")["insertTextFormat"], 1);
    tokio::fs::write(&config, "[completion]\nreserve_arg=true\n")
        .await
        .unwrap();
    client
        .send(json!({"jsonrpc":"2.0","id":11,"method":"cmake-analyzer/reloadConfig","params":{}}))
        .await;
    assert!(client.response(11).await.get("error").is_none());
    let conflicting = complete(&mut client, &shared, 0, 3, 12).await;
    assert_eq!(
        item(&conflicting, "Choose")["textEdit"]["newText"],
        "Choose"
    );
    assert_eq!(item(&conflicting, "Choose")["insertTextFormat"], 1);
    client.finish().await;
    p.remove().await;
}
