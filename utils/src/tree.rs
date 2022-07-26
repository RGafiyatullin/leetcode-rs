use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;
pub type Node = Rc<RefCell<TreeNode>>;

pub struct TreeNode {
    pub val: i32,
    pub left: Option<Node>,
    pub right: Option<Node>,
}

pub fn construct_tree_from_bft(bft: &[Option<i32>]) -> Option<Node> {
    let mut bft = bft.into_iter().copied().fuse();
    if let Some(Some(root_val)) = bft.next() {
        let root = Rc::new(RefCell::new(TreeNode {
            val: root_val,
            left: None,
            right: None,
        }));

        let mut queue = VecDeque::from([root.clone()]);
        while let Some(parent) = queue.pop_front() {
            let mut parent = parent.borrow_mut();
            if let Some(next) = bft.next() {
                if let Some(left_val) = next {
                    let left = Rc::new(RefCell::new(TreeNode {
                        val: left_val,
                        left: None,
                        right: None,
                    }));
                    parent.left = Some(Rc::clone(&left));
                    queue.push_back(left);
                }
            } else {
                break;
            }
            if let Some(next) = bft.next() {
                if let Some(right_val) = next {
                    let right = Rc::new(RefCell::new(TreeNode {
                        val: right_val,
                        left: None,
                        right: None,
                    }));
                    parent.right = Some(Rc::clone(&right));
                    queue.push_back(right)
                }
            } else {
                break;
            }
        }

        Some(root)
    } else {
        None
    }
}
