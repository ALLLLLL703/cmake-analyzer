//! Builtin and synthetic schema keyword-prefix probes: required operands, alternatives,
//! repetitions, sorting, duplicates and cancellation. No external side effects.

// Contains AI-generated documentation edits.
// AI-generated schema argument-keyword completion tests.
use crate::parser::semanticer::{
    model::CommandKind,
    parsing::completion_keywords,
    schema::{ArgSchema, ArgSchemaKind as K, CommandSchema, CommandSchemaFrom, builtin},
};

/// Probe builtin 3.31 argument prefixes across libraries, links, file/list operations and cache
/// assignment. Require case-sensitive contextual keywords, no suggestions in required/opaque value
/// slots and version-aware availability.
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
/// literal_keywords_follow_prefix_operations_required_slots_and_opaque_tails();
/// ```
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
/// Wrap synthetic argument kinds as an external example command for prefix-matcher tests. Only
/// allocates local schema data; no external side effects.
///
/// # Arguments
/// - `kinds` `[in]`: Owned argument nodes moved into ArgSchema records in their supplied order.
///
/// # Returns
/// An external CommandSchema named example with CommandKind::Other and the supplied structured
/// arguments.
///
/// # Examples
/// Local test-module context:
/// ```text
/// let grammar = schema(vec![K::Keyword("FIRST".into())]);
/// assert_eq!(completion_keywords(&grammar, &["F"], || false), ["FIRST"]);
/// ```
fn schema(kinds: Vec<K>) -> CommandSchema {
    CommandSchema {
        args: kinds.into_iter().map(|kind| ArgSchema { kind }).collect(),
        command_name: "example".into(),
        from: CommandSchemaFrom::External,
        kind: CommandKind::Other("example".into()),
    }
}
/// Build a synthetic optional PRE, PUBLIC/intersection-or-PUBLICITY choice and twice-required
/// ITEM/value repetition. Require both ambiguous keyword prefixes, an ITEM continuation and no
/// suggestion where a required value is pending.
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
/// If a stated invariant fails.
///
/// # Examples
/// Local test-module call:
/// ```text
/// keyword_prefixes_preserve_allof_choices_optional_suffixes_and_repeat_minima();
/// ```
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
/// Build alternatives containing SECOND and duplicate FIRST. Require sorted unique results and no
/// results when the cancellation predicate is true.
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
/// If a stated invariant fails.
///
/// # Examples
/// Local test-module call:
/// ```text
/// duplicate_keywords_are_sorted_and_cancellation_stops_probes();
/// ```
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
