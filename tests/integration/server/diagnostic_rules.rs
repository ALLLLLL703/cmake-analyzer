// AI-generated tests.
use super::session::{Project, Session};
use serde_json::{Value, json};
fn diagnostics(report: &Value) -> Vec<(&str, i64)> {
    report["params"]["diagnostics"]
        .as_array()
        .unwrap()
        .iter()
        .map(|item| {
            (
                item["message"].as_str().unwrap(),
                item["severity"].as_i64().unwrap(),
            )
        })
        .collect()
}
#[tokio::test]
async fn per_rule_levels_reload_and_invalid_levels_preserve_live_configuration() {
    let p = Project::new().await;
    let body = "unresolved_external_call()\nmessage(\"${UNKNOWN_RULE_VARIABLE}\")\n";
    let root = p.write("CMakeLists.txt", body).await;
    let mut client = Session::start(&p.0, vec![], true).await;
    client.open(&root, body).await;
    let initial = client.diagnostics(&root, 1).await;
    let items = diagnostics(&initial);
    assert!(
        items.iter().any(
            |(message, severity)| message.contains("unresolved_external_call") && *severity == 1
        ),
        "{initial}"
    );
    assert!(
        items
            .iter()
            .any(|(message, severity)| message.contains("UNKNOWN_RULE_VARIABLE") && *severity == 2),
        "{initial}"
    );
    let config = p.0.join(".cmake-analyzer.toml");
    for (index, (name, severity)) in [
        ("hint", Some(4)),
        ("info", Some(3)),
        ("warn", Some(2)),
        ("error", Some(1)),
        ("none", None),
    ]
    .into_iter()
    .enumerate()
    {
        tokio::fs::write(
            &config,
            format!(
                "[diagnostics.rules]\nunknown_function=\"{name}\"\nunknown_variable=\"{name}\"\n"
            ),
        )
        .await
        .unwrap();
        let id = index as i32 + 10;
        client.send(json!({"jsonrpc":"2.0","id":id,"method":"cmake-analyzer/reloadConfig","params":null})).await;
        let response = client.response(id).await;
        assert!(response.get("error").is_none(), "{response}");
        let report = client.diagnostics(&root, 1).await;
        let items = diagnostics(&report);
        match severity {
            Some(expected) => {
                assert_eq!(items.len(), 2, "{report}");
                assert!(
                    items.iter().all(|(_, severity)| *severity == expected),
                    "{report}"
                );
            }
            None => assert!(items.is_empty(), "{report}"),
        }
    }
    tokio::fs::write(&config, "[diagnostics.rules]\nunknown_function=\"fatal\"\n")
        .await
        .unwrap();
    client
        .send(json!({"jsonrpc":"2.0","id":20,"method":"cmake-analyzer/reloadConfig","params":null}))
        .await;
    assert!(client.response(20).await.get("error").is_some());
    client.finish().await;
    p.remove().await;
}
