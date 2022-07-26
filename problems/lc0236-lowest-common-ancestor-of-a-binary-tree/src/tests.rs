use crate::solution::*;

use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

const CASES: &[(&[Option<i32>], i32, i32, i32)] = &[
    (
        &[
            Some(3),
            Some(5),
            Some(1),
            Some(6),
            Some(2),
            Some(0),
            Some(8),
            None,
            None,
            Some(7),
            Some(4),
        ],
        5,
        1,
        3,
    ),
    (
        &[
            Some(3),
            Some(5),
            Some(1),
            Some(6),
            Some(2),
            Some(0),
            Some(8),
            None,
            None,
            Some(7),
            Some(4),
        ],
        5,
        4,
        5,
    ),
];

fn construct_tree(bft: &[Option<i32>]) -> Option<Node> {
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

#[test]
fn run_all_cases() {
    for &(bft, p, q, expected) in CASES.iter() {
        let tree = construct_tree(bft);
        let p = Some(Rc::new(RefCell::new(TreeNode {
            val: p,
            left: None,
            right: None,
        })));
        let q = Some(Rc::new(RefCell::new(TreeNode {
            val: q,
            left: None,
            right: None,
        })));
        let actual = Solution::lowest_common_ancestor(tree, p, q)
            .unwrap()
            .borrow()
            .val;

        assert_eq!(actual, expected);
    }
}

#[test]
fn run_specific_case() {
    let &(bft, p, q, expected) = &CASES[1];

    let tree = construct_tree(bft);
    let p = Some(Rc::new(RefCell::new(TreeNode {
        val: p,
        left: None,
        right: None,
    })));
    let q = Some(Rc::new(RefCell::new(TreeNode {
        val: q,
        left: None,
        right: None,
    })));
    let actual = Solution::lowest_common_ancestor(tree, p, q)
        .unwrap()
        .borrow()
        .val;

    assert_eq!(actual, expected);
}
