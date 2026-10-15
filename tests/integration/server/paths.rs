// AI-generated known-input path warning and generated/output exclusion tests.
use super::session::{Project, Session, uri};
use serde_json::json;
#[tokio::test]
async fn input_paths_warn_without_checking_outputs_generated_unknown_or_optional_paths() {
    let p = Project::new().await;
    tokio::fs::create_dir_all(p.0.join("existing_dir"))
        .await
        .unwrap();
    tokio::fs::write(p.0.join("input.in"), "input")
        .await
        .unwrap();
    let text = "file(WRITE written.c content)\nconfigure_file(input.in configured.c)\nfile(GENERATE OUTPUT generated.c CONTENT \"INPUT\")\nadd_custom_command(OUTPUT custom.c custom2.c COMMAND tool)\nset_source_files_properties(marked.c PROPERTIES GENERATED TRUE)\nadd_executable(app missing.c written.c configured.c generated.c custom.c custom2.c marked.c \"$<TARGET_OBJECTS:objects>\")\nfile(READ missing.txt DATA)\nfile(READ existing_dir DATA)\naux_source_directory(input.in SOURCES)\nfile(READ \"${UNKNOWN}\" DATA)\nfile(REMOVE remove_missing)\nfile(MAKE_DIRECTORY future_dir)\nfile(DOWNLOAD https://example.invalid output_download)\ninclude(optional.cmake OPTIONAL)\ninclude(required.cmake)\nif(EXISTS conditional.txt)\nfile(READ conditional.txt DATA)\nendif()\nset(INPUT later.txt)\nfile(READ \"${INPUT}\" DATA)\n";
    let root = p.write("CMakeLists.txt", text).await;
    let mut client = Session::start(&p.0, vec![], true).await;
    client.open(&root, text).await;
    let report = client.diagnostics(&root, 1).await;
    let diagnostics = report["params"]["diagnostics"].as_array().unwrap();
    for name in [
        "missing.c",
        "missing.txt",
        "existing_dir",
        "input.in",
        "later.txt",
    ] {
        assert!(
            diagnostics
                .iter()
                .any(|item| item["severity"] == 2
                    && item["message"].as_str().unwrap().contains(name)),
            "{name}: {report}"
        );
    }
    assert!(
        diagnostics
            .iter()
            .any(|item| item["severity"] == 2 && item["range"]["start"]["line"] == 15),
        "{report}"
    );
    for name in [
        "written.c",
        "configured.c",
        "generated.c",
        "custom.c",
        "custom2.c",
        "marked.c",
        "output_download",
        "future_dir",
        "remove_missing",
        "optional.cmake",
        "conditional.txt",
        "TARGET_OBJECTS",
        "CONDITION",
    ] {
        assert!(
            !diagnostics
                .iter()
                .any(|item| item["message"].as_str().unwrap().contains(name)),
            "{name}: {report}"
        );
    }
    let warning = diagnostics
        .iter()
        .find(|item| item["message"].as_str().unwrap().contains("missing.c"))
        .unwrap();
    assert_eq!(warning["range"]["start"]["character"], 19);
    tokio::fs::write(p.0.join("later.txt"), "now exists")
        .await
        .unwrap();
    client.send(json!({"jsonrpc":"2.0","method":"textDocument/didSave","params":{"textDocument":{"uri":uri(&root)}}})).await;
    let report = client.diagnostics(&root, 1).await;
    assert!(
        !report["params"]["diagnostics"]
            .as_array()
            .unwrap()
            .iter()
            .any(|item| item["message"].as_str().unwrap().contains("later.txt")),
        "{report}"
    );
    client.finish().await;
    p.remove().await;
}
