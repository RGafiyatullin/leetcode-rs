pub use utils::tree::TreeNode;

pub struct Solution;

use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;
pub type Node = Rc<RefCell<TreeNode>>;

impl Solution {
    pub fn lowest_common_ancestor(
        root: Option<Node>,
        p: Option<Node>,
        q: Option<Node>,
    ) -> Option<Node> {
        let root =
            root.expect("p and q are defined, not equal to each other and exist in the tree");
        let p = p.expect("p and q are defined, not equal to each other and exist in the tree");
        let q = q.expect("p and q are defined, not equal to each other and exist in the tree");

        let p_val = p.borrow().val;
        let mut p_path = Option::<Path>::None;

        let q_val = q.borrow().val;
        let mut q_path = Option::<Path>::None;

        let mut search_queue = VecDeque::from([(root.to_owned(), Path::Nil)]);

        while let Some((node, path)) = search_queue.pop_front() {
            let node = node.borrow();
            let node_val = node.val;

            if node_val == p_val {
                assert!(p_path.is_none());
                p_path = Some(path.clone());
            }

            if node_val == q_val {
                assert!(q_path.is_none());
                q_path = Some(path.clone());
            }

            if p_path.is_none() || q_path.is_none() {
                if let Some(left) = node.left.to_owned() {
                    search_queue.push_back((left, path.go(LR::L)));
                }
                if let Some(right) = node.right.to_owned() {
                    search_queue.push_back((right, path.go(LR::R)));
                }
            }
        }

        p_path.zip(q_path).map(|(p_path, q_path)| {
            let p_path = p_path.to_vec();
            let q_path = q_path.to_vec();

            // eprintln!("p_path: {:?}", p_path);
            // eprintln!("q_path: {:?}", q_path);

            p_path
                .into_iter()
                .zip(q_path)
                .take_while(|(p, q)| p == q)
                .map(|(one, _)| one)
                .fold(root, |n, lr| {
                    let n = n.borrow();
                    match lr {
                        LR::L => n.left.clone().expect("we've been here"),
                        LR::R => n.right.clone().expect("we've been here"),
                    }
                })
        })
    }
}

#[derive(Debug, Clone)]
enum Path {
    Nil,
    Cons(LR, Rc<Self>),
}

impl Path {
    fn go(&self, lr: LR) -> Self {
        Self::Cons(lr, Rc::new(self.clone()))
    }

    fn as_cons(&self) -> Option<(LR, &Self)> {
        match self {
            Self::Nil => None,
            Self::Cons(lr, tail) => Some((*lr, tail)),
        }
    }
    fn to_vec(&self) -> Vec<LR> {
        let mut out = Vec::new();

        let mut this = self;
        while let Some((lr, tail)) = this.as_cons() {
            out.push(lr);
            this = tail;
        }
        out.reverse();

        out
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum LR {
    L,
    R,
}
