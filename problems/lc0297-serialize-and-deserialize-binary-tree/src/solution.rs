pub use utils::tree::TreeNode;

use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

pub struct Codec;

impl Codec {
    pub fn new() -> Self {
        Self
    }

    pub fn serialize(&self, root: Option<Rc<RefCell<TreeNode>>>) -> String {
        let mut out = String::new();

        let mut tasks = VecDeque::from([root]);

        while let Some(node_opt) = tasks.pop_front() {
            if let Some(node) = node_opt {
                let node = node.borrow();

                tasks.extend([node.left.clone(), node.right.clone()]);
                out.push_str(node.val.to_string().as_str());
                out.push(' ');
            } else {
                out.push_str("_ ");
            }
        }

        out
    }

    pub fn deserialize(&self, data: String) -> Option<Rc<RefCell<TreeNode>>> {
        let mut bft = data.split_ascii_whitespace().map(|s| {
            if s == "_" {
                None
            } else {
                Some(s.parse::<i32>().unwrap())
            }
        });

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
}
