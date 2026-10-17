use super::super::*;

fn condition(v: (u32, u32)) -> Vec<ArgSchemaKind> {
    let mut unary = vec![
        vec![one_of(&["COMMAND", "POLICY", "DEFINED"]), Text],
        vec![k("TARGET"), Target],
        vec![
            one_of(&["EXISTS", "IS_DIRECTORY", "IS_SYMLINK", "IS_ABSOLUTE"]),
            Path,
        ],
    ];
    if v >= (3, 3) {
        unary.push(vec![k("TEST"), TestName]);
    }
    if v >= (3, 29) {
        unary.push(vec![
            one_of(&["IS_READABLE", "IS_WRITABLE", "IS_EXECUTABLE"]),
            Path,
        ]);
    }
    if v >= (4, 4) {
        unary.push(vec![k("DIAGNOSTIC"), Text]);
    }
    unary.push(vec![Value]);
    let atom = vec![many(k("NOT")), choice(unary)];
    let mut binary = vec![
        "LESS",
        "GREATER",
        "EQUAL",
        "STRLESS",
        "STRGREATER",
        "STREQUAL",
        "VERSION_LESS",
        "VERSION_GREATER",
        "VERSION_EQUAL",
        "MATCHES",
        "IS_NEWER_THAN",
    ];
    if v >= (3, 3) {
        binary.push("IN_LIST");
    }
    if v >= (3, 7) {
        binary.extend([
            "LESS_EQUAL",
            "GREATER_EQUAL",
            "STRLESS_EQUAL",
            "STRGREATER_EQUAL",
            "VERSION_LESS_EQUAL",
            "VERSION_GREATER_EQUAL",
        ]);
    }
    if v >= (3, 24) {
        binary.push("PATH_EQUAL");
    }
    let mut logical = vec![one_of(&["AND", "OR"])];
    logical.extend(atom.clone());
    let mut result = atom;
    result.push(options(vec![vec![one_of(&binary), Value], logical]));
    vec![ArgSchemaKind::Grammar {
        placeholder: "condition".into(),
        args: result,
    }]
}
fn foreach(v: (u32, u32)) -> Vec<ArgSchemaKind> {
    let mut variants = vec![
        vec![
            Out,
            k("RANGE"),
            Integer,
            optional(vec![Integer, optional(vec![Integer])]),
        ],
        vec![
            Out,
            k("IN"),
            options(vec![
                vec![k("LISTS"), many(Variable)],
                vec![k("ITEMS"), many(Value)],
            ]),
        ],
        vec![Out, many(Value)],
    ];
    if v >= (3, 17) {
        variants.push(vec![some(Out), k("IN"), k("ZIP_LISTS"), some(Variable)]);
    }
    vec![choice(variants)]
}
fn filename(v: (u32, u32)) -> Vec<ArgSchemaKind> {
    let mut modes = vec!["DIRECTORY", "NAME", "EXT", "NAME_WE", "PATH"];
    if v >= (3, 14) {
        modes.extend(["LAST_EXT", "NAME_WLE"]);
    }
    let mut absolute = vec![one_of(&["ABSOLUTE", "REALPATH"])];
    if v >= (3, 4) {
        absolute.push(optional(vec![k("BASE_DIR"), Path]));
    }
    vec![
        Out,
        Path,
        choice(vec![
            vec![one_of(&modes)],
            absolute,
            vec![k("PROGRAM"), optional(vec![k("PROGRAM_ARGS"), Out])],
        ]),
        optional(vec![k("CACHE")]),
    ]
}
fn separate(v: (u32, u32)) -> Vec<ArgSchemaKind> {
    let mut modes = vec!["UNIX_COMMAND", "WINDOWS_COMMAND"];
    if v >= (3, 9) {
        modes.push("NATIVE_COMMAND");
    }
    let mut modern = vec![Out, one_of(&modes)];
    if v >= (3, 19) {
        modern.push(optional(vec![
            k("PROGRAM"),
            optional(vec![k("SEPARATE_ARGS")]),
        ]));
    }
    modern.push(Text);
    vec![choice(vec![vec![Variable], modern])]
}
fn message(v: (u32, u32)) -> Vec<ArgSchemaKind> {
    let mut modes = vec![
        "FATAL_ERROR",
        "SEND_ERROR",
        "WARNING",
        "AUTHOR_WARNING",
        "STATUS",
        "DEPRECATION",
    ];
    if v >= (3, 15) {
        modes.extend(["NOTICE", "VERBOSE", "DEBUG", "TRACE"]);
    }
    if v >= (3, 17) {
        modes.extend(["CHECK_START", "CHECK_PASS", "CHECK_FAIL"]);
    }
    if v >= (3, 26) {
        modes.push("CONFIGURE_LOG");
    }
    vec![optional(vec![one_of(&modes)]), many(Text)]
}
fn set(v: (u32, u32)) -> Vec<ArgSchemaKind> {
    let normal = BASE_SCHEMAS
        .iter()
        .find(|schema| schema.command_name == "set")
        .map(|schema| {
            schema
                .args
                .iter()
                .map(|argument| argument.kind.clone())
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();
    if v < (4, 2) {
        return normal;
    }
    vec![choice(vec![
        normal,
        vec![
            ArgSchemaKind::CacheEntry,
            options(vec![
                vec![
                    k("TYPE"),
                    one_of(&["BOOL", "FILEPATH", "PATH", "STRING", "INTERNAL"]),
                ],
                vec![k("HELP"), many(Text)],
                vec![k("FORCE")],
            ]),
            k("VALUE"),
            many(Value),
        ],
    ])]
}
pub(super) fn schemas(v: (u32, u32)) -> Vec<(&'static str, Vec<ArgSchemaKind>)> {
    let mut source_group = vec![vec![
        Text,
        choice(vec![
            vec![Expression],
            vec![options(vec![
                vec![k("REGULAR_EXPRESSION"), Expression],
                vec![k("FILES"), many(Path)],
            ])],
        ]),
    ]];
    if v >= (3, 8) {
        source_group.push(vec![
            k("TREE"),
            Path,
            options(vec![vec![k("PREFIX"), Text], vec![k("FILES"), many(Path)]]),
        ]);
    }
    let mut include = vec![
        vec![k("OPTIONAL")],
        vec![k("RESULT_VARIABLE"), Out],
        vec![k("NO_POLICY_SCOPE")],
    ];
    if v >= (4, 4) {
        include.push(vec![k("NO_DIAGNOSTIC_SCOPE")]);
    }
    let mut result = vec![
        ("include", vec![ModulePath, options(include)]),
        ("foreach", foreach(v)),
        ("get_filename_component", filename(v)),
        ("separate_arguments", separate(v)),
        ("message", message(v)),
        ("set", set(v)),
        ("source_group", vec![choice(source_group)]),
        (
            "load_cache",
            vec![
                InputDirectory,
                choice(vec![
                    vec![k("READ_WITH_PREFIX"), Text, some(Text)],
                    vec![options(vec![
                        vec![k("EXCLUDE"), many(Text)],
                        vec![k("INCLUDE_INTERNALS"), many(Text)],
                    ])],
                ]),
            ],
        ),
        (
            "create_test_sourcelist",
            vec![
                Out,
                OutputPath,
                some(InputFile),
                options(vec![
                    vec![k("EXTRA_INCLUDE"), Text],
                    vec![k("FUNCTION"), Text],
                ]),
            ],
        ),
    ];
    if v >= (3, 25) {
        let mut scopes = vec!["POLICIES", "VARIABLES"];
        if v >= (4, 4) {
            scopes.push("DIAGNOSTICS");
        }
        result.push((
            "block",
            vec![options(vec![
                vec![k("SCOPE_FOR"), some(one_of(&scopes))],
                vec![k("PROPAGATE"), some(Variable)],
            ])],
        ));
    }
    for name in ["if", "elseif", "while"] {
        result.push((name, condition(v)));
    }
    for name in ["else", "endif", "endwhile"] {
        result.push((name, vec![optional(condition(v))]));
    }
    result
}
