use crate::solution::*;
use utils::tree::{self, TreeNode};

use std::cell::RefCell;
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

#[test]
fn run_all_cases() {
    for &(bft, p, q, expected) in CASES.iter() {
        let tree = tree::tree_from_bft(bft);
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

    let tree = tree::tree_from_bft(bft);
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
