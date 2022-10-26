use utils::tree::TreeNode;
pub struct Solution;

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn sorted_array_to_bst(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        assert!(nums.len() >= 1);
        assert!(nums.len() <= 10_000);

        let mut root = Option::<Rc<RefCell<TreeNode>>>::None;

        let mut tasks = vec![Task {
            assert_lt: None,
            assert_gt: None,
            items: &nums[..],
            target: Target::Root(&mut root),
        }];

        while let Some(task) = tasks.pop() {
            if !task.items.is_empty() {
                let mid_idx = task.items.len() / 2;
                let mid_val = task.items[mid_idx];

                if let Some(assert_lt) = task.assert_lt {
                    assert!(mid_val < assert_lt);
                }
                if let Some(assert_gt) = task.assert_gt {
                    assert!(mid_val > assert_gt);
                }

                let node = TreeNode { val: mid_val, left: None, right: None };
                let node = RefCell::new(node);
                let node = Rc::new(node);

                {
                    let node = Rc::clone(&node);
                    match task.target {
                        Target::Root(root) => {
                            *root = Some(node);
                        },
                        Target::LeftOf(parent) => {
                            let mut parent = parent.borrow_mut();
                            assert!(parent.left.is_none());
                            parent.left = Some(node);
                        },
                        Target::RightOf(parent) => {
                            let mut parent = parent.borrow_mut();
                            assert!(parent.right.is_none());
                            parent.right = Some(node);
                        },
                    }
                }

                tasks.push(Task {
                    assert_lt: Some(mid_val),
                    assert_gt: task.assert_gt,
                    items: &task.items[0..mid_idx],
                    target: Target::LeftOf(Rc::clone(&node)),
                });

                tasks.push(Task {
                    assert_lt: task.assert_lt,
                    assert_gt: Some(mid_val),
                    items: &task.items[mid_idx + 1..],
                    target: Target::RightOf(Rc::clone(&node)),
                });
            }
        }

        root
    }
}

struct Task<'a> {
    assert_lt: Option<i32>,
    assert_gt: Option<i32>,

    items: &'a [i32],
    target: Target<'a>,
}

enum Target<'a> {
    Root(&'a mut Option<Rc<RefCell<TreeNode>>>),
    LeftOf(Rc<RefCell<TreeNode>>),
    RightOf(Rc<RefCell<TreeNode>>),
}
