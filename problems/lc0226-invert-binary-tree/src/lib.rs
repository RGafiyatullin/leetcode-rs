pub struct Solution;

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
  pub val: i32,
  pub left: Option<Rc<RefCell<TreeNode>>>,
  pub right: Option<Rc<RefCell<TreeNode>>>,
}

// 
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }

use std::rc::Rc;
use std::cell::RefCell;

impl Solution {
    pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(root) = root {
            let mut queue = vec![Rc::clone(&root)];
            
            while let Some(node) = queue.pop() {
                let mut node = node.borrow_mut();
                let mut next_left = None;
                let mut next_right = None;
                if let Some(left) = node.left.take() {
                    queue.push(Rc::clone(&left));
                    next_right = Some(left);
                }
                if let Some(right) = node.right.take() {
                    queue.push(Rc::clone(&right));
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