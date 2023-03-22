pub struct Solution;

use std::collections::{HashMap, VecDeque};

const MAX_VERTICES: i32 = 100_000;
const MAX_DISTANCE: i32 = 10_000;

impl Solution {
    pub fn min_score(n: i32, roads: Vec<Vec<i32>>) -> i32 {
        assert!(n >= 2 && n <= MAX_VERTICES as i32);

        let n = n as usize;
        let final_destination = n - 1;

        let mut index: Vec<HashMap<usize, usize>> = vec![Default::default(); n];

        for entry in roads {
            let [vertex_a, vertex_b, edge_cost] = entry[..] else { panic!("Invalid entry: {:#?}", entry) };

            assert!(vertex_a >= 1 && vertex_a <= MAX_VERTICES);
            assert!(vertex_b >= 1 && vertex_b <= MAX_VERTICES);
            assert!(edge_cost >= 1 && edge_cost <= MAX_DISTANCE);

            let vertex_a = vertex_a as usize - 1;
            let vertex_b = vertex_b as usize - 1;
            let edge_cost = edge_cost as usize;

            index[vertex_a].insert(vertex_b, edge_cost);
            index[vertex_b].insert(vertex_a, edge_cost);
        }
        // eprintln!("INDEX: {:#?}", index);

        let mut memo: Vec<Option<usize>> = vec![Default::default(); n];

        let mut tasks = [Task { at: 0, min_road: usize::MAX }].into_iter().collect::<VecDeque<_>>();

        while let Some(Task { at, min_road }) = tasks.pop_front() {
            let should_give_up = memo[at].map(|mr| mr <= min_road).unwrap_or(false);

            if should_give_up {
                continue
            }

            memo[at] = Some(min_road);

            if at == final_destination {
                continue
            }

            for (&next, &cost) in index[at].iter() {
                tasks.push_back(Task { at: next, min_road: cost.min(min_road) });
            }
        }

        // eprintln!("MEMO: {:#?}", memo);

        memo.last().copied().flatten().map(|d| d as i32).unwrap_or(-1)
    }
}

#[derive(Debug, Clone, Copy)]
struct Task {
    at: usize,
    // cost_so_far: usize,
    min_road: usize,
}
