//! Private completion tests attached by `src/server/completion.rs`: required-prefix snippets and
//! lexical command-site selection over shared snapshots.

// Contains AI-generated documentation edits.
// AI-generated command completion syntax and snippet tests.
#[path = "completion/imports.rs"]
mod imports;
#[path = "completion/keywords.rs"]
mod keywords;
#[path = "completion/names.rs"]
mod names;
use super::{arguments, context, model::Parameter};
use crate::{parser::semanticer::schema::builtin, workspace::Workspace};

/// Build snippets from 3.31 schemas for condition, closing block, required version, library and set
/// calls. Compare exact common required prefixes and escape dollar, brace and backslash placeholder
/// text.
///
/// Only local values and analysis buffers are created or changed; no file, network or process side
/// effects.
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
/// # Examples
/// Local test-module call:
/// ```text
/// snippets_keep_required_prefixes_without_guessing_overloads_or_optional_arguments();
/// ```
#[test]
fn snippets_keep_required_prefixes_without_guessing_overloads_or_optional_arguments() {
    let schemas = builtin::schemas_for_version(3, 31).unwrap();
    for (command, expected) in [
        ("if", "if(${1:condition})$0"),
        ("endif", "endif()$0"),
        (
            "cmake_minimum_required",
            "cmake_minimum_required(VERSION ${1:version})$0",
        ),
        ("add_library", "add_library(${1:target})$0"),
        ("set", "set(${1:variable})$0"),
    ] {
        assert_eq!(
            arguments::snippet(command, &arguments::parameters(&schemas[command])),
            expected
        );
    }
    assert_eq!(
        arguments::snippet("f", &[Parameter::Placeholder("$x}\\y".into())]),
        "f(${1:\\$x\\}\\\\y})$0"
    );
}
/// Replace an overlay with comment, literal, argument and command-prefix cases and compare site
/// availability at end-of-text.
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
/// Retain a before snapshot, change to a comment and require independent lexical allocations with
/// old-site stability.
///
/// # Flow
/// ```text
///   [overlay lexical cases]
///    |
///   [capture command site]-----------> [retained before snapshot]
///                                     |
///                                    [check immutable lexical state]
///                                     |
///                                    < checks hold? >
///                                     /         \
///                                   [pass]     [panic]
/// ```
///
/// # Examples
/// Direct local call outside any Tokio runtime (the test attribute creates its runtime):
/// ```text
/// lexical_context_is_shared_and_comments_arguments_and_invalid_identifiers_are_excluded();
/// ```
#[tokio::test]
async fn lexical_context_is_shared_and_comments_arguments_and_invalid_identifiers_are_excluded() {
    let root =
        std::env::temp_dir().join(format!("completion-context-{}.cmake", std::process::id()));
    let mut workspace = Workspace::new(&root, None).unwrap();
    let header = "cmake_minimum_required(VERSION 3.31)\n";
    for (source, allowed) in [
        ("# if", false),
        ("#[=[if", false),
        ("#[=[if]=] if", true),
        ("\"if", false),
        ("[=[if", false),
        ("[=[if]=]", false),
        ("message(\"if", false),
        ("message(\nif", false),
        ("if(TRUE)\nif", true),
        ("foo-bar", false),
        ("${if}", false),
        ("if", true),
        ("", true),
    ] {
        let text = format!("{header}{source}");
        workspace.set_overlay(&root, text.clone()).unwrap();
        workspace.flush_documents().await.unwrap();
        let snapshot = workspace.snapshot(&root).unwrap().unwrap();
        assert_eq!(
            context::site(&snapshot, text.len()).is_some(),
            allowed,
            "{source}"
        );
    }
    let before = workspace.snapshot(&root).unwrap().unwrap();
    workspace
        .set_overlay(&root, format!("{header}# if"))
        .unwrap();
    workspace.flush_documents().await.unwrap();
    assert!(context::site(&before, before.text.len()).is_some());
    let after = workspace.snapshot(&root).unwrap().unwrap();
    assert!(context::site(&after, after.text.len()).is_none());
    assert!(!std::sync::Arc::ptr_eq(&before.lexed, &after.lexed));
}
