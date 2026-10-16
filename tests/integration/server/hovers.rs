// AI-generated variable-hover position, context and encoding tests.
use super::session::{Project, Session, uri};
use serde_json::{Value, json};
pub(super) async fn hover(
    client: &mut Session,
    path: &std::path::Path,
    line: usize,
    character: usize,
    id: i32,
) -> Value {
    client.send(json!({"jsonrpc":"2.0","id":id,"method":"textDocument/hover","params":{"textDocument":{"uri":uri(path)},"position":{"line":line,"character":character}}})).await;
    let result = client.response(id).await;
    assert!(result.get("error").is_none(), "{result}");
    result
}
#[tokio::test]
async fn values_follow_each_reference_and_edits_instead_of_final_assignment() {
    let p = Project::new().await;
    let text = "set(A one)\nmessage(\"😀 ${A}\")\nset(A two)\nmessage(\"${A}\")\nmessage(\"${MISSING}\")\n";
    let root = p.write("CMakeLists.txt", text).await;
    let mut client = Session::start(&p.0, vec![], true).await;
    client.open(&root, text).await;
    client.diagnostics(&root, 1).await;
    for (id, line, character, expected, absent) in [
        (10, 1, 4, "one", "two"),
        (11, 2, 14, "one", "two"),
        (12, 3, 4, "two", "one"),
        (13, 4, 11, "two", "one"),
    ] {
        let result = hover(&mut client, &root, line, character, id).await;
        let value = result["result"]["contents"]["value"].as_str().unwrap();
        assert!(value.contains(&format!("\"{expected}\"")), "{result}");
        assert!(!value.contains(&format!("\"{absent}\"")), "{result}");
        assert_eq!(result["result"]["range"]["start"]["line"], line);
    }
    let unknown = hover(&mut client, &root, 5, 11, 14).await;
    assert!(
        unknown["result"]["contents"]["value"]
            .as_str()
            .unwrap()
            .contains("Unknown")
    );
    let command = hover(&mut client, &root, 4, 0, 15).await;
    assert!(command.get("error").is_none(), "{command}");
    client.send(json!({"jsonrpc":"2.0","method":"textDocument/didChange","params":{"textDocument":{"uri":uri(&root),"version":2},"contentChanges":[{"range":{"start":{"line":3,"character":6},"end":{"line":3,"character":9}},"text":"three"}]}})).await;
    let changed = hover(&mut client, &root, 4, 11, 16).await;
    assert!(
        changed["result"]["contents"]["value"]
            .as_str()
            .unwrap()
            .contains("\"three\""),
        "{changed}"
    );
    client.finish().await;
    p.remove().await;
}
#[tokio::test]
async fn repeated_include_hover_keeps_both_invocation_values() {
    let p = Project::new().await;
    let text = "set(A first)\ninclude(shared.cmake)\nset(A second)\ninclude(shared.cmake)\n";
    let root = p.write("CMakeLists.txt", text).await;
    let shared = p.write("shared.cmake", "message(\"${A}\")\n").await;
    let mut client = Session::start(&p.0, vec![], true).await;
    client.open(&root, text).await;
    client.diagnostics(&root, 1).await;
    client.open(&shared, "message(\"${A}\")\n").await;
    client.diagnostics(&shared, 1).await;
    let result = hover(&mut client, &shared, 1, 11, 10).await;
    let value = result["result"]["contents"]["value"].as_str().unwrap();
    assert!(
        value.contains("\"first\"") && value.contains("\"second\""),
        "{result}"
    );
    assert!(value.matches("Static:").count() >= 2);
    assert!(!value.contains("Static ("));
    client.finish().await;
    p.remove().await;
}
