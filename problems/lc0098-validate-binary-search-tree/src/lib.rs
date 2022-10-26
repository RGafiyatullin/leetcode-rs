use utils::tree::TreeNode;

pub struct Solution;

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let mut tasks = root
            .into_iter()
            .map(|node| Task { node, gt: None, lt: None })
            .collect::<Vec<_>>();

        while let Some(task) = tasks.pop() {
            let Task { node, gt, lt } = task;

            let node = node.borrow();
            let node_val = node.val;

            if let Some(gt) = gt {
                if node_val <= gt {
                    return false
                }
            }
            if let Some(lt) = lt {
                if node_val >= lt {
                    return false
                }
            }

            if let Some(node) = node.left.clone() {
                tasks.push(Task { node, gt, lt: Some(node_val) })
            }
            if let Some(node) = node.right.clone() {
                tasks.push(Task { node, gt: Some(node_val), lt })
            }
        }

        return true
    }
}

struct Task {
    node: Rc<RefCell<TreeNode>>,
    gt: Option<i32>,
    lt: Option<i32>,
}
