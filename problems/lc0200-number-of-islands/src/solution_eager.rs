pub struct Solution;

use std::collections::{HashMap, HashSet};
type EdgeSet = HashMap<(usize, usize), HashSet<(usize, usize)>>;

impl Solution {
    pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {
        let mut graph_count = 0;

        let mut edges: EdgeSet = Default::default();
        for (lo, hi) in grid_to_edges(&grid) {
            edges.entry(lo).or_default().insert(hi);
            edges.entry(hi).or_default().insert(lo);
        }

        while let Some(&entry_point) =  edges.keys().next() {
            let _vertices_count = take_graph(&mut edges, entry_point);
            graph_count += 1;
            // eprintln!("#{} [vertices-count: {}]", graph_count, vertices_count);
        }

        graph_count
    }
}

fn take_graph(edges: &mut EdgeSet, entry_point: (usize, usize)) {
    let mut queue = vec![entry_point];

    while let Some(vertex) = queue.pop() {
        if let Some(outbound) = edges.remove(&vertex) {
            for neighbour in outbound {
                queue.push(neighbour);
            }
        }
    }
}

fn grid_to_edges<'a, G, R>(grid: &'a G) -> impl IntoIterator<Item = ((usize, usize), (usize, usize))> + 'a
where
    G: AsRef<[R]> + 'a,
    R: AsRef<[char]> + 'a,
{
    let mut edges = Vec::new();

    for (row_idx, row) in grid.as_ref().iter().enumerate() {
        for (col_idx, this_is_land) in row.as_ref().iter().enumerate() {
            let this_is_land = *this_is_land;

            if this_is_land == '1' {
                edges.push(((row_idx, col_idx), (row_idx, col_idx)));

                if row_idx != 0 && grid.as_ref()[row_idx - 1].as_ref()[col_idx] == '1' {
                    edges.push(((row_idx - 1, col_idx), (row_idx, col_idx)));
                }

                if col_idx != 0 && grid.as_ref()[row_idx].as_ref()[col_idx - 1] == '1' {
                    edges.push( ((row_idx, col_idx - 1), (row_idx, col_idx)) );
                }
            }
        }
    }
    
    edges
}
