// AI-generated schema argument-keyword completion tests.
use crate::parser::semanticer::{
    model::CommandKind,
    parsing::completion_keywords,
    schema::{ArgSchema, ArgSchemaKind as K, CommandSchema, CommandSchemaFrom, builtin},
};

#[test]
fn literal_keywords_follow_prefix_operations_required_slots_and_opaque_tails() {
    let schemas = builtin::schemas_for_version(3, 31).unwrap();
    for (name, args, expected) in [
        ("add_library", vec!["Core", "ST"], vec!["STATIC"]),
        (
            "add_library",
            vec!["Core", "STATIC", "IMPORTED", "G"],
            vec!["GLOBAL"],
        ),
        ("add_library", vec!["Core", "ALIAS", ""], vec![]),
        ("add_library", vec!["Core", "source.c", "ST"], vec![]),
        ("target_link_libraries", vec!["App", "PU"], vec!["PUBLIC"]),
        (
            "target_link_libraries",
            vec!["App", "PUBLIC", "PRI"],
            vec!["PRIVATE"],
        ),
        (
            "file",
            vec!["READ", "input", "OUT", ""],
            vec!["HEX", "LIMIT", "OFFSET"],
        ),
        ("file", vec!["READ", "input", "OUT", "OFFSET", ""], vec![]),
        ("file", vec!["COPY", "input", "D"], vec!["DESTINATION"]),
        ("file", vec!["READ", ""], vec![]),
        (
            "file",
            vec!["GLOB", "OUT", "CON"],
            vec!["CONFIGURE_DEPENDS"],
        ),
        ("file", vec!["unmodeled", ""], vec![]),
        ("list", vec!["AP"], vec!["APPEND"]),
        ("list", vec!["APPEND", "VAR", "R"], vec![]),
        ("set", vec!["VAR", "CACHE", "B"], vec!["BOOL"]),
        ("add_library", vec!["Core", "st"], vec![]),
    ] {
        assert_eq!(
            completion_keywords(&schemas[name], &args, || false),
            expected,
            "{name} {args:?}"
        );
    }
    let old = builtin::schemas_for_version(3, 0).unwrap();
    assert!(!old.contains_key("include_guard"));
    assert_eq!(
        completion_keywords(&schemas["include_guard"], &["G"], || false),
        vec!["GLOBAL"]
    );
    assert_eq!(
        completion_keywords(&schemas["cmake_language"], &["CA"], || false),
        vec!["CALL"]
    );
}
fn schema(kinds: Vec<K>) -> CommandSchema {
    CommandSchema {
        args: kinds.into_iter().map(|kind| ArgSchema { kind }).collect(),
        command_name: "example".into(),
        from: CommandSchemaFrom::External,
        kind: CommandKind::Other("example".into()),
    }
}
#[test]
fn keyword_prefixes_preserve_allof_choices_optional_suffixes_and_repeat_minima() {
    let schema = schema(vec![
        K::Optional(vec![K::Keyword("PRE".into())]),
        K::Choice(vec![
            vec![
                K::AllOf(vec![K::Keyword("PUBLIC".into()), K::Value]),
                K::Value,
            ],
            vec![K::Keyword("PUBLICITY".into())],
        ]),
        K::Repeat {
            args: vec![K::Keyword("ITEM".into()), K::Value],
            min: 2,
        },
    ]);
    assert_eq!(
        completion_keywords(&schema, &["PUB"], || false),
        vec!["PUBLIC", "PUBLICITY"]
    );
    assert_eq!(
        completion_keywords(&schema, &["PUBLIC"], || false),
        vec!["PUBLIC", "PUBLICITY"]
    );
    assert_eq!(
        completion_keywords(&schema, &["PRE", "PUBLIC", "value", "IT"], || false),
        vec!["ITEM"]
    );
    assert!(
        completion_keywords(&schema, &["PRE", "PUBLIC", "value", "ITEM", "IT"], || false)
            .is_empty()
    );
    assert!(completion_keywords(&schema, &[], || false).is_empty());
}
#[test]
fn duplicate_keywords_are_sorted_and_cancellation_stops_probes() {
    let schema = schema(vec![K::Choice(vec![
        vec![K::Keyword("SECOND".into())],
        vec![K::Keyword("FIRST".into())],
        vec![K::Keyword("FIRST".into())],
    ])]);
    assert_eq!(
        completion_keywords(&schema, &[""], || false),
        vec!["FIRST", "SECOND"]
    );
    assert!(completion_keywords(&schema, &[""], || true).is_empty());
}
