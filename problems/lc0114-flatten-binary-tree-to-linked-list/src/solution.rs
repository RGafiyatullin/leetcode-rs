pub struct Solution;
use utils::tree::TreeNode;

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn flatten(root: &mut Option<Rc<RefCell<TreeNode>>>) {
        let (mut head, mut tail) = (
            Option::<Rc<RefCell<TreeNode>>>::None,
            Option::<Rc<RefCell<TreeNode>>>::None,
        );

        let mut queue: Vec<_> = root.take().into_iter().collect();

        while let Some(node) = queue.pop() {
            let mut node_mut = node.borrow_mut();
            let left_opt = node_mut.left.take();
            let right_opt = node_mut.right.take();
            std::mem::drop(node_mut);

            if let Some(tail) = (tail.as_mut().into_iter().chain(head.as_mut())).next() {
                let mut tail_mut = tail.borrow_mut();
                assert!(tail_mut.left.is_none());
                assert!(tail_mut.right.is_none());

                tail_mut.right = Some(Rc::clone(&node));
            }

            queue.extend(right_opt.into_iter().chain(left_opt));

            if head.is_none() {
                assert!(tail.is_none());
                head = Some(node);
            } else {
                tail = Some(node);
            }
        }

        *root = head;
    }
}
