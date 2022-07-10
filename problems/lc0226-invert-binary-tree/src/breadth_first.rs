use super::*;

pub struct Solution;

use std::rc::Rc;
use std::cell::RefCell;

use std::collections::VecDeque;

impl Solution {
    pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(root) = root {
            let mut queue: VecDeque<_> = vec![Rc::clone(&root)].into();
            
            while let Some(node) = queue.pop_front() {
                let mut node = node.borrow_mut();
                let mut next_left = None;
                let mut next_right = None;
                if let Some(left) = node.left.take() {
                    queue.push_back(Rc::clone(&left));
                    next_right = Some(left);
                }
                if let Some(right) = node.right.take() {
                    queue.push_back(Rc::clone(&right));
                    next_left = Some(right);
                }
                node.left = next_left;
                node.right = next_right;
            }

            Some(root)
        } else {
            None
        }
    }
}
