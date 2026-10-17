use super::{
    ArgSchemaKind, Boolean, Command, Expression, InputFile, InputPath, Integer, Number, Out,
    OutputPath, Path, Target, Text, Url, Value, Variable, choice, flags, k, many, one_of, optional,
    options, some,
};

pub(super) fn schemas(version: (u32, u32)) -> Vec<(&'static str, Vec<ArgSchemaKind>)> {
    let mut result = vec![
        ("file", file(version)),
        ("string", string(version)),
        ("list", list(version)),
        ("math", math(version)),
    ];
    if version >= (3, 20) {
        result.push(("cmake_path", cmake_path()));
    }
    result
}

fn hashes(version: (u32, u32)) -> ArgSchemaKind {
    let mut words = vec!["MD5", "SHA1", "SHA224", "SHA256", "SHA384", "SHA512"];
    if version >= (3, 8) {
        words.extend(["SHA3_224", "SHA3_256", "SHA3_384", "SHA3_512"]);
    }
    one_of(&words)
}

fn permissions() -> ArgSchemaKind {
    some(one_of(&[
        "OWNER_READ",
        "OWNER_WRITE",
        "OWNER_EXECUTE",
        "GROUP_READ",
        "GROUP_WRITE",
        "GROUP_EXECUTE",
        "WORLD_READ",
        "WORLD_WRITE",
        "WORLD_EXECUTE",
        "SETUID",
        "SETGID",
    ]))
}

fn newline() -> Vec<ArgSchemaKind> {
    vec![
        k("NEWLINE_STYLE"),
        one_of(&["UNIX", "DOS", "WIN32", "LF", "CRLF"]),
    ]
}

fn file(version: (u32, u32)) -> Vec<ArgSchemaKind> {
    let mut strings_options = vec![
        vec![
            one_of(&[
                "LENGTH_MAXIMUM",
                "LENGTH_MINIMUM",
                "LIMIT_COUNT",
                "LIMIT_INPUT",
                "LIMIT_OUTPUT",
            ]),
            Integer,
        ],
        vec![one_of(&["NEWLINE_CONSUME", "NO_HEX_CONVERSION"])],
        vec![k("REGEX"), Expression],
    ];
    if version >= (3, 1) {
        let encodings = if version >= (3, 2) {
            one_of(&["UTF-8", "UTF-16LE", "UTF-16BE", "UTF-32LE", "UTF-32BE"])
        } else {
            k("UTF-8")
        };
        strings_options.push(vec![k("ENCODING"), encodings]);
    }
    let mut forms = vec![
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
        vec![k("STRINGS"), InputFile, Out, options(strings_options)],
        vec![hashes(version), InputFile, Out],
        vec![
            k("TIMESTAMP"),
            InputFile,
            Out,
            optional(vec![Text]),
            optional(vec![k("UTC")]),
        ],
        vec![one_of(&["WRITE", "APPEND"]), OutputPath, many(Text)],
        vec![one_of(&["REMOVE", "REMOVE_RECURSE"]), many(Path)],
        vec![k("RELATIVE_PATH"), Out, Path, Path],
        vec![one_of(&["TO_CMAKE_PATH", "TO_NATIVE_PATH"]), Path, Out],
    ];
    for (mode, input) in [("INPUT", InputFile), ("CONTENT", Text)] {
        let mut opts = vec![vec![k("CONDITION"), Expression]];
        if version >= (3, 19) {
            opts.push(vec![k("TARGET"), Target]);
        }
        if version >= (3, 20) {
            let mut source_permissions = vec!["NO_SOURCE_PERMISSIONS"];
            if mode == "INPUT" {
                source_permissions.push("USE_SOURCE_PERMISSIONS");
            }
            opts.extend([
                vec![one_of(&source_permissions)],
                vec![k("FILE_PERMISSIONS"), permissions()],
                newline(),
            ]);
        }
        forms.push(vec![
            k("GENERATE"),
            k("OUTPUT"),
            OutputPath,
            k(mode),
            input,
            options(opts),
        ]);
    }
    for recurse in [false, true] {
        let mut opts = vec![vec![k("RELATIVE"), Path]];
        if recurse {
            opts.push(vec![k("FOLLOW_SYMLINKS")]);
        }
        if version >= (3, 3) {
            opts.push(vec![k("LIST_DIRECTORIES"), Boolean]);
        }
        if version >= (3, 12) {
            opts.push(vec![k("CONFIGURE_DEPENDS")]);
        }
        forms.push(vec![
            k(if recurse { "GLOB_RECURSE" } else { "GLOB" }),
            Out,
            options(opts),
            some(Text),
        ]);
    }
    let mut mkdir = vec![
        k("MAKE_DIRECTORY"),
        if version >= (3, 30) {
            many(OutputPath)
        } else {
            some(OutputPath)
        },
    ];
    if version >= (3, 31) {
        mkdir.push(optional(vec![k("RESULT"), Out]));
    }
    forms.push(mkdir);
    let mut rename = vec![k("RENAME"), InputPath, OutputPath];
    if version >= (3, 21) {
        rename.push(options(vec![vec![k("RESULT"), Out], vec![k("NO_REPLACE")]]));
    }
    forms.push(rename);
    let mut copy_opts = vec![
        vec![one_of(&["NO_SOURCE_PERMISSIONS", "USE_SOURCE_PERMISSIONS"])],
        vec![
            one_of(&["FILE_PERMISSIONS", "DIRECTORY_PERMISSIONS"]),
            permissions(),
        ],
    ];
    if version >= (3, 15) {
        copy_opts.push(vec![k("FOLLOW_SYMLINK_CHAIN")]);
    }
    forms.push(vec![
        one_of(&["COPY", "INSTALL"]),
        some(InputPath),
        k("DESTINATION"),
        OutputPath,
        options(copy_opts),
        optional(vec![k("FILES_MATCHING")]),
        many(choice(vec![
            vec![
                k("PATTERN"),
                Text,
                optional(vec![k("EXCLUDE")]),
                optional(vec![k("PERMISSIONS"), permissions()]),
            ],
            vec![
                k("REGEX"),
                Expression,
                optional(vec![k("EXCLUDE")]),
                optional(vec![k("PERMISSIONS"), permissions()]),
            ],
        ])),
    ]);
    forms.push(vec![
        k("UPLOAD"),
        InputFile,
        Url,
        options(transfer_options(version, false)),
    ]);
    let mut download_options = transfer_options(version, true);
    if version >= (3, 24) {
        download_options.push(vec![one_of(&["RANGE_START", "RANGE_END"]), Integer]);
    }
    if version >= (3, 19) {
        forms.push(vec![k("DOWNLOAD"), Url, options(download_options.clone())]);
    }
    download_options.extend([
        vec![k("EXPECTED_HASH"), Text],
        vec![k("EXPECTED_MD5"), Text],
    ]);
    forms.push(vec![
        k("DOWNLOAD"),
        Url,
        OutputPath,
        options(download_options),
    ]);
    if version >= (3, 2) {
        forms.push(vec![
            k("LOCK"),
            OutputPath,
            options(vec![
                vec![one_of(&["DIRECTORY", "RELEASE"])],
                vec![k("GUARD"), one_of(&["FUNCTION", "FILE", "PROCESS"])],
                vec![k("RESULT_VARIABLE"), Out],
                vec![k("TIMEOUT"), Number],
            ]),
        ]);
    }
    if version >= (3, 12) {
        forms.push(vec![
            one_of(&["TOUCH", "TOUCH_NOCREATE"]),
            if version >= (3, 30) {
                many(OutputPath)
            } else {
                some(OutputPath)
            },
        ]);
    }
    if version >= (3, 14) {
        forms.push(vec![k("SIZE"), InputFile, Out]);
        let mut read_symlink = vec![k("READ_SYMLINK"), InputPath, Out];
        if version >= (4, 4) {
            read_symlink.push(optional(vec![k("RESULT"), Out]));
        }
        forms.push(read_symlink);
        // A symbolic link may point at a path that does not exist.
        forms.push(vec![
            k("CREATE_LINK"),
            Path,
            OutputPath,
            options(vec![
                vec![k("RESULT"), Out],
                vec![one_of(&["COPY_ON_ERROR", "SYMBOLIC"])],
            ]),
        ]);
    }
    if version >= (3, 16) {
        forms.push(runtime_dependencies(version));
    }
    if version >= (3, 18) {
        forms.push(vec![
            k("CONFIGURE"),
            k("OUTPUT"),
            OutputPath,
            k("CONTENT"),
            Text,
            options(vec![vec![one_of(&["ESCAPE_QUOTES", "@ONLY"])], newline()]),
        ]);
        forms.extend(archives(version));
    }
    if version >= (3, 19) {
        forms.push(vec![
            one_of(&["CHMOD", "CHMOD_RECURSE"]),
            some(InputPath),
            options(vec![vec![
                one_of(&["PERMISSIONS", "FILE_PERMISSIONS", "DIRECTORY_PERMISSIONS"]),
                permissions(),
            ]]),
        ]);
        let mut opts = vec![vec![k("BASE_DIRECTORY"), Path]];
        if version >= (3, 21) {
            opts.push(vec![k("EXPAND_TILDE")]);
        }
        forms.push(vec![k("REAL_PATH"), InputPath, Out, options(opts)]);
    }
    if version >= (3, 21) {
        let mut opts = vec![vec![k("RESULT"), Out], vec![k("ONLY_IF_DIFFERENT")]];
        if version >= (3, 26) {
            opts.push(vec![k("INPUT_MAY_BE_RECENT")]);
        }
        forms.push(vec![k("COPY_FILE"), InputFile, OutputPath, options(opts)]);
    }
    vec![choice(forms)]
}

fn transfer_options(version: (u32, u32), download: bool) -> Vec<Vec<ArgSchemaKind>> {
    let mut opts = vec![
        vec![one_of(&["INACTIVITY_TIMEOUT", "TIMEOUT"]), Number],
        vec![one_of(&["LOG", "STATUS"]), Out],
        vec![k("SHOW_PROGRESS")],
    ];
    if version >= (3, 7) {
        opts.push(vec![one_of(&["USERPWD", "HTTPHEADER"]), Text]);
    }
    if version >= (3, 11) {
        opts.extend([
            vec![k("NETRC"), one_of(&["IGNORED", "OPTIONAL", "REQUIRED"])],
            vec![k("NETRC_FILE"), InputFile],
        ]);
    }
    if download || version >= (3, 18) {
        opts.extend([
            vec![k("TLS_VERIFY"), Boolean],
            vec![k("TLS_CAINFO"), InputFile],
        ]);
    }
    if version >= (3, 30) {
        opts.push(vec![
            k("TLS_VERSION"),
            one_of(&["1.0", "1.1", "1.2", "1.3"]),
        ]);
    }
    opts
}

fn archives(version: (u32, u32)) -> Vec<Vec<ArgSchemaKind>> {
    let mut compression = vec!["None", "BZip2", "GZip", "XZ", "Zstd"];
    if version >= (4, 3) {
        compression.extend(["Deflate", "LZMA", "LZMA2", "PPMd"]);
    }
    let mut compression_option = vec![k("COMPRESSION"), one_of(&compression)];
    if version >= (3, 19) {
        compression_option.push(optional(vec![k("COMPRESSION_LEVEL"), Integer]));
    }
    let mut create_opts = vec![
        vec![
            k("FORMAT"),
            one_of(&["7zip", "gnutar", "pax", "paxr", "raw", "zip"]),
        ],
        compression_option,
        vec![k("MTIME"), Text],
        vec![k("VERBOSE")],
    ];
    if version >= (3, 31) {
        create_opts.push(vec![k("WORKING_DIRECTORY"), Path]);
    }
    if version >= (4, 3) {
        create_opts.push(vec![k("THREADS"), Integer]);
    }
    let mut extract_opts = vec![
        vec![k("DESTINATION"), OutputPath],
        vec![k("PATTERNS"), some(Text)],
        vec![one_of(&["LIST_ONLY", "VERBOSE"])],
    ];
    if version >= (3, 24) {
        extract_opts.push(vec![k("TOUCH")]);
    }
    if version >= (4, 4) {
        // iconv names are accepted as well as the documented common encodings.
        let encoding = choice(vec![
            vec![one_of(&["UTF-8", "OEM", "UTF-16LE", "UTF-16BE"])],
            vec![Text],
        ]);
        create_opts.push(vec![k("ENCODING"), encoding.clone()]);
        extract_opts.push(vec![k("ENCODING"), encoding]);
    }
    vec![
        vec![
            k("ARCHIVE_CREATE"),
            k("OUTPUT"),
            OutputPath,
            k("PATHS"),
            some(InputPath),
            options(create_opts),
        ],
        vec![
            k("ARCHIVE_EXTRACT"),
            k("INPUT"),
            InputFile,
            options(extract_opts),
        ],
    ]
}

fn runtime_dependencies(version: (u32, u32)) -> Vec<ArgSchemaKind> {
    let mut opts = vec![
        vec![
            one_of(&["RESOLVED_DEPENDENCIES_VAR", "UNRESOLVED_DEPENDENCIES_VAR"]),
            Out,
        ],
        vec![k("CONFLICTING_DEPENDENCIES_PREFIX"), Text],
        vec![
            one_of(&["EXECUTABLES", "LIBRARIES", "MODULES"]),
            many(InputFile),
        ],
        // These directories are search hints, not required existing inputs.
        vec![k("DIRECTORIES"), many(Path)],
        vec![k("BUNDLE_EXECUTABLE"), InputFile],
        vec![
            one_of(&[
                "PRE_INCLUDE_REGEXES",
                "PRE_EXCLUDE_REGEXES",
                "POST_INCLUDE_REGEXES",
                "POST_EXCLUDE_REGEXES",
            ]),
            many(Expression),
        ],
    ];
    if version >= (3, 21) {
        opts.push(vec![
            one_of(&["POST_INCLUDE_FILES", "POST_EXCLUDE_FILES"]),
            many(Path),
        ]);
    }
    vec![k("GET_RUNTIME_DEPENDENCIES"), options(opts)]
}

fn math(version: (u32, u32)) -> Vec<ArgSchemaKind> {
    let mut args = vec![k("EXPR"), Out, Expression];
    if version >= (3, 13) {
        args.push(optional(vec![
            k("OUTPUT_FORMAT"),
            one_of(&["HEXADECIMAL", "DECIMAL"]),
        ]));
    }
    args
}

fn cmake_path() -> Vec<ArgSchemaKind> {
    vec![choice(vec![
        vec![
            k("GET"),
            Variable,
            choice(vec![
                vec![
                    one_of(&[
                        "ROOT_NAME",
                        "ROOT_DIRECTORY",
                        "ROOT_PATH",
                        "FILENAME",
                        "RELATIVE_PART",
                        "PARENT_PATH",
                    ]),
                    Out,
                ],
                vec![
                    one_of(&["EXTENSION", "STEM"]),
                    optional(vec![k("LAST_ONLY")]),
                    Out,
                ],
            ]),
        ],
        vec![
            one_of(&[
                "HAS_ROOT_NAME",
                "HAS_ROOT_DIRECTORY",
                "HAS_ROOT_PATH",
                "HAS_FILENAME",
                "HAS_EXTENSION",
                "HAS_STEM",
                "HAS_RELATIVE_PART",
                "HAS_PARENT_PATH",
                "IS_ABSOLUTE",
                "IS_RELATIVE",
                "HASH",
            ]),
            Variable,
            Out,
        ],
        vec![
            k("IS_PREFIX"),
            Variable,
            Path,
            optional(vec![k("NORMALIZE")]),
            Out,
        ],
        vec![
            k("COMPARE"),
            Path,
            one_of(&["EQUAL", "NOT_EQUAL"]),
            Path,
            Out,
        ],
        vec![k("SET"), Out, optional(vec![k("NORMALIZE")]), Path],
        vec![
            one_of(&["APPEND", "APPEND_STRING"]),
            Variable,
            many(Text),
            optional(vec![k("OUTPUT_VARIABLE"), Out]),
        ],
        vec![
            one_of(&["REMOVE_FILENAME", "NORMAL_PATH"]),
            Variable,
            optional(vec![k("OUTPUT_VARIABLE"), Out]),
        ],
        vec![
            k("REPLACE_FILENAME"),
            Variable,
            Path,
            optional(vec![k("OUTPUT_VARIABLE"), Out]),
        ],
        vec![
            k("REMOVE_EXTENSION"),
            Variable,
            optional(vec![k("LAST_ONLY")]),
            optional(vec![k("OUTPUT_VARIABLE"), Out]),
        ],
        vec![
            k("REPLACE_EXTENSION"),
            Variable,
            optional(vec![k("LAST_ONLY")]),
            Text,
            optional(vec![k("OUTPUT_VARIABLE"), Out]),
        ],
        vec![
            k("RELATIVE_PATH"),
            Variable,
            optional(vec![k("BASE_DIRECTORY"), Path]),
            optional(vec![k("OUTPUT_VARIABLE"), Out]),
        ],
        vec![
            k("ABSOLUTE_PATH"),
            Variable,
            optional(vec![k("BASE_DIRECTORY"), Path]),
            optional(vec![k("NORMALIZE")]),
            optional(vec![k("OUTPUT_VARIABLE"), Out]),
        ],
        vec![
            k("NATIVE_PATH"),
            Variable,
            optional(vec![k("NORMALIZE")]),
            Out,
        ],
        vec![
            k("CONVERT"),
            Text,
            one_of(&["TO_CMAKE_PATH_LIST", "TO_NATIVE_PATH_LIST"]),
            Out,
            optional(vec![k("NORMALIZE")]),
        ],
    ])]
}

fn list(version: (u32, u32)) -> Vec<ArgSchemaKind> {
    let mut filter_modes = vec![vec![k("REGEX"), Expression]];
    let mut actions = vec![
        vec![one_of(&["APPEND", "PREPEND"]), Text],
        vec![one_of(&["TOLOWER", "TOUPPER", "STRIP", "GENEX_STRIP"])],
        vec![k("REPLACE"), Expression, Text],
    ];
    let mut selectors = vec![
        vec![k("AT"), some(Integer)],
        vec![k("FOR"), Integer, Integer, optional(vec![Integer])],
        vec![k("REGEX"), Expression],
    ];
    if version >= (4, 4) {
        filter_modes.push(vec![k("PREDICATE"), Command]);
        actions.push(vec![k("APPLY"), Command]);
        selectors.push(vec![k("PREDICATE"), Command]);
    }
    let mut forms = vec![
        vec![k("LENGTH"), Variable, Out],
        vec![k("GET"), Variable, some(Integer), Out],
        vec![k("FIND"), Variable, Value, Out],
        vec![k("APPEND"), Variable, many(Value)],
        vec![k("INSERT"), Variable, Integer, some(Value)],
        vec![k("REMOVE_ITEM"), Variable, some(Value)],
        vec![k("REMOVE_AT"), Variable, some(Integer)],
        vec![one_of(&["REMOVE_DUPLICATES", "REVERSE"]), Variable],
    ];
    let mut sort = vec![k("SORT"), Variable];
    if version >= (3, 13) {
        let compare = if version >= (3, 18) {
            one_of(&["STRING", "FILE_BASENAME", "NATURAL"])
        } else {
            one_of(&["STRING", "FILE_BASENAME"])
        };
        let modifiers = options(vec![
            vec![k("CASE"), one_of(&["SENSITIVE", "INSENSITIVE"])],
            vec![k("ORDER"), one_of(&["ASCENDING", "DESCENDING"])],
        ]);
        let mut comparison = vec![vec![k("COMPARE"), compare]];
        if version >= (4, 4) {
            comparison.push(vec![k("COMPARATOR"), Command]);
        }
        sort.extend([
            modifiers.clone(),
            optional(vec![choice(comparison)]),
            modifiers,
        ]);
    }
    forms.push(sort);
    if version >= (3, 6) {
        forms.push(vec![
            k("FILTER"),
            Variable,
            one_of(&["INCLUDE", "EXCLUDE"]),
            choice(filter_modes),
        ]);
    }
    if version >= (3, 12) {
        forms.extend([
            vec![k("JOIN"), Variable, Text, Out],
            vec![k("SUBLIST"), Variable, Integer, Integer, Out],
            vec![
                k("TRANSFORM"),
                Variable,
                choice(actions),
                optional(vec![choice(selectors)]),
                optional(vec![k("OUTPUT_VARIABLE"), Out]),
            ],
        ]);
    }
    if version >= (3, 15) {
        forms.extend([
            vec![k("PREPEND"), Variable, many(Value)],
            vec![one_of(&["POP_BACK", "POP_FRONT"]), Variable, many(Out)],
        ]);
    }
    vec![choice(forms)]
}

fn string(version: (u32, u32)) -> Vec<ArgSchemaKind> {
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
        vec![hashes(version), Out, Text],
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
        let mut operations = vec![
            vec![one_of(&["GET", "TYPE", "LENGTH"]), Text, many(Value)],
            vec![k("REMOVE"), Text, some(Value)],
            vec![k("MEMBER"), Text, many(Value), Integer],
            vec![k("SET"), Text, some(Value), Text],
            vec![k("EQUAL"), Text, Text],
        ];
        if version >= (4, 3) {
            operations.extend([
                vec![k("GET_RAW"), Text, many(Value)],
                vec![k("STRING_ENCODE"), Text],
            ]);
        }
        if version >= (4, 4) {
            operations.push(vec![k("PARTIAL_EQUAL"), Text, Text]);
        }
        forms.push(vec![
            k("JSON"),
            Out,
            optional(vec![k("ERROR_VARIABLE"), Out]),
            choice(operations),
        ]);
    }
    vec![choice(forms)]
}

#[cfg(test)]
#[path = "../../../../../tests/unit/parser/semanticer/keyword_transform.rs"]
mod keyword_transform;
