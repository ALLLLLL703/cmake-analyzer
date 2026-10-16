use super::{ArgSchemaKind,Language,Text,Version,choice,k,many,options};

pub(super) fn args(version:(u32,u32))->Vec<ArgSchemaKind> {
    let mut named=vec![vec![k("VERSION"),Version],vec![k("LANGUAGES"),many(Language)]];
    if version>=(3,9) {named.push(vec![k("DESCRIPTION"),Text]);}
    if version>=(3,12) {named.push(vec![k("HOMEPAGE_URL"),super::Url]);}
    // These options became stable in 4.3; experimental feature gates are not modeled.
    if version>=(4,3) {named.extend([vec![k("COMPAT_VERSION"),Version],vec![k("SPDX_LICENSE"),Text]]);}
    // Legacy positional languages and named options are separate signatures.
    vec![Text,choice(vec![vec![many(Language)],vec![options(named)]])]
}
