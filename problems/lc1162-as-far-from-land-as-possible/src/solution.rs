pub struct Solution;

use std::collections::VecDeque;

impl Solution {
    pub fn max_distance(grid: Vec<Vec<i32>>) -> i32 {
        let mut dist_map: Vec<Vec<Option<usize>>> =
            grid.iter().map(|row| vec![None; row.len()]).collect();
        let mut queue: VecDeque<((usize, usize), usize)> = VecDeque::new();

        for (row_idx, row) in grid.iter().enumerate() {
            for (col_idx, &cell) in row.iter().enumerate() {
                if cell == 1 {
                    queue.push_back(((row_idx, col_idx), 0));
                }
            }
        }

        while let Some(((row_idx, col_idx), this_dist)) = queue.pop_front() {
            if let Some(prior_this_dist) = dist_map[row_idx][col_idx] {
                if prior_this_dist <= this_dist {
                    // this cell has already been visited with a smaller distance, so we can skip it
                    continue
                }
            }
            dist_map[row_idx][col_idx] = Some(this_dist);
            let next_dist = this_dist + 1;

            let left = col_idx.checked_sub(1).map(|c| (row_idx, c));
            let up = row_idx.checked_sub(1).map(|r| (r, col_idx));
            let right =
                Some(col_idx + 1).filter(|c| *c < dist_map[row_idx].len()).map(|c| (row_idx, c));
            let down = Some(row_idx + 1).filter(|r| *r < dist_map.len()).map(|r| (r, col_idx));

            left.into_iter()
                .chain(up)
                .chain(right)
                .chain(down)
                .for_each(|neighbour| queue.push_back((neighbour, next_dist)));
        }

        let dist = dist_map
            .into_iter()
            .map(IntoIterator::into_iter)
            .flatten()
            .filter_map(std::convert::identity)
            .max()
            .unwrap_or(0);
        if dist == 0 {
            -1
        } else {
            dist as i32
        }
    }
}
