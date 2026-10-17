// AI-generated definition/reference navigation tests.
//! Bound-symbol definitions and references through framed LSP requests.
//! [`definition`] and [`references`] add one header line to fixture coordinates; child
//! modules cover dependencies, scopes and targets without changing source bindings.
use super::session::{HEADER, Project, Session, uri};
use serde_json::{Value, json};
use std::path::Path;
#[path = "navigation/dependencies.rs"]
mod dependencies;
#[path = "navigation/scopes.rs"]
mod scopes;
#[path = "navigation/targets.rs"]
mod targets;
/// Send textDocument/definition at a body-relative position.
///
/// # Arguments
/// - `client` `[in,out]`: writes the request and consumes its reply; unrelated messages append
///   to pending.
/// - `path` `[in]`: document path converted to a fixture URI.
/// - `line` `[in]`: zero-based body line; one is added for HEADER.
/// - `character` `[in]`: zero-based UTF-16 column, passed unchanged.
/// - `id` `[in]`: request/response correlation ID.
///
/// # Returns
/// The full response envelope; result is locations or null, or error may be present.
///
/// # Panics
/// Panics on transport/deadline failure; no success assertion is made here.
///
/// # Examples
/// ```text
/// let reply = definition(&mut client, &root, 0, 0, 10).await;
/// ```
pub(super) async fn definition(
    client: &mut Session,
    path: &Path,
    line: usize,
    character: usize,
    id: i32,
) -> Value {
    client.send(json!({"jsonrpc":"2.0","id":id,"method":"textDocument/definition","params":{"textDocument":{"uri":uri(path)},"position":{"line":line+1,"character":character}}})).await;
    client.response(id).await
}
/// Send textDocument/references at a header-free body position.
///
/// # Arguments
/// - `client` `[in,out]`: writes the request and consumes its reply; unrelated messages append
///   to pending.
/// - `path` `[in]`: document path converted to a fixture URI.
/// - `line` `[in]`: zero-based body line; one is added for HEADER.
/// - `character` `[in]`: zero-based UTF-16 column.
/// - `include` `[in]`: value of context.includeDeclaration sent unchanged.
/// - `id` `[in]`: request/response correlation ID.
///
/// # Returns
/// The full response envelope, normally a location array in result; errors are not asserted
/// here.
///
/// # Panics
/// Panics on session transport/deadline failures.
///
/// # Examples
/// ```text
/// let reply = references(&mut client, &root, 0, 4, true, 11).await;
/// ```
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
/// Check cross-file bindings, declaration inclusion and UTF-16 navigation rather than name
/// guesses.
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
/// Import External/V, navigate a case-folded function call and an emoji-prefixed variable, and
/// compare exact declaration/use ranges. Toggle includeDeclaration for reference counts; bind
/// ARG to its formal. Unknown variables and non-symbol argument positions return null
/// definitions or empty references.
///
/// # Flow
/// ```text
/// [Temporary sources + settings]      [Expected: bound locations + null exclusions]
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
/// cross_file_functions_variables_parameters_and_utf16_ranges_use_bound_symbols(); // #[tokio::test] supplies the runtime
/// ```
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
/// Check builtin message references stay distinct from later user-shadow calls.
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
/// Include another builtin call and then define message. Before the shadow, require null
/// definition and three builtin references excluding the later call; afterward require the user
/// declaration and distinct one/two reference counts. The function keyword itself has no
/// definition.
///
/// # Flow
/// ```text
/// [Temporary sources + settings]      [Expected: separate builtin / user sets]
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
/// builtin_calls_have_no_definition_and_do_not_merge_with_shadowing_functions(); // #[tokio::test] supplies the runtime
/// ```
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
/// Check repeated includes deduplicate locations and navigation reads pending dependency edits.
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
/// Include the same V declaration twice: require three locations with declaration, two without.
/// Open and edit the dependency to move its declaration, immediately query from the root and
/// require one definition at its new line.
///
/// # Flow
/// ```text
/// [Temporary sources + settings]      [Expected: deduplicated moved definition]
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
/// repeated_visits_deduplicate_locations_and_pending_edits_refresh_navigation(); // #[tokio::test] supplies the runtime
/// ```
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
/// Check standalone formal/global V bindings and percent-encoded closed-file definition URIs.
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
/// Navigate local and global V uses in an unowned file and require distinct declaration ranges
/// plus two formal references. Restart with a root importing a filename containing spaces, #
/// and Chinese characters; require its percent-encoded closed-file URI.
///
/// # Flow
/// ```text
/// [Temporary sources + settings]      [Expected: scoped ranges + encoded URI]
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
/// standalone_scopes_and_encoded_closed_file_uris_do_not_guess_by_name(); // #[tokio::test] supplies the runtime
/// ```
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
