// AI-generated implementation.
use super::{ArgSchema, ArgSchemaKind, CommandSchema, CommandSchemaFrom};
use crate::parser::semanticer::model::{BuiltinCommand, CommandKind};
use ArgSchemaKind::{
    Action, BindValue, Boolean, Command, Condition, Expression, InputDirectory, InputFile,
    InputPath, Integer, Language, List, ModulePath, Number, Out, OutputPath, PackageName, Path,
    Property, String as Text, Target, TargetExe, TargetLib, TestName, Unmodeled, Unset, Url, Value,
    Variable, Version,
};
use std::{collections::HashMap, sync::LazyLock};

pub const MIN_VERSION: (u32, u32) = (3, 0);
pub const MAX_VERSION: (u32, u32) = (4, 4);
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
            vec![some(Language), optional(vec![k("OPTIONAL")])],
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

/// Ordered schema layers; later entries can replace earlier command signatures.
pub(crate) fn schema_layers(
    major: u32,
    minor: u32,
) -> Option<impl Iterator<Item = &'static CommandSchema>> {
    if !matches!((major, minor), (3, 0..=31) | (4, 0..=4)) {
        return None;
    }
    Some(
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
            ),
    )
}
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
fn k(word: &str) -> ArgSchemaKind {
    ArgSchemaKind::Keyword(word.to_owned())
}
fn optional(args: Vec<ArgSchemaKind>) -> ArgSchemaKind {
    ArgSchemaKind::Optional(args)
}
fn many(kind: ArgSchemaKind) -> ArgSchemaKind {
    ArgSchemaKind::Repeat {
        args: vec![kind],
        min: 0,
    }
}
fn some(kind: ArgSchemaKind) -> ArgSchemaKind {
    ArgSchemaKind::Repeat {
        args: vec![kind],
        min: 1,
    }
}
fn choice(args: Vec<Vec<ArgSchemaKind>>) -> ArgSchemaKind {
    ArgSchemaKind::Choice(args)
}
fn one_of(words: &[&str]) -> ArgSchemaKind {
    choice(words.iter().map(|word| vec![k(word)]).collect())
}
fn flags(words: &[&str]) -> ArgSchemaKind {
    many(one_of(words))
}
fn options(args: Vec<Vec<ArgSchemaKind>>) -> ArgSchemaKind {
    many(choice(args))
}
fn pairs() -> ArgSchemaKind {
    ArgSchemaKind::Repeat {
        args: vec![Property, Value],
        min: 1,
    }
}
fn target_sections(prefix: Vec<ArgSchemaKind>, operand: ArgSchemaKind) -> Vec<ArgSchemaKind> {
    let mut args = vec![Target];
    args.extend(prefix);
    args.push(ArgSchemaKind::Repeat {
        args: vec![one_of(&["INTERFACE", "PUBLIC", "PRIVATE"]), many(operand)],
        min: 1,
    });
    args
}
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
