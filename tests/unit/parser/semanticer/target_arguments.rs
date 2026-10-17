//! Private full-call target-role intersections attached by `parsing::outputs`. Overlay snapshots
//! supply commands whose copied schemas are mutated locally to create ambiguous branches.

// Contains AI-generated documentation edits.
// AI-generated full-call target-slot intersection tests.
use super::target_arguments;
use crate::{
    parser::semanticer::schema::{ArgSchema, ArgSchemaKind as K, NameRole},
    workspace::Workspace,
};
use std::sync::Arc;

/// Create overlay add_dependencies commands, clone each semantic command and replace its schema
/// with equal-specificity choices through copy-on-write.
///
/// Mutates only local workspace overlays/snapshots and may read supporting paths; no source-file
/// writes or CMake execution.
///
/// # Arguments
/// None. The test builds its own fixture and expected results.
///
/// # Returns
/// `()` after all checks pass; no data or output parameter is returned.
///
/// # Panics
/// If a stated invariant fails or an unwrapped fixture/setup operation fails.
///
/// # Algorithm
/// Require only argument indices marked as targets by every matching branch, including the
/// no-common-slot case.
///
/// # Flow
/// ```text
///   [overlay commands]
///    |
///   [clone + replace choice schema]----------> [equally ranked full matches]
///                                             |
///                                            [intersect shared target slots]
///                                             |
///                                            < checks hold? >
///                                             /         \
///                                           [pass]     [panic]
/// ```
///
/// # Examples
/// Direct local call outside any Tokio runtime (the test attribute creates its runtime):
/// ```text
/// equally_specific_full_call_branches_keep_only_shared_target_slots();
/// ```
#[tokio::test]
async fn equally_specific_full_call_branches_keep_only_shared_target_slots() {
    let root = std::env::temp_dir().join(format!("target-slots-{}.cmake", std::process::id()));
    let mut workspace = Workspace::new(&root, None).unwrap();
    for (arguments, branches, expected) in [
        (
            "A B",
            vec![vec![K::Target, K::Value], vec![K::Value, K::Target]],
            vec![],
        ),
        (
            "A B",
            vec![vec![K::Target, K::Target], vec![K::Value, K::Target]],
            vec![(1, NameRole::Target)],
        ),
        (
            "A B C",
            vec![
                vec![K::Target, K::Value, K::Target],
                vec![K::Value, K::Target, K::Target],
            ],
            vec![(2, NameRole::Target)],
        ),
    ] {
        workspace
            .set_overlay(
                &root,
                format!("cmake_minimum_required(VERSION 3.31)\nadd_dependencies({arguments})\n"),
            )
            .unwrap();
        workspace.flush_documents().await.unwrap();
        let snapshot = workspace.snapshot(&root).unwrap().unwrap();
        let mut command = snapshot.semantics.commands[1].clone();
        Arc::make_mut(command.schema.as_mut().unwrap()).args = vec![ArgSchema {
            kind: K::Choice(branches),
        }];
        let texts: Vec<_> = command
            .args
            .iter()
            .map(|arg| arg.content.as_str())
            .collect();
        assert_eq!(target_arguments(&command, &texts), expected, "{arguments}");
    }
}
