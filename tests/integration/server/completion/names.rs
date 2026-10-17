// AI-generated schema-driven variable/target completion integration tests.
//! Case-sensitive variable and target names in typed arguments and expansions.
//! Tests isolate lexical, directory and imported visibility using current unsaved overlays.
use super::{complete, item, start};
use crate::server::session::{HEADER, Project, Session, uri};
use serde_json::{Value, json};
use std::path::Path;
/// Remove the first | marker and compute body-relative cursor coordinates; no side effects.
///
/// # Arguments
/// - `source` `[in]`: fixture source containing a | marker; later markers are retained.
///
/// # Returns
/// A tuple of (owned marker-free source, zero-based body line, UTF-16 column on that line).
///
/// # Panics
/// Panics if no | marker is present.
///
/// # Examples
/// ```text
/// let (text, line, column) = cursor(r#"message("😀 ${MY|}")"#);
/// ```
fn cursor(source: &str) -> (String, usize, usize) {
    let offset = source.find('|').unwrap();
    let before = &source[..offset];
    let line = before.bytes().filter(|byte| *byte == b'\n').count();
    let column = before.rsplit('\n').next().unwrap().encode_utf16().count();
    (source.replacen('|', "", 1), line, column)
}
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
    let (text, line, column) = cursor(marked);
    client.send(json!({"jsonrpc":"2.0","method":"textDocument/didChange","params":{"textDocument":{"uri":uri(path),"version":version},"contentChanges":[{"text":format!("{HEADER}{text}")}]}})).await;
    complete(client, path, line, column, id).await
}
/// Check whether any completion item has an exact case-sensitive label; no side effects.
///
/// # Arguments
/// - `items` `[in]`: completion result array, borrowed without mutation.
/// - `name` `[in]`: label spelling to compare.
///
/// # Returns
/// true if at least one label matches; false otherwise.
///
/// # Panics
/// Panics if items is not a JSON array.
///
/// # Examples
/// ```text
/// let present = has(&items, "MY_FIRST");
/// ```
fn has(items: &Value, name: &str) -> bool {
    items
        .as_array()
        .unwrap()
        .iter()
        .any(|item| item["label"] == name)
}

#[tokio::test]
/// Check variable roles and ordinary expansions offer names with case-sensitive,
/// wrapper-preserving edits.
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
/// Enable reserve_arg and query set/unset/list/output-variable slots plus complete/incomplete
/// expansions. Require MY_FIRST/MY_SECOND but not my_lower, kind 6, plain text and no snippet
/// placeholders; inspect the emoji-prefixed replacement range. Value slots,
/// bracket/escaped/environment references, comments and unknown splitting yield no items.
/// Native CMAKE_CURRENT_SOURCE_DIR still completes.
///
/// # Flow
/// ```text
/// [Temporary sources + settings]      [Expected: case-sensitive Variable edits]
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
/// schema_variable_slots_and_expansions_offer_case_sensitive_names_not_values(); // #[tokio::test] supplies the runtime
/// ```
async fn schema_variable_slots_and_expansions_offer_case_sensitive_names_not_values() {
    let p = Project::new().await;
    tokio::fs::write(
        p.0.join(".cmake-analyzer.toml"),
        "[completion]\nreserve_arg=true\n",
    )
    .await
    .unwrap();
    let base = "set(MY_FIRST yes)\nset(MY_SECOND yes)\nset(my_lower yes)\n";
    let root = p.write("CMakeLists.txt", base).await;
    let mut client = start(&p, true).await;
    client.open(&root, base).await;
    client.diagnostics(&root, 1).await;
    for (index, call) in [
        "set(MY|)",
        "unset(MY|)",
        "list(APPEND MY|)",
        "file(READ input MY|)",
        "message(\"😀 ${MY|_SUFFIX}\")",
        "message(\"${MY|",
        "set(\"MY|\" yes)",
        "unset([=[MY|]=])",
    ]
    .iter()
    .enumerate()
    {
        let matches = changed(
            &mut client,
            &root,
            &format!("{base}{call}"),
            index as i32 + 2,
            index as i32 + 10,
        )
        .await;
        assert!(has(&matches, "MY_FIRST"), "{call}: {matches}");
        assert!(has(&matches, "MY_SECOND"));
        assert!(!has(&matches, "my_lower"));
        for candidate in matches.as_array().unwrap() {
            assert_eq!(candidate["kind"], 6);
            assert_eq!(candidate["insertTextFormat"], 1);
            assert!(
                !candidate["textEdit"]["newText"]
                    .as_str()
                    .unwrap()
                    .contains("${1:")
            );
        }
        if index == 4 {
            let edit = &item(&matches, "MY_FIRST")["textEdit"];
            assert_eq!(
                edit["range"],
                json!({"start":{"line":4,"character":14},"end":{"line":4,"character":23}})
            );
        }
    }
    for (index, call) in [
        "set(NAME MY|)",
        "file(WRITE path MY|)",
        "message(MY|)",
        "message([=[${MY|}]=])",
        "message(\"\\${MY|}\")",
        "message(\"$ENV{MY|}\")",
        "# ${MY|}",
        "list(APPEND ${UNKNOWN} MY|)",
    ]
    .iter()
    .enumerate()
    {
        let matches = changed(
            &mut client,
            &root,
            &format!("{base}{call}"),
            index as i32 + 20,
            index as i32 + 30,
        )
        .await;
        assert!(matches.as_array().unwrap().is_empty(), "{call}: {matches}");
    }
    let native = changed(
        &mut client,
        &root,
        &format!("{base}message(\"${{CMAKE_CURRENT_SOURCE_D|}}\")"),
        40,
        50,
    )
    .await;
    assert!(has(&native, "CMAKE_CURRENT_SOURCE_DIR"));
    client.finish().await;
    p.remove().await;
}
#[tokio::test]
/// Check literal target/alias completion kinds, delimiter ranges and executable/library role
/// filtering.
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
/// Declare library, executable, custom target and alias. Require all in general target slots as
/// plain EnumMember 20, with exact quoted payload range. Library aliases exclude
/// executable/custom targets and executable aliases exclude libraries. Known APP expansion
/// preserves roles, PUBLIC is keyword-only, and replacing the overlay removes old target names.
///
/// # Flow
/// ```text
/// [Temporary sources + settings]      [Expected: typed targets + alias filtering]
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
/// schema_targets_use_enum_member_and_keep_alias_names_quotes_and_typed_roles(); // #[tokio::test] supplies the runtime
/// ```
async fn schema_targets_use_enum_member_and_keep_alias_names_quotes_and_typed_roles() {
    let p = Project::new().await;
    let base = "add_library(MyLib STATIC source.c)\nadd_executable(MyApp main.c)\nadd_custom_target(MyCustom)\nadd_library(My::Alias ALIAS MyLib)\nset(MY_VARIABLE yes)\n";
    let root = p.write("CMakeLists.txt", base).await;
    let mut client = start(&p, true).await;
    client.open(&root, base).await;
    client.diagnostics(&root, 1).await;
    for (index, call) in [
        "get_target_property(OUT My| TYPE)",
        "target_link_libraries(MyApp PUBLIC My|)",
        "add_dependencies(MyApp My|)",
        "get_target_property(OUT \"My|Lib\" TYPE)",
        "get_target_property(OUT \"My|",
    ]
    .iter()
    .enumerate()
    {
        let matches = changed(
            &mut client,
            &root,
            &format!("{base}{call}"),
            index as i32 + 2,
            index as i32 + 10,
        )
        .await;
        for name in ["MyLib", "MyApp", "MyCustom", "My::Alias"] {
            assert!(has(&matches, name), "{call}: {matches}");
        }
        for candidate in matches.as_array().unwrap() {
            assert_eq!(candidate["kind"], 20);
            assert_eq!(candidate["insertTextFormat"], 1);
            assert_eq!(candidate["textEdit"]["newText"], candidate["label"]);
        }
        if index == 3 {
            assert_eq!(
                item(&matches, "MyLib")["textEdit"]["range"],
                json!({"start":{"line":6,"character":25},"end":{"line":6,"character":30}})
            );
        }
    }
    let libraries = changed(
        &mut client,
        &root,
        &format!("{base}add_library(NewAlias ALIAS My|)"),
        10,
        20,
    )
    .await;
    assert!(has(&libraries, "MyLib"));
    assert!(has(&libraries, "My::Alias"));
    assert!(!has(&libraries, "MyApp"));
    assert!(!has(&libraries, "MyCustom"));
    let executables = changed(
        &mut client,
        &root,
        &format!("{base}add_executable(NewAlias ALIAS My|)"),
        11,
        21,
    )
    .await;
    assert!(has(&executables, "MyApp"));
    assert!(!has(&executables, "MyLib"));
    let expanded = changed(
        &mut client,
        &root,
        &format!("{base}set(APP MyApp)\ntarget_link_libraries(${{APP}} PUBLIC My|)"),
        12,
        24,
    )
    .await;
    assert!(has(&expanded, "MyLib"), "{expanded}");
    let keyword = changed(
        &mut client,
        &root,
        &format!("{base}target_link_libraries(MyApp PUBLIC|)"),
        13,
        22,
    )
    .await;
    assert_eq!(keyword.as_array().unwrap().len(), 1);
    assert_eq!(item(&keyword, "PUBLIC")["kind"], 14);
    let renamed = changed(
        &mut client,
        &root,
        "add_library(NewLib INTERFACE)\nget_target_property(OUT N| TYPE)",
        14,
        23,
    )
    .await;
    assert!(has(&renamed, "NewLib"));
    assert!(!has(&renamed, "MyLib"));
    client.finish().await;
    p.remove().await;
}
#[tokio::test]
/// Check variable/target completion respects includes, directory ancestry and function/macro
/// formals.
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
/// Root sees SHARED_MODULE/SHARED_ROOT but not CHILD_ONLY; ordinary ChildLib is visible while
/// ChildImported is not. Current function locals/formals and macro formals complete, unrelated
/// function locals/formals do not. Opening the child exposes inherited shared root/module
/// variables.
///
/// # Flow
/// ```text
/// [Temporary sources + settings]      [Expected: scope-qualified completion names]
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
/// imports_and_parent_directory_variables_are_available_without_leaking_child_or_function_locals(); // #[tokio::test] supplies the runtime
/// ```
async fn imports_and_parent_directory_variables_are_available_without_leaking_child_or_function_locals()
 {
    let p = Project::new().await;
    p.write("cmake/Vars.cmake","set(SHARED_MODULE yes)\nfunction(other HIDDEN_PARAM)\nset(PRIVATE_OTHER yes)\nendfunction()\n").await;
    let child=p.write("child/CMakeLists.txt","set(CHILD_ONLY yes)\nadd_library(ChildLib INTERFACE)\nadd_library(ChildImported STATIC IMPORTED)\n").await;
    let base = "set(SHARED_ROOT yes)\ninclude(Vars)\nadd_subdirectory(child)\n";
    let root = p.write("CMakeLists.txt", base).await;
    let mut client = start(&p, true).await;
    client.open(&root, base).await;
    client.diagnostics(&root, 1).await;
    let shared = changed(
        &mut client,
        &root,
        &format!("{base}message(${{SHARED_|}})"),
        2,
        10,
    )
    .await;
    assert!(has(&shared, "SHARED_MODULE"));
    assert!(has(&shared, "SHARED_ROOT"));
    let hidden = changed(
        &mut client,
        &root,
        &format!("{base}message(${{CHILD_|}})"),
        3,
        11,
    )
    .await;
    assert!(!has(&hidden, "CHILD_ONLY"));
    let targets = changed(
        &mut client,
        &root,
        &format!("{base}target_link_libraries(ChildLib Child|)"),
        4,
        12,
    )
    .await;
    assert!(has(&targets, "ChildLib"));
    assert!(!has(&targets, "ChildImported"));
    let local = "function(current VISIBLE_PARAM)\nset(PRIVATE_CURRENT yes)\nmessage(${PRIVATE_|})\nendfunction()\n";
    let private = changed(&mut client, &root, &format!("{base}{local}"), 5, 13).await;
    assert!(has(&private, "PRIVATE_CURRENT"));
    assert!(!has(&private, "PRIVATE_OTHER"));
    let formal = changed(
        &mut client,
        &root,
        &format!("{base}{}", local.replace("PRIVATE_|", "VISIBLE_|")),
        6,
        14,
    )
    .await;
    assert!(has(&formal, "VISIBLE_PARAM"));
    assert!(!has(&formal, "HIDDEN_PARAM"));
    let macro_parameter = changed(
        &mut client,
        &root,
        &format!("{base}macro(local_macro MAC_PARAM)\nmessage(${{MAC_|}})\nendmacro()\n"),
        7,
        15,
    )
    .await;
    assert!(has(&macro_parameter, "MAC_PARAM"));
    client.open(&child,"set(CHILD_ONLY yes)\nadd_library(ChildLib INTERFACE)\nadd_library(ChildImported STATIC IMPORTED)\n").await;
    client.diagnostics(&child, 1).await;
    let inherited = changed(
        &mut client,
        &child,
        "set(CHILD_ONLY yes)\nmessage(${SHARED_|})",
        2,
        20,
    )
    .await;
    assert!(has(&inherited, "SHARED_ROOT"));
    assert!(has(&inherited, "SHARED_MODULE"));
    client.finish().await;
    p.remove().await;
}
#[tokio::test]
/// Check shadowed native list has no builtin variable slots while ordinary expansions still
/// complete.
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
/// Declare user list and MY_VARIABLE in a standalone file. Bare list(APPEND MY...) yields []; a
/// ${MY...} argument still offers MY_VARIABLE. Replace the overlay with NEW_VARIABLE and
/// require only the new name.
///
/// # Flow
/// ```text
/// [Temporary sources + settings]      [Expected: shadow suppression / fresh names]
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
/// user_shadows_do_not_inherit_native_parameter_roles_but_expansions_still_work(); // #[tokio::test] supplies the runtime
/// ```
async fn user_shadows_do_not_inherit_native_parameter_roles_but_expansions_still_work() {
    let p = Project::new().await;
    let base = "set(MY_VARIABLE yes)\nfunction(list input)\nendfunction()\n";
    let root = p.write("loose.cmake", base).await;
    let mut client = start(&p, true).await;
    client.open(&root, base).await;
    client.diagnostics(&root, 1).await;
    let bare = changed(
        &mut client,
        &root,
        &format!("{base}list(APPEND MY|)"),
        2,
        10,
    )
    .await;
    assert!(bare.as_array().unwrap().is_empty());
    let expansion = changed(&mut client, &root, &format!("{base}list(${{MY|}})"), 3, 11).await;
    assert!(has(&expansion, "MY_VARIABLE"));
    let renamed = changed(
        &mut client,
        &root,
        "set(NEW_VARIABLE yes)\nmessage(${NEW_|})",
        4,
        12,
    )
    .await;
    assert!(has(&renamed, "NEW_VARIABLE"));
    assert!(!has(&renamed, "MY_VARIABLE"));
    client.finish().await;
    p.remove().await;
}
