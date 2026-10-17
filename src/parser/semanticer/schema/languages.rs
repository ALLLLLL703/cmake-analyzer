//! Builtin language-name suggestions filtered by introduction version.
//! [`supported`] supplies the LanguageType completion role without probing a
//! compiler, reading modules or executing CMake. The list is not validation:
//! custom language modules may accept additional names. For example, HIP is
//! suggested from 3.21 and ASM_POASM from 4.4; `NONE` is added by project-specific
//! completion, not by this catalogue.

use crate::parser::semanticer::model::CMakeVersion;

/// CMake's SUPPORTED_LANGUAGES names paired with earliest supported major/minor.
/// Catalogue order is preserved, not sorted; 3.0 is the baseline for older names.
/// Example: `LANGUAGES.iter().find(|(name, _)| *name == "HIP").unwrap().1`
/// reads `(3, 21)` without checking a compiler.
static LANGUAGES: &[(&str, (u32, u32))] = &[
    ("C", (3, 0)),
    ("CXX", (3, 0)),
    ("CSharp", (3, 8)),
    ("CUDA", (3, 8)),
    ("OBJC", (3, 16)),
    ("OBJCXX", (3, 16)),
    ("Fortran", (3, 0)),
    ("HIP", (3, 21)),
    ("ISPC", (3, 18)),
    ("Swift", (3, 15)),
    ("ASM", (3, 0)),
    ("ASM_NASM", (3, 0)),
    ("ASM_MARMASM", (3, 26)),
    ("ASM_MASM", (3, 0)),
    ("ASM_POASM", (4, 4)),
    ("ASM-ATT", (3, 0)),
];

/// Borrow builtin language spellings available at the requested version.
/// No side effects: this only filters static data, without allocating a name list.
///
/// # Arguments
/// - `version` `[in]`: optional CMake version; `Some` filters by major/minor,
///   ignoring patch, while `None` exposes every recorded name.
///
/// # Returns
/// Iterator of static language names in catalogue order. An unsupported/future
/// version is still compared directly; no fallback or support-range rejection
/// occurs here. No `NONE` entry is returned.
///
/// # Examples
/// ```text
/// let old = CMakeVersion { major: 3, minor: 20, patch: 0 };
/// assert!(!supported(Some(old)).any(|name| name == "HIP"));
/// let all: Vec<_> = supported(None).collect();
/// assert!(all.contains(&"ASM_POASM"));
/// assert!(!all.contains(&"NONE"));
/// ```
pub(crate) fn supported(version: Option<CMakeVersion>) -> impl Iterator<Item = &'static str> {
    LANGUAGES
        .iter()
        .filter(move |(_, introduced)| {
            version.is_none_or(|version| (version.major, version.minor) >= *introduced)
        })
        .map(|(name, _)| *name)
}
