pub struct Solution;
use utils::tree::TreeNode;

use std::cell::RefCell;
use std::rc::Rc;

const MAX_EDGES: usize = 3;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Edge {
    Parent = 0,
    Left = 1,
    Right = 2,
}

impl Solution {
    pub fn max_path_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut nodes = vec![];
        init::from_root(&mut nodes, root);

        // graph_dump(&nodes);
        let mut scores = vec![Default::default(); nodes.len()];

        if !nodes.is_empty() {
            walk::from_all_nodes(&nodes, &mut scores);
        }

        // scores_dump(&scores);

        nodes
            .into_iter()
            .zip(scores)
            .flat_map(|(node, scores)| {
                let v = node.value;

                std::iter::once(v)
                    .chain(scores[0].map(|s| s + v))
                    .chain(scores[1].map(|s| s + v))
                    .chain(scores[2].map(|s| s + v))
                    .chain(scores[0].zip(scores[1]).map(|(s1, s2)| s1 + s2 + v))
                    .chain(scores[1].zip(scores[2]).map(|(s1, s2)| s1 + s2 + v))
                    .chain(scores[2].zip(scores[0]).map(|(s1, s2)| s1 + s2 + v))
            })
            .max()
            .unwrap_or_default()
    }
}

// fn graph_dump(nodes: &[Node]) {
//     eprintln!("===");
//     for (idx, node) in nodes.iter().copied().enumerate() {
//         eprintln!("[{:5}] {:?}", idx, node);
//     }
//     eprintln!("===");
// }

// fn scores_dump(scores: &[[Option<i32>; MAX_EDGES]]) {
//     eprintln!("===");
//     for (idx, scores) in scores.iter().copied().enumerate() {
//         eprintln!("[{:5}] {:?}", idx, scores);
//     }
//     eprintln!("===");
// }

#[derive(Debug, Clone, Copy)]
pub struct Node {
    value: i32,
    edges: [Option<usize>; MAX_EDGES],
}

mod init {
    use super::*;
    use std::collections::VecDeque;

    struct Task {
        node: Option<Rc<RefCell<TreeNode>>>,
        parent: Option<(usize, Edge)>,
    }

    #[inline(always)] // leetcode, i hate you not running everything with `--release`...
    pub fn from_root(nodes: &mut Vec<Node>, root: Option<Rc<RefCell<TreeNode>>>) {
        let mut tasks = std::iter::once(Task { node: root, parent: None }).collect::<VecDeque<_>>();
        while let Some(task) = tasks.pop_back() {
            process_task(&mut tasks, nodes, task);
        }
    }

    #[inline(always)] // leetcode, i hate you not running everything with `--release`...
    fn process_task(
        tasks: &mut VecDeque<Task>,
        nodes: &mut Vec<Node>,
        task: Task,
    ) -> Option<usize> {
        let node = task.node?;
        let node = node.borrow();

        let this_id = nodes.len();
        let parent_id_opt = task.parent.map(|(parent_id, this_side)| {
            nodes[parent_id].edges[this_side as usize] = Some(this_id);
            parent_id
        });

        nodes.push(Node { value: node.val, edges: [parent_id_opt, None, None] });
        tasks.extend([
            Task { node: node.left.clone(), parent: Some((this_id, Edge::Left)) },
            Task { node: node.right.clone(), parent: Some((this_id, Edge::Right)) },
        ]);

        Some(this_id)
    }
}

mod walk {
    use super::*;
    use std::collections::VecDeque;

    struct Task {
        this_id: usize,
        prev_id: Option<usize>,
        score: i32,
    }

    #[inline(always)] // leetcode, i hate you not running everything with `--release`...
    pub fn from_all_nodes(nodes: &[Node], scores: &mut [[Option<i32>; MAX_EDGES]]) {
        assert_eq!(nodes.len(), scores.len());

        let mut tasks = nodes
            .iter()
            .enumerate()
            .map(|(node_id, node)| Task { this_id: node_id, prev_id: None, score: 0 })
            .collect::<VecDeque<_>>();

        while let Some(task) = tasks.pop_back() {
            process_task(&mut tasks, nodes, scores, task);
        }
    }

    #[inline(always)] // leetcode, i hate you not running everything with `--release`...
    fn process_task(
        tasks: &mut VecDeque<Task>,
        nodes: &[Node],
        scores: &mut [[Option<i32>; MAX_EDGES]],
        task: Task,
    ) -> Option<()> {
        let this_id = task.this_id;
        let prev_id = task.prev_id;

        let from_direction = nodes[task.this_id]
            .edges
            .iter()
            .copied()
            .enumerate()
            .find(|(_, node_id)| *node_id == prev_id)
            .map(|(idx, _)| idx);

        if let Some(from_direction) = from_direction {
            let score = &mut scores[this_id][from_direction];
            if score.iter().copied().any(|v| v >= task.score) {
                return None
            }
            *score = Some(task.score);
        }

        tasks.extend(
            nodes[this_id]
                .edges
                .iter()
                .copied()
                .filter_map(std::convert::identity)
                .filter(|&next_id| Some(next_id) != prev_id)
                .map(|next_id| Task {
                    this_id: next_id,
                    prev_id: Some(this_id),
                    score: task.score + nodes[this_id].value,
                }),
        );

        Some(())
    }
}
