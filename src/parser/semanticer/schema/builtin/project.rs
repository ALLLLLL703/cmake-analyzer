use super::{ArgSchemaKind, LanguageType, Text, Version, choice, k, many, options};

pub(super) fn args(version: (u32, u32)) -> Vec<ArgSchemaKind> {
    let mut named = vec![
        vec![k("VERSION"), Version],
        vec![k("LANGUAGES"), many(LanguageType)],
    ];
    if version >= (3, 9) {
        named.push(vec![k("DESCRIPTION"), Text]);
    }
    if version >= (3, 12) {
        named.push(vec![k("HOMEPAGE_URL"), super::Url]);
    }
    // Earlier experimental forms are known syntax; feature-gate activation is not evaluated.
    if version >= (4, 1) {
        named.push(vec![k("COMPAT_VERSION"), Version]);
    }
    if version >= (4, 2) {
        named.push(vec![k("SPDX_LICENSE"), Text]);
    }
    // Legacy positional languages and named options are separate signatures.
    vec![
        Text,
        choice(vec![vec![many(LanguageType)], vec![options(named)]]),
    ]
}
