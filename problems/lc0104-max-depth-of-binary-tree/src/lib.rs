pub struct Solution;

pub use utils::tree::TreeNode;

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut max = 0;

        let mut tasks = root.into_iter().map(|node| (node, 1)).collect::<Vec<_>>();

        while let Some(task) = tasks.pop() {
            let (node, depth) = task;

            max = std::cmp::max(max, depth);

            let node = (*node).borrow();

            if let Some(left) = node.left.as_ref() {
                tasks.push((Rc::clone(left), depth + 1));
            }
            if let Some(right) = node.right.as_ref() {
                tasks.push((Rc::clone(right), depth + 1));
            }
        }

        max
    }
}
