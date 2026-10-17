// AI-generated tests.
//! Module/package catalogue and imported-target completion from local fixtures.
//! Search paths, indexing modes and directory visibility are checked without running sources.
//! Tests close sessions and recursively remove fixture trees.
use super::{complete, item};
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
/// - `version` `[in]`: didChange document version; request ID is version + 100.
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
/// let items = changed(&mut client, &root, "include(Loc|)", 2).await;
/// ```
async fn changed(client: &mut Session, path: &Path, marked: &str, version: i32) -> Value {
    let offset = marked.find('|').unwrap();
    let before = &marked[..offset];
    client.send(json!({"jsonrpc":"2.0","method":"textDocument/didChange","params":{"textDocument":{"uri":uri(path),"version":version},"contentChanges":[{"text":format!("{HEADER}{}",marked.replacen('|',"",1))}]}})).await;
    complete(
        client,
        path,
        before.bytes().filter(|byte| *byte == b'\n').count(),
        before.rsplit('\n').next().unwrap().encode_utf16().count(),
        version + 100,
    )
    .await
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
/// let present = has(&items, "LocalModule");
/// ```
fn has(items: &Value, name: &str) -> bool {
    items
        .as_array()
        .unwrap()
        .iter()
        .any(|item| item["label"] == name)
}
/// Overwrite app/.cmake-analyzer.toml with isolated module, builtin and package search paths.
/// Does not reload the running server; callers invoke reload separately.
///
/// # Arguments
/// - `p` `[in]`: fixture containing the existing app directory.
/// - `extra` `[in]`: TOML appended within `[modules]`, normally indexing modes or enable=false.
///
/// # Returns
/// `()` after the configuration write; no output parameter.
///
/// # Panics
/// Panics if the configuration write fails.
///
/// # Examples
/// ```text
/// config(&p, "external_include=\"functions\"\n").await;
/// ```
async fn config(p: &Project, extra: &str) {
    tokio::fs::write(p.0.join("app/.cmake-analyzer.toml"),format!("[modules]\nsearch_paths=[\"../modules\"]\nbuiltin_directory=\"../builtin\"\npackage_search_paths=[\"../prefix\"]\n{extra}")).await.unwrap();
}
/// Reload the current configuration sources and require a successful RPC response.
///
/// # Arguments
/// - `client` `[in,out]`: writes the request and consumes its reply; unrelated messages append
///   to pending.
/// - `id` `[in]`: request/response correlation ID.
///
/// # Returns
/// `()` after a reply without error; no output parameter.
///
/// # Panics
/// Panics on transport/deadline failure or a response error.
///
/// # Examples
/// ```text
/// reload(&mut client, 300).await;
/// ```
async fn reload(client: &mut Session, id: i32) {
    client
        .send(json!({"jsonrpc":"2.0","id":id,"method":"cmake-analyzer/reloadConfig","params":null}))
        .await;
    let result = client.response(id).await;
    assert!(result.get("error").is_none(), "{result}");
}
#[tokio::test]
/// Check discovered module/package names appear only in schema slots and replace payloads, not
/// delimiters.
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
/// Populate module, builtin, FindDemo and config-package search fixtures. Complete
/// bare/quoted/bracket/incomplete includes and compare exact ranges. Package slots offer
/// Demo/ConfigPkg/other but not finder prefixes or version-only packages. Wrong roles, comments
/// and user shadows exclude modules; expansions offer variables instead. Disable modules by
/// reload and require no module/package names.
///
/// # Flow
/// ```text
/// [Temporary sources + settings]      [Expected: catalogue items + wrapper ranges]
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
/// schema_module_and_package_slots_use_local_catalogues_and_preserve_wrappers(); // #[tokio::test] supplies the runtime
/// ```
async fn schema_module_and_package_slots_use_local_catalogues_and_preserve_wrappers() {
    let p = Project::new().await;
    let root = p.write("app/CMakeLists.txt", "").await;
    p.write("modules/LocalModule.cmake", "").await;
    p.write("builtin/LocalBuiltin.cmake", "").await;
    p.write("modules/FindDemo.cmake", "").await;
    p.write("prefix/lib/cmake/ConfigPkg/ConfigPkgConfig.cmake", "")
        .await;
    p.write("prefix/share/Other/cmake/other-config.cmake", "")
        .await;
    p.write("prefix/share/Version/PhantomConfigVersion.cmake", "")
        .await;
    config(&p, "").await;
    let mut client = Session::start(&p.0.join("app"), vec![], true).await;
    client.open(&root, "").await;
    client.diagnostics(&root, 1).await;
    for (index, source) in [
        "include(Loc|)",
        "include(\"Loc|alModule\")",
        "include([=[Loc|alModule]=])",
        "include(\"Loc|",
        "include(Loc|",
    ]
    .into_iter()
    .enumerate()
    {
        let result = changed(&mut client, &root, source, index as i32 + 2).await;
        let entry = item(&result, "LocalModule");
        assert_eq!(entry["kind"], 9);
        assert_eq!(entry["insertTextFormat"], 1);
        assert_eq!(entry["textEdit"]["newText"], "LocalModule");
        if index == 0 {
            assert!(has(&result, "LocalBuiltin"));
        }
        let start = source.find("Loc").unwrap();
        let length = if source.contains("alModule") { 11 } else { 3 };
        assert_eq!(
            entry["textEdit"]["range"],
            json!({"start":{"line":1,"character":start},"end":{"line":1,"character":start+length}}),
            "{source}: {result}"
        );
    }
    let packages = changed(&mut client, &root, "find_package(|)", 10).await;
    for name in ["Demo", "ConfigPkg", "other"] {
        assert_eq!(item(&packages, name)["kind"], 9, "{packages}");
    }
    assert!(!has(&packages, "FindDemo"));
    assert!(!has(&packages, "Phantom"));
    let find_module = changed(&mut client, &root, "include(FindD|)", 11).await;
    assert!(has(&find_module, "FindDemo"));
    for (index, source) in [
        "message(Loc|)",
        "include(LocalModule OPTIONAL Loc|)",
        "find_package(Demo COMPONENTS Loc|)",
        "# include(Loc|)",
        "function(include X)\nendfunction()\ninclude(Loc|)",
    ]
    .into_iter()
    .enumerate()
    {
        let result = changed(&mut client, &root, source, index as i32 + 12).await;
        assert!(!has(&result, "LocalModule"), "{source}: {result}");
    }
    let variable = changed(
        &mut client,
        &root,
        "set(LocalVariable yes)\ninclude(\"${Loc|}\")",
        20,
    )
    .await;
    assert!(has(&variable, "LocalVariable"));
    assert!(!has(&variable, "LocalModule"));
    config(&p, "enable=false\n").await;
    reload(&mut client, 300).await;
    assert!(!has(
        &changed(&mut client, &root, "include(Loc|)", 21).await,
        "LocalModule"
    ));
    assert!(!has(
        &changed(&mut client, &root, "find_package(D|)", 22).await,
        "Demo"
    ));
    client.finish().await;
    p.remove().await;
}
#[tokio::test]
/// Check local module/prefix/package-directory values and reload discover names without running
/// source.
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
/// Use CMAKE_MODULE_PATH, CMAKE_PREFIX_PATH and HintPkg_DIR overlays to find
/// HintModule/HintPkg. Add NewModule on disk, reload the catalogue and require its completion.
/// A marker-writing module must never create app/marker.txt.
///
/// # Flow
/// ```text
/// [Temporary sources + settings]      [Expected: hint paths + reloaded catalogue]
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
/// local_search_values_and_catalogue_reload_find_new_packages_without_execution(); // #[tokio::test] supplies the runtime
/// ```
async fn local_search_values_and_catalogue_reload_find_new_packages_without_execution() {
    let p = Project::new().await;
    let root = p.write("app/CMakeLists.txt", "").await;
    config(&p, "").await;
    p.write(
        "custom/HintModule.cmake",
        "file(WRITE marker.txt forbidden)\n",
    )
    .await;
    p.write("hidden/HintPkgConfig.cmake", "").await;
    let mut client = Session::start(&p.0.join("app"), vec![], true).await;
    client.open(&root, "").await;
    client.diagnostics(&root, 1).await;
    assert!(has(
        &changed(
            &mut client,
            &root,
            "set(CMAKE_MODULE_PATH \"${CMAKE_CURRENT_SOURCE_DIR}/../custom\")\ninclude(Hint|)",
            2
        )
        .await,
        "HintModule"
    ));
    assert!(has(
        &changed(
            &mut client,
            &root,
            "set(CMAKE_PREFIX_PATH \"${CMAKE_CURRENT_SOURCE_DIR}/../hidden\")\nfind_package(Hint|)",
            3
        )
        .await,
        "HintPkg"
    ));
    assert!(has(
        &changed(
            &mut client,
            &root,
            "set(HintPkg_DIR \"${CMAKE_CURRENT_SOURCE_DIR}/../hidden\")\nfind_package(Hint|)",
            4
        )
        .await,
        "HintPkg"
    ));
    p.write("modules/NewModule.cmake", "").await;
    reload(&mut client, 300).await;
    assert!(has(
        &changed(&mut client, &root, "include(New|)", 5).await,
        "NewModule"
    ));
    assert!(!p.0.join("app/marker.txt").exists());
    client.finish().await;
    p.remove().await;
}
#[tokio::test]
/// Check repeated package snapshots retain per-invocation imported-target visibility.
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
/// A package includes Targets.cmake inside a condition and defines private/global imported
/// targets. Import it in root and child: root completion sees both. Replace root with
/// child-only import: private disappears from root completion while the global target remains.
///
/// # Flow
/// ```text
/// [Temporary sources + settings]      [Expected: private / global target visibility]
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
/// repeated_package_sources_preserve_imported_directory_visibility(); // #[tokio::test] supplies the runtime
/// ```
async fn repeated_package_sources_preserve_imported_directory_visibility() {
    let p = Project::new().await;
    let root = p.write("app/CMakeLists.txt", "").await;
    p.write(
        "app/child/CMakeLists.txt",
        "find_package(Scoped CONFIG REQUIRED)\n",
    )
    .await;
    p.write(
        "prefix/lib/cmake/Scoped/ScopedConfig.cmake",
        "if(TRUE)\ninclude(\"${CMAKE_CURRENT_LIST_DIR}/Targets.cmake\")\nendif()\n",
    )
    .await;
    p.write("prefix/lib/cmake/Scoped/Targets.cmake","add_library(Scoped::Private INTERFACE IMPORTED)\nadd_library(Scoped::Global INTERFACE IMPORTED GLOBAL)\n").await;
    config(&p, "").await;
    let mut client = Session::start(&p.0.join("app"), vec![], true).await;
    client.open(&root, "").await;
    client.diagnostics(&root, 1).await;
    let both=changed(&mut client,&root,"add_executable(App main.cpp)\nfind_package(Scoped CONFIG REQUIRED)\nadd_subdirectory(child)\ntarget_link_libraries(App Scoped::|)",2).await;
    assert!(has(&both, "Scoped::Private"), "{both}");
    assert!(has(&both, "Scoped::Global"));
    let child_only=changed(&mut client,&root,"add_executable(App main.cpp)\nadd_subdirectory(child)\ntarget_link_libraries(App Scoped::|)",3).await;
    assert!(!has(&child_only, "Scoped::Private"), "{child_only}");
    assert!(has(&child_only, "Scoped::Global"));
    client.finish().await;
    p.remove().await;
}
#[tokio::test]
/// Check package target completion follows literal conditional/cyclic includes and indexing
/// modes.
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
/// Build ConfigPkg with direct, nested, alias and cyclicly included imported targets plus a
/// forbidden write. Complete names through bare/quoted/bracket target operands as plain
/// EnumMember kind 20. external_include=functions keeps only the direct target;
/// external_find_package=functions/none removes package targets. Require no marker execution.
///
/// # Flow
/// ```text
/// [Temporary sources + settings]      [Expected: package targets / mode exclusions]
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
/// package_library_names_follow_literal_conditional_target_imports_and_index_modes(); // #[tokio::test] supplies the runtime
/// ```
async fn package_library_names_follow_literal_conditional_target_imports_and_index_modes() {
    let p = Project::new().await;
    let root = p.write("app/CMakeLists.txt", "").await;
    p.write("prefix/lib/cmake/ConfigPkg/ConfigPkgConfig.cmake","add_library(ConfigPkg::Direct INTERFACE IMPORTED)\nif(NOT TARGET ConfigPkg::Core)\ninclude(\"${CMAKE_CURRENT_LIST_DIR}/ConfigPkgTargets.cmake\")\nendif()\n").await;
    p.write("prefix/lib/cmake/ConfigPkg/ConfigPkgTargets.cmake","if(NOT TARGET ConfigPkg::Core)\nadd_library(ConfigPkg::Core STATIC IMPORTED)\nadd_library(ConfigPkg::Headers INTERFACE IMPORTED)\nadd_library(ConfigPkg::Alias ALIAS ConfigPkg::Core)\nendif()\ninclude(\"${CMAKE_CURRENT_LIST_DIR}/More.cmake\")\n").await;
    p.write("prefix/lib/cmake/ConfigPkg/More.cmake","add_library(ConfigPkg::More UNKNOWN IMPORTED)\ninclude(\"${CMAKE_CURRENT_LIST_DIR}/ConfigPkgTargets.cmake\")\nfile(WRITE marker.txt forbidden)\n").await;
    config(&p, "").await;
    let mut client = Session::start(&p.0.join("app"), vec![], true).await;
    client.open(&root, "").await;
    client.diagnostics(&root, 1).await;
    let base = "add_executable(App main.cpp)\nfind_package(ConfigPkg CONFIG REQUIRED)\n";
    for (index, argument) in [
        "ConfigPkg::|",
        "\"ConfigPkg::|Core\"",
        "[=[ConfigPkg::|Core]=]",
    ]
    .into_iter()
    .enumerate()
    {
        let result = changed(
            &mut client,
            &root,
            &format!("{base}target_link_libraries(App PRIVATE {argument})"),
            index as i32 + 2,
        )
        .await;
        for name in [
            "ConfigPkg::Direct",
            "ConfigPkg::Core",
            "ConfigPkg::Headers",
            "ConfigPkg::Alias",
            "ConfigPkg::More",
        ] {
            let entry = item(&result, name);
            assert_eq!(entry["kind"], 20);
            assert_eq!(entry["insertTextFormat"], 1);
            assert_eq!(entry["textEdit"]["newText"], name);
        }
    }
    config(&p, "external_include=\"functions\"\n").await;
    reload(&mut client, 300).await;
    let result = changed(
        &mut client,
        &root,
        &format!("{base}target_link_libraries(App PRIVATE ConfigPkg::|)"),
        6,
    )
    .await;
    assert!(has(&result, "ConfigPkg::Direct"));
    assert!(!has(&result, "ConfigPkg::Core"));
    for (index, mode) in ["functions", "none"].into_iter().enumerate() {
        config(&p, &format!("external_find_package=\"{mode}\"\n")).await;
        reload(&mut client, index as i32 + 301).await;
        let result = changed(
            &mut client,
            &root,
            &format!("{base}target_link_libraries(App PRIVATE ConfigPkg::|)"),
            index as i32 + 7,
        )
        .await;
        assert!(!has(&result, "ConfigPkg::Core"));
        assert!(!has(&result, "ConfigPkg::Direct"));
    }
    assert!(!p.0.join("app/marker.txt").exists());
    client.finish().await;
    p.remove().await;
}
