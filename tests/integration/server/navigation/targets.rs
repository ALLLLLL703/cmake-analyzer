// AI-generated target definition/reference navigation tests.
#[path = "targets_owners.rs"]
mod owners;
use super::super::session::{HEADER, Project, Session, uri};
use super::{definition, references};
use serde_json::{Value, json};
fn column(text: &str, line: usize, name: &str) -> usize {
    let text = text.lines().nth(line).unwrap();
    text[..text.find(name).unwrap()].encode_utf16().count()
}
fn count(response: &Value) -> usize {
    response["result"].as_array().unwrap().len()
}

#[tokio::test]
async fn literal_targets_aliases_and_schema_operands_have_distinct_definitions_and_references() {
    let p = Project::new().await;
    let text = "add_library(\"Core\" STATIC core.c)\nadd_executable(App main.c)\nadd_custom_target(Assets)\nadd_library(CoreAlias ALIAS Core)\ntarget_link_libraries(App PUBLIC \"Core\" CoreAlias external)\nadd_dependencies(App Assets)\nget_target_property(🌟 [=[Core]=] TYPE)\ntarget_include_directories(Core PRIVATE Core)\nset(NAME Core)\nmessage(\"${NAME}\")\nset(APP App)\ntarget_link_libraries(${APP} PUBLIC Core)\nset(APP \"App;Other\")\ntarget_link_libraries(${APP} PUBLIC Core)\n";
    let root = p.write("CMakeLists.txt", text).await;
    let mut client = Session::start(&p.0, vec![], true).await;
    client.open(&root, text).await;
    client.diagnostics(&root, 1).await;
    let found = definition(&mut client, &root, 4, column(text, 4, "Core"), 10).await;
    assert_eq!(
        found["result"],
        json!([{"uri":uri(&root),"range":{"start":{"line":1,"character":13},"end":{"line":1,"character":17}}}]),
        "{found}"
    );
    let alias = definition(&mut client, &root, 4, column(text, 4, "CoreAlias"), 11).await;
    assert_eq!(
        alias["result"][0]["range"]["start"],
        json!({"line":4,"character":12})
    );
    let original = text.lines().nth(3).unwrap().rfind("Core").unwrap();
    assert_eq!(
        definition(&mut client, &root, 3, original, 12).await["result"],
        found["result"]
    );
    assert_eq!(
        definition(&mut client, &root, 6, column(text, 6, "Core"), 13).await["result"],
        found["result"]
    );
    let refs = references(&mut client, &root, 0, 13, false, 14).await;
    assert_eq!(count(&refs), 5, "{refs}");
    assert_eq!(
        count(&references(&mut client, &root, 0, 13, true, 15).await),
        6
    );
    assert_eq!(
        count(&references(&mut client, &root, 3, 12, false, 16).await),
        1
    );
    assert_eq!(
        definition(&mut client, &root, 5, column(text, 5, "Assets"), 17).await["result"][0]["range"]
            ["start"],
        json!({"line":3,"character":18})
    );
    assert!(
        definition(&mut client, &root, 4, column(text, 4, "external"), 18).await["result"]
            .is_null()
    );
    let quote = column(text, 4, "Core") - 1;
    assert!(definition(&mut client, &root, 4, quote, 19).await["result"].is_null());
    let variable = definition(&mut client, &root, 9, column(text, 9, "NAME"), 20).await;
    assert_eq!(
        variable["result"][0]["range"]["start"],
        json!({"line":9,"character":4})
    );
    assert_eq!(
        definition(&mut client, &root, 11, column(text, 11, "Core"), 21).await["result"],
        found["result"]
    );
    assert!(
        definition(&mut client, &root, 13, column(text, 13, "Core"), 22).await["result"].is_null()
    );
    client.finish().await;
    p.remove().await;
}

#[tokio::test]
async fn imported_and_alias_visibility_follows_invocation_directories_not_file_paths() {
    let p = Project::new().await;
    let common = p
        .write(
            "cmake/targets.cmake",
            "add_library(Scoped INTERFACE IMPORTED)\nadd_library(ScopedAlias ALIAS Scoped)\n",
        )
        .await;
    let shared = p
        .write(
            "cmake/shared.cmake",
            "add_library(Shared INTERFACE IMPORTED)\n",
        )
        .await;
    let a_text = "include(../cmake/targets.cmake)\nadd_library(ATarget INTERFACE)\nadd_library(Everywhere INTERFACE IMPORTED GLOBAL)\nadd_dependencies(ATarget Scoped ScopedAlias Shared)\nadd_subdirectory(deep)\n";
    let a = p.write("a/CMakeLists.txt", a_text).await;
    let deep_text =
        "add_custom_target(Deep)\nadd_dependencies(Deep Scoped ScopedAlias Everywhere)\n";
    let deep = p.write("a/deep/CMakeLists.txt", deep_text).await;
    let b_text = "include(../cmake/targets.cmake)\nadd_library(BTarget INTERFACE)\nadd_dependencies(BTarget Scoped ScopedAlias Shared Everywhere ATarget)\n";
    let b = p.write("b/CMakeLists.txt", b_text).await;
    let text = "add_executable(App main.c)\ninclude(cmake/shared.cmake)\nadd_subdirectory(a)\nadd_subdirectory(b)\nadd_dependencies(App Scoped ScopedAlias ATarget Everywhere Shared)\n";
    let root = p.write("CMakeLists.txt", text).await;
    let mut client = Session::start(&p.0, vec![], true).await;
    client.open(&root, text).await;
    client.diagnostics(&root, 1).await;
    assert!(
        definition(&mut client, &root, 4, column(text, 4, "Scoped"), 10).await["result"].is_null()
    );
    assert!(
        definition(&mut client, &root, 4, column(text, 4, "ScopedAlias"), 11).await["result"]
            .is_null()
    );
    assert_eq!(
        definition(&mut client, &root, 4, column(text, 4, "Shared"), 12).await["result"][0]["uri"],
        uri(&shared)
    );
    assert_eq!(
        definition(&mut client, &root, 4, column(text, 4, "ATarget"), 13).await["result"][0]["uri"],
        uri(&a)
    );
    assert_eq!(
        definition(&mut client, &a, 3, column(a_text, 3, "ScopedAlias"), 14).await["result"][0]["uri"],
        uri(&common)
    );
    let a_refs = references(&mut client, &a, 3, column(a_text, 3, "Scoped"), false, 15).await;
    assert_eq!(count(&a_refs), 3, "{a_refs}");
    assert!(
        !a_refs["result"]
            .as_array()
            .unwrap()
            .iter()
            .any(|location| location["uri"] == uri(&b))
    );
    let b_refs = references(&mut client, &b, 2, column(b_text, 2, "Scoped"), false, 16).await;
    assert_eq!(count(&b_refs), 2, "{b_refs}");
    assert!(
        !b_refs["result"]
            .as_array()
            .unwrap()
            .iter()
            .any(|location| location["uri"] == uri(&a) || location["uri"] == uri(&deep))
    );
    assert_eq!(
        definition(
            &mut client,
            &deep,
            1,
            column(deep_text, 1, "ScopedAlias"),
            17
        )
        .await["result"][0]["uri"],
        uri(&common)
    );
    client.finish().await;
    p.remove().await;
}

#[tokio::test]
async fn user_shadows_dynamic_names_and_ordinary_values_do_not_create_target_bindings() {
    let p = Project::new().await;
    let text = "add_library(Real INTERFACE)\nset(NAME Real)\ntarget_sources(\"${NAME}\" PRIVATE Real)\nfunction(target_sources ARG)\nendfunction()\ntarget_sources(Real PRIVATE item.c)\nfunction(add_library ARG)\nendfunction()\nadd_library(Fake INTERFACE)\ntarget_link_libraries(Real PRIVATE Fake Unknown)\n";
    let root = p.write("CMakeLists.txt", text).await;
    let mut client = Session::start(&p.0, vec![], true).await;
    client.open(&root, text).await;
    client.diagnostics(&root, 1).await;
    let refs = references(&mut client, &root, 0, 12, false, 10).await;
    assert_eq!(count(&refs), 1, "{refs}");
    assert!(
        definition(&mut client, &root, 5, column(text, 5, "Real"), 11).await["result"].is_null()
    );
    assert!(
        definition(&mut client, &root, 9, column(text, 9, "Fake"), 12).await["result"].is_null()
    );
    assert!(
        definition(&mut client, &root, 8, column(text, 8, "Fake"), 13).await["result"].is_null()
    );
    assert_eq!(
        definition(&mut client, &root, 2, column(text, 2, "NAME"), 14).await["result"][0]["range"]
            ["start"],
        json!({"line":2,"character":4})
    );
    client.finish().await;
    p.remove().await;
}

#[tokio::test]
async fn standalone_target_references_follow_pending_utf16_edits() {
    let p = Project::new().await;
    let text = "add_library(Old INTERFACE)\nget_target_property(🌟 Old TYPE)\n";
    let loose = p.write("loose.cmake", text).await;
    let mut client = Session::start(&p.0, vec![], true).await;
    client.open(&loose, text).await;
    client.diagnostics(&loose, 1).await;
    assert_eq!(
        count(&references(&mut client, &loose, 0, 12, true, 10).await),
        2
    );
    let updated = "\nadd_library(New INTERFACE)\nget_target_property(🌟 New TYPE)\nget_target_property(OUT Old TYPE)\n";
    client.send(json!({"jsonrpc":"2.0","method":"textDocument/didChange","params":{"textDocument":{"uri":uri(&loose),"version":2},"contentChanges":[{"text":format!("{HEADER}{updated}")}]}})).await;
    let new = definition(&mut client, &loose, 2, column(updated, 2, "New"), 11).await;
    assert_eq!(
        new["result"][0]["range"]["start"],
        json!({"line":2,"character":12}),
        "{new}"
    );
    assert!(
        definition(&mut client, &loose, 3, column(updated, 3, "Old"), 12).await["result"].is_null()
    );
    assert_eq!(
        count(&references(&mut client, &loose, 1, 12, true, 13).await),
        2
    );
    client.finish().await;
    p.remove().await;
}
