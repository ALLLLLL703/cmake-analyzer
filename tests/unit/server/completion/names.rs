//! Typed name-role inference and innermost ordinary variable-expansion replacement. Overlay edits
//! preserve literal/escaped contexts and honor cancellation.

// Contains AI-generated documentation edits.
// AI-generated schema-prefix and variable-expansion completion tests.
use super::super::{argument_context, inputs, names};
use crate::{
    parser::semanticer::{
        parsing::completion_roles,
        schema::{NameRole, builtin},
    },
    workspace::Workspace,
};
use lspally::server::model::Cancellation;

/// Probe variable, target and library roles for set/unset/list/file, optional FORCE, repeated
/// dependencies and library aliases. Value/source positions must yield no name role.
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
/// schema_prefixes_follow_operations_optional_groups_repeats_and_typed_targets();
/// ```
#[test]
fn schema_prefixes_follow_operations_optional_groups_repeats_and_typed_targets() {
    let schemas = builtin::schemas_for_version(3, 31).unwrap();
    for (name, args, expected) in [
        ("set", vec!["MY"], vec![NameRole::Variable]),
        ("set", vec!["KNOWN", "MY"], vec![]),
        ("unset", vec!["MY"], vec![NameRole::Variable]),
        ("list", vec!["APPEND", "MY"], vec![NameRole::Variable]),
        ("list", vec!["APPEND", "KNOWN", "MY"], vec![]),
        ("file", vec!["READ", "path", "MY"], vec![NameRole::Variable]),
        ("file", vec!["WRITE", "path", "MY"], vec![]),
        (
            "mark_as_advanced",
            vec!["FORCE", "MY"],
            vec![NameRole::Variable],
        ),
        (
            "target_link_libraries",
            vec!["app", "PUBLIC", "li"],
            vec![NameRole::Target],
        ),
        ("target_link_libraries", vec!["app", "PUBLIC"], vec![]),
        (
            "add_dependencies",
            vec!["app", "one", ""],
            vec![NameRole::Target],
        ),
        (
            "add_library",
            vec!["alias", "ALIAS", "li"],
            vec![NameRole::Library],
        ),
        ("add_library", vec!["library", "STATIC", "source"], vec![]),
    ] {
        assert_eq!(
            completion_roles(&schemas[name], &args),
            expected,
            "{name} {args:?}"
        );
    }
}
/// Mutate add_dependencies' repeated target schema to require two operands. An unfinished first
/// dependency must still offer a target role before the complete-call minimum is met.
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
/// incomplete_repeats_can_complete_before_the_minimum_argument_count();
/// ```
#[test]
fn incomplete_repeats_can_complete_before_the_minimum_argument_count() {
    use crate::parser::semanticer::schema::ArgSchemaKind;
    let mut schemas = builtin::schemas_for_version(3, 31).unwrap();
    let schema = schemas.get_mut("add_dependencies").unwrap();
    schema.args[1].kind = ArgSchemaKind::Repeat {
        args: vec![ArgSchemaKind::Target],
        min: 2,
    };
    assert_eq!(
        completion_roles(schema, &["app", "fi"]),
        vec![NameRole::Target]
    );
}
/// Use a removable `|` cursor marker in overlay strings to check replacement of ordinary innermost
/// variable names, including partial expansions.
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
/// Escaped/environment/cache/bracket/comment forms must not qualify; a cancelled name collection
/// must return an error.
///
/// # Flow
/// ```text
///   [cursor-marked expansion cases]
///    |
///   [find innermost replace range]-----------> [pre-cancel name collection]
///                                             |
///                                            [compare ranges; require error]
///                                             |
///                                            < checks hold? >
///                                             /         \
///                                           [pass]     [panic]
/// ```
///
/// # Examples
/// Direct local call outside any Tokio runtime (the test attribute creates its runtime):
/// ```text
/// variable_expansion_edits_only_the_innermost_name_and_respects_literal_escaping();
/// ```
#[tokio::test]
async fn variable_expansion_edits_only_the_innermost_name_and_respects_literal_escaping() {
    let root =
        std::env::temp_dir().join(format!("completion-expansion-{}.cmake", std::process::id()));
    let mut workspace = Workspace::new(&root, None).unwrap();
    for (body, expected) in [
        ("message(\"${CA|_SUFFIX}\")", Some("CA_SUFFIX")),
        ("message(\"${${CA|}}\")", Some("CA")),
        ("message(\"${CA|", Some("CA")),
        ("message(\"\\${CA|}\")", None),
        ("message(\"$ENV{CA|}\")", None),
        ("message(\"$CACHE{CA|}\")", None),
        ("message([=[${CA|}]=])", None),
        ("# ${CA|}", None),
    ] {
        let text = format!("cmake_minimum_required(VERSION 3.31)\n{body}");
        let offset = text.find('|').unwrap();
        let text = text.replace('|', "");
        workspace.set_overlay(&root, text.clone()).unwrap();
        workspace.flush_documents().await.unwrap();
        let snapshot = workspace.snapshot(&root).unwrap().unwrap();
        let site = argument_context::argument(&snapshot, offset)
            .and_then(|argument| argument_context::expansion(&snapshot, &argument, offset));
        assert_eq!(
            site.map(|site| text[site.replace].to_owned()).as_deref(),
            expected,
            "{body}"
        );
    }
    workspace
        .set_overlay(
            &root,
            "cmake_minimum_required(VERSION 3.31)\nset(MY_VARIABLE yes)\nmessage(${MY)",
        )
        .unwrap();
    workspace.flush_documents().await.unwrap();
    let input = inputs::capture(&workspace, &root).unwrap();
    let offset = input.snapshot.text.len() - 1;
    let cancel = Cancellation::default();
    cancel.cancel();
    assert!(names::collect(&[input], offset, "MY", true, &cancel).is_err());
}
