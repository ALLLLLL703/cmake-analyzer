//! Per-version immutable final keyword trees shared by analysis and completion.
//! [`get`] allocates only on a slot's first access. It combines focused builders
//! and removes names absent from native registration, so an override cannot
//! introduce a command too early. No project commands, help queries or I/O run.

use super::{CommandSchema, MAX_VERSION, make_schema, native_layers};
use std::{
    collections::HashSet,
    sync::{LazyLock, OnceLock},
};

/// Slots for 3.0 through 3.31, before the 4.x range.
/// Example: `assert_eq!(CMAKE3_VERSION_COUNT, 32);`.
const CMAKE3_VERSION_COUNT: usize = 32;
/// Total fixed slot count: 32 CMake 3.x pairs plus five CMake 4.x pairs.
/// Example: `assert_eq!(SUPPORTED_VERSION_COUNT, 37);`.
const SUPPORTED_VERSION_COUNT: usize = CMAKE3_VERSION_COUNT + MAX_VERSION.1 as usize + 1;
/// Lazily allocated slot array; each version initializes its own trees at most once.
/// Example: `SCHEMAS[0].get()` reads the optional initialized 3.0 tree list.
static SCHEMAS: LazyLock<[OnceLock<Vec<CommandSchema>>; SUPPORTED_VERSION_COUNT]> =
    LazyLock::new(|| std::array::from_fn(|_| OnceLock::new()));

/// Borrow final overrides, initializing the requested version's cache slot once.
/// Allocates metadata and updates only the internal cache; no project commands or I/O.
///
/// # Arguments
/// - `major` `[in]`: validated major 3 or 4, supplied by `schema_layers`.
/// - `minor` `[in]`: validated minor 0–31 for 3 or 0–4 for 4.
///
/// # Returns
/// Static slice of structured command overrides in lookup/build/process/transform
/// append order. Only names present in `native_layers` are retained; repeated
/// requests borrow the same initialized vector without cloning its trees.
///
/// # Panics
/// Panics if the computed slot is outside the 37-slot array, or if unvalidated
/// slot arithmetic overflows with overflow checks enabled. This helper does
/// not validate version pairs: some invalid pairs would address another slot
/// rather than panic, so callers must enforce the supported range first.
///
/// # Algorithm
/// Convert the supported version to its contiguous slot. Within `get_or_init`,
/// gather native names, concatenate the four builder catalogues, filter names,
/// then wrap roots/classify commands. Each version owns independent trees.
///
/// # Flow
/// ```text
///        [Validated major/minor]
///                  |
///           [Select OnceLock]
///                  |
///           < initialized? >
///             /          \
///            v            v
///       [Borrow it]   [Native name set]
///            |            |
///            |     [Concatenate builders]
///            |            |
///            |     [Filter + wrap + store]
///            |            |
///            +------+-----+
///                   v
///             [Static slice]
/// ```
/// # Examples
/// ```text
/// let first = get(4, 4);
/// let again = get(4, 4);
/// assert!(std::ptr::eq(first.as_ptr(), again.as_ptr()));
/// ```
pub(super) fn get(major: u32, minor: u32) -> &'static [CommandSchema] {
    let slot = if major == 3 {
        minor as usize
    } else {
        CMAKE3_VERSION_COUNT + minor as usize
    };
    SCHEMAS[slot].get_or_init(|| {
        let names: HashSet<_> = native_layers(major, minor)
            .map(|schema| schema.command_name.as_str())
            .collect();
        let version = (major, minor);
        let mut specs = super::all_lookup::schemas(version);
        specs.extend(super::all_build::schemas(version));
        specs.extend(super::all_process::schemas(version));
        specs.extend(super::all_transform::schemas(version));
        specs
            .into_iter()
            .filter(|(name, _)| names.contains(name))
            .map(|(name, args)| make_schema(name, args))
            .collect()
    })
}
