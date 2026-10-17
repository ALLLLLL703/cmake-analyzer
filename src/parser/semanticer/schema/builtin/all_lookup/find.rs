use super::super::*;

pub(super) fn schemas(v: (u32, u32)) -> Vec<(&'static str, Vec<ArgSchemaKind>)> {
    ["find_file", "find_library", "find_path", "find_program"]
        .into_iter()
        .map(|name| (name, args(v, name)))
        .collect()
}
fn args(v: (u32, u32), command: &str) -> Vec<ArgSchemaKind> {
    let paths = many(choice(vec![vec![k("ENV"), Text], vec![Path]]));
    let mut opts = vec![
        vec![k("NAMES"), many(Text)],
        vec![k("HINTS"), paths.clone()],
        vec![k("PATHS"), paths],
        vec![k("PATH_SUFFIXES"), many(Text)],
        vec![k("DOC"), Text],
    ];
    for flag in [
        "NO_DEFAULT_PATH",
        "NO_CMAKE_PATH",
        "NO_CMAKE_ENVIRONMENT_PATH",
        "NO_SYSTEM_ENVIRONMENT_PATH",
        "NO_CMAKE_SYSTEM_PATH",
        "CMAKE_FIND_ROOT_PATH_BOTH",
        "ONLY_CMAKE_FIND_ROOT_PATH",
        "NO_CMAKE_FIND_ROOT_PATH",
    ] {
        opts.push(vec![k(flag)]);
    }
    if matches!(command, "find_library" | "find_program") {
        opts.push(vec![k("NAMES_PER_DIR")]);
    }
    if v >= (3, 12) {
        opts.push(vec![k("NO_PACKAGE_ROOT_PATH")]);
    }
    if v >= (3, 18) {
        opts.push(vec![k("REQUIRED")]);
    }
    if v >= (3, 21) {
        opts.push(vec![k("NO_CACHE")]);
    }
    if v >= (3, 24) {
        opts.extend([
            vec![k("NO_CMAKE_INSTALL_PREFIX")],
            vec![
                k("REGISTRY_VIEW"),
                one_of(&["64", "32", "64_32", "32_64", "HOST", "TARGET", "BOTH"]),
            ],
        ]);
    }
    if v >= (3, 25) {
        opts.push(vec![k("VALIDATOR"), Command]);
    }
    if v >= (4, 1) {
        opts.push(vec![k("OPTIONAL")]);
    }
    // Both shorthand names/paths and keyword groups are valid; search paths need not exist.
    opts.push(vec![Text]);
    vec![Out, options(opts)]
}
