// AI-generated target definition/reference navigation tests.
//! Literal target/alias definitions and typed operand references.
//! Tests distinguish imported invocation-directory visibility, user shadows and pending edits.
#[path = "targets_owners.rs"]
mod owners;
use super::super::session::{HEADER, Project, Session, uri};
use super::{definition, references};
use serde_json::{Value, json};
/// Compute the UTF-16 column of the first exact name occurrence on a body line; no side
/// effects.
///
/// # Arguments
/// - `text` `[in]`: header-free multiline fixture source.
/// - `line` `[in]`: zero-based body line to inspect.
/// - `name` `[in]`: substring whose first occurrence determines the column.
///
/// # Returns
/// The count of UTF-16 code units before the first occurrence; no HEADER line adjustment.
///
/// # Panics
/// Panics if the line or substring is absent.
///
/// # Examples
/// ```text
/// let col = column("get_target_property(🌟 Core TYPE)", 0, "Core");
/// ```
fn column(text: &str, line: usize, name: &str) -> usize {
    let text = text.lines().nth(line).unwrap();
    text[..text.find(name).unwrap()].encode_utf16().count()
}
/// Count locations in a navigation response result array; no side effects.
///
/// # Arguments
/// - `response` `[in]`: full response envelope whose result must be an array.
///
/// # Returns
/// The result array length; no deduplication is performed here.
///
/// # Panics
/// Panics if result is null, absent or not an array.
///
/// # Examples
/// ```text
/// let locations = count(&reply);
/// ```
fn count(response: &Value) -> usize {
    response["result"].as_array().unwrap().len()
}

#[tokio::test]
/// Check target/alias payload spans and schema-role references exclude ordinary values.
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
/// Declare Core/App/Assets/CoreAlias; navigate quoted/bracket target operands and the alias
/// original. Require exact declaration ranges and 5/6 Core references without/with declaration,
/// one alias use, and Assets binding. External names and quote delimiters return null; known
/// scalar APP expansion preserves a later target role while list-valued APP suppresses it.
///
/// # Flow
/// ```text
/// [Temporary sources + settings]      [Expected: target counts + typed exclusions]
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
/// literal_targets_aliases_and_schema_operands_have_distinct_definitions_and_references(); // #[tokio::test] supplies the runtime
/// ```
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
/// Check imported target and alias visibility follows include invocation directories.
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
/// Visit the same imported-target module from sibling directories plus a nested child. Root
/// cannot see Scoped/ScopedAlias but sees root Shared and ordinary ATarget. References for a
/// and b remain separate; a/deep inherits a's ScopedAlias declaration.
///
/// # Flow
/// ```text
/// [Temporary sources + settings]      [Expected: directory-qualified target sets]
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
/// imported_and_alias_visibility_follows_invocation_directories_not_file_paths(); // #[tokio::test] supplies the runtime
/// ```
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
/// Check native target roles are not inherited by shadows or inferred from ordinary values.
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
/// Declare Real, expand NAME in target_sources, then shadow target_sources and add_library.
/// Require only one bound Real reference; shadow arguments and Fake declarations/uses have null
/// definitions, while NAME still links to its set declaration.
///
/// # Flow
/// ```text
/// [Temporary sources + settings]      [Expected: one Real use / null Fake links]
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
/// user_shadows_dynamic_names_and_ordinary_values_do_not_create_target_bindings(); // #[tokio::test] supplies the runtime
/// ```
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
/// Check file-local target navigation refreshes renamed declarations and UTF-16 operand spans.
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
/// Start with Old and require two references including declaration. Replace the overlay with a
/// moved New declaration, emoji-bearing use and stale Old use; require the new range, null Old
/// definition and two New references.
///
/// # Flow
/// ```text
/// [Temporary sources + settings]      [Expected: moved New / stale Old null]
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
/// standalone_target_references_follow_pending_utf16_edits(); // #[tokio::test] supplies the runtime
/// ```
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
