// AI-generated Workspace identity and standalone macro navigation tests.
use super::{definition, references};
use crate::server::session::{Project, Session, uri};
use serde_json::json;
#[tokio::test]
async fn shared_source_resolves_each_workspace_without_mixing_file_ids() {
    let p = Project::new().await;
    let shared = p.write("shared.cmake", "message(\"${V}\")\n").await;
    let a = p
        .write("a/CMakeLists.txt", "set(V a)\ninclude(../shared.cmake)\n")
        .await;
    let b = p
        .write("b/CMakeLists.txt", "set(V b)\ninclude(../shared.cmake)\n")
        .await;
    let folders = vec![
        json!({"uri":uri(a.parent().unwrap()),"name":"a"}),
        json!({"uri":uri(b.parent().unwrap()),"name":"b"}),
    ];
    let mut client = Session::start(&p.0, folders, true).await;
    client.open(&shared, "message(\"${V}\")\n").await;
    client.diagnostics(&shared, 1).await;
    let result = definition(&mut client, &shared, 0, 11, 10).await;
    let locations = result["result"].as_array().unwrap();
    assert_eq!(locations.len(), 2, "{result}");
    for file in [&a, &b] {
        assert!(
            locations.iter().any(|location| location["uri"] == uri(file)
                && location["range"]["start"] == json!({"line":1,"character":4})),
            "{result}"
        );
    }
    let result = references(&mut client, &shared, 0, 11, true, 11).await;
    assert_eq!(result["result"].as_array().unwrap().len(), 3, "{result}");
    assert_eq!(
        references(&mut client, &shared, 0, 11, false, 12).await["result"]
            .as_array()
            .unwrap()
            .len(),
        1
    );
    client.finish().await;
    p.remove().await;
}
#[tokio::test]
async fn standalone_macro_parameters_remain_distinct_from_surrounding_variables() {
    let p = Project::new().await;
    let text = "set(P global)\nmacro(M P)\nmessage(\"${P}\")\nendmacro()\nmessage(\"${P}\")\nM(argument)\n";
    let path = p.write("loose.cmake", text).await;
    let mut client = Session::start(&p.0, vec![], true).await;
    client.open(&path, text).await;
    client.diagnostics(&path, 1).await;
    let result = definition(&mut client, &path, 2, 11, 10).await;
    assert_eq!(result["result"].as_array().unwrap().len(), 1);
    assert_eq!(
        result["result"][0]["range"]["start"],
        json!({"line":2,"character":8}),
        "{result}"
    );
    let result = definition(&mut client, &path, 4, 11, 11).await;
    assert_eq!(
        result["result"][0]["range"]["start"],
        json!({"line":1,"character":4})
    );
    let result = definition(&mut client, &path, 5, 0, 12).await;
    assert_eq!(
        result["result"][0]["range"]["start"],
        json!({"line":2,"character":6})
    );
    assert_eq!(
        references(&mut client, &path, 2, 11, true, 13).await["result"]
            .as_array()
            .unwrap()
            .len(),
        2
    );
    client.finish().await;
    p.remove().await;
}
