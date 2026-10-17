//! Build native command catalogues for CMake 3.0–3.31 and 4.0–4.4.
//! Construction allocates strings/vectors and initializes lazy caches only; no
//! project command, compiler, filesystem probe or network request is executed.
//! [`schemas_for_version`] materializes owned trees for analysis. Completion
//! borrows the same ordered layers through `schema_layers`, resolving duplicate
//! names by keeping the last entry. Early base trees may be incomplete; the final
//! cached keyword layer replaces them with structured grammars without adding
//! names before their registration version. Scalars remain roles, not validators.
//!
//! ```text
//! [3.0 base]       [Version request]
//!      |                  |
//!      v                  v
//! [Eligible 3.x/4.x replacements]
//!      |                  |
//!      +--------+---------+
//!               v
//! [Registered-name-filtered keyword trees]
//!               |
//!       +-------+--------+
//!       v                v
//! [Borrow layers]   [Clone into map]
//!  completion         analysis
//! ```
// AI-generated implementation.
mod all_build;
mod all_lookup;
mod all_process;
mod all_transform;
mod find_package;
mod keyword_schemas;
mod project;
use super::{ArgSchema, ArgSchemaKind, CommandSchema, CommandSchemaFrom};
use crate::parser::semanticer::model::{BuiltinCommand, CommandKind};
use ArgSchemaKind::{
    Action, BindValue, Boolean, Command, Condition, Expression, InputDirectory, InputFile,
    InputPath, Integer, LanguageType, List, ModulePath, Number, Out, OutputPath, PackageName, Path,
    Property, String as Text, Target, TargetExe, TargetLib, TestName, Unmodeled, Unset, Url, Value,
    Variable, Version,
};
use std::{collections::HashMap, sync::LazyLock};

/// Oldest supported major/minor pair; completion uses it as its fallback.
/// Example: `assert_eq!(MIN_VERSION, (3, 0));`.
pub const MIN_VERSION: (u32, u32) = (3, 0);
/// Newest supported major/minor pair; does not include unknown future minors.
/// Example: `assert_eq!(MAX_VERSION, (4, 4));`.
pub const MAX_VERSION: (u32, u32) = (4, 4);
/// Native control-flow/declaration names classified as keywords by `make_schema`.
/// Example: `KEYWORD_NAMES.contains(&"if")` is true.
static KEYWORD_NAMES: &[&str] = &[
    "block",
    "break",
    "continue",
    "else",
    "elseif",
    "endblock",
    "endforeach",
    "endfunction",
    "endif",
    "endmacro",
    "endwhile",
    "foreach",
    "function",
    "if",
    "macro",
    "return",
    "while",
];

/// Lazy 3.0 registration and initial signatures; some tails need the final keyword layer.
/// Consumers should prefer [`schemas_for_version`] for effective trees.
/// Example: `BASE_SCHEMAS.iter().any(|schema| schema.command_name == "file")`.
pub static BASE_SCHEMAS: LazyLock<Vec<CommandSchema>> = LazyLock::new(|| {
    vec![
        ("add_compile_options", vec![many(Text)]),
        (
            "add_custom_command",
            vec![choice(vec![
                vec![k("OUTPUT"), some(OutputPath), Unmodeled],
                vec![
                    k("TARGET"),
                    Target,
                    one_of(&["PRE_BUILD", "PRE_LINK", "POST_BUILD"]),
                    Unmodeled,
                ],
            ])],
        ),
        ("add_custom_target", vec![Target, Unmodeled]),
        ("add_definitions", vec![many(Text)]),
        ("add_dependencies", vec![Target, some(Target)]),
        ("add_executable", executable_args(false)),
        ("add_library", library_args(false, false)),
        (
            "add_subdirectory",
            vec![
                InputDirectory,
                optional(vec![OutputPath]),
                optional(vec![k("EXCLUDE_FROM_ALL")]),
            ],
        ),
        (
            "add_test",
            vec![choice(vec![
                vec![TestName, Command, many(Value)],
                vec![
                    k("NAME"),
                    TestName,
                    k("COMMAND"),
                    Command,
                    many(Value),
                    Unmodeled,
                ],
            ])],
        ),
        ("aux_source_directory", vec![InputDirectory, Out]),
        ("break", vec![]),
        ("build_command", vec![Out, Unmodeled]),
        ("build_name", vec![Out]),
        ("cmake_host_system_information", vec![Unmodeled]),
        (
            "cmake_minimum_required",
            vec![k("VERSION"), Version, optional(vec![k("FATAL_ERROR")])],
        ),
        (
            "cmake_policy",
            vec![choice(vec![
                vec![k("VERSION"), Version],
                vec![k("SET"), Text, one_of(&["NEW", "OLD"])],
                vec![k("GET"), Text, Out],
                vec![one_of(&["PUSH", "POP"])],
            ])],
        ),
        (
            "configure_file",
            vec![
                InputFile,
                OutputPath,
                options(vec![
                    vec![k("COPYONLY")],
                    vec![k("ESCAPE_QUOTES")],
                    vec![k("@ONLY")],
                    vec![
                        k("NEWLINE_STYLE"),
                        one_of(&["UNIX", "DOS", "WIN32", "LF", "CRLF"]),
                    ],
                ]),
            ],
        ),
        (
            "create_test_sourcelist",
            vec![Out, OutputPath, some(InputFile), Unmodeled],
        ),
        ("ctest_build", vec![Unmodeled]),
        ("ctest_configure", vec![Unmodeled]),
        ("ctest_coverage", vec![Unmodeled]),
        ("ctest_empty_binary_directory", vec![Path]),
        ("ctest_memcheck", vec![Unmodeled]),
        ("ctest_read_custom_files", vec![some(InputDirectory)]),
        (
            "ctest_run_script",
            vec![optional(vec![k("NEW_PROCESS")]), some(InputFile)],
        ),
        (
            "ctest_sleep",
            vec![choice(vec![vec![Number], vec![Number, Number, Number]])],
        ),
        (
            "ctest_start",
            vec![
                one_of(&["Experimental", "Continuous", "Nightly"]),
                Unmodeled,
            ],
        ),
        ("ctest_submit", vec![Unmodeled]),
        ("ctest_test", vec![Unmodeled]),
        ("ctest_update", vec![Unmodeled]),
        ("ctest_upload", vec![k("FILES"), some(InputFile), Unmodeled]),
        (
            "define_property",
            vec![
                one_of(&[
                    "GLOBAL",
                    "DIRECTORY",
                    "TARGET",
                    "SOURCE",
                    "TEST",
                    "VARIABLE",
                    "CACHED_VARIABLE",
                ]),
                k("PROPERTY"),
                Property,
                Unmodeled,
            ],
        ),
        ("else", vec![optional(vec![many(Condition)])]),
        ("elseif", vec![some(Condition)]),
        (
            "enable_language",
            vec![some(LanguageType), optional(vec![k("OPTIONAL")])],
        ),
        ("enable_testing", vec![]),
        ("endforeach", vec![optional(vec![Variable])]),
        ("endfunction", vec![optional(vec![Command])]),
        ("endif", vec![optional(vec![many(Condition)])]),
        ("endmacro", vec![optional(vec![Command])]),
        ("endwhile", vec![optional(vec![many(Condition)])]),
        ("exec_program", vec![Command, Unmodeled]),
        ("execute_process", vec![Unmodeled]),
        ("export", vec![Unmodeled]),
        (
            "export_library_dependencies",
            vec![Path, optional(vec![k("APPEND")])],
        ),
        ("file", file_args()),
        ("find_file", vec![Out, Unmodeled]),
        ("find_library", vec![Out, Unmodeled]),
        (
            "find_package",
            vec![PackageName, optional(vec![Version]), Unmodeled],
        ),
        ("find_path", vec![Out, Unmodeled]),
        ("find_program", vec![Out, Unmodeled]),
        ("fltk_wrap_ui", vec![Target, some(InputFile)]),
        (
            "foreach",
            vec![
                Out,
                choice(vec![
                    vec![
                        k("RANGE"),
                        Integer,
                        optional(vec![Integer, optional(vec![Integer])]),
                    ],
                    vec![k("IN"), Unmodeled],
                    vec![many(Value)],
                ]),
            ],
        ),
        ("function", vec![Command, many(Variable)]),
        ("get_cmake_property", vec![Out, Property]),
        (
            "get_directory_property",
            vec![
                Out,
                optional(vec![k("DIRECTORY"), Path]),
                choice(vec![vec![Property], vec![k("DEFINITION"), Variable]]),
            ],
        ),
        (
            "get_filename_component",
            vec![
                Out,
                Path,
                one_of(&[
                    "DIRECTORY",
                    "NAME",
                    "EXT",
                    "NAME_WE",
                    "PATH",
                    "ABSOLUTE",
                    "REALPATH",
                    "PROGRAM",
                ]),
                Unmodeled,
            ],
        ),
        (
            "get_property",
            vec![
                Out,
                one_of(&[
                    "GLOBAL",
                    "DIRECTORY",
                    "TARGET",
                    "SOURCE",
                    "TEST",
                    "CACHE",
                    "VARIABLE",
                ]),
                Unmodeled,
            ],
        ),
        ("get_source_file_property", vec![Out, Path, Property]),
        ("get_target_property", vec![Out, Target, Property]),
        ("get_test_property", vec![TestName, Property, Out]),
        ("if", vec![some(Condition)]),
        (
            "include",
            vec![
                ModulePath,
                options(vec![
                    vec![k("OPTIONAL")],
                    vec![k("RESULT_VARIABLE"), Out],
                    vec![k("NO_POLICY_SCOPE")],
                ]),
            ],
        ),
        (
            "include_directories",
            vec![
                optional(vec![one_of(&["AFTER", "BEFORE"])]),
                optional(vec![k("SYSTEM")]),
                many(InputDirectory),
            ],
        ),
        (
            "include_external_msproject",
            vec![Target, InputFile, Unmodeled],
        ),
        (
            "include_regular_expression",
            vec![Expression, optional(vec![Expression])],
        ),
        ("install", vec![Unmodeled]),
        ("install_files", vec![Path, Unmodeled]),
        ("install_programs", vec![Path, Unmodeled]),
        ("install_targets", vec![Path, Unmodeled]),
        ("link_directories", vec![many(Path)]),
        (
            "link_libraries",
            vec![many(choice(vec![
                vec![one_of(&["debug", "optimized", "general"]), Value],
                vec![Value],
            ]))],
        ),
        ("list", list_args(0)),
        ("load_cache", vec![InputDirectory, Unmodeled]),
        ("load_command", vec![Command, some(Path)]),
        ("macro", vec![Command, many(Variable)]),
        ("make_directory", vec![OutputPath]),
        (
            "mark_as_advanced",
            vec![optional(vec![one_of(&["CLEAR", "FORCE"])]), some(Variable)],
        ),
        ("math", vec![k("EXPR"), Out, Expression]),
        (
            "message",
            vec![
                optional(vec![one_of(&[
                    "FATAL_ERROR",
                    "SEND_ERROR",
                    "WARNING",
                    "AUTHOR_WARNING",
                    "STATUS",
                ])]),
                many(Text),
            ],
        ),
        (
            "option",
            vec![
                Out,
                Text,
                optional(vec![ArgSchemaKind::AllOf(vec![
                    BindValue { var_place: 0 },
                    Boolean,
                ])]),
            ],
        ),
        ("output_required_files", vec![InputFile, OutputPath]),
        ("project", vec![Text, Unmodeled]),
        ("qt_wrap_cpp", vec![Target, Out, some(InputFile)]),
        ("qt_wrap_ui", vec![Target, Out, Out, some(InputFile)]),
        ("remove", vec![Variable, some(Value)]),
        ("remove_definitions", vec![many(Text)]),
        ("return", vec![]),
        (
            "separate_arguments",
            vec![choice(vec![
                vec![Variable],
                vec![Out, one_of(&["UNIX_COMMAND", "WINDOWS_COMMAND"]), Text],
            ])],
        ),
        (
            "set",
            vec![
                Out,
                choice(vec![
                    vec![many(BindValue { var_place: 0 })],
                    vec![many(BindValue { var_place: 0 }), k("PARENT_SCOPE")],
                    vec![
                        many(BindValue { var_place: 0 }),
                        k("CACHE"),
                        one_of(&["BOOL", "FILEPATH", "PATH", "STRING", "INTERNAL"]),
                        Text,
                        optional(vec![k("FORCE")]),
                    ],
                ]),
            ],
        ),
        ("set_directory_properties", vec![k("PROPERTIES"), pairs()]),
        ("set_property", vec![Unmodeled]),
        (
            "set_source_files_properties",
            vec![some(InputFile), k("PROPERTIES"), pairs()],
        ),
        (
            "set_target_properties",
            vec![some(Target), k("PROPERTIES"), pairs()],
        ),
        (
            "set_tests_properties",
            vec![some(TestName), k("PROPERTIES"), pairs()],
        ),
        ("site_name", vec![Out]),
        (
            "source_group",
            vec![
                Text,
                choice(vec![
                    vec![Expression],
                    vec![options(vec![
                        vec![k("REGULAR_EXPRESSION"), Expression],
                        vec![k("FILES"), some(InputFile)],
                    ])],
                ]),
            ],
        ),
        ("string", string_args((3, 0))),
        ("subdir_depends", vec![Path, some(Path)]),
        ("subdirs", vec![Unmodeled]),
        ("target_compile_definitions", target_sections(vec![], Text)),
        (
            "target_compile_options",
            target_sections(vec![optional(vec![k("BEFORE")])], Text),
        ),
        (
            "target_include_directories",
            target_sections(
                vec![
                    optional(vec![k("SYSTEM")]),
                    optional(vec![one_of(&["AFTER", "BEFORE"])]),
                ],
                InputDirectory,
            ),
        ),
        (
            "target_link_libraries",
            vec![
                Target,
                many(choice(vec![
                    vec![one_of(&[
                        "PRIVATE",
                        "PUBLIC",
                        "INTERFACE",
                        "LINK_PRIVATE",
                        "LINK_PUBLIC",
                        "LINK_INTERFACE_LIBRARIES",
                        "debug",
                        "optimized",
                        "general",
                    ])],
                    vec![Target],
                ])),
            ],
        ),
        ("try_compile", vec![Out, Unmodeled]),
        ("try_run", vec![Out, Out, Unmodeled]),
        (
            "unset",
            vec![Unset, optional(vec![one_of(&["CACHE", "PARENT_SCOPE"])])],
        ),
        ("use_mangled_mesa", vec![Path, Path]),
        ("utility_source", vec![Out, Text, Path, some(Path)]),
        ("variable_requires", vec![Variable, Out, some(Variable)]),
        ("variable_watch", vec![Variable, optional(vec![Command])]),
        ("while", vec![some(Condition)]),
        (
            "write_file",
            vec![OutputPath, Text, optional(vec![k("APPEND")])],
        ),
    ]
    .into_iter()
    .map(|(name, args)| make_schema(name, args))
    .collect()
});

/// Lazy 3.x additions/replacements paired with their introduction minor.
/// Order matters for repeated names; final keyword overrides are applied afterward.
/// Example: `CMAKE_3_ADDITIONS.iter().any(|(minor, schema)| *minor == 25 && schema.command_name
/// == "block")`.
pub static CMAKE_3_ADDITIONS: LazyLock<Vec<(u32, CommandSchema)>> = LazyLock::new(|| {
    vec![
        (1, "target_compile_features", target_sections(vec![], Text)),
        (1, "target_sources", target_sections(vec![], InputFile)),
        (1, "string", string_args((3, 1))),
        (2, "continue", vec![]),
        (4, "string", string_args((3, 4))),
        (6, "list", list_args(6)),
        (5, "cmake_parse_arguments", parse_arguments(false, false)),
        (7, "cmake_parse_arguments", parse_arguments(true, false)),
        (7, "string", string_args((3, 7))),
        (
            10,
            "include_guard",
            vec![optional(vec![one_of(&["DIRECTORY", "GLOBAL"])])],
        ),
        (11, "add_executable", executable_args(true)),
        (11, "add_library", library_args(true, false)),
        (10, "string", string_args((3, 10))),
        (12, "add_compile_definitions", vec![many(Text)]),
        (12, "string", string_args((3, 12))),
        (12, "list", list_args(12)),
        (13, "list", list_args(13)),
        (15, "list", list_args(15)),
        (15, "string", string_args((3, 15))),
        (13, "add_link_options", vec![many(Text)]),
        (
            13,
            "link_directories",
            vec![optional(vec![one_of(&["AFTER", "BEFORE"])]), many(Path)],
        ),
        (
            13,
            "target_link_directories",
            target_sections(vec![optional(vec![k("BEFORE")])], Path),
        ),
        (
            13,
            "target_link_options",
            target_sections(vec![optional(vec![k("BEFORE")])], Text),
        ),
        (
            16,
            "target_precompile_headers",
            vec![choice(vec![
                target_sections(vec![], Path),
                vec![Target, k("REUSE_FROM"), Target],
            ])],
        ),
        (18, "cmake_language", vec![Unmodeled]),
        (18, "list", list_args(18)),
        (18, "string", string_args((3, 18))),
        (19, "string", string_args((3, 19))),
        (
            18,
            "get_source_file_property",
            vec![
                Out,
                Path,
                optional(vec![choice(vec![
                    vec![k("DIRECTORY"), Path],
                    vec![k("TARGET_DIRECTORY"), Target],
                ])]),
                Property,
            ],
        ),
        (19, "add_library", library_args(true, true)),
        (20, "cmake_path", vec![Unmodeled]),
        (
            25,
            "block",
            vec![options(vec![
                vec![k("SCOPE_FOR"), some(one_of(&["POLICIES", "VARIABLES"]))],
                vec![k("PROPAGATE"), some(Variable)],
            ])],
        ),
        (25, "endblock", vec![]),
        (
            25,
            "return",
            vec![optional(vec![k("PROPAGATE"), many(Variable)])],
        ),
        (27, "cmake_file_api", vec![k("QUERY"), Unmodeled]),
        (
            31,
            "cmake_pkg_config",
            vec![
                one_of(&["EXTRACT", "POPULATE", "IMPORT"]),
                PackageName,
                Unmodeled,
            ],
        ),
    ]
    .into_iter()
    .map(|(minor, name, args)| (minor, make_schema(name, args)))
    .collect()
});

/// Lazy 4.x additions/replacements paired with their introduction minor.
/// A 4.x request also inherits all eligible 3.x entries.
/// Example: `CMAKE_4_ADDITIONS.iter().any(|(minor, schema)| *minor == 4 && schema.command_name ==
/// "discover_tests")`.
pub static CMAKE_4_ADDITIONS: LazyLock<Vec<(u32, CommandSchema)>> = LazyLock::new(|| {
    vec![
        (1, "add_dependencies", vec![Target, many(Target)]),
        (2, "string", string_args((4, 2))),
        (3, "cmake_instrumentation", vec![Unmodeled]),
        (
            4,
            "cmake_diagnostic",
            vec![choice(vec![
                vec![k("SET"), Text, Text, optional(vec![k("RECURSE")])],
                vec![
                    one_of(&["PROMOTE", "DEMOTE"]),
                    Text,
                    Text,
                    optional(vec![k("NO_RECURSE")]),
                ],
                vec![k("GET"), Text, Out],
                vec![one_of(&["PUSH", "POP"])],
            ])],
        ),
        (4, "discover_tests", vec![Unmodeled]),
        (4, "cmake_parse_arguments", parse_arguments(true, true)),
    ]
    .into_iter()
    .map(|(minor, name, args)| (minor, make_schema(name, args)))
    .collect()
});

/// Borrow ordered native and final keyword trees for one supported version.
/// Initializes lazy data on demand; no project commands or I/O.
///
/// # Arguments
/// - `major` `[in]`: requested CMake major version.
/// - `minor` `[in]`: requested minor version; patch versions do not affect selection.
///
/// # Returns
/// `Some(iterator)` for 3.0–3.31 or 4.0–4.4; `None` otherwise. Duplicate
/// command names are intentional: consumers must keep the last occurrence.
///
/// # Algorithm
/// Reject unsupported versions, then chain native layers and cached structured
/// keyword overrides. Unlike the owned map, this does not clone grammar trees.
///
/// # Flow
/// ```text
///           [major, minor]
///                  |
///            < supported? >
///              /       \
///             v         v
///          [No]       [Yes]
///             |         |
///          [None]  [Native layers]
///                       |
///                  [Keyword overrides]
///                       |
///                  [Borrowed iterator]
/// ```
/// # Examples
/// ```text
/// let layers = schema_layers(4, 4).unwrap();
/// let file = layers.filter(|schema| schema.command_name == "file").last().unwrap();
/// assert!(!file.args.is_empty());
/// ```
pub(crate) fn schema_layers(
    major: u32,
    minor: u32,
) -> Option<impl Iterator<Item = &'static CommandSchema>> {
    if !matches!((major, minor), (3, 0..=31) | (4, 0..=4)) {
        return None;
    }
    Some(native_layers(major, minor).chain(keyword_schemas::get(major, minor).iter()))
}
/// Borrow base registrations followed by eligible 3.x and 4.x replacements.
/// Only lazy catalogue initialization occurs; no project commands or I/O.
///
/// # Arguments
/// - `major` `[in]`: comparison major; this internal helper does not validate support.
/// - `minor` `[in]`: comparison minor.
///
/// # Returns
/// Iterator over static schemas, retaining duplicate names and source order.
/// It omits final keyword overrides; use `schema_layers` for effective trees.
///
/// # Algorithm
/// Chain the base with entries whose introduction pair is no newer than the
/// request. Tuple comparison makes a 4.x request inherit all 3.x additions.
///
/// # Flow
/// ```text
///  [Base]        [3.x entries]       [4.x entries]
///     |               |                  |
///     |        [Filter by pair]   [Filter by pair]
///     |               |                  |
///     +---------------+------------------+
///                     v
///           [Ordered borrowed layers]
/// ```
/// # Examples
/// ```text
/// let registered = native_layers(3, 25).any(|schema| schema.command_name == "block");
/// assert!(registered);
/// ```
fn native_layers(major: u32, minor: u32) -> impl Iterator<Item = &'static CommandSchema> {
    BASE_SCHEMAS
        .iter()
        .chain(
            CMAKE_3_ADDITIONS
                .iter()
                .filter_map(move |(introduced, schema)| {
                    ((3, *introduced) <= (major, minor)).then_some(schema)
                }),
        )
        .chain(
            CMAKE_4_ADDITIONS
                .iter()
                .filter_map(move |(introduced, schema)| {
                    ((4, *introduced) <= (major, minor)).then_some(schema)
                }),
        )
}
/// Materialize an owned effective command catalogue for semantic analysis.
/// Allocates/clones trees and initializes lazy caches; no project commands or I/O.
///
/// # Arguments
/// - `major` `[in]`: CMake major, either 3 or 4 for supported requests.
/// - `minor` `[in]`: CMake minor within 0–31 for major 3 or 0–4 for major 4.
///
/// # Returns
/// `Some(map)` keyed by lowercase native names for a supported version;
/// `None` for every unsupported pair. Later layers overwrite earlier signatures
/// in the new map, not in the shared source catalogues.
///
/// # Algorithm
/// Read the same ordered layers used by completion. Clone each schema into a
/// fresh map under its name; repeated names replace earlier values. Patch
/// versions, policy settings and experimental feature activation are not inputs.
///
/// # Flow
/// ```text
///               [schema_layers]
///                      |
///               < supported? >
///                 /        \
///                v          v
///             [None]   [Fresh map]
///                           |
///                   +-------v-------+
///                   | Next schema?  |<----+
///                   +-------+-------+     |
///                       yes |             |
///                           v             |
///                  [Clone and overwrite]-+
///                       no  |
///                           v
///                      [Some(map)]
/// ```
/// # Examples
/// ```
/// use cmake_analyzer::parser::semanticer::schema::builtin::schemas_for_version;
/// let baseline = schemas_for_version(3, 0).unwrap();
/// assert!(!baseline.contains_key("block"));
/// let modern = schemas_for_version(3, 25).unwrap();
/// assert!(modern.contains_key("block"));
/// assert!(schemas_for_version(5, 0).is_none());
/// ```
pub fn schemas_for_version(major: u32, minor: u32) -> Option<HashMap<String, CommandSchema>> {
    let layers = schema_layers(major, minor)?;
    let mut schemas = HashMap::with_capacity(
        BASE_SCHEMAS.len() + CMAKE_3_ADDITIONS.len() + CMAKE_4_ADDITIONS.len(),
    );
    for schema in layers {
        schemas.insert(schema.command_name.clone(), schema.clone());
    }
    Some(schemas)
}
/// Wrap root kinds and classify a native command by its registered name.
/// Allocates the name and root wrappers only; no commands or I/O.
///
/// # Arguments
/// - `name` `[in]`: lowercase native name; membership in `KEYWORD_NAMES` chooses keyword
///   classification.
/// - `args` `[in]`: owned ordered roots moved into `ArgSchema` wrappers.
///
/// # Returns
/// Owned schema with matching origin and builtin command kind. Input trees are
/// not validated and the name is not lowercased here.
///
/// # Examples
/// ```text
/// let schema = make_schema("if", vec![Condition]);
/// assert!(matches!(schema.from, CommandSchemaFrom::KeyWord));
/// ```
fn make_schema(name: &str, args: Vec<ArgSchemaKind>) -> CommandSchema {
    let keyword = KEYWORD_NAMES.contains(&name);
    CommandSchema {
        args: args.into_iter().map(|kind| ArgSchema { kind }).collect(),
        command_name: name.to_owned(),
        from: if keyword {
            CommandSchemaFrom::KeyWord
        } else {
            CommandSchemaFrom::Builtin
        },
        kind: CommandKind::Builtin(if keyword {
            BuiltinCommand::KeyWord
        } else {
            BuiltinCommand::Other
        }),
    }
}
/// Own an exact literal token for structural matching; allocates one string only.
/// No project commands or I/O.
///
/// # Arguments
/// - `word` `[in]`: required case-sensitive spelling, such as `READ`.
///
/// # Returns
/// A `Keyword` node containing a copy of `word`.
///
/// # Examples
/// ```text
/// let read = k("READ");
/// assert!(matches!(read, ArgSchemaKind::Keyword(word) if word == "READ"));
/// ```
fn k(word: &str) -> ArgSchemaKind {
    ArgSchemaKind::Keyword(word.to_owned())
}
/// Wrap a whole ordered sequence that may be skipped; no external side effects.
///
/// # Arguments
/// - `args` `[in]`: owned children consumed together when present.
///
/// # Returns
/// An `Optional` node; does not make each child independently optional.
///
/// # Examples
/// ```text
/// let result = optional(vec![k("RESULT_VARIABLE"), Out]);
/// ```
fn optional(args: Vec<ArgSchemaKind>) -> ArgSchemaKind {
    ArgSchemaKind::Optional(args)
}
/// Allow zero or more occurrences of one node; allocates a child vector only.
/// No project commands or I/O.
///
/// # Arguments
/// - `kind` `[in]`: owned operand or group to repeat.
///
/// # Returns
/// A `Repeat` node with minimum zero.
///
/// # Examples
/// ```text
/// let files = many(InputFile); // An empty source list is structurally accepted.
/// ```
fn many(kind: ArgSchemaKind) -> ArgSchemaKind {
    ArgSchemaKind::Repeat {
        args: vec![kind],
        min: 0,
    }
}
/// Require one or more occurrences of one node; allocates a child vector only.
/// No project commands or I/O.
///
/// # Arguments
/// - `kind` `[in]`: owned operand or group to repeat.
///
/// # Returns
/// A `Repeat` node with minimum one.
///
/// # Examples
/// ```text
/// let files = some(InputFile); // At least one source token is required.
/// ```
fn some(kind: ArgSchemaKind) -> ArgSchemaKind {
    ArgSchemaKind::Repeat {
        args: vec![kind],
        min: 1,
    }
}
/// Keep alternative ordered sequences without choosing an overload now.
/// Only owns the supplied vectors; no external side effects.
///
/// # Arguments
/// - `args` `[in]`: owned alternatives; each inner vector is one sequence.
///
/// # Returns
/// A `Choice` node for the structural matcher to explore.
///
/// # Examples
/// ```text
/// let input = choice(vec![vec![k("INPUT"), InputFile], vec![k("CONTENT"), Text]]);
/// ```
fn choice(args: Vec<Vec<ArgSchemaKind>>) -> ArgSchemaKind {
    ArgSchemaKind::Choice(args)
}
/// Build alternatives for exact single-token spellings; allocates grammar data only.
/// No commands or I/O.
///
/// # Arguments
/// - `words` `[in]`: permitted case-sensitive words, borrowed only during construction.
///
/// # Returns
/// A choice with one keyword node per branch; an empty slice yields no alternatives.
///
/// # Examples
/// ```text
/// let visibility = one_of(&["PUBLIC", "PRIVATE", "INTERFACE"]);
/// ```
fn one_of(words: &[&str]) -> ArgSchemaKind {
    choice(words.iter().map(|word| vec![k(word)]).collect())
}
/// Allow any number of flags from a literal vocabulary; allocates grammar data only.
/// No commands or I/O. Duplicate flags are not rejected by this tree.
///
/// # Arguments
/// - `words` `[in]`: case-sensitive flag spellings.
///
/// # Returns
/// Zero-or-more repetition of a one-keyword choice.
///
/// # Examples
/// ```text
/// let flags = flags(&["WIN32", "EXCLUDE_FROM_ALL"]);
/// ```
fn flags(words: &[&str]) -> ArgSchemaKind {
    many(one_of(words))
}
/// Allow repeated option groups in arbitrary order; allocates grammar data only.
/// No commands or I/O. This does not enforce uniqueness or option dependencies.
///
/// # Arguments
/// - `args` `[in]`: owned alternative sequences, usually a keyword followed by its operands.
///
/// # Returns
/// Zero-or-more repetition of a choice of complete groups.
///
/// # Examples
/// ```text
/// let opts = options(vec![vec![k("TIMEOUT"), Number], vec![k("OUTPUT_VARIABLE"), Out]]);
/// ```
fn options(args: Vec<Vec<ArgSchemaKind>>) -> ArgSchemaKind {
    many(choice(args))
}
/// Describe one or more property/value pairs; allocates grammar data only.
/// No commands or I/O.
///
/// # Arguments
/// None.
///
/// # Returns
/// A repetition of `Property, Value` with minimum one pair.
///
/// # Examples
/// ```text
/// let properties = pairs(); // For PROPERTIES POSITION_INDEPENDENT_CODE ON.
/// ```
fn pairs() -> ArgSchemaKind {
    ArgSchemaKind::Repeat {
        args: vec![Property, Value],
        min: 1,
    }
}
/// Build a target followed by modifiers and required visibility sections.
/// Allocates grammar data only; no project commands or I/O.
///
/// # Arguments
/// - `prefix` `[in]`: owned modifier nodes inserted after the target name.
/// - `operand` `[in]`: owned scalar/group moved into the repeated section-content node.
///
/// # Returns
/// An ordered target sequence ending in one-or-more visibility groups; each group allows zero
/// operands.
///
/// # Algorithm
/// Start with a Target, append the supplied prefix, then repeat PUBLIC/PRIVATE/INTERFACE plus a
/// possibly empty operand list.
///
/// # Flow
/// ```text
///             [Target + prefix]
///                     |
///            +--------+--------+
///            v                 v
///  [Visibility group]   [Repeated operands]
///            |                 |
///            +--------+--------+
///                     v
///              [Owned grammar]
/// ```
/// # Examples
/// ```text
/// let args = target_sections(vec![optional(vec![k("BEFORE")])], Text);
/// ```
fn target_sections(prefix: Vec<ArgSchemaKind>, operand: ArgSchemaKind) -> Vec<ArgSchemaKind> {
    let mut args = vec![Target];
    args.extend(prefix);
    args.push(ArgSchemaKind::Repeat {
        args: vec![one_of(&["INTERFACE", "PUBLIC", "PRIVATE"]), many(operand)],
        min: 1,
    });
    args
}
/// Build the initial add_executable overloads used by base/version registration.
/// Allocates grammar data only; no project commands or I/O.
///
/// # Arguments
/// - `modern` `[in]`: whether to allow empty source lists and add the alias overload (3.11
///   registration).
///
/// # Returns
/// A target name and choice of source, imported or eligible alias forms; final keyword layers may
/// replace this seed.
///
/// # Algorithm
/// Choose source cardinality from modern, retain imported flags, and append the alias alternative
/// only in modern mode.
///
/// # Flow
/// ```text
///             [modern?]
///                     |
///            +--------+--------+
///            v                 v
///  [Sources/imported]   [Alias when modern]
///            |                 |
///            +--------+--------+
///                     v
///              [Owned grammar]
/// ```
/// # Examples
/// ```text
/// let args = executable_args(true); // Includes ALIAS <executable>.
/// ```
fn executable_args(modern: bool) -> Vec<ArgSchemaKind> {
    let sources = if modern {
        many(InputFile)
    } else {
        some(InputFile)
    };
    let mut signatures = vec![
        vec![
            flags(&["WIN32", "MACOSX_BUNDLE", "EXCLUDE_FROM_ALL"]),
            sources,
        ],
        vec![k("IMPORTED"), optional(vec![k("GLOBAL")])],
    ];
    if modern {
        signatures.push(vec![k("ALIAS"), TargetExe]);
    }
    vec![TargetExe, choice(signatures)]
}
/// Build initial add_library forms for base and selected replacement versions.
/// Allocates grammar data only; no project commands or I/O.
///
/// # Arguments
/// - `empty_sources` `[in]`: allow zero sources and imported OBJECT in this seed.
/// - `interface_sources` `[in]`: allow INTERFACE libraries with sources and EXCLUDE_FROM_ALL.
///
/// # Returns
/// Library target followed by source, interface, imported or alias alternatives; no library is
/// created.
///
/// # Algorithm
/// Select source count, imported kinds and interface shape independently, then collect the
/// overloads under one choice.
///
/// # Flow
/// ```text
///             [Feature booleans]
///                     |
///            +--------+--------+
///            v                 v
///  [Sources/interface]   [Imported/alias]
///            |                 |
///            +--------+--------+
///                     v
///              [Owned grammar]
/// ```
/// # Examples
/// ```text
/// let args = library_args(true, true); // Seed for 3.19.
/// ```
fn library_args(empty_sources: bool, interface_sources: bool) -> Vec<ArgSchemaKind> {
    let sources = if empty_sources {
        many(InputFile)
    } else {
        some(InputFile)
    };
    let imported = if empty_sources {
        one_of(&[
            "STATIC",
            "SHARED",
            "MODULE",
            "OBJECT",
            "UNKNOWN",
            "INTERFACE",
        ])
    } else {
        one_of(&["STATIC", "SHARED", "MODULE", "UNKNOWN", "INTERFACE"])
    };
    let interface = if interface_sources {
        vec![
            k("INTERFACE"),
            optional(vec![k("EXCLUDE_FROM_ALL")]),
            many(InputFile),
        ]
    } else {
        vec![k("INTERFACE")]
    };
    vec![
        TargetLib,
        choice(vec![
            vec![
                optional(vec![one_of(&["STATIC", "SHARED", "MODULE", "OBJECT"])]),
                optional(vec![k("EXCLUDE_FROM_ALL")]),
                sources,
            ],
            interface,
            vec![imported, k("IMPORTED"), optional(vec![k("GLOBAL")])],
            vec![k("ALIAS"), TargetLib],
        ]),
    ]
}
/// Describe cmake_parse_arguments modes without parsing a project call.
/// Allocates grammar data only; no project commands or I/O.
///
/// # Arguments
/// - `argv` `[in]`: include PARSE_ARGV and its starting index.
/// - `argn` `[in]`: include PARSE_ARGN without a starting index.
///
/// # Returns
/// One choice of classic prefix/list/value form and enabled special modes; prefix-derived output
/// names are not expanded here.
///
/// # Algorithm
/// Keep the classic mode, then append each requested special signature with its own literal
/// dispatch word.
///
/// # Flow
/// ```text
///             [Classic form]
///                     |
///            +--------+--------+
///            v                 v
///  [argv enabled?]   [argn enabled?]
///            |                 |
///            +--------+--------+
///                     v
///              [Owned grammar]
/// ```
/// # Examples
/// ```text
/// let args = parse_arguments(true, false); // Classic and PARSE_ARGV forms.
/// ```
fn parse_arguments(argv: bool, argn: bool) -> Vec<ArgSchemaKind> {
    let mut signatures = vec![vec![Text, List, List, List, many(Value)]];
    if argv {
        signatures.push(vec![k("PARSE_ARGV"), Integer, Text, List, List, List]);
    }
    if argn {
        signatures.push(vec![k("PARSE_ARGN"), Text, List, List, List]);
    }
    vec![choice(signatures)]
}
/// Build the original partial file grammar retained as a registration seed.
/// Allocates grammar data only; no project commands or I/O.
///
/// # Arguments
/// None.
///
/// # Returns
/// A choice with explicit basic path/output roles and opaque tails; not the final effective file
/// grammar.
///
/// # Algorithm
/// List basic operations including READ/WRITE/hash/copy/transfer; keep Action/Unmodeled fallback
/// for this seed. The final transform layer replaces it.
///
/// # Flow
/// ```text
///             [Operation choice]
///                     |
///            +--------+--------+
///            v                 v
///  [READ/hash outputs]   [WRITE/copy paths]
///            |                 |
///            +--------+--------+
///                     v
///              [Owned grammar]
/// ```
/// # Examples
/// ```text
/// let seed = file_args(); // READ has InputFile then Out.
/// ```
fn file_args() -> Vec<ArgSchemaKind> {
    vec![choice(vec![
        vec![
            k("READ"),
            InputFile,
            Out,
            options(vec![
                vec![k("OFFSET"), Integer],
                vec![k("LIMIT"), Integer],
                vec![k("HEX")],
            ]),
        ],
        vec![one_of(&["WRITE", "APPEND"]), OutputPath, many(Text)],
        vec![
            one_of(&["MD5", "SHA1", "SHA224", "SHA256", "SHA384", "SHA512"]),
            InputFile,
            Out,
        ],
        vec![k("STRINGS"), InputFile, Out, Unmodeled],
        vec![one_of(&["GLOB", "GLOB_RECURSE"]), Out, Unmodeled],
        vec![
            one_of(&["REMOVE", "REMOVE_RECURSE", "MAKE_DIRECTORY"]),
            many(Path),
        ],
        vec![one_of(&["RENAME"]), InputPath, OutputPath],
        vec![
            one_of(&["COPY", "INSTALL"]),
            some(InputPath),
            k("DESTINATION"),
            OutputPath,
            Unmodeled,
        ],
        vec![k("RELATIVE_PATH"), Out, Path, Path],
        vec![one_of(&["TO_CMAKE_PATH", "TO_NATIVE_PATH"]), Path, Out],
        vec![k("DOWNLOAD"), Url, OutputPath, Unmodeled],
        vec![k("UPLOAD"), InputFile, Url, Unmodeled],
        vec![
            k("GENERATE"),
            k("OUTPUT"),
            OutputPath,
            choice(vec![vec![k("INPUT"), InputFile], vec![k("CONTENT"), Text]]),
            Unmodeled,
        ],
        vec![k("TIMESTAMP"), Unmodeled],
        vec![Action, Unmodeled],
    ])]
}
/// Build historical list replacement seeds keyed by 3.x minor.
/// Allocates grammar data only; no project commands or I/O.
///
/// # Arguments
/// - `minor` `[in]`: 3.x minor controlling FILTER, TRANSFORM, sorting and pop/prepend additions.
///
/// # Returns
/// One choice of eligible list operations; this is later replaced by the structured transform
/// layer.
///
/// # Algorithm
/// Start with list reads/mutations, extend sort options and add operations at their known minor
/// thresholds.
///
/// # Flow
/// ```text
///             [3.x minor]
///                     |
///            +--------+--------+
///            v                 v
///  [Base operations]   [Eligible additions]
///            |                 |
///            +--------+--------+
///                     v
///              [Owned grammar]
/// ```
/// # Examples
/// ```text
/// let seed = list_args(15); // Includes POP_BACK output variables.
/// ```
fn list_args(minor: u32) -> Vec<ArgSchemaKind> {
    let mut forms = vec![
        vec![k("LENGTH"), Variable, Out],
        vec![k("GET"), Variable, some(Integer), Out],
        vec![k("FIND"), Variable, Value, Out],
        vec![k("APPEND"), Variable, many(Value)],
        vec![k("INSERT"), Variable, Integer, some(Value)],
        vec![k("REMOVE_ITEM"), Variable, many(Value)],
        vec![k("REMOVE_AT"), Variable, some(Integer)],
        vec![one_of(&["REMOVE_DUPLICATES", "REVERSE"]), Variable],
    ];
    let mut sort = vec![k("SORT"), Variable];
    if minor >= 13 {
        let compare = if minor >= 18 {
            one_of(&["STRING", "FILE_BASENAME", "NATURAL"])
        } else {
            one_of(&["STRING", "FILE_BASENAME"])
        };
        sort.push(options(vec![
            vec![k("COMPARE"), compare],
            vec![k("CASE"), one_of(&["SENSITIVE", "INSENSITIVE"])],
            vec![k("ORDER"), one_of(&["ASCENDING", "DESCENDING"])],
        ]));
    }
    forms.push(sort);
    if minor >= 6 {
        forms.push(vec![
            k("FILTER"),
            Variable,
            one_of(&["INCLUDE", "EXCLUDE"]),
            k("REGEX"),
            Expression,
        ]);
    }
    if minor >= 12 {
        forms.extend([
            vec![k("JOIN"), Variable, Text, Out],
            vec![k("SUBLIST"), Variable, Integer, Integer, Out],
            vec![
                k("TRANSFORM"),
                Variable,
                choice(vec![
                    vec![one_of(&["APPEND", "PREPEND"]), Text],
                    vec![one_of(&["TOLOWER", "TOUPPER", "STRIP", "GENEX_STRIP"])],
                    vec![k("REPLACE"), Expression, Text],
                ]),
                optional(vec![choice(vec![
                    vec![k("AT"), some(Integer)],
                    vec![k("FOR"), Integer, Integer, optional(vec![Integer])],
                    vec![k("REGEX"), Expression],
                ])]),
                optional(vec![k("OUTPUT_VARIABLE"), Out]),
            ],
        ]);
    }
    if minor >= 15 {
        forms.extend([
            vec![k("PREPEND"), Variable, many(Value)],
            vec![one_of(&["POP_BACK", "POP_FRONT"]), Variable, many(Out)],
        ]);
    }
    vec![choice(forms)]
}
/// Build historical string replacement seeds for version registrations.
/// Allocates grammar data only; no project commands or I/O.
///
/// # Arguments
/// - `version` `[in]`: major/minor used for comparisons, JSON and regex operation additions.
///
/// # Returns
/// One choice of eligible string operations with variable outputs; final transform trees refine it.
///
/// # Algorithm
/// Gate regex/comparison variants and append UUID, mutation, encoding and JSON operations at
/// their introduction versions.
///
/// # Flow
/// ```text
///             [Version]
///                     |
///            +--------+--------+
///            v                 v
///  [Base text/regex]   [Versioned JSON/etc.]
///            |                 |
///            +--------+--------+
///                     v
///              [Owned grammar]
/// ```
/// # Examples
/// ```text
/// let seed = string_args((4, 2)); // Includes REGEX QUOTE.
/// ```
fn string_args(version: (u32, u32)) -> Vec<ArgSchemaKind> {
    let mut regex = vec![
        vec![one_of(&["MATCH", "MATCHALL"]), Expression, Out, some(Text)],
        vec![k("REPLACE"), Expression, Text, Out, some(Text)],
    ];
    if version >= (4, 2) {
        regex.push(vec![k("QUOTE"), Out, some(Text)]);
    }
    let comparisons = if version >= (3, 7) {
        one_of(&[
            "LESS",
            "GREATER",
            "EQUAL",
            "NOTEQUAL",
            "LESS_EQUAL",
            "GREATER_EQUAL",
        ])
    } else {
        one_of(&["LESS", "GREATER", "EQUAL", "NOTEQUAL"])
    };
    let mut forms = vec![
        vec![k("REGEX"), choice(regex)],
        vec![k("REPLACE"), Text, Text, Out, some(Text)],
        vec![
            one_of(&["TOLOWER", "TOUPPER", "LENGTH", "MAKE_C_IDENTIFIER"]),
            Text,
            Out,
        ],
        vec![k("SUBSTRING"), Text, Integer, Integer, Out],
        vec![k("STRIP"), Text, Out],
        vec![k("ASCII"), some(Integer), Out],
        vec![
            k("CONFIGURE"),
            Text,
            Out,
            flags(&["@ONLY", "ESCAPE_QUOTES"]),
        ],
        vec![
            one_of(&["MD5", "SHA1", "SHA224", "SHA256", "SHA384", "SHA512"]),
            Out,
            Text,
        ],
        vec![k("FIND"), Text, Text, Out, optional(vec![k("REVERSE")])],
        vec![k("COMPARE"), comparisons, Text, Text, Out],
        vec![k("CONCAT"), Out, many(Text)],
        vec![
            k("RANDOM"),
            options(vec![
                vec![k("LENGTH"), Integer],
                vec![k("ALPHABET"), Text],
                vec![k("RANDOM_SEED"), Integer],
            ]),
            Out,
        ],
        vec![
            k("TIMESTAMP"),
            Out,
            optional(vec![Text]),
            optional(vec![k("UTC")]),
        ],
    ];
    if version >= (3, 1) {
        forms.extend([
            vec![k("GENEX_STRIP"), Text, Out],
            vec![
                k("UUID"),
                Out,
                k("NAMESPACE"),
                Text,
                k("NAME"),
                Text,
                k("TYPE"),
                one_of(&["MD5", "SHA1"]),
                optional(vec![k("UPPER")]),
            ],
        ]);
    }
    if version >= (3, 4) {
        forms.push(vec![k("APPEND"), Variable, many(Text)]);
    }
    if version >= (3, 10) {
        forms.push(vec![k("PREPEND"), Variable, many(Text)]);
    }
    if version >= (3, 12) {
        forms.push(vec![k("JOIN"), Text, Out, many(Text)]);
    }
    if version >= (3, 15) {
        forms.push(vec![k("REPEAT"), Text, Integer, Out]);
    }
    if version >= (3, 18) {
        forms.push(vec![k("HEX"), Text, Out]);
    }
    if version >= (3, 19) {
        forms.push(vec![
            k("JSON"),
            Out,
            optional(vec![k("ERROR_VARIABLE"), Out]),
            choice(vec![
                vec![
                    one_of(&["GET", "TYPE", "LENGTH", "REMOVE"]),
                    Text,
                    many(Value),
                ],
                vec![k("MEMBER"), Text, many(Value), Integer],
                vec![k("SET"), Text, some(Value), Text],
                vec![k("EQUAL"), Text, Text],
            ]),
        ]);
    }
    vec![choice(forms)]
}
