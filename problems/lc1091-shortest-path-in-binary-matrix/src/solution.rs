pub struct Solution;

use std::collections::VecDeque;

impl Solution {
    pub fn shortest_path_binary_matrix(grid: Vec<Vec<i32>>) -> i32 {
        assert!(grid.len() >= 1);
        assert!(grid.len() <= 100);

        let mut queue = VecDeque::<(usize, usize, usize)>::new();
        let mut visited = grid
            .iter()
            .map(|r| vec![Option::<usize>::None; r.len()])
            .collect::<Vec<_>>();

        if grid[0][0] == 0 {
            queue.push_back((0, 0, 1));
        }

        let destination = (
            grid.len() - 1,
            grid.last().map(|row| row.len() - 1).expect("1 <= n <= 100"),
        );
        // eprintln!("destination: {:?}", destination);

        let mut best_distance = Option::<usize>::None;

        while let Some((row, col, steps_so_far)) = queue.pop_front() {
            // eprintln!("- {:?} @ {}", (row, col), steps_so_far);
            if (row, col) == destination {
                best_distance = best_distance
                    .map(|b| std::cmp::min(b, steps_so_far))
                    .or(Some(steps_so_far));
            }
            for (n_row, n_col) in neighbours(&grid, row, col) {
                // eprintln!("  ? {:?}", (n_row, n_col));
                let n_steps = steps_so_far + 1;
                match (grid[n_row][n_col] == 0, visited[n_row][n_col]) {
                    (false, _) => {}
                    (true, Some(better)) if better <= n_steps => {}
                    (true, _) => {
                        visited[n_row][n_col] = Some(n_steps);
                        queue.push_back((n_row, n_col, n_steps));
                    }
                }
            }
        }

        best_distance.map(|i| i as i32).unwrap_or(-1)
    }
}

fn neighbours<'a, G, R>(
    grid: &'a G,
    row: usize,
    col: usize,
) -> impl Iterator<Item = (usize, usize)> + 'a
where
    G: AsRef<[R]> + 'a,
    R: AsRef<[i32]> + 'a,
{
    let grid = grid.as_ref();
    let max_row = grid.len() - 1;

    let row_lo = row.checked_sub(1).unwrap_or(0);
    let row_hi = std::cmp::min(row + 1, max_row);

    (row_lo..=row_hi)
        .map(move |row_idx| {
            let row = grid[row_idx].as_ref();
            let max_col = row.len() - 1;
            let col_lo = col.checked_sub(1).unwrap_or(0);
            let col_hi = std::cmp::min(max_col, col + 1);

            (col_lo..=col_hi).map(move |col_idx| (row_idx, col_idx))
        })
        .flatten()
        .filter(move |(r, c)| (*r, *c) != (row, col))
}
