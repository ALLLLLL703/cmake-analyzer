// AI-generated tests.
use crate::workspace::{Workspace, completion::model::Request, model::WorkspaceLimits};
use std::sync::atomic::{AtomicU64, Ordering};
static NEXT: AtomicU64 = AtomicU64::new(0);
#[tokio::test]
async fn catalogue_cancellation_returns_before_reading_sources() {
    let path = std::env::temp_dir().join("cmake-cancelled-catalogue/CMakeLists.txt");
    let mut workspace = Workspace::new(&path, None).unwrap();
    let error = workspace
        .completion_catalog(
            &path,
            &Request {
                modules: true,
                packages: true,
                targets: true,
                command: None,
            },
            || true,
        )
        .await
        .err()
        .unwrap();
    assert_eq!(error.kind(), std::io::ErrorKind::Interrupted);
}
#[tokio::test]
async fn conditional_target_candidates_respect_workspace_file_budget() {
    let root = std::env::temp_dir().join(format!(
        "cmake-target-catalogue-{}-{}",
        std::process::id(),
        NEXT.fetch_add(1, Ordering::Relaxed)
    ));
    tokio::fs::create_dir_all(root.join("pkg")).await.unwrap();
    let entry = root.join("CMakeLists.txt");
    let target = root.join("pkg/Targets.cmake");
    tokio::fs::write(
        &entry,
        "cmake_minimum_required(VERSION 3.31)\nfind_package(Pack CONFIG REQUIRED)\n",
    )
    .await
    .unwrap();
    tokio::fs::write(root.join("pkg/PackConfig.cmake"),"cmake_minimum_required(VERSION 3.31)\nif(TRUE)\ninclude(\"${CMAKE_CURRENT_LIST_DIR}/Targets.cmake\")\nendif()\n").await.unwrap();
    tokio::fs::write(
        &target,
        "cmake_minimum_required(VERSION 3.31)\nadd_library(Pack::Core INTERFACE IMPORTED)\n",
    )
    .await
    .unwrap();
    let mut workspace = Workspace::with_limits(
        &entry,
        None,
        WorkspaceLimits {
            files: 2,
            ..WorkspaceLimits::default()
        },
    )
    .unwrap();
    workspace.set_external_indexing(
        crate::workspace::model::ExternalIndex::Full,
        crate::workspace::model::ExternalIndex::Full,
        vec![root.join("pkg")],
    );
    workspace.flush().await.unwrap();
    assert_eq!(workspace.root_graph().unwrap().invocations.len(), 2);
    let result = workspace
        .completion_catalog(
            &entry,
            &Request {
                targets: true,
                ..Request::default()
            },
            || false,
        )
        .await
        .unwrap();
    assert!(result.targets.iter().all(|source|!source.snapshot.build.facts.iter().any(|fact|matches!(&fact.effect,crate::parser::build_struct::model::BuildEffect::Target(target) if target.name.text=="Pack::Core"))));
    assert!(workspace.snapshot(&target).unwrap().is_none());
    let mut cached = Workspace::new(&entry, None).unwrap();
    cached.set_external_indexing(
        crate::workspace::model::ExternalIndex::Full,
        crate::workspace::model::ExternalIndex::Full,
        vec![root.join("pkg")],
    );
    cached.flush().await.unwrap();
    cached
        .completion_catalog(
            &entry,
            &Request {
                targets: true,
                ..Request::default()
            },
            || false,
        )
        .await
        .unwrap();
    let before = cached.snapshot(&target).unwrap().unwrap();
    cached.set_overlay(&entry,"cmake_minimum_required(VERSION 3.31)\nfind_package(Pack CONFIG REQUIRED)\nmessage(after)\n").unwrap();
    cached.flush().await.unwrap();
    assert!(std::sync::Arc::ptr_eq(
        &before,
        &cached.snapshot(&target).unwrap().unwrap()
    ));
    cached
        .set_overlay(
            &entry,
            "cmake_minimum_required(VERSION 3.31)\nmessage(no_package)\n",
        )
        .unwrap();
    cached.flush().await.unwrap();
    assert!(cached.snapshot(&target).unwrap().is_none());
    tokio::fs::remove_dir_all(root).await.unwrap();
}
