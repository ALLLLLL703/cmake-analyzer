// AI-generated tests.
//! Rule-level diagnostic severity reloads for unknown commands and variables.
//! Only fixture configuration is written; no source is executed.
use super::session::{Project, Session};
use serde_json::{Value, json};
/// Extract borrowed diagnostic messages paired with numeric LSP severities; no side effects.
///
/// # Arguments
/// - `report` `[in]`: publishDiagnostics notification with an array of string messages/integer
///   severities.
///
/// # Returns
/// A vector in publication order; each tuple is (borrowed message, numeric severity).
///
/// # Panics
/// Panics if the expected array, string or integer fields have other JSON types.
///
/// # Examples
/// ```text
/// let levels = diagnostics(&report);
/// ```
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
/// Check unknown-function/variable severity mappings republish on configuration-only changes.
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
/// Require default severities 1 and 2, then reload hint/info/warn/error/none for both rules and
/// require exactly two matching severities or no diagnostics. Reload unsupported fatal and
/// require an error. No post-error publication is asserted.
///
/// # Flow
/// ```text
/// [Temporary sources + settings]      [Expected: severity 4/3/2/1 or none]
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
/// per_rule_levels_reload_and_invalid_levels_preserve_live_configuration(); // #[tokio::test] supplies the runtime
/// ```
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
