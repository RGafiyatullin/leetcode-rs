pub struct Solution;

use std::collections::{HashMap, HashSet};

type EdgeSet = HashMap<(usize, usize), HashSet<(usize, usize)>>;

impl Solution {
    pub fn max_area_of_island(grid: Vec<Vec<i32>>) -> i32 {
        let mut max_area: usize = 0;

        let mut edges: EdgeSet = Default::default();
        for (lo, hi) in grid_to_edges(&grid) {
            edges.entry(lo).or_default().insert(hi);
            edges.entry(hi).or_default().insert(lo);
        }

        while let Some(&entry_point) = edges.keys().next() {
            let vertices_count = take_graph(&mut edges, entry_point).into_iter().count();
            max_area = std::cmp::max(vertices_count, max_area);
        }

        max_area as i32
    }
}

fn take_graph(
    edges: &mut EdgeSet,
    entry_point: (usize, usize),
) -> impl IntoIterator<Item = (usize, usize)> {
    let mut queue = vec![entry_point];

    let mut visited = HashSet::new();

    while let Some(vertex) = queue.pop() {
        visited.insert(vertex);
        if let Some(outbound) = edges.remove(&vertex) {
            for neighbour in outbound {
                queue.push(neighbour);
            }
        }
    }

    visited
}

type CellType = i32;
const LAND: CellType = 1;

fn grid_to_edges<'a, G, R>(
    grid: &'a G,
) -> impl IntoIterator<Item = ((usize, usize), (usize, usize))> + 'a
where
    G: AsRef<[R]> + 'a,
    R: AsRef<[CellType]> + 'a,
{
    grid.as_ref()
        .iter()
        .enumerate()
        .scan(Option::<&R>::None, |prev_row, (row_idx, row)| {
            let prev_row = std::mem::replace(prev_row, Some(row));

            let row_edges = row
                .as_ref()
                .iter()
                .enumerate()
                .scan(
                    Option::<CellType>::None,
                    move |prev_cell, (col_idx, sq_type): (usize, &CellType)| {
                        let sq_type = *sq_type;

                        let prev_cell = std::mem::replace(prev_cell, Some(sq_type));

                        let this_is_land = sq_type == LAND;
                        let left_is_land = prev_cell.map_or(false, |c| c == LAND);
                        let up_is_land = prev_row.map_or(false, |r| r.as_ref()[col_idx] == LAND);

                        let mut edges = [None, None, None];

                        if this_is_land && up_is_land {
                            edges[0] = Some(((row_idx - 1, col_idx), (row_idx, col_idx)));
                        }

                        if this_is_land && left_is_land {
                            edges[1] = Some(((row_idx, col_idx - 1), (row_idx, col_idx)));
                        }

                        if this_is_land {
                            edges[2] = Some(((row_idx, col_idx), (row_idx, col_idx)));
                        }

                        Some(edges)
                    },
                )
                .flatten();

            Some(row_edges)
        })
        .flatten()
        .filter_map(std::convert::identity)
}
