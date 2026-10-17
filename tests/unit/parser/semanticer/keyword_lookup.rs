// AI-generated tests.
use super::schemas;
use crate::parser::semanticer::{
    parsing::{completion_keywords, completion_roles},
    schema::{ArgSchemaKind as K, CommandSchema, NameRole, builtin},
};
fn schema(version: (u32, u32), name: &str) -> CommandSchema {
    let (_, args) = schemas(version)
        .into_iter()
        .find(|(command, _)| *command == name)
        .unwrap();
    super::super::make_schema(name, args)
}
fn keywords(version: (u32, u32), name: &str, args: &[&str]) -> Vec<String> {
    completion_keywords(&schema(version, name), args, || false)
        .into_iter()
        .map(str::to_owned)
        .collect()
}
#[test]
fn keyword_lookup_project_and_package_examples_use_real_grammars() {
    assert_eq!(keywords((3, 0), "project", &["App", "LA"]), ["LANGUAGES"]);
    assert_eq!(
        keywords((3, 31), "project", &["App", "VERSION", "1.2", "DE"]),
        ["DESCRIPTION"]
    );
    assert!(keywords((3, 31), "project", &["App", "DESCRIPTION", ""]).is_empty());
    assert_eq!(
        keywords((3, 31), "project", &["App", "LANGUAGES", "CXX", "HO"]),
        ["HOMEPAGE_URL"]
    );
    assert_eq!(
        keywords((3, 0), "find_package", &["Qt", "RE"]),
        ["REQUIRED"]
    );
    assert_eq!(
        keywords((3, 31), "find_package", &["Qt", "REQUIRED", "COMP"]),
        ["COMPONENTS"]
    );
    assert_eq!(
        keywords(
            (3, 31),
            "find_package",
            &["Qt", "COMPONENTS", "Core", "OPT"]
        ),
        ["OPTIONAL_COMPONENTS"]
    );
}
#[test]
fn keyword_lookup_modes_and_required_scalar_slots_are_contextual() {
    let module = keywords((3, 31), "find_package", &["Qt", "MODULE", ""]);
    assert!(
        !module.iter().any(|word| matches!(
            word.as_str(),
            "CONFIG" | "NO_MODULE" | "PATHS" | "NO_DEFAULT_PATH"
        )),
        "{module:?}"
    );
    let config = keywords((3, 31), "find_package", &["Qt", "CONFIG", ""]);
    assert!(!config.contains(&"MODULE".to_owned()), "{config:?}");
    assert!(config.contains(&"REQUIRED".to_owned()));
    let view = keywords((3, 31), "find_package", &["Qt", "REGISTRY_VIEW", ""]);
    assert_eq!(
        view,
        ["32", "32_64", "64", "64_32", "BOTH", "HOST", "TARGET"]
    );
    assert!(
        keywords(
            (3, 31),
            "find_program",
            &["EXEC", "NAMES", "tool", "DOC", ""]
        )
        .is_empty()
    );
    assert!(keywords((3, 31), "find_program", &["EXEC", "VALIDATOR", ""]).is_empty());
    assert_eq!(
        keywords((3, 31), "find_library", &["LIB", "HINTS", "E"]),
        ["ENV"]
    );
}
#[test]
fn keyword_lookup_version_boundaries_include_known_experimental_syntax() {
    for (name, prefix, args, before, after) in [
        ("project", "DESCRIPTION", vec!["App", "DE"], (3, 8), (3, 9)),
        (
            "project",
            "HOMEPAGE_URL",
            vec!["App", "HO"],
            (3, 11),
            (3, 12),
        ),
        (
            "project",
            "COMPAT_VERSION",
            vec!["App", "CO"],
            (4, 0),
            (4, 1),
        ),
        ("project", "SPDX_LICENSE", vec!["App", "SP"], (4, 1), (4, 2)),
        ("find_package", "GLOBAL", vec!["Qt", "G"], (3, 23), (3, 24)),
        (
            "find_package",
            "OPTIONAL",
            vec!["Qt", "OPTIONAL"],
            (4, 0),
            (4, 1),
        ),
        (
            "find_package",
            "UNWIND_INCLUDE",
            vec!["Qt", "UNW"],
            (4, 1),
            (4, 2),
        ),
        (
            "find_program",
            "REQUIRED",
            vec!["EXEC", "REQ"],
            (3, 17),
            (3, 18),
        ),
        (
            "find_file",
            "NO_CACHE",
            vec!["FILE", "NO_CAC"],
            (3, 20),
            (3, 21),
        ),
        (
            "find_path",
            "VALIDATOR",
            vec!["DIR", "VAL"],
            (3, 24),
            (3, 25),
        ),
        (
            "include",
            "NO_DIAGNOSTIC_SCOPE",
            vec!["Module", "NO_DIA"],
            (4, 3),
            (4, 4),
        ),
        ("if", "DIAGNOSTIC", vec!["DIA"], (4, 3), (4, 4)),
        (
            "block",
            "DIAGNOSTICS",
            vec!["SCOPE_FOR", "DIA"],
            (4, 3),
            (4, 4),
        ),
    ] {
        assert!(
            !keywords(before, name, &args).contains(&prefix.to_owned()),
            "{name} {before:?}"
        );
        assert!(
            keywords(after, name, &args).contains(&prefix.to_owned()),
            "{name} {after:?}"
        );
    }
}
#[test]
fn keyword_lookup_properties_conditions_and_foreach_keep_typed_positions() {
    assert_eq!(
        keywords((3, 31), "get_property", &["OUT", "SOURCE", "file.c", "TAR"]),
        ["TARGET_DIRECTORY"]
    );
    assert_eq!(
        completion_roles(&schema((3, 31), "get_property"), &["OUT", "TARGET", "Li"]),
        [NameRole::Target]
    );
    assert_eq!(
        keywords((3, 31), "set_property", &["TARGET", "Lib", "APP"]),
        ["APPEND", "APPEND_STRING"]
    );
    assert!(keywords((3, 31), "set_property", &["TARGET", "Lib", "PROPERTY", ""]).is_empty());
    assert_eq!(
        keywords((3, 31), "foreach", &["item", "IN", "ZI"]),
        ["ZIP_LISTS"]
    );
    assert_eq!(
        completion_roles(&schema((3, 31), "foreach"), &["item", "IN", "LISTS", "VA"]),
        [NameRole::Variable]
    );
    assert_eq!(
        keywords((3, 31), "if", &["value", "ST"]),
        [
            "STREQUAL",
            "STRGREATER",
            "STRGREATER_EQUAL",
            "STRLESS",
            "STRLESS_EQUAL"
        ]
    );
    assert_eq!(
        completion_roles(&schema((3, 31), "if"), &["TARGET", "Li"]),
        [NameRole::Target]
    );
    assert_eq!(
        keywords((3, 31), "message", &["CHE"]),
        ["CHECK_FAIL", "CHECK_PASS", "CHECK_START"]
    );
    assert!(keywords((3, 31), "message", &["STATUS", "CHE"]).is_empty());
    for command in ["if", "elseif", "while"] {
        assert_eq!(keywords((4, 4), command, &["DIA"]), ["DIAGNOSTIC"]);
        assert!(keywords((4, 4), command, &["DIAGNOSTIC", ""]).is_empty());
    }
}
#[test]
fn keyword_lookup_modern_cache_options_do_not_leak_into_normal_set_calls() {
    assert!(keywords((4, 1), "set", &["CACHE{VALUE}", "TY"]).is_empty());
    assert_eq!(keywords((4, 2), "set", &["CACHE{VALUE}", "TY"]), ["TYPE"]);
    assert_eq!(
        keywords((4, 2), "set", &["CACHE{VALUE}", "TYPE", ""]),
        ["BOOL", "FILEPATH", "INTERNAL", "PATH", "STRING"]
    );
    assert!(keywords((4, 2), "set", &["NORMAL", "TY"]).is_empty());
    assert_eq!(
        keywords(
            (4, 2),
            "set",
            &["CACHE{VALUE}", "HELP", "description", "VAL"]
        ),
        ["VALUE"]
    );
}
fn minimum(args: &[K]) -> Vec<String> {
    args.iter()
        .flat_map(|kind| match kind {
            K::Keyword(word) => vec![word.clone()],
            K::CacheEntry => vec!["CACHE{ENTRY}".into()],
            K::Grammar { args, .. } => minimum(args),
            K::Optional(_) => vec![],
            K::Repeat { args, min } => (0..*min).flat_map(|_| minimum(args)).collect(),
            K::Choice(groups) => groups.first().map_or_else(Vec::new, |group| minimum(group)),
            K::AllOf(args) => vec![
                args.iter()
                    .find_map(|kind| {
                        if let K::Keyword(word) = kind {
                            Some(word.clone())
                        } else {
                            None
                        }
                    })
                    .unwrap_or("operand".into()),
            ],
            _ => vec!["operand".into()],
        })
        .collect()
}
fn witnesses(args: &[K], mut prefix: Vec<String>, out: &mut Vec<(Vec<String>, String)>) {
    for kind in args {
        match kind {
            K::Keyword(word) => out.push((prefix.clone(), word.clone())),
            K::Grammar { args, .. } | K::Optional(args) | K::Repeat { args, .. } => {
                witnesses(args, prefix.clone(), out)
            }
            K::Choice(groups) => {
                for group in groups {
                    witnesses(group, prefix.clone(), out);
                }
            }
            _ => {}
        }
        prefix.extend(minimum(std::slice::from_ref(kind)));
    }
}
#[test]
fn keyword_lookup_every_literal_is_reachable_at_each_supported_version() {
    for version in (0..=31)
        .map(|minor| (3, minor))
        .chain((0..=4).map(|minor| (4, minor)))
    {
        for (name, args) in schemas(version) {
            let mut examples = Vec::new();
            witnesses(&args, Vec::new(), &mut examples);
            let schema = super::super::make_schema(name, args);
            for (mut prefix, word) in examples {
                prefix.push(word.clone());
                let args: Vec<_> = prefix.iter().map(String::as_str).collect();
                assert!(
                    completion_keywords(&schema, &args, || false).contains(&word.as_str()),
                    "{version:?} {name} {args:?}: {word}"
                );
            }
        }
    }
}
#[test]
fn keyword_lookup_effective_builtin_catalog_has_no_opaque_keyword_tails() {
    fn opaque(kind: &K) -> bool {
        match kind {
            K::Unmodeled | K::Action | K::Unknown => true,
            K::Optional(args)
            | K::AllOf(args)
            | K::Repeat { args, .. }
            | K::Grammar { args, .. } => args.iter().any(opaque),
            K::Choice(groups) => groups.iter().flatten().any(opaque),
            _ => false,
        }
    }
    for version in (0..=31)
        .map(|minor| (3, minor))
        .chain((0..=4).map(|minor| (4, minor)))
    {
        let schemas = builtin::schemas_for_version(version.0, version.1).unwrap();
        for (name, schema) in &schemas {
            assert!(
                !schema.args.iter().any(|arg| opaque(&arg.kind)),
                "{version:?}: {name}"
            );
        }
    }
}
#[test]
fn keyword_lookup_registered_version_layers_share_cached_schema_data() {
    for version in (0..=31)
        .map(|minor| (3, minor))
        .chain((0..=4).map(|minor| (4, minor)))
    {
        let schemas = builtin::schemas_for_version(version.0, version.1).unwrap();
        assert!(
            completion_keywords(&schemas["project"], &["App", "LA"], || false)
                .contains(&"LANGUAGES")
        );
        assert!(
            completion_keywords(&schemas["find_package"], &["Qt", "RE"], || false)
                .contains(&"REQUIRED")
        );
    }
    assert_eq!(builtin::schemas_for_version(4, 4).unwrap().len(), 132);
    let a = super::super::keyword_schemas::get(3, 31);
    let b = super::super::keyword_schemas::get(3, 31);
    assert!(std::ptr::eq(a, b));
    assert!(!a.is_empty());
    // Search hints remain generic paths, not required existing input-directory operands.
    let find = schema((3, 31), "find_file");
    fn has_input(kind: &K) -> bool {
        match kind {
            K::InputDirectory | K::InputFile => true,
            K::Optional(args)
            | K::AllOf(args)
            | K::Repeat { args, .. }
            | K::Grammar { args, .. } => args.iter().any(has_input),
            K::Choice(groups) => groups.iter().flatten().any(has_input),
            _ => false,
        }
    }
    assert!(!find.args.iter().any(|arg| has_input(&arg.kind)));
}
