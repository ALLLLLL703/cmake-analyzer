// AI-generated tests.
use super::{complete, item, start};
use crate::server::session::{Project, uri};
use serde_json::json;

#[tokio::test]
async fn previously_missing_builtin_keywords_work_through_the_lsp_and_version_changes() {
    let p = Project::new().await;
    let root = p.write("CMakeLists.txt", "").await;
    let mut client = start(&p, true).await;
    client.open(&root, "").await;
    client.diagnostics(&root, 1).await;
    for (index, (version, marked, label)) in [
        ("3.0", "find_package(Qt RE|QUIRED)", "REQUIRED"),
        ("3.0", "project(Core LA|NGUAGES)", "LANGUAGES"),
        ("3.9", "project(Core DE|SCRIPTION)", "DESCRIPTION"),
        ("3.12", "project(Core HO|MEPAGE_URL)", "HOMEPAGE_URL"),
        (
            "3.31",
            "file(GLOB OUT CON|FIGURE_DEPENDS)",
            "CONFIGURE_DEPENDS",
        ),
        (
            "3.31",
            "execute_process(COMMAND tool COMMAND_ECHO ST|DOUT)",
            "STDOUT",
        ),
        (
            "3.31",
            "install(TARGETS App RUNTIME DES|TINATION bin)",
            "DESTINATION",
        ),
        (
            "3.31",
            "get_property(OUT TARGET App PRO|PERTY TYPE)",
            "PROPERTY",
        ),
        ("4.1", "project(Core CO|MPAT_VERSION)", "COMPAT_VERSION"),
        ("4.2", "set(CACHE{ENTRY} TY|PE STRING VALUE x)", "TYPE"),
        ("4.4", "block(SCOPE_FOR DIA|GNOSTICS)", "DIAGNOSTICS"),
        ("4.4", "if(DIA|GNOSTIC CMD_AUTHOR)", "DIAGNOSTIC"),
        (
            "4.4",
            "include(Module NO_DIA|GNOSTIC_SCOPE)",
            "NO_DIAGNOSTIC_SCOPE",
        ),
    ]
    .iter()
    .enumerate()
    {
        let offset = marked.find('|').unwrap();
        let character = marked[..offset].encode_utf16().count();
        let source = marked.replacen('|', "", 1);
        client.send(json!({"jsonrpc":"2.0","method":"textDocument/didChange","params":{"textDocument":{"uri":uri(&root),"version":index as i32+2},"contentChanges":[{"text":format!("cmake_minimum_required(VERSION {version})\n{source}\n")}]}})).await;
        let result = complete(&mut client, &root, 0, character, index as i32 + 10).await;
        let candidate = item(&result, label);
        assert_eq!(candidate["kind"], 14, "{version} {source}: {result}");
        assert_eq!(candidate["textEdit"]["newText"], *label);
        assert_eq!(candidate["insertTextFormat"], 1);
    }
    client.finish().await;
    p.remove().await;
}
