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
