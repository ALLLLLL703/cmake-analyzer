//! Owner-qualified navigation requests and retained snapshots; no I/O on construction.
//! File IDs from different owners must never be looked up in the same map.

use crate::{
    parser::semanticer::model::{FileId, ReferenceIndex},
    workspace::model::DocumentSnapshot,
};
use std::{collections::HashMap, sync::Arc};

/// Selection policy used by [`super::locations::symbol`] for already-bound references.
/// [`super::query`] captures inputs before applying it; the policy itself has no side effects.
///
/// # Examples
/// ```text
/// let query = Query::References { include_declaration: false };
/// ```
#[derive(Clone, Copy)]
pub(in crate::server) enum Query {
    /// Return the selected symbol's source declaration, if any.
    /// Example: `Query::Definition` omits native commands without a source definition.
    Definition,
    /// Return bound occurrences. Example: `Query::References { include_declaration: true }`.
    References {
        /// Whether declaration ranges are included, even if repeated visits mark them as writes.
        /// Example: `Query::References { include_declaration: false }` excludes the declaration's exact file/span.
        include_declaration: bool,
    },
}
/// One owner's immutable navigation view captured by [`super::query`].
/// [`super::locations::query`] uses the index or replays a file-local index when absent.
/// Unlike completion candidates, these records carry bindings. Construction performs no I/O.
///
/// # Flow
/// ```text
///      [Owner A graph]             [Owner B graph]
///             |                          |
///             v                          v
///       [Input A]                  [Input B]
///        /     \                    /     \
///   [index] [files]             [index] [files]
///        \     /                    \     /
///     [qualified ranges]        [qualified ranges]
///             +------------+-------------+
///                          v
///                [URI/range deduplication]
/// ```
/// # Examples
/// ```text
/// let input = Input { snapshot, index: None, targets: None, files };
/// ```
pub(super) struct Input {
    /// Queried source revision. Example: `input.snapshot.semantics.id` selects its local file ID.
    pub snapshot: Arc<DocumentSnapshot>,
    /// Bindings for this graph owner only. Example: `input.index.as_ref().map(|i| i.variables.len())`.
    pub index: Option<Arc<ReferenceIndex>>,
    /// Owner-qualified target bindings, absent for file-only replay.
    /// Example: `input.targets.as_ref().map(|targets| targets.len())` counts declarations.
    pub targets: Option<Arc<Vec<crate::parser::build_struct::model::TargetDeclaration>>>,
    /// Owner-local file IDs mapped to client/encoded URI and matching immutable source.
    /// Example: `input.files.get(&input.snapshot.semantics.id)` finds the queried file's text.
    pub files: HashMap<FileId, (String, Arc<DocumentSnapshot>)>,
}
