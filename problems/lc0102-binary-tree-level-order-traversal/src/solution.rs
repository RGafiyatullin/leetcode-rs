use super::*;

pub struct Solution;

use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

impl Solution {
    pub fn level_order(mut root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut queue = VecDeque::new();
        let mut out = Vec::new();

        if let Some(root) = root.take() {
            out.push(vec![]);
            queue.push_back((root, 1));
        }

        while let Some((node, level)) = queue.pop_front() {
            let mut node = node.borrow_mut();

            use std::cmp::Ordering;
            match out.len().cmp(&level) {
                Ordering::Equal => {},
                Ordering::Less => {
                    out.push(vec![]);
                },
                Ordering::Greater => panic!(
                    "Queue order violated! [out.len = {}; node.level = {}]",
                    out.len(),
                    level
                ),
            }

            out.last_mut().expect("Empty `out`").push(node.val);

            if let Some(left) = node.left.take() {
                queue.push_back((left, level + 1));
            }
            if let Some(right) = node.right.take() {
                queue.push_back((right, level + 1));
            }
        }

        out
    }
}
