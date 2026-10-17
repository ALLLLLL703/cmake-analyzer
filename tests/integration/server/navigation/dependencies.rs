// AI-generated include/module and subdirectory definition navigation tests.
use super::{definition, references};
use crate::server::session::{HEADER, Project, Session, uri};
use serde_json::json;

#[tokio::test]
async fn unfinished_modules_and_package_entrypoints_navigate_without_execution() {
    let p = Project::new().await;
    let module = p
        .write("cmake/Partial.cmake", "file(WRITE should-not-run yes)\n")
        .await;
    let finder = p
        .write(
            "cmake/FindDemo.cmake",
            "function(package_call)\nendfunction()\n",
        )
        .await;
    let config = p
        .write(
            "prefix/lib/cmake/Demo/DemoConfig.cmake",
            "function(config_call)\nendfunction()\n",
        )
        .await;
    tokio::fs::write(
        p.0.join(".cmake-analyzer.toml"),
        "[modules]\npackage_search_paths=[\"prefix\"]\n",
    )
    .await
    .unwrap();
    let root = p.write("CMakeLists.txt", "").await;
    let mut client = Session::start(&p.0, vec![], true).await;
    client.open(&root, "").await;
    client.diagnostics(&root, 1).await;
    for (index,(body,line,column,target)) in [
        ("include(Partial",0,15,&module),
        ("include(\"Partial",0,10,&module),
        ("find_package(Demo MODULE)",0,15,&finder),
        ("find_package(Demo CONFIG)",0,15,&config),
        ("set(Demo_DIR \"${CMAKE_CURRENT_SOURCE_DIR}/prefix/lib/cmake/Demo\")\nfind_package(Demo",1,16,&finder),
        ("set(Demo_DIR \"${CMAKE_CURRENT_SOURCE_DIR}/prefix/lib/cmake/Demo\")\nfind_package(Demo CONFIG",1,15,&config),
    ].into_iter().enumerate() {
        client.send(json!({"jsonrpc":"2.0","method":"textDocument/didChange","params":{"textDocument":{"uri":uri(&root),"version":index+2},"contentChanges":[{"text":format!("{HEADER}{body}")}]}})).await;
        let result=definition(&mut client,&root,line,column,index as i32+10).await;
        assert_eq!(result["result"][0]["uri"],uri(target),"{body}: {result}");
    }
    assert!(!p.0.join("should-not-run").exists());
    client.finish().await;
    p.remove().await;
}
#[tokio::test]
async fn module_precedence_relative_files_and_subdirectory_source_targets_are_reused() {
    let p = Project::new().await;
    let module = p.write("cmake/Foo.cmake", "set(FOO yes)\n").await;
    p.write("builtin/Foo.cmake", "set(FOO builtin)\n").await;
    let native = p
        .write("builtin/OnlyBuiltin.cmake", "set(BUILTIN yes)\n")
        .await;
    let relative = p.write("relative.cmake", "set(RELATIVE yes)\n").await;
    p.write("图 #.cmake", "set(UNICODE yes)\n").await;
    p.write("child folder/CMakeLists.txt", "set(CHILD yes)\n")
        .await;
    p.write("binary/CMakeLists.txt", "set(WRONG yes)\n").await;
    tokio::fs::write(
        p.0.join(".cmake-analyzer.toml"),
        "[modules]\nbuiltin_directory=\"builtin\"\n",
    )
    .await
    .unwrap();
    let text = "include(Foo)\ninclude(OnlyBuiltin)\ninclude(\"./relative.cmake\" OPTIONAL)\nadd_subdirectory(\"child folder\" binary)\ninclude(./missing.cmake OPTIONAL)\ninclude(\"图 #.cmake\")\n";
    let root = p.write("CMakeLists.txt", text).await;
    let mut client = Session::start(&p.0, vec![], true).await;
    client.open(&root, text).await;
    client.diagnostics(&root, 1).await;
    for (id, line, column, target) in [
        (10, 0, 8, &module),
        (11, 1, 8, &native),
        (12, 2, 11, &relative),
    ] {
        let result = definition(&mut client, &root, line, column, id).await;
        assert_eq!(
            result["result"],
            json!([{"uri":uri(target),"range":{"start":{"line":0,"character":0},"end":{"line":0,"character":0}}}]),
            "{result}"
        );
    }
    let result = definition(&mut client, &root, 3, 20, 13).await;
    assert!(
        result["result"][0]["uri"]
            .as_str()
            .unwrap()
            .ends_with("/child%20folder/CMakeLists.txt"),
        "{result}"
    );
    assert!(definition(&mut client, &root, 3, 33, 14).await["result"].is_null());
    assert!(definition(&mut client, &root, 4, 9, 15).await["result"].is_null());
    assert!(definition(&mut client, &root, 0, 11, 16).await["result"].is_null());
    let unicode = definition(&mut client, &root, 5, 9, 18).await;
    assert!(
        unicode["result"][0]["uri"]
            .as_str()
            .unwrap()
            .ends_with("/%E5%9B%BE%20%23.cmake"),
        "{unicode}"
    );
    assert!(
        references(&mut client, &root, 0, 8, true, 17).await["result"]
            .as_array()
            .unwrap()
            .is_empty()
    );
    client.finish().await;
    p.remove().await;
}
#[tokio::test]
async fn conditional_deferred_and_standalone_literals_resolve_without_executing_them() {
    let p = Project::new().await;
    let module = p
        .write("module.cmake", "file(WRITE should-not-run marker)\n")
        .await;
    let child = p
        .write(
            "child/CMakeLists.txt",
            "file(WRITE child-should-not-run marker)\n",
        )
        .await;
    let text = "if(FALSE)\ninclude([=[./module.cmake]=])\nendif()\nfunction(load)\ninclude(\"./module.cmake\")\nendfunction()\nadd_subdirectory(child)\nfunction(include)\nendfunction()\ninclude(\"./module.cmake\")\n";
    let loose = p.write("loose.cmake", text).await;
    let mut client = Session::start(&p.0, vec![], true).await;
    client.open(&loose, text).await;
    client.diagnostics(&loose, 1).await;
    for (id, line, column, target) in [
        (10, 1, 13, &module),
        (11, 4, 12, &module),
        (12, 6, 18, &child),
    ] {
        let result = definition(&mut client, &loose, line, column, id).await;
        assert_eq!(result["result"][0]["uri"], uri(target), "{result}");
    }
    assert!(definition(&mut client, &loose, 9, 12, 13).await["result"].is_null());
    assert_eq!(
        definition(&mut client, &loose, 9, 0, 14).await["result"][0]["range"]["start"],
        json!({"line":8,"character":9})
    );
    assert!(!p.0.join("should-not-run").exists());
    assert!(!p.0.join("child-should-not-run").exists());
    client.finish().await;
    let root = p.write("CMakeLists.txt", text).await;
    let mut client = Session::start(&p.0, vec![], true).await;
    client.open(&root, text).await;
    client.diagnostics(&root, 1).await;
    for (id, line) in [(20, 1), (21, 4)] {
        let result = definition(&mut client, &root, line, 13, id).await;
        assert_eq!(result["result"][0]["uri"], uri(&module), "{result}");
    }
    client.finish().await;
    p.remove().await;
}
#[tokio::test]
async fn expanded_path_suffixes_jump_to_files_while_variable_names_keep_their_definitions() {
    let p = Project::new().await;
    let first = p.write("first.cmake", "set(A first)\n").await;
    let second = p.write("second.cmake", "set(A second)\n").await;
    let text = "set(BASE \"${CMAKE_CURRENT_SOURCE_DIR}\")\ninclude(\"${BASE}/first.cmake\")\n";
    let root = p.write("CMakeLists.txt", text).await;
    let mut client = Session::start(&p.0, vec![], true).await;
    client.open(&root, text).await;
    client.diagnostics(&root, 1).await;
    let result = definition(&mut client, &root, 1, 20, 10).await;
    assert_eq!(result["result"][0]["uri"], uri(&first), "{result}");
    let variable = definition(&mut client, &root, 1, 12, 11).await;
    assert_eq!(variable["result"][0]["uri"], uri(&root));
    assert_eq!(
        variable["result"][0]["range"]["start"],
        json!({"line":1,"character":4})
    );
    client.send(json!({"jsonrpc":"2.0","method":"textDocument/didChange","params":{"textDocument":{"uri":uri(&root),"version":2},"contentChanges":[{"text":format!("{HEADER}{}",text.replace("first.cmake","second.cmake"))}]}})).await;
    assert_eq!(
        definition(&mut client, &root, 1, 20, 12).await["result"][0]["uri"],
        uri(&second)
    );
    client.finish().await;
    p.remove().await;
}
#[tokio::test]
async fn shared_callers_keep_directory_context_and_open_unsaved_targets_are_valid() {
    let p = Project::new().await;
    let shared = p.write("shared.cmake", "include(./local.cmake)\n").await;
    let a = p.write("a/local.cmake", "set(A yes)\n").await;
    let b = p.write("b/local.cmake", "set(B yes)\n").await;
    let root_a = p
        .write("a/CMakeLists.txt", "include(../shared.cmake)\n")
        .await;
    let root_b = p
        .write("b/CMakeLists.txt", "include(../shared.cmake)\n")
        .await;
    let folders = vec![
        json!({"uri":uri(root_a.parent().unwrap()),"name":"a"}),
        json!({"uri":uri(root_b.parent().unwrap()),"name":"b"}),
    ];
    let mut client = Session::start(&p.0, folders, true).await;
    client.open(&shared, "include(./local.cmake)\n").await;
    client.diagnostics(&shared, 1).await;
    let result = definition(&mut client, &shared, 0, 11, 10).await;
    let locations = result["result"].as_array().unwrap();
    assert_eq!(locations.len(), 2, "{result}");
    for target in [&a, &b] {
        assert!(
            locations
                .iter()
                .any(|location| location["uri"] == uri(target)),
            "{result}"
        );
    }
    client.finish().await;
    let root = p
        .write("CMakeLists.txt", "include(./unsaved.cmake)\n")
        .await;
    let target = p.0.join("unsaved.cmake");
    let mut client = Session::start(&p.0, vec![], true).await;
    client.open(&root, "include(./unsaved.cmake)\n").await;
    client.diagnostics(&root, 1).await;
    client.open(&target, "set(NEW yes)\n").await;
    client.diagnostics(&target, 1).await;
    assert_eq!(
        definition(&mut client, &root, 0, 11, 20).await["result"][0]["uri"],
        uri(&target)
    );
    assert!(!target.exists());
    client.finish().await;
    p.remove().await;
}
