//! Project name, language and metadata signature alternatives.
//! Building [`ArgSchemaKind`] trees does not initialize a project, probe compilers
//! or enable experimental features. `project(app LANGUAGES C CXX)` exposes open
//! language-name roles; legacy positional languages form a separate alternative
//! from named metadata options. Version checks describe syntax availability only.

use super::{ArgSchemaKind, LanguageType, Text, Version, choice, k, many, options};

/// Describe project name with either positional languages or named metadata/language groups.
/// Allocates grammar data only; no external side effects.
///
/// # Arguments
/// - `version` `[in]`: requested CMake major/minor controlling syntax additions.
///
/// # Returns
/// Text project name followed by a choice of repeated LanguageType slots or repeated named
/// options; custom language names remain open.
///
/// # Algorithm
/// Start named options with VERSION/LANGUAGES and add description, homepage, compatibility
/// version and license by introduction. Keep legacy languages separate from named options; do not
/// test feature-gate activation.
///
/// # Flow
/// ```text
///                               [args inputs]
///                                     |
///                  +------------------+------------------+
///                  v                                     v
/// +--------------------------------+    +--------------------------------+
/// |  Legacy positional languages   |    |   Named languages / metadata   |
/// +----------------+---------------+    +----------------+---------------+
///                  |                                     |
///                  +------------------+------------------+
///                                     v
///                              [Owned grammar]
/// ```
///
/// # Examples
/// ```text
/// let args = args((4, 2)); // Named metadata includes SPDX_LICENSE.
/// ```
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
