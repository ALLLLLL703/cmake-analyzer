// AI-generated builtin/source-documentation hover regressions.
use super::{
    hovers::hover,
    session::{HEADER, Project, Session, uri},
};
use cmake_analyzer::server::{ServerOptions, hovers::include};
use serde_json::json;
#[test]
fn embedded_catalog_covers_every_vendored_command_and_variable_page() {
    assert_eq!(include::VERSION, "4.4.4");
    assert_eq!(include::COMMANDS.len(), 132);
    assert_eq!(include::VARIABLES.len(), 810);
    for table in [include::COMMANDS, include::VARIABLES] {
        assert!(table.windows(2).all(|pair| pair[0].0 < pair[1].0));
        for (name, doc) in table {
            assert!(doc.starts_with("CMake 4.4.4"), "{name}");
            assert!(doc.ends_with("<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->\n"), "{name}");
            assert!(doc.contains("CMake 4.4.4"), "{name}");
            assert!(doc.contains("https://cmake.org/cmake/help/v4.4/"), "{name}");
        }
    }
}
#[tokio::test]
async fn builtin_commands_and_exact_multi_template_and_argument_variables_have_docs() {
    let p = Project::new().await;
    let text = "project(MyProject LANGUAGES CXX)\nset(CMAKE_CXX_FLAGS_DEBUG flags)\nmessage(\"${CMAKE_CURRENT_SOURCE_DIR}\" \"${CMAKE_CXX_FLAGS_DEBUG}\" \"${MyProject_SOURCE_DIR}\")\nmessage(\"${CMAKE_ARGV7}\")\nfunction(f P)\nmessage(\"${ARGV0}\")\nendfunction()\n";
    let root = p.write("CMakeLists.txt", text).await;
    let mut client = Session::start(&p.0, vec![], true).await;
    client.open(&root, text).await;
    client.diagnostics(&root, 1).await;
    for (id, line, character) in [
        (10, 1, 0),
        (11, 2, 0),
        (12, 2, 4),
        (13, 3, 12),
        (14, 4, 12),
        (15, 6, 12),
    ] {
        let result = hover(&mut client, &root, line, character, id).await;
        let content = result["result"]["contents"]["value"].as_str().unwrap();
        assert!(
            content.contains("CMake 4.4.4") && content.contains("Official documentation"),
            "{result}"
        );
        assert!(!content.contains("Script-generated"), "{result}");
        assert!(!content.contains("scripts/gen/"), "{result}");
        assert_eq!(result["result"]["contents"]["kind"], "plaintext");
    }
    let line = text.lines().nth(2).unwrap();
    let character = line.find("MyProject_SOURCE_DIR").unwrap();
    let result = hover(&mut client, &root, 3, character, 16).await;
    assert!(
        result["result"]["contents"]["value"]
            .as_str()
            .unwrap()
            .contains("Official documentation"),
        "{result}"
    );
    client.finish().await;
    p.remove().await;
}
#[tokio::test]
async fn imported_function_macro_and_set_docs_follow_bindings_and_comment_only_edits() {
    let p = Project::new().await;
    let shared_text = "# External function documentation.\nfunction(external_fn)\nendfunction()\n#[=[\nExternal variable documentation.\n]=]\nset(EXTERNAL one)\n# External macro documentation.\nmacro(external_macro)\nendmacro()\n";
    let shared = p.write("shared.cmake", shared_text).await;
    let text = "# Outdated variable documentation.\nset(EXTERNAL old)\ninclude(shared.cmake)\nexternal_fn()\nmessage(\"${EXTERNAL}\")\nexternal_macro()\n";
    let root = p.write("CMakeLists.txt", text).await;
    let mut client = Session::start(&p.0, vec![], true).await;
    client.open(&root, text).await;
    client.diagnostics(&root, 1).await;
    client.open(&shared, shared_text).await;
    client.diagnostics(&shared, 1).await;
    for (id, line, character, expected) in [
        (10, 4, 0, "External function documentation."),
        (11, 5, 12, "External variable documentation."),
        (12, 6, 0, "External macro documentation."),
    ] {
        let result = hover(&mut client, &root, line, character, id).await;
        assert!(
            result["result"]["contents"]["value"]
                .as_str()
                .unwrap()
                .contains(expected),
            "{result}"
        );
        assert!(
            !result["result"]["contents"]["value"]
                .as_str()
                .unwrap()
                .contains("Outdated variable documentation"),
            "{result}"
        );
    }
    let edited = shared_text.replace(
        "External function documentation.",
        "Updated function documentation 🌟.",
    );
    client.send(json!({"jsonrpc":"2.0","method":"textDocument/didChange","params":{"textDocument":{"uri":uri(&shared),"version":2},"contentChanges":[{"text":format!("{HEADER}{edited}")}]}})).await;
    let result = hover(&mut client, &root, 4, 0, 13).await;
    let content = result["result"]["contents"]["value"].as_str().unwrap();
    assert!(
        content.contains("Updated function documentation 🌟.")
            && !content.contains("External function documentation."),
        "{result}"
    );
    client.finish().await;
    p.remove().await;
}
#[tokio::test]
async fn blank_lines_and_trailing_comments_do_not_attach_and_markdown_respects_byte_limit() {
    let p = Project::new().await;
    tokio::fs::write(
        p.0.join(".cmake-analyzer.toml"),
        "[runtime]\nmax_view_bytes=128\n",
    )
    .await
    .unwrap();
    let text = format!(
        "# Detached documentation marker.\n\nfunction(no_docs)\nendfunction()\nno_docs() # trailing marker\nfunction(next_fn)\nendfunction()\nnext_fn()\n# {}\nfunction(documented)\nendfunction()\ndocumented()\n",
        "🌟".repeat(100)
    );
    let root = p.write("CMakeLists.txt", &text).await;
    let mut options = ServerOptions::new(p.0.clone());
    options.config.user_file = None;
    let mut client = Session::start_hover_format(options, vec![], true, true).await;
    client.open(&root, &text).await;
    client.diagnostics(&root, 1).await;
    for (id, line) in [(10, 5), (11, 8), (12, 12)] {
        let result = hover(&mut client, &root, line, 0, id).await;
        let content = result["result"]["contents"]["value"].as_str().unwrap();
        assert!(
            !content.contains("Detached documentation marker")
                && !content.contains("trailing marker"),
            "{result}"
        );
        assert!(content.len() <= 128, "{result}");
        assert_eq!(result["result"]["contents"]["kind"], "markdown");
        if line == 12 {
            assert!(content.contains('🌟'), "{result}");
        }
    }
    client.finish().await;
    p.remove().await;
}
