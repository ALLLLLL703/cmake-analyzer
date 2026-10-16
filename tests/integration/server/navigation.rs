// AI-generated definition/reference navigation tests.
use super::session::{HEADER, Project, Session, uri};
use serde_json::{Value, json};
use std::path::Path;
#[path = "navigation/scopes.rs"]
mod scopes;
async fn definition(
    client: &mut Session,
    path: &Path,
    line: usize,
    character: usize,
    id: i32,
) -> Value {
    client.send(json!({"jsonrpc":"2.0","id":id,"method":"textDocument/definition","params":{"textDocument":{"uri":uri(path)},"position":{"line":line+1,"character":character}}})).await;
    client.response(id).await
}
async fn references(
    client: &mut Session,
    path: &Path,
    line: usize,
    character: usize,
    include: bool,
    id: i32,
) -> Value {
    client.send(json!({"jsonrpc":"2.0","id":id,"method":"textDocument/references","params":{"textDocument":{"uri":uri(path)},"position":{"line":line+1,"character":character},"context":{"includeDeclaration":include}}})).await;
    client.response(id).await
}
#[tokio::test]
async fn cross_file_functions_variables_parameters_and_utf16_ranges_use_bound_symbols() {
    let p = Project::new().await;
    let shared = p
        .write(
            "shared.cmake",
            "set(V root)\nfunction(External ARG)\nmessage(\"${ARG}\")\nendfunction()\n",
        )
        .await;
    let text =
        "include(shared.cmake)\nexternal(value)\nmessage(🌟 \"${V}\")\nmessage(\"${MISSING}\")\n";
    let root = p.write("CMakeLists.txt", text).await;
    let mut client = Session::start(&p.0, vec![], true).await;
    client.open(&root, text).await;
    client.diagnostics(&root, 1).await;
    let result = definition(&mut client, &root, 1, 0, 10).await;
    assert_eq!(
        result["result"],
        json!([{"uri":uri(&shared),"range":{"start":{"line":2,"character":9},"end":{"line":2,"character":17}}}]),
        "{result}"
    );
    let result = references(&mut client, &shared, 1, 9, false, 11).await;
    assert_eq!(result["result"].as_array().unwrap().len(), 1);
    assert_eq!(result["result"][0]["uri"], uri(&root));
    let result = references(&mut client, &shared, 1, 9, true, 12).await;
    assert_eq!(result["result"].as_array().unwrap().len(), 2);
    let line = text.lines().nth(2).unwrap();
    let column = line[..line.find('V').unwrap()].encode_utf16().count();
    let result = definition(&mut client, &root, 2, column, 13).await;
    assert_eq!(result["result"][0]["uri"], uri(&shared));
    assert_eq!(
        result["result"][0]["range"]["start"],
        json!({"line":1,"character":4})
    );
    let result = references(&mut client, &shared, 0, 4, false, 14).await;
    assert_eq!(result["result"].as_array().unwrap().len(), 1);
    assert_eq!(
        result["result"][0]["range"]["start"],
        json!({"line":3,"character":column})
    );
    let result = definition(&mut client, &shared, 2, 12, 15).await;
    assert_eq!(
        result["result"][0]["range"]["start"],
        json!({"line":2,"character":18})
    );
    assert!(definition(&mut client, &root, 3, 12, 16).await["result"].is_null());
    assert_eq!(
        references(&mut client, &root, 3, 12, true, 17).await["result"],
        json!([])
    );
    assert!(definition(&mut client, &root, 1, 8, 18).await["result"].is_null());
    client.finish().await;
    p.remove().await;
}
#[tokio::test]
async fn builtin_calls_have_no_definition_and_do_not_merge_with_shadowing_functions() {
    let p = Project::new().await;
    p.write("shared.cmake", "message(shared)\n").await;
    let text = "include(shared.cmake)\nmessage(before)\nMESSAGE(again)\nfunction(message P)\nendfunction()\nmessage(after)\n";
    let root = p.write("CMakeLists.txt", text).await;
    let mut client = Session::start(&p.0, vec![], true).await;
    client.open(&root, text).await;
    client.diagnostics(&root, 1).await;
    assert!(definition(&mut client, &root, 1, 0, 10).await["result"].is_null());
    let result = references(&mut client, &root, 1, 0, true, 11).await;
    assert_eq!(result["result"].as_array().unwrap().len(), 3, "{result}");
    assert!(
        !result["result"]
            .as_array()
            .unwrap()
            .iter()
            .any(|location| location["range"]["start"]["line"] == 6)
    );
    let result = definition(&mut client, &root, 5, 0, 12).await;
    assert_eq!(result["result"][0]["uri"], uri(&root));
    assert_eq!(
        result["result"][0]["range"]["start"],
        json!({"line":4,"character":9})
    );
    assert_eq!(
        references(&mut client, &root, 5, 0, false, 13).await["result"]
            .as_array()
            .unwrap()
            .len(),
        1
    );
    assert_eq!(
        references(&mut client, &root, 3, 9, true, 14).await["result"]
            .as_array()
            .unwrap()
            .len(),
        2
    );
    assert!(definition(&mut client, &root, 3, 0, 15).await["result"].is_null());
    client.finish().await;
    p.remove().await;
}
#[tokio::test]
async fn repeated_visits_deduplicate_locations_and_pending_edits_refresh_navigation() {
    let p = Project::new().await;
    let shared = p
        .write("shared.cmake", "set(V one)\nmessage(\"${V}\")\n")
        .await;
    let text = "include(shared.cmake)\ninclude(shared.cmake)\nmessage(\"${V}\")\n";
    let root = p.write("CMakeLists.txt", text).await;
    let mut client = Session::start(&p.0, vec![], true).await;
    client.open(&root, text).await;
    client.diagnostics(&root, 1).await;
    assert_eq!(
        references(&mut client, &root, 2, 11, true, 10).await["result"]
            .as_array()
            .unwrap()
            .len(),
        3
    );
    assert_eq!(
        references(&mut client, &root, 2, 11, false, 11).await["result"]
            .as_array()
            .unwrap()
            .len(),
        2
    );
    client
        .open(&shared, "set(V one)\nmessage(\"${V}\")\n")
        .await;
    client.diagnostics(&shared, 1).await;
    client.send(json!({"jsonrpc":"2.0","method":"textDocument/didChange","params":{"textDocument":{"uri":uri(&shared),"version":2},"contentChanges":[{"text":format!("{HEADER}# moved\nset(V two)\nmessage(\"${{V}}\")\n")}]}})).await;
    let result = definition(&mut client, &root, 2, 11, 12).await;
    assert_eq!(result["result"].as_array().unwrap().len(), 1);
    assert_eq!(
        result["result"][0]["range"]["start"],
        json!({"line":2,"character":4}),
        "{result}"
    );
    client.finish().await;
    p.remove().await;
}
#[tokio::test]
async fn standalone_scopes_and_encoded_closed_file_uris_do_not_guess_by_name() {
    let p = Project::new().await;
    let standalone = p
        .write(
            "loose.cmake",
            "set(V global)\nfunction(F V)\nmessage(\"${V}\")\nendfunction()\nmessage(\"${V}\")\n",
        )
        .await;
    let mut client = Session::start(&p.0, vec![], true).await;
    client
        .open(
            &standalone,
            "set(V global)\nfunction(F V)\nmessage(\"${V}\")\nendfunction()\nmessage(\"${V}\")\n",
        )
        .await;
    client.diagnostics(&standalone, 1).await;
    let local = definition(&mut client, &standalone, 2, 11, 10).await;
    assert_eq!(
        local["result"][0]["range"]["start"],
        json!({"line":2,"character":11})
    );
    let global = definition(&mut client, &standalone, 4, 11, 11).await;
    assert_eq!(
        global["result"][0]["range"]["start"],
        json!({"line":1,"character":4})
    );
    assert_eq!(
        references(&mut client, &standalone, 2, 11, true, 12).await["result"]
            .as_array()
            .unwrap()
            .len(),
        2
    );
    client.finish().await;
    p.write("my # 文档.cmake", "set(ENCODED value)\n").await;
    let text = "include(\"my # 文档.cmake\")\nmessage(\"${ENCODED}\")\n";
    let root = p.write("CMakeLists.txt", text).await;
    let mut client = Session::start(&p.0, vec![], true).await;
    client.open(&root, text).await;
    client.diagnostics(&root, 1).await;
    let result = definition(&mut client, &root, 1, 12, 20).await;
    let target = result["result"][0]["uri"].as_str().unwrap();
    assert!(
        target.ends_with("/my%20%23%20%E6%96%87%E6%A1%A3.cmake"),
        "{result}"
    );
    client.finish().await;
    p.remove().await;
}
