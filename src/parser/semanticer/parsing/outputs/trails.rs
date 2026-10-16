use crate::parser::semanticer::model::TargetTrailId;
use std::cell::RefCell;

#[derive(Clone, Copy)]
struct Node {
    previous: Option<TargetTrailId>,
    argument: usize,
}
/// Repeat states share argument histories instead of copying every growing prefix.
/// An arena also avoids recursive linked-list destruction for large link lists.
#[derive(Default)]
pub(super) struct TargetTrails(RefCell<Vec<Node>>);
impl TargetTrails {
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
