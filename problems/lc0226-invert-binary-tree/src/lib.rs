
// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
  pub val: i32,
  pub left: Option<std::rc::Rc<std::cell::RefCell<Self>>>,
  pub right: Option<std::rc::Rc<std::cell::RefCell<Self>>>,
}

pub mod breadth_first;
pub mod depth_first;