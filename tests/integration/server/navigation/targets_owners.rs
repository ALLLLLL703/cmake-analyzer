// AI-generated multi-owner target navigation regression test.
//! Shared target uses with owner-local definitions and deduplicated locations.
//! Two workspaces deliberately declare the same spelling without sharing declaration identity.
use super::super::super::session::{Project, Session, uri};
use super::super::{definition, references};
use super::{column, count};
use serde_json::json;

#[tokio::test]
/// Check shared target uses preserve two workspace-local declarations without duplicate use
/// locations.
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
/// Create roots a/b declaring Common and importing one shared target_sources call. Shared
/// definition returns both roots; references count three without declarations and five with.
/// Query a alone and require two uses excluding b.
///
/// # Flow
/// ```text
/// [Temporary sources + settings]      [Expected: 2 definitions / 3 or 5 references]
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
/// shared_target_uses_keep_workspace_local_definitions_and_deduplicate_locations(); // #[tokio::test] supplies the runtime
/// ```
async fn shared_target_uses_keep_workspace_local_definitions_and_deduplicate_locations() {
    let p = Project::new().await;
    let source = "target_sources(Common INTERFACE item.c)\n";
    let shared = p.write("shared.cmake", source).await;
    let root_source = "add_library(Common INTERFACE)\nadd_executable(App main.c)\ninclude(../shared.cmake)\ntarget_link_libraries(App PRIVATE Common)\n";
    let a = p.write("a/CMakeLists.txt", root_source).await;
    let b = p.write("b/CMakeLists.txt", root_source).await;
    let folders = vec![
        json!({"uri":uri(a.parent().unwrap()),"name":"a"}),
        json!({"uri":uri(b.parent().unwrap()),"name":"b"}),
    ];
    let mut client = Session::start(&p.0, folders, true).await;
    client.open(&shared, source).await;
    client.diagnostics(&shared, 1).await;
    let found = definition(&mut client, &shared, 0, column(source, 0, "Common"), 10).await;
    assert_eq!(count(&found), 2, "{found}");
    for root in [&a, &b] {
        assert!(
            found["result"]
                .as_array()
                .unwrap()
                .iter()
                .any(|location| location["uri"] == uri(root)),
            "{found}"
        );
    }
    assert_eq!(
        count(
            &references(
                &mut client,
                &shared,
                0,
                column(source, 0, "Common"),
                false,
                11
            )
            .await
        ),
        3
    );
    assert_eq!(
        count(
            &references(
                &mut client,
                &shared,
                0,
                column(source, 0, "Common"),
                true,
                12
            )
            .await
        ),
        5
    );
    let only_a = references(&mut client, &a, 0, 12, false, 13).await;
    assert_eq!(count(&only_a), 2, "{only_a}");
    assert!(
        !only_a["result"]
            .as_array()
            .unwrap()
            .iter()
            .any(|location| location["uri"] == uri(&b))
    );
    client.finish().await;
    p.remove().await;
}
