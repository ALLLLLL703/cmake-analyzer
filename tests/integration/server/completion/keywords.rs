// AI-generated schema argument-keyword completion integration tests.
//! Schema keyword completion in payloads, mixed roles and excluded contexts.
//! Marker-based edits query current overlays; snippet capability must not change keyword edits.
use super::{complete, item, start};
use crate::server::session::{HEADER, Project, Session, uri};
use serde_json::{Value, json};
use std::path::Path;

/// Replace the entire open overlay and immediately request completion at its marker.
/// Writes didChange/completion frames, not fixture disk contents.
///
/// # Arguments
/// - `client` `[in,out]`: writes the request and consumes its reply; unrelated messages append
///   to pending.
/// - `path` `[in]`: document path converted to a fixture URI.
/// - `marked` `[in]`: full header-free replacement source; first | marks the cursor and is
///   removed.
/// - `version` `[in]`: didChange document version.
/// - `id` `[in]`: request/response correlation ID.
///
/// # Returns
/// The cloned successful completion result from the shared complete helper.
///
/// # Panics
/// Panics if the marker is absent, sending fails or completion returns an error.
///
/// # Algorithm
/// Split at the first marker to count body newlines and UTF-16 column; remove that marker,
/// prepend HEADER, send the full-text versioned change, then query completion.
/// The shared complete helper adds the one header line; this request is an ordering barrier.
///
/// # Flow
/// ```text
/// [Marked body source]
///          |
///          +-------------------------+
///          v                         v
///  [Remove first marker]    [Count line / UTF-16 column]
///          |                         |
///          v                         |
///  [Send full didChange]             |
///          |                         |
///          +-------------+-----------+
///                        v
///      [complete: add header line, send query]
///                        |
///                        v
///              [Completion result]
/// ```
///
/// # Examples
/// ```text
/// let items = changed(&mut client, &root, "include(Loc|)", 2, 10).await;
/// ```
async fn changed(client: &mut Session, path: &Path, marked: &str, version: i32, id: i32) -> Value {
    let offset = marked.find('|').unwrap();
    let before = &marked[..offset];
    let line = before.bytes().filter(|byte| *byte == b'\n').count();
    let column = before.rsplit('\n').next().unwrap().encode_utf16().count();
    client.send(json!({"jsonrpc":"2.0","method":"textDocument/didChange","params":{"textDocument":{"uri":uri(path),"version":version},"contentChanges":[{"text":format!("{HEADER}{}",marked.replacen('|',"",1))}]}})).await;
    complete(client, path, line, column, id).await
}
/// Check whether any completion item has an exact case-sensitive label; no side effects.
///
/// # Arguments
/// - `items` `[in]`: completion result array, borrowed without mutation.
/// - `label` `[in]`: label spelling to compare.
///
/// # Returns
/// true if at least one label matches; false otherwise.
///
/// # Panics
/// Panics if items is not a JSON array.
///
/// # Examples
/// ```text
/// let present = has(&items, "PUBLIC");
/// ```
fn has(items: &Value, label: &str) -> bool {
    items
        .as_array()
        .unwrap()
        .iter()
        .any(|item| item["label"] == label)
}

#[tokio::test]
/// Check schema keywords use plain sorted edits covering only argument payloads, even with
/// reserve_arg.
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
/// Enable snippets, then query native keywords in bare/quoted/bracket/incomplete calls,
/// including an emoji before HEX. Require kind 14, plain format 1, exact keyword text and
/// UTF-16 payload ranges, with strictly sorted labels.
///
/// # Flow
/// ```text
/// [Temporary sources + settings]      [Expected: sorted payload-only keyword edits]
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
/// schema_keywords_replace_only_argument_payloads_and_never_insert_snippets(); // #[tokio::test] supplies the runtime
/// ```
async fn schema_keywords_replace_only_argument_payloads_and_never_insert_snippets() {
    let p = Project::new().await;
    tokio::fs::write(
        p.0.join(".cmake-analyzer.toml"),
        "[completion]\nreserve_arg=true\n",
    )
    .await
    .unwrap();
    let root = p.write("CMakeLists.txt", "").await;
    let mut client = start(&p, true).await;
    client.open(&root, "").await;
    client.diagnostics(&root, 1).await;
    for (index, (source, keyword)) in [
        ("add_library(Core ST|ATIC)", "STATIC"),
        ("add_library(Core \"ST|ATIC\")", "STATIC"),
        ("add_library(Core [=[ST|ATIC]=])", "STATIC"),
        ("add_library(Core \"ST|", "STATIC"),
        ("file(RE|AD input OUT)", "READ"),
        ("file(READ \"😀\" OUT HE|X)", "HEX"),
        ("file(COPY input D|ESTINATION out)", "DESTINATION"),
        ("target_link_libraries(App PUBLIC lib PRI|VATE)", "PRIVATE"),
        ("add_library(Core STATIC IMPORTED G|LOBAL)", "GLOBAL"),
        ("target_link_libraries(App de|bug lib)", "debug"),
    ]
    .iter()
    .enumerate()
    {
        let matches = changed(
            &mut client,
            &root,
            source,
            index as i32 + 2,
            index as i32 + 10,
        )
        .await;
        let entry = item(&matches, keyword);
        assert_eq!(entry["kind"], 14, "{source}: {matches}");
        assert_eq!(entry["insertTextFormat"], 1);
        assert_eq!(entry["textEdit"]["newText"], *keyword);
        let clean = source.replacen('|', "", 1);
        let existing = if index == 3 { "ST" } else { *keyword };
        let offset = clean.find(existing).unwrap();
        let column = clean[..offset].encode_utf16().count();
        assert_eq!(
            entry["textEdit"]["range"],
            json!({"start":{"line":1,"character":column},"end":{"line":1,"character":column+existing.len()}}),
            "{source}: {matches}"
        );
        let labels: Vec<_> = matches
            .as_array()
            .unwrap()
            .iter()
            .map(|item| item["label"].as_str().unwrap())
            .collect();
        assert!(labels.windows(2).all(|pair| pair[0] < pair[1]), "{matches}");
    }
    client.finish().await;
    p.remove().await;
}

#[tokio::test]
/// Check schema keywords coexist with target roles but not variable-reference completion.
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
/// Complete a target-link prefix and require PUBLIC/PRIVATE keywords plus PUBLIC_LIB, not
/// PUBLIC_VAR. In a variable expansion require only the variable class. Known OP/OWNER scalar
/// expansions preserve HEX/PUBLIC roles; a fully typed PUBLIC still completes as Keyword.
///
/// # Flow
/// ```text
/// [Temporary sources + settings]      [Expected: mixed targets / isolated variables]
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
/// context_filters_keywords_and_keeps_variable_and_target_candidates(); // #[tokio::test] supplies the runtime
/// ```
async fn context_filters_keywords_and_keeps_variable_and_target_candidates() {
    let p = Project::new().await;
    let base =
        "add_executable(App main.c)\nadd_library(PUBLIC_LIB INTERFACE)\nset(PUBLIC_VAR yes)\n";
    let root = p.write("CMakeLists.txt", base).await;
    let mut client = start(&p, true).await;
    client.open(&root, base).await;
    client.diagnostics(&root, 1).await;
    let mixed = changed(
        &mut client,
        &root,
        &format!("{base}target_link_libraries(App P|)"),
        2,
        10,
    )
    .await;
    assert_eq!(item(&mixed, "PUBLIC")["kind"], 14);
    assert_eq!(item(&mixed, "PRIVATE")["kind"], 14);
    assert_eq!(item(&mixed, "PUBLIC_LIB")["kind"], 20);
    assert!(!has(&mixed, "PUBLIC_VAR"));
    let expansion = changed(
        &mut client,
        &root,
        &format!("{base}target_link_libraries(App \"${{PU|}}\")"),
        3,
        11,
    )
    .await;
    assert_eq!(item(&expansion, "PUBLIC_VAR")["kind"], 6);
    assert!(!has(&expansion, "PUBLIC"));
    let opcode = changed(
        &mut client,
        &root,
        &format!("{base}set(OP READ)\nfile(${{OP}} input OUT HE|)"),
        4,
        12,
    )
    .await;
    assert!(has(&opcode, "HEX"), "{opcode}");
    let known = changed(
        &mut client,
        &root,
        &format!("{base}set(OWNER App)\ntarget_link_libraries(${{OWNER}} PU|)"),
        5,
        13,
    )
    .await;
    assert!(has(&known, "PUBLIC"), "{known}");
    let full = changed(
        &mut client,
        &root,
        &format!("{base}target_link_libraries(App PUBLIC|)"),
        6,
        14,
    )
    .await;
    assert_eq!(item(&full, "PUBLIC")["kind"], 14);
    client.finish().await;
    p.remove().await;
}

#[tokio::test]
/// Check scalar and unsupported contexts never invent schema keyword candidates.
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
/// Iterate source tails, scalar OFFSET/RELATIVE/DESCRIPTION slots, unknown calls, wrong-case
/// prefixes, strings/comments, unknown/list-valued owner expansions and a user
/// target_link_libraries shadow. Each marked completion must be an empty array.
///
/// # Flow
/// ```text
/// [Temporary sources + settings]      [Expected: empty excluded completions]
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
/// scalar_values_opaque_tails_comments_expansions_and_native_shadows_do_not_invent_keywords(); // #[tokio::test] supplies the runtime
/// ```
async fn scalar_values_opaque_tails_comments_expansions_and_native_shadows_do_not_invent_keywords()
{
    let p = Project::new().await;
    let root = p.write("CMakeLists.txt", "").await;
    let mut client = start(&p, true).await;
    client.open(&root, "").await;
    client.diagnostics(&root, 1).await;
    for (index, source) in [
        "add_library(Core source.c ST|)",
        "file(READ input OUT OFFSET HE|)",
        "file(GLOB OUT CONFIGURE_DEPENDS RELATIVE CON|)",
        "project(Core DESCRIPTION LA|)",
        "unknown_function(PU|)",
        "add_library(Core st|)",
        "message(\"text ST|\")",
        "# add_library(Core ST|)",
        "target_link_libraries(${UNKNOWN} PU|)",
        "set(OWNER \"App;Other\")\ntarget_link_libraries(${OWNER} PU|)",
        "function(target_link_libraries ARG)\nendfunction()\ntarget_link_libraries(App PU|)",
    ]
    .iter()
    .enumerate()
    {
        let matches = changed(
            &mut client,
            &root,
            source,
            index as i32 + 2,
            index as i32 + 10,
        )
        .await;
        assert!(
            matches.as_array().unwrap().is_empty(),
            "{source}: {matches}"
        );
    }
    client.finish().await;
    p.remove().await;
}
