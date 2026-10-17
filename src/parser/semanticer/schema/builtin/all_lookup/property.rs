use super::super::*;

fn source_scope(v: (u32, u32), multiple: bool) -> Vec<ArgSchemaKind> {
    if v < (3, 18) {
        return Vec::new();
    }
    let directory = if multiple { some(Path) } else { Path };
    let target = if multiple { some(Target) } else { Target };
    vec![options(vec![
        vec![k("DIRECTORY"), directory],
        vec![k("TARGET_DIRECTORY"), target],
    ])]
}
fn scopes(v: (u32, u32), setting: bool) -> ArgSchemaKind {
    let mut source = vec![k("SOURCE"), if setting { many(Path) } else { Path }];
    source.extend(source_scope(v, setting));
    let mut test = vec![k("TEST"), if setting { many(TestName) } else { TestName }];
    if v >= (3, 28) {
        test.push(optional(vec![k("DIRECTORY"), Path]));
    }
    let mut scopes = vec![
        vec![k("GLOBAL")],
        vec![k("DIRECTORY"), optional(vec![Path])],
        vec![k("TARGET"), if setting { many(Target) } else { Target }],
        source,
        test,
        vec![k("CACHE"), if setting { many(Text) } else { Text }],
    ];
    if !setting {
        scopes.push(vec![k("VARIABLE")]);
    }
    if v >= (3, 1) {
        scopes.push(vec![k("INSTALL"), if setting { many(Path) } else { Path }]);
    }
    if v >= (4, 3) {
        scopes.push(vec![
            k("FILE_SET"),
            if setting { many(Text) } else { Text },
            k("TARGET"),
            Target,
        ]);
    }
    choice(scopes)
}
pub(super) fn schemas(v: (u32, u32)) -> Vec<(&'static str, Vec<ArgSchemaKind>)> {
    let mut define_opts = vec![
        vec![k("INHERITED")],
        vec![k("BRIEF_DOCS"), some(Text)],
        vec![k("FULL_DOCS"), some(Text)],
    ];
    if v >= (3, 23) {
        define_opts.push(vec![k("INITIALIZE_FROM_VARIABLE"), Variable]);
    }
    let mut source_get = vec![Out, Path];
    source_get.extend(source_scope(v, false));
    source_get.push(Property);
    let mut source_set = vec![some(Path)];
    source_set.extend(source_scope(v, true));
    source_set.extend([k("PROPERTIES"), pairs()]);
    let mut test_get = vec![TestName, Property];
    let mut test_set = vec![some(TestName)];
    if v >= (3, 28) {
        let directory = optional(vec![k("DIRECTORY"), Path]);
        test_get.push(directory.clone());
        test_set.push(directory);
    }
    test_get.push(Out);
    test_set.extend([k("PROPERTIES"), pairs()]);
    vec![
        (
            "get_property",
            vec![
                Out,
                scopes(v, false),
                k("PROPERTY"),
                Property,
                optional(vec![one_of(&["SET", "DEFINED", "BRIEF_DOCS", "FULL_DOCS"])]),
            ],
        ),
        (
            "set_property",
            vec![
                scopes(v, true),
                flags(&["APPEND", "APPEND_STRING"]),
                k("PROPERTY"),
                Property,
                many(Value),
            ],
        ),
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
                options(define_opts),
            ],
        ),
        ("get_source_file_property", source_get),
        ("set_source_files_properties", source_set),
        ("get_test_property", test_get),
        ("set_tests_properties", test_set),
    ]
}
