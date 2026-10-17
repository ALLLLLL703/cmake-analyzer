//! Share target-slot histories in a flat arena during grammar matching.
//!
//! Chains increase by argument index; intersection retains only bindings common to
//! equally specific parses. Arena indices are valid only inside this match.
// Contains AI-generated documentation additions.
use crate::parser::semanticer::model::TargetTrailId;
use std::cell::RefCell;

/// One append-only target-slot history node. Matching shares prefixes instead of cloning growing index lists; construction has no external effects.
///
/// [`TargetTrails`] owns the flat node arena.
///
/// # Examples
/// ```text
/// let node = Node { previous: None, argument: 2 };
/// ```
#[derive(Clone, Copy)]
struct Node {
    /// Earlier target-slot history tail in this arena.
    /// Example: `node.previous.is_none()`.
    previous: Option<TargetTrailId>,
    /// Current source argument slot; chains increase by argument index.
    /// Example: `node.argument == 2`.
    argument: usize,
}
/// Append-only shared histories for one full-call target-role match. Flat ownership avoids recursive chain destruction; mutation is local to this arena, with no I/O.
///
/// [`Self::intersect`] retains agreed slots across ambiguous branches.
///
/// # Flow
/// ```text
///  [state A]       [state B]
///      |              |
///      v              v
///  [tail A]        [tail B]
///      \             /
///       +----+------+
///            v
///      [shared prefix]
/// ```
///
/// # Examples
/// ```text
/// let trails = TargetTrails::default();
/// let tail = trails.push(None, 2);
/// assert_eq!(trails.indices(tail), vec![2]);
/// ```
#[derive(Default)]
pub(super) struct TargetTrails(
    /// Match-local nodes behind interior mutability. Example: `trails.0.borrow().len()`.
    RefCell<Vec<Node>>,
);
impl TargetTrails {
    /// Append one target binding to a shared history. Mutates only the interior arena; no I/O.
    ///
    /// # Arguments
    /// - `self` `[in,out]`: shared arena receiving one node through RefCell.
    /// - `previous` `[in]`: optional earlier tail in this same arena.
    /// - `argument` `[in]`: source-slot index appended after earlier slots.
    ///
    /// # Returns
    /// Some ID for the newly appended node; this method never returns None.
    ///
    /// # Panics
    /// Panics on an overlapping RefCell borrow. Internal callers release borrows before appending.
    ///
    /// # Examples
    /// ```text
    /// let trails = TargetTrails::default();
    /// let tail = trails.push(None, 2);
    /// assert_eq!(trails.indices(tail), vec![2]);
    /// ```
    pub(super) fn push(
        &self,
        previous: Option<TargetTrailId>,
        argument: usize,
    ) -> Option<TargetTrailId> {
        let mut nodes = self.0.borrow_mut();
        let id = TargetTrailId::new(nodes.len());
        nodes.push(Node { previous, argument });
        Some(id)
    }
    /// Materialize a shared history in source-slot order. No mutation or I/O.
    ///
    /// # Arguments
    /// - `self` `[in]`: arena owning all history nodes.
    /// - `trail` `[in]`: optional valid tail, consumed as traversal state.
    ///
    /// # Returns
    /// Argument indices in insertion order; empty for None.
    ///
    /// # Panics
    /// Panics for an invalid/foreign trail index or overlapping mutable RefCell borrow. Histories must be acyclic.
    ///
    /// # Examples
    /// ```text
    /// let trails = TargetTrails::default();
    /// let tail = trails.push(None, 1);
    /// assert_eq!(trails.indices(tail), vec![1]);
    /// ```
    pub(super) fn indices(&self, mut trail: Option<TargetTrailId>) -> Vec<usize> {
        let nodes = self.0.borrow();
        let mut result = Vec::new();
        while let Some(id) = trail {
            let node = nodes[id.index()];
            result.push(node.argument);
            trail = node.previous;
        }
        result.reverse();
        result
    }
    // Descending argument chains intersect until they reach a shared arena tail.
    /// Retain argument bindings common to two increasing histories, reusing any shared tail. May append common nodes locally; no I/O.
    ///
    /// # Arguments
    /// - `self` `[in,out]`: interior arena read during comparison and appended after releasing its borrow.
    /// - `a` `[in]`: first valid history tail in this arena.
    /// - `b` `[in]`: second valid history tail in this arena.
    ///
    /// # Returns
    /// Tail of their binding intersection; None when no argument is common.
    ///
    /// # Panics
    /// Panics for foreign/invalid indices or conflicting RefCell borrows; chains must increase by argument index.
    ///
    /// # Algorithm
    /// Walk backward in descending argument order, advancing the greater head. Save equal heads until tails coincide or either chain ends, then rebuild saved common heads onto the shared tail.
    ///
    /// # Flow
    /// ```text
    ///  [tail A]             [tail B]
    ///      |                    |
    ///      +---------+----------+
    ///                v
    ///           <same tail?> --yes--> [reuse tail]
    ///                | no                |
    ///           <head ordering?>          |
    ///            /      |       \         |
    ///        [A>]     [equal]   [B>]       |
    ///          |        |        |        |
    ///      [advance A] [save] [advance B]  |
    ///          +--------+--------+        |
    ///                |                   |
    ///           [repeat]          [rebuild saved heads]
    /// ```
    ///
    /// # Examples
    /// ```text
    /// let trails = TargetTrails::default();
    /// let a = trails.push(None, 1);
    /// let b = trails.push(a, 3);
    /// assert_eq!(trails.indices(trails.intersect(a, b)), vec![1]);
    /// ```
    pub(super) fn intersect(
        &self,
        mut a: Option<TargetTrailId>,
        mut b: Option<TargetTrailId>,
    ) -> Option<TargetTrailId> {
        let mut common = Vec::new();
        {
            let nodes = self.0.borrow();
            while a != b {
                let (Some(left), Some(right)) = (a, b) else {
                    a = None;
                    break;
                };
                let left = nodes[left.index()];
                let right = nodes[right.index()];
                match left.argument.cmp(&right.argument) {
                    std::cmp::Ordering::Greater => a = left.previous,
                    std::cmp::Ordering::Less => b = right.previous,
                    std::cmp::Ordering::Equal => {
                        common.push(left.argument);
                        a = left.previous;
                        b = right.previous;
                    }
                }
            }
        }
        for argument in common.into_iter().rev() {
            a = self.push(a, argument);
        }
        a
    }
}
