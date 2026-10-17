use super::{
    ArgSchemaKind, PackageName, Path, Text, Version, choice, k, many, one_of, optional, options,
};

pub(super) fn args(version: (u32, u32)) -> Vec<ArgSchemaKind> {
    let mut common = vec![
        vec![k("EXACT")],
        vec![k("QUIET")],
        vec![k("REQUIRED"), many(Text)],
        vec![k("COMPONENTS"), many(Text)],
        vec![k("OPTIONAL_COMPONENTS"), many(Text)],
        vec![k("NO_POLICY_SCOPE")],
    ];
    if version >= (3, 24) {
        common.extend([
            vec![k("GLOBAL")],
            vec![k("BYPASS_PROVIDER")],
            vec![
                k("REGISTRY_VIEW"),
                one_of(&["64", "32", "64_32", "32_64", "HOST", "TARGET", "BOTH"]),
            ],
        ]);
    }
    if version >= (4, 1) {
        common.push(vec![k("OPTIONAL"), many(Text)]);
    }
    if version >= (4, 2) {
        common.push(vec![k("UNWIND_INCLUDE")]);
    }
    let mut module = common.clone();
    module.push(vec![k("MODULE")]);
    let mut config = common;
    for flag in [
        "CONFIG",
        "NO_MODULE",
        "NO_DEFAULT_PATH",
        "NO_CMAKE_PATH",
        "NO_CMAKE_ENVIRONMENT_PATH",
        "NO_SYSTEM_ENVIRONMENT_PATH",
        "NO_CMAKE_PACKAGE_REGISTRY",
        "NO_CMAKE_BUILDS_PATH",
        "NO_CMAKE_SYSTEM_PATH",
        "NO_CMAKE_SYSTEM_PACKAGE_REGISTRY",
        "CMAKE_FIND_ROOT_PATH_BOTH",
        "ONLY_CMAKE_FIND_ROOT_PATH",
        "NO_CMAKE_FIND_ROOT_PATH",
    ] {
        config.push(vec![k(flag)]);
    }
    if version >= (3, 12) {
        config.push(vec![k("NO_PACKAGE_ROOT_PATH")]);
    }
    if version >= (3, 24) {
        config.push(vec![k("NO_CMAKE_INSTALL_PREFIX")]);
    }
    config.extend([
        vec![k("NAMES"), many(Text)],
        vec![k("CONFIGS"), many(Text)],
        // Search hints may legitimately name nonexistent directories.
        vec![k("HINTS"), many(Path)],
        vec![k("PATHS"), many(Path)],
        vec![k("PATH_SUFFIXES"), many(Text)],
    ]);
    vec![
        PackageName,
        optional(vec![Version]),
        choice(vec![vec![options(module)], vec![options(config)]]),
    ]
}
