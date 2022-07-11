use super::*;

pub struct Solution;

use std::collections::VecDeque;
use std::rc::Rc;
use std::cell::RefCell;

type Node = Rc<RefCell<TreeNode>>;

impl Solution {
    pub fn right_side_view(root: Option<Node>) -> Vec<i32> {
        let mut out: Vec<i32> = vec![];

        let mut queue = VecDeque::<(usize, Node)>::new();

        if let Some(node) = root {
            queue.push_back((0, node));
        }

        while let Some((h, node)) = queue.pop_back() {
            let mut node = node.borrow_mut();
            if out.len() < h + 1 {
                assert!(out.len() == h);
                out.push(node.val);
            }

            let next_h = h + 1;
            
            if let Some(left) = node.left.take() {
                queue.push_back((next_h, left));
            }

            if let Some(right) = node.right.take() {
                queue.push_back((next_h, right));
            }
            
        }

        out
    }
}
