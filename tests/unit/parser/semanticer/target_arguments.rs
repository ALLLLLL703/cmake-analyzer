// AI-generated full-call target-slot intersection tests.
use super::target_arguments;
use crate::{
    parser::semanticer::schema::{ArgSchema, ArgSchemaKind as K, NameRole},
    workspace::Workspace,
};
use std::sync::Arc;

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
