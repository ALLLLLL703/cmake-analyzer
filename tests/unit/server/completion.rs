// AI-generated command completion syntax and snippet tests.
#[path = "completion/names.rs"]
mod names;
use super::{arguments, context, model::Parameter};
use crate::{parser::semanticer::schema::builtin, workspace::Workspace};

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
