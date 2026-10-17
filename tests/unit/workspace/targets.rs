// AI-generated target-reference aggregation and snapshot regression tests.

//! Check owner-qualified target references without mutating retained file snapshots.
//! Writes temporary source and analyzes overlays; no configure or project commands are executed.

use super::Project;
use cmake_analyzer::parser::{build_struct::model::BuildEffect, semanticer::model::ReferenceKind};
use std::sync::Arc;

/// Include Core, read it with target_sources, shift root spans, then rename the declaration to NewCore.
/// Writes/reads/removes temporary files and updates overlays; no project execution.
///
/// # Arguments
/// None; the test constructs its own fixtures.
///
/// # Returns
/// `()` after all checks pass.
///
/// # Panics
/// If fixture setup or any documented assertion fails.
///
/// # Algorithm
/// Require owner references to refresh while local/old snapshots remain unchanged, shared argument/source arrays, graph reuse on shift, and removal of the stale read on rename.
///
/// # Flow
/// ```text
/// [included Core] --> [root target_sources read]
///        |                       |
///        v                       v
/// [local definition only] [owner definition + read]
///                                |
///                                v
///                       [shift root / flush]
///                                |
///                                v
///                  [new read key/span; old data unchanged]
///                                |
///                                v
/// [rename to NewCore] --> [definition only, no stale read]
/// ```
///
/// # Examples
/// Local test-module context, outside any running Tokio runtime:
/// the macro-generated test wrapper runs its own runtime and returns `()`.
/// Prefer the test harness; ignored measurements require explicit selection.
/// ```text
/// target_references_stay_on_declarations_without_mutating_shared_file_snapshots();
/// ```
#[tokio::test]
async fn target_references_stay_on_declarations_without_mutating_shared_file_snapshots() {
    let p = Project::new().await;
    p.write(
        "CMakeLists.txt",
        "include(targets.cmake)\ntarget_sources(Core PRIVATE item.c)\n",
    )
    .await;
    p.write("targets.cmake", "add_library(Core INTERFACE)\n")
        .await;
    let mut workspace = p.workspace();
    workspace.flush().await.unwrap();
    let file = p.0.join("targets.cmake");
    let root = p.0.join("CMakeLists.txt");
    let snapshot = workspace.snapshot(&file).unwrap().unwrap();
    let local = snapshot
        .build
        .facts
        .iter()
        .find_map(|fact| {
            if let BuildEffect::Target(target) = &fact.effect {
                Some(target)
            } else {
                None
            }
        })
        .unwrap();
    assert_eq!(local.references.len(), 1);
    assert_eq!(local.references[0].kind, ReferenceKind::Definition);
    let old = workspace.root_graph().unwrap().targets.clone();
    assert_eq!(old[0].references.len(), 2);
    assert!(Arc::ptr_eq(&local.arguments, &old[0].arguments));
    assert!(Arc::ptr_eq(&local.sources, &old[0].sources));
    let old_read = old[0]
        .references
        .iter()
        .find(|reference| reference.kind == ReferenceKind::Read)
        .unwrap()
        .clone();
    workspace.set_overlay(&root,"cmake_minimum_required(VERSION 3.31)\n# shifted\ninclude(targets.cmake)\ntarget_sources(Core PRIVATE item.c)\n").unwrap();
    workspace.flush().await.unwrap();
    assert!(!workspace.graph_rebuilt);
    let current = workspace.root_graph().unwrap().targets.clone();
    let current_read = current[0]
        .references
        .iter()
        .find(|reference| reference.kind == ReferenceKind::Read)
        .unwrap();
    assert_ne!(current_read.source.analysis, old_read.source.analysis);
    assert!(current_read.span.start_byte > old_read.span.start_byte);
    assert_eq!(
        snapshot.build.facts.iter().find_map(|fact| {
            if let BuildEffect::Target(target) = &fact.effect {
                Some(target.references.len())
            } else {
                None
            }
        }),
        Some(1)
    );
    assert_eq!(
        old[0]
            .references
            .iter()
            .find(|reference| reference.kind == ReferenceKind::Read)
            .unwrap()
            .span,
        old_read.span
    );
    workspace
        .set_overlay(
            &file,
            "cmake_minimum_required(VERSION 3.31)\nadd_library(NewCore INTERFACE)\n",
        )
        .unwrap();
    workspace.flush().await.unwrap();
    let targets = &workspace.root_graph().unwrap().targets;
    assert_eq!(targets[0].name.text, "NewCore");
    assert_eq!(targets[0].references.len(), 1);
    drop(p);
}

/// Include a guarded CoreAlias from two child directories while declaring duplicate target names.
/// Writes/reads/removes temporary files; no project execution.
///
/// # Arguments
/// None; the test constructs its own fixtures.
///
/// # Returns
/// `()` after all checks pass.
///
/// # Panics
/// If fixture setup or any documented assertion fails.
///
/// # Algorithm
/// Require one alias with three references, including TARGET, and two ambiguous Duplicate declarations with definition-only references.
///
/// # Flow
/// ```text
/// [two children include guarded CoreAlias]
///                      |
///                      v
///         [root links CoreAlias / Duplicate]
///                      |
///          +-----------+------------+
///          v                        v
/// [one alias / 3 references]  [two Duplicate declarations]
///                                   |
///                                   v
///                           [definition-only each]
/// ```
///
/// # Examples
/// Local test-module context, outside any running Tokio runtime:
/// the macro-generated test wrapper runs its own runtime and returns `()`.
/// Prefer the test harness; ignored measurements require explicit selection.
/// ```text
/// global_aliases_are_visible_above_their_declaration_directory_and_duplicates_do_not_bind();
/// ```
#[tokio::test]
async fn global_aliases_are_visible_above_their_declaration_directory_and_duplicates_do_not_bind() {
    let p = Project::new().await;
    p.write("CMakeLists.txt","add_library(Core INTERFACE)\nadd_library(Duplicate INTERFACE)\nadd_executable(App main.c)\nadd_subdirectory(child)\nadd_subdirectory(other)\ntarget_link_libraries(App PRIVATE CoreAlias Duplicate)\n").await;
    p.write(
        "child/CMakeLists.txt",
        "include(../aliases.cmake)\nadd_library(Duplicate INTERFACE)\n",
    )
    .await;
    p.write("other/CMakeLists.txt", "include(../aliases.cmake)\n")
        .await;
    p.write(
        "aliases.cmake",
        "if(NOT TARGET CoreAlias)\nadd_library(CoreAlias ALIAS Core)\nendif()\n",
    )
    .await;
    let mut workspace = p.workspace();
    workspace.flush().await.unwrap();
    let targets = &workspace.root_graph().unwrap().targets;
    let alias = targets
        .iter()
        .find(|target| target.name.text == "CoreAlias")
        .unwrap();
    // The now-modeled TARGET predicate also references the alias guarded below it.
    assert_eq!(alias.references.len(), 3);
    assert_eq!(
        targets
            .iter()
            .filter(|target| target.name.text == "CoreAlias")
            .count(),
        1
    );
    let duplicates: Vec<_> = targets
        .iter()
        .filter(|target| target.name.text == "Duplicate")
        .collect();
    assert_eq!(duplicates.len(), 2);
    assert!(duplicates.iter().all(|target| target.references.len() == 1));
    drop(p);
}

/// Link App against 2,048 repeated literal Core operands.
/// Writes/reads/removes temporary fixture text; no project execution.
///
/// # Arguments
/// None; the test constructs its own fixtures.
///
/// # Returns
/// `()` after all checks pass.
///
/// # Panics
/// If fixture setup or any documented assertion fails.
///
/// # Algorithm
/// Require Core to retain 2,049 references including its definition.
///
/// # Flow
/// ```text
/// [Core definition]       [2048 repeated Core operands]
///         |                          |
///         +-------------+------------+
///                       v
///               [Workspace::flush]
///                       |
///                       v
///           [require 2049 Core references]
/// ```
///
/// # Examples
/// Local test-module context, outside any running Tokio runtime:
/// the macro-generated test wrapper runs its own runtime and returns `()`.
/// Prefer the test harness; ignored measurements require explicit selection.
/// ```text
/// large_link_lists_keep_every_target_reference();
/// ```
#[tokio::test]
async fn large_link_lists_keep_every_target_reference() {
    let p = Project::new().await;
    let libraries = std::iter::repeat_n("Core", 2048)
        .collect::<Vec<_>>()
        .join(" ");
    p.write("CMakeLists.txt",&format!("add_library(Core INTERFACE)\nadd_executable(App main.c)\ntarget_link_libraries(App PRIVATE {libraries})\n")).await;
    let mut workspace = p.workspace();
    workspace.flush().await.unwrap();
    let core = workspace
        .root_graph()
        .unwrap()
        .targets
        .iter()
        .find(|target| target.name.text == "Core")
        .unwrap();
    assert_eq!(core.references.len(), 2049);
}

/// Include a user function named add_library before calling add_library(NotNative INTERFACE).
/// Writes/reads/removes temporary files; no project execution.
///
/// # Arguments
/// None; the test constructs its own fixtures.
///
/// # Returns
/// `()` after all checks pass.
///
/// # Panics
/// If fixture setup or any documented assertion fails.
///
/// # Algorithm
/// Require the owner target list to be empty because the call is user-defined, not native.
///
/// # Flow
/// ```text
/// [include commands.cmake] --> [user function add_library]
///                                       |
///                                       v
/// [add_library(NotNative ...)] --> [user-bound call]
///                                       |
///                                       v
///                             [owner targets empty]
/// ```
///
/// # Examples
/// Local test-module context, outside any running Tokio runtime:
/// the macro-generated test wrapper runs its own runtime and returns `()`.
/// Prefer the test harness; ignored measurements require explicit selection.
/// ```text
/// imported_user_functions_suppress_native_target_declarations_in_the_owner_index();
/// ```
#[tokio::test]
async fn imported_user_functions_suppress_native_target_declarations_in_the_owner_index() {
    let p = Project::new().await;
    p.write(
        "CMakeLists.txt",
        "include(commands.cmake)\nadd_library(NotNative INTERFACE)\n",
    )
    .await;
    p.write(
        "commands.cmake",
        "function(add_library ARG)\nendfunction()\n",
    )
    .await;
    let mut workspace = p.workspace();
    workspace.flush().await.unwrap();
    assert!(workspace.root_graph().unwrap().targets.is_empty());
    drop(p);
}
