// AI-generated variable-hover position, context and encoding tests.
//! Variable hover values at each reference, including UTF-16 and repeated visits.
//! [`hover`] accepts protocol lines including the injected fixture header.
use super::session::{Project, Session, uri};
use serde_json::{Value, json};
/// Send textDocument/hover at protocol coordinates and require no response error.
///
/// # Arguments
/// - `client` `[in,out]`: writes the request and consumes its reply; unrelated messages append
///   to pending.
/// - `path` `[in]`: document path converted to a fixture URI.
/// - `line` `[in]`: zero-based protocol line, already including HEADER; no offset is added.
/// - `character` `[in]`: zero-based UTF-16 column, passed unchanged.
/// - `id` `[in]`: request/response correlation ID.
///
/// # Returns
/// The full successful response; result may be null or a hover object.
///
/// # Panics
/// Panics on transport/deadline failure or a response error.
///
/// # Examples
/// ```text
/// let reply = hover(&mut client, &root, 1, 0, 10).await;
/// ```
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
/// Check hover uses position-specific values and UTF-16 incremental edits.
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
/// Hover assignments and references before/after A changes, requiring only the value valid
/// there and the queried line range. Missing variables show Unknown. Replace the second
/// assignment through didChange and require the next reference to show three.
///
/// # Flow
/// ```text
/// [Temporary sources + settings]      [Expected: one / two / three hovers]
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
/// values_follow_each_reference_and_edits_instead_of_final_assignment(); // #[tokio::test] supplies the runtime
/// ```
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
/// Check one included reference reports both static invocation values without stale
/// presentation.
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
/// Include shared.cmake with A=first then A=second, open the shared reference and hover it.
/// Require both quoted values, at least two Static: sections and no old Static ( heading.
///
/// # Flow
/// ```text
/// [Temporary sources + settings]      [Expected: two static hover contexts]
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
/// repeated_include_hover_keeps_both_invocation_values(); // #[tokio::test] supplies the runtime
/// ```
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
