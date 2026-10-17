//! Assemble project/package, find, property and control-flow overrides.
//! `schemas` concatenates owned [`ArgSchemaKind`] trees for the final keyword
//! cache; it performs no lookup, property mutation, project execution or I/O.
//! For example, `include(CTest RESULT_VARIABLE loaded)` marks a module slot
//! and an output-variable slot without loading CTest during construction.

use super::*;
mod control;
mod find;
mod property;

/// Collect all lookup/control overrides in their effective append order.
/// Allocates grammar data only; no external side effects.
///
/// # Arguments
/// - `version` `[in]`: requested CMake major/minor controlling syntax additions.
///
/// # Returns
/// Owned (command name, ordered roots) pairs: project/package first, then find, property and
/// control groups.
///
/// # Algorithm
/// Seed project and find_package, then append each focused child catalogue. No deduplication
/// occurs here; final layering handles replacements.
///
/// # Flow
/// ```text
///                             [schemas inputs]
///                                     |
///                  +------------------+------------------+
///                  v                                     v
/// +--------------------------------+    +--------------------------------+
/// |     project / find_package     |    |   find / property / control    |
/// +----------------+---------------+    +----------------+---------------+
///                  |                                     |
///                  +------------------+------------------+
///                                     v
///                              [Owned grammar]
/// ```
///
/// # Examples
/// ```text
/// let overrides = schemas((4, 4));
/// assert!(overrides.iter().any(|(name, _)| *name == "project"));
/// ```
pub(super) fn schemas(version: (u32, u32)) -> Vec<(&'static str, Vec<ArgSchemaKind>)> {
    let mut result = vec![
        ("project", super::project::args(version)),
        ("find_package", super::find_package::args(version)),
    ];
    result.extend(find::schemas(version));
    result.extend(property::schemas(version));
    result.extend(control::schemas(version));
    result
}
// Contains AI-generated test-module documentation edits.
/// Lookup grammar regression tests loaded from the package test tree.
#[cfg(test)]
#[path = "../../../../../tests/unit/parser/semanticer/keyword_lookup.rs"]
mod tests;
