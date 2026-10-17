use super::{CommandSchema, MAX_VERSION, make_schema, native_layers};
use std::{
    collections::HashSet,
    sync::{LazyLock, OnceLock},
};

const CMAKE3_VERSION_COUNT: usize = 32;
const SUPPORTED_VERSION_COUNT: usize = CMAKE3_VERSION_COUNT + MAX_VERSION.1 as usize + 1;
static SCHEMAS: LazyLock<[OnceLock<Vec<CommandSchema>>; SUPPORTED_VERSION_COUNT]> =
    LazyLock::new(|| std::array::from_fn(|_| OnceLock::new()));

/// Cache only versions actually requested; completion borrows their immutable grammar trees.
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
