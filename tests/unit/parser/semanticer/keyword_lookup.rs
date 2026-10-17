//! Private lookup/control schema tests attached by `schema::builtin::all_lookup`. Generated
//! argument prefixes check literal reachability and versioned type roles without CMake execution.

// Contains AI-generated documentation edits.
// AI-generated tests.
use super::schemas;
use crate::parser::semanticer::{
    parsing::{completion_keywords, completion_roles},
    schema::{ArgSchemaKind as K, CommandSchema, NameRole, builtin},
};
/// Find one enclosing-family grammar and wrap it with the shared builtin schema constructor.
/// Allocates schema data only; does not inspect the host or run CMake.
///
/// # Arguments
/// - `version` `[in]`: CMake (major, minor) selecting the grammar catalog.
/// - `name` `[in]`: Exact command name expected in that catalog.
///
/// # Returns
/// An owned CommandSchema from the shared make_schema helper, retaining the selected typed
/// arguments.
///
/// # Panics
/// If the requested command is absent at that version.
///
/// # Examples
/// Local test-module context:
/// ```text
/// let grammar = schema((4, 4), "project");
/// assert!(!grammar.args.is_empty());
/// ```
fn schema(version: (u32, u32), name: &str) -> CommandSchema {
    let (_, args) = schemas(version)
        .into_iter()
        .find(|(command, _)| *command == name)
        .unwrap();
    super::super::make_schema(name, args)
}
/// Probe argument keyword completions with cancellation disabled and own the returned words. Calls
/// [`schema`] and the production matcher; no process or file side effects.
///
/// # Arguments
/// - `version` `[in]`: CMake (major, minor) selecting the schema.
/// - `name` `[in]`: Exact command name in this grammar family.
/// - `args` `[in]`: Argument prefix through the active final slot; an empty final string means no
///   letters typed there.
///
/// # Returns
/// Sorted, deduplicated owned keyword strings; empty when no literal fits the active slot.
///
/// # Panics
/// If the requested command is absent at that version.
///
/// # Examples
/// Local test-module context:
/// ```text
/// assert_eq!(keywords((3, 31), "project", &["App", "LA"]), ["LANGUAGES"]);
/// ```
fn keywords(version: (u32, u32), name: &str, args: &[&str]) -> Vec<String> {
    completion_keywords(&schema(version, name), args, || false)
        .into_iter()
        .map(str::to_owned)
        .collect()
}
/// Probe project language/version/description and find_package required/component prefixes.
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
/// # Algorithm
/// Require real grammar continuations and no keyword in a required description operand.
///
/// # Flow
/// ```text
///   [project / package prefixes]
///    |
///   [probe actual schemas]----------------> [required description operand]
///                                          |
///                                         [check keyword or empty result]
///                                          |
///                                         < checks hold? >
///                                          /         \
///                                        [pass]     [panic]
/// ```
///
/// # Examples
/// Local test-module call:
/// ```text
/// keyword_lookup_project_and_package_examples_use_real_grammars();
/// ```
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
/// Contrast MODULE and CONFIG package modes, enumerate REGISTRY_VIEW values and suppress completion
/// inside DOC/VALIDATOR operands.
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
/// # Algorithm
/// A library HINTS prefix must still offer ENV.
///
/// # Flow
/// ```text
///   [MODULE / CONFIG forms]
///    |
///   [probe contextual choices]----------> [DOC / VALIDATOR / HINTS]
///                                        |
///                                       [check scalars and ENV]
///                                        |
///                                       < checks hold? >
///                                        /         \
///                                      [pass]     [panic]
/// ```
///
/// # Examples
/// Local test-module call:
/// ```text
/// keyword_lookup_modes_and_required_scalar_slots_are_contextual();
/// ```
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
/// Probe known project/package/search/include/condition/block keywords immediately before and at
/// introductions, including 4.x compatibility, license and diagnostic syntax.
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
/// # Algorithm
/// Probe known project/package/search/include/condition/block keywords immediately before and at
/// introductions, including 4.x compatibility, license and diagnostic syntax.
///
/// # Flow
/// ```text
///   [lookup introduction table]
///    |
///   [probe before version]---------------> [probe introduced version]
///                                         |
///                                        [check absence then presence]
///                                         |
///                                        < checks hold? >
///                                         /         \
///                                       [pass]     [panic]
/// ```
///
/// # Examples
/// Local test-module call:
/// ```text
/// keyword_lookup_version_boundaries_include_known_experimental_syntax();
/// ```
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
/// Probe property scopes, foreach list roles, condition target roles/string comparisons and message
/// modes.
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
/// # Algorithm
/// Require typed positions while excluding message-body and diagnostic-name scalars from keyword
/// suggestions.
///
/// # Flow
/// ```text
///   [property/loop/condition/message]
///    |
///   [probe keywords + roles]-------------------> [scalar body/name cases]
///                                               |
///                                              [check typed and empty positions]
///                                               |
///                                              < checks hold? >
///                                               /         \
///                                             [pass]     [panic]
/// ```
///
/// # Examples
/// Local test-module call:
/// ```text
/// keyword_lookup_properties_conditions_and_foreach_keep_typed_positions();
/// ```
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
/// Compare 4.1 and 4.2 CACHE{...} set forms. Require modern TYPE/HELP/VALUE continuations and
/// cache-type choices only for cache entries, never ordinary variable assignments.
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
/// keyword_lookup_modern_cache_options_do_not_leak_into_normal_set_calls();
/// ```
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
/// Build a representative minimal argument sequence for lookup witness generation. Omits optional
/// nodes, honors repeat minima and uses the first choice; changes no inputs or external state.
///
/// # Arguments
/// - `args` `[in]`: Schema sequence whose grammar wrappers and intersections need representative
///   operands.
///
/// # Returns
/// Owned argument strings: literal spellings, CACHE{ENTRY} for cache entries, a keyword
/// representative for intersections or operand for other scalar nodes.
///
/// # Algorithm
/// Recursively expand grammar wrappers, required repeats and the first choice; omit optionals. Use
/// one keyword from each AllOf intersection, or operand when none exists.
///
/// # Flow
/// ```text
///       [argument sequence]
///                |
///       +--------+---------+
///       |                  |
/// [required structure]  [optional]
///       |                  |
/// [expand representative] [omit]
///       |                  |
///       +--------+---------+
///                v
///          [owned strings]
/// ```
///
/// # Examples
/// Local test-module context:
/// ```text
/// assert_eq!(minimum(&[K::Keyword("NAME".into()), K::Value]), ["NAME", "operand"]);
/// ```
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
/// Append literal-position witnesses by walking structured grammar branches. Each witness pairs
/// prior representative arguments with the next keyword; no I/O or input mutation.
///
/// # Arguments
/// - `args` `[in]`: Argument sequence walked recursively through groups and alternatives.
/// - `prefix` `[in]`: Owned preceding argument strings; extended locally as each node is passed.
/// - `out` `[in,out]`: Existing witness vector; append (preceding arguments, keyword) pairs without
///   clearing earlier entries.
///
/// # Returns
/// `()` after appending witnesses. Prefixes exclude the active keyword; callers append that word
/// before probing completion.
///
/// # Algorithm
/// For each node, record a literal or visit every branch with a cloned prefix. Then extend the
/// local prefix with minimum representatives before visiting following siblings.
///
/// # Flow
/// ```text
///       [node + prior prefix]
///                 |
///        +--------+--------+
///        |                 |
///    [keyword]        [group/choice]
///        |                 |
/// [append witness]  [visit branches]
///        |                 |
///        +--------+--------+
///                 v
///      [extend prefix with minimum]
///                 |
///           [next sibling]
/// ```
///
/// # Examples
/// Local test-module context:
/// ```text
/// let mut examples = Vec::new();
/// witnesses(&[K::Keyword("NAME".into())], vec![], &mut examples);
/// assert_eq!(examples, vec![(vec![], "NAME".into())]);
/// ```
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
/// Walk lookup schema trees for CMake 3.0–3.31 and 4.0–4.4, constructing minimal prefixes for every
/// literal.
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
/// # Algorithm
/// Insert each literal at its witnessed position and require the real completion matcher to recover
/// it.
///
/// # Flow
/// ```text
///   [all supported lookup schemas]
///    |
///   [generate minimal witnesses]------------> [production prefix matcher]
///                                            |
///                                           [require every literal reachable]
///                                            |
///                                           < checks hold? >
///                                            /         \
///                                          [pass]     [panic]
/// ```
///
/// # Examples
/// Local test-module call:
/// ```text
/// keyword_lookup_every_literal_is_reachable_at_each_supported_version();
/// ```
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
/// Recursively inspect every effective builtin schema for every supported version, rejecting
/// Unmodeled, Action and Unknown nodes even inside alternatives or grammar wrappers.
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
/// # Algorithm
/// Recursively inspect every effective builtin schema for every supported version, rejecting
/// Unmodeled, Action and Unknown nodes even inside alternatives or grammar wrappers.
///
/// # Flow
/// ```text
///   [all effective builtin schemas]
///    |
///   [recurse groups + choices]---------------> [Unmodeled/Action/Unknown]
///                                             |
///                                            [reject any fallback descendant]
///                                             |
///                                            < checks hold? >
///                                             /         \
///                                           [pass]     [panic]
/// ```
///
/// # Examples
/// Local test-module call:
/// ```text
/// keyword_lookup_effective_builtin_catalog_has_no_opaque_keyword_tails();
/// ```
#[test]
fn keyword_lookup_effective_builtin_catalog_has_no_opaque_keyword_tails() {
    /// Detect an unstructured fallback anywhere below one schema node. Used only inside the
    /// effective-catalog test; reads data without side effects.
    ///
    /// # Arguments
    /// - `kind` `[in]`: Node inspected recursively through groups, grammar wrappers and choices.
    ///
    /// # Returns
    /// True for any Unmodeled, Action or Unknown descendant; false for fully modeled subtrees.
    ///
    /// # Algorithm
    /// Return true for a fallback leaf. For each container, stop when any descendant is opaque;
    /// otherwise return false.
    ///
    /// # Flow
    /// ```text
    ///    [node]
    ///        |
    ///  < fallback leaf? >
    ///    /          \
    /// [true]     [container?]
    ///                |
    ///        [any opaque child?]
    ///            /         \
    ///         [true]     [false]
    /// ```
    ///
    /// # Examples
    /// Local test-module context:
    /// ```text
    /// assert!(opaque(&K::Optional(vec![K::Unmodeled])));
    /// ```
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
/// Probe project/package completions across all supported versions, check 132 effective schemas at
/// 4.4 and require repeated 3.31 keyword-layer lookup to return the same allocation.
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
/// # Algorithm
/// Recursively verify find_file hints are not required existing input paths.
///
/// # Flow
/// ```text
///   [all supported versions]
///    |
///   [probe project/package keywords]----------> [repeat cached-layer lookup]
///                                              |
///                                             [check identity + generic hints]
///                                              |
///                                             < checks hold? >
///                                              /         \
///                                            [pass]     [panic]
/// ```
///
/// # Examples
/// Local test-module call:
/// ```text
/// keyword_lookup_registered_version_layers_share_cached_schema_data();
/// ```
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
    /// Detect required existing-file/directory roles in a schema subtree. The cache test uses this to
    /// keep search hints generic; no side effects.
    ///
    /// # Arguments
    /// - `kind` `[in]`: Node inspected through optional/repeated/intersection/grammar groups and
    ///   choices.
    ///
    /// # Returns
    /// True if any InputDirectory or InputFile node occurs; false for generic paths and other roles.
    ///
    /// # Algorithm
    /// Return true for an InputFile/InputDirectory leaf. For containers, stop when any child has an
    /// input role; generic paths and other scalar nodes return false.
    ///
    /// # Flow
    /// ```text
    ///    [node]
    ///        |
    /// < required input? >
    ///    /          \
    /// [true]     [container?]
    ///                |
    ///       [any input descendant?]
    ///           /         \
    ///        [true]     [false]
    /// ```
    ///
    /// # Examples
    /// Local test-module context:
    /// ```text
    /// assert!(has_input(&K::InputFile));
    /// assert!(!has_input(&K::Path));
    /// ```
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
