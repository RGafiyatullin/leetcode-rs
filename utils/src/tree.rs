use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;
pub type Node = Rc<RefCell<TreeNode>>;

pub struct TreeNode {
    pub val: i32,
    pub left: Option<Node>,
    pub right: Option<Node>,
}

pub fn tree_from_bft(bft: &[Option<i32>]) -> Option<Node> {
    let mut bft = bft.into_iter().copied().fuse();
    if let Some(Some(root_val)) = bft.next() {
        let root = Rc::new(RefCell::new(TreeNode { val: root_val, left: None, right: None }));

        let mut queue = VecDeque::from([root.clone()]);
        while let Some(parent) = queue.pop_front() {
            let mut parent = parent.borrow_mut();
            if let Some(next) = bft.next() {
                if let Some(left_val) = next {
                    let left =
                        Rc::new(RefCell::new(TreeNode { val: left_val, left: None, right: None }));
                    parent.left = Some(Rc::clone(&left));
                    queue.push_back(left);
                }
            } else {
                break
            }
            if let Some(next) = bft.next() {
                if let Some(right_val) = next {
                    let right =
                        Rc::new(RefCell::new(TreeNode { val: right_val, left: None, right: None }));
                    parent.right = Some(Rc::clone(&right));
                    queue.push_back(right)
                }
            } else {
                break
            }
        }

        Some(root)
    } else {
        None
    }
}

pub fn tree_to_bft(tree: Option<Node>) -> Vec<Option<i32>> {
    let mut out = Vec::new();

    let mut tasks = VecDeque::from([tree]);

    while let Some(node_opt) = tasks.pop_front() {
        out.push(node_opt.map(|node| {
            let node = node.borrow();
            tasks.extend([node.left.clone(), node.right.clone()]);
            node.val
        }))
    }

    while let Some(None) = out.last() {
        out.pop();
    }

    out
}

#[test]
fn test_tree_from_and_to_bft() {
    const NODE_COUNT: usize = 1000000;
    let bft_in = (0..NODE_COUNT)
        .map(|i| if i % 7 == 1 || i % 11 == 1 { None } else { Some(i as i32) })
        .chain(vec![Some(NODE_COUNT as i32)])
        .collect::<Vec<_>>();
    let tree = tree_from_bft(&bft_in[..]);
    let bft_out = tree_to_bft(tree);
    assert_eq!(bft_in, bft_out);
}
