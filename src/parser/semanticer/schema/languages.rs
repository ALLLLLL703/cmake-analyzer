use crate::parser::semanticer::model::CMakeVersion;

// CMake's command/include/SUPPORTED_LANGUAGES catalogue and introduction versions.
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

pub(crate) fn supported(version: Option<CMakeVersion>) -> impl Iterator<Item = &'static str> {
    LANGUAGES
        .iter()
        .filter(move |(_, introduced)| {
            version.is_none_or(|version| (version.major, version.minor) >= *introduced)
        })
        .map(|(name, _)| *name)
}
