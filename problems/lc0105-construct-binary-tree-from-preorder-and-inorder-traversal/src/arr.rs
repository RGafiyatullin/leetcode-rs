#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

// impl TreeNode {
//     #[inline]
//     pub fn new(val: i32) -> Self {
//         TreeNode {
//             val,
//             left: None,
//             right: None,
//         }
//     }
// }

pub struct Solution;

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        let mut inorder_rev = vec![0; 6001];

        inorder.into_iter().enumerate().for_each(|(idx, v)| {
            inorder_rev[to_key(v)] = idx;
        });

        let mut root = None;

        for val in preorder {
            insert_into(&mut root, val, &inorder_rev)
        }

        root
    }
}

fn insert_into(node: &mut Option<Rc<RefCell<TreeNode>>>, val: i32, inorder: &[usize]) {
    if let Some(node) = node.as_ref() {
        let mut node = node.borrow_mut();
        assert_ne!(node.val, val);

        let node_ord = inorder[to_key(node.val)];
        let ord = inorder[to_key(val)];

        if ord < node_ord {
            insert_into(&mut node.left, val, inorder);
        } else {
            insert_into(&mut node.right, val, inorder);
        }
    } else {
        *node = Some(make_node(val));
    }
}

fn make_node(val: i32) -> Rc<RefCell<TreeNode>> {
    Rc::new(RefCell::new(TreeNode {
        val,
        left: None,
        right: None,
    }))
}

fn to_key(v: i32) -> usize {
    assert!(v >= -3000);
    assert!(v <= 3000);
    (v + 3000) as usize
}
