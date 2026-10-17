// AI-generated tests.
use super::{complete, item, start};
use crate::server::session::{Project, Session, uri};
use serde_json::{Value, json};
use std::path::Path;
async fn changed(
    client: &mut Session,
    path: &Path,
    marked: &str,
    cmake: &str,
    version: i32,
) -> Value {
    let offset = marked.find('|').unwrap();
    let before = &marked[..offset];
    client.send(json!({"jsonrpc":"2.0","method":"textDocument/didChange","params":{"textDocument":{"uri":uri(path),"version":version},"contentChanges":[{"text":format!("cmake_minimum_required(VERSION {cmake})\n{}",marked.replacen('|',"",1))}]}})).await;
    complete(
        client,
        path,
        before.bytes().filter(|byte| *byte == b'\n').count(),
        before.rsplit('\n').next().unwrap().encode_utf16().count(),
        version + 100,
    )
    .await
}
fn has(items: &Value, name: &str) -> bool {
    items
        .as_array()
        .unwrap()
        .iter()
        .any(|item| item["label"] == name)
}
#[tokio::test]
async fn all_supported_languages_complete_from_typed_slots_and_preserve_delimiters() {
    let p = Project::new().await;
    let root = p.write("CMakeLists.txt", "").await;
    let mut client = start(&p, true).await;
    client.open(&root, "").await;
    client.diagnostics(&root, 1).await;
    let all = changed(&mut client, &root, "project(P LANGUAGES |)", "4.4", 2).await;
    for name in [
        "C",
        "CXX",
        "CSharp",
        "CUDA",
        "OBJC",
        "OBJCXX",
        "Fortran",
        "HIP",
        "ISPC",
        "Swift",
        "ASM",
        "ASM_NASM",
        "ASM_MARMASM",
        "ASM_MASM",
        "ASM_POASM",
        "ASM-ATT",
        "NONE",
    ] {
        assert_eq!(item(&all, name)["kind"], 20, "{all}");
        assert_eq!(item(&all, name)["insertTextFormat"], 1);
        assert_eq!(item(&all, name)["textEdit"]["newText"], name);
    }
    for (index, source) in [
        "project(P C|)",
        "project(P LANGUAGES C C|)",
        "project(P LANGUAGES \"C|XX\")",
        "project(P LANGUAGES [=[C|XX]=])",
        "project(P LANGUAGES \"C|",
        "enable_language(C| OPTIONAL)",
    ]
    .iter()
    .enumerate()
    {
        let result = changed(&mut client, &root, source, "4.4", index as i32 + 3).await;
        assert_eq!(
            item(&result, "CXX")["textEdit"]["newText"],
            "CXX",
            "{source}: {result}"
        );
        let clean = source.replacen('|', "", 1);
        let start = source[..source.find('|').unwrap()].rfind('C').unwrap();
        let length = if clean[start..].starts_with("CXX") {
            3
        } else {
            1
        };
        assert_eq!(
            item(&result, "CXX")["textEdit"]["range"],
            json!({"start":{"line":1,"character":start},"end":{"line":1,"character":start+length}}),
            "{source}: {result}"
        );
    }
    let enabled = changed(&mut client, &root, "enable_language(|)", "4.4", 20).await;
    assert!(has(&enabled, "ASM_POASM"));
    assert!(!has(&enabled, "NONE"));
    for (index, source) in [
        "project(P VERSION |)",
        "project(|)",
        "message(|)",
        "project(P LANGUAGES ${C|})",
        "function(project X)\nendfunction()\nproject(P LANGUAGES |)",
    ]
    .iter()
    .enumerate()
    {
        let result = changed(&mut client, &root, source, "4.4", index as i32 + 21).await;
        assert!(!has(&result, "CXX"), "{source}: {result}");
    }
    client.finish().await;
    p.remove().await;
}
#[tokio::test]
async fn language_names_follow_cmake_introduction_versions() {
    let p = Project::new().await;
    let root = p.write("CMakeLists.txt", "").await;
    let mut client = start(&p, false).await;
    client.open(&root, "").await;
    client.diagnostics(&root, 1).await;
    for (index, (before, since, name)) in [
        ("3.7", "3.8", "CSharp"),
        ("3.7", "3.8", "CUDA"),
        ("3.14", "3.15", "Swift"),
        ("3.15", "3.16", "OBJC"),
        ("3.15", "3.16", "OBJCXX"),
        ("3.17", "3.18", "ISPC"),
        ("3.20", "3.21", "HIP"),
        ("3.25", "3.26", "ASM_MARMASM"),
        ("4.3", "4.4", "ASM_POASM"),
    ]
    .iter()
    .enumerate()
    {
        let old = changed(
            &mut client,
            &root,
            "project(P LANGUAGES |)",
            before,
            index as i32 * 2 + 2,
        )
        .await;
        assert!(!has(&old, name), "{before}: {old}");
        let new = changed(
            &mut client,
            &root,
            "project(P LANGUAGES |)",
            since,
            index as i32 * 2 + 3,
        )
        .await;
        assert!(has(&new, name), "{since}: {new}");
    }
    client.finish().await;
    p.remove().await;
}
