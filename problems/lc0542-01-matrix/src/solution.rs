pub struct Solution;

use std::collections::VecDeque;

impl Solution {
    pub fn update_matrix(mut mat: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let row_count = mat.len();
        let col_count = mat[0].len();

        let mut queue = VecDeque::<(usize, usize, i32)>::new();

        for row in 0..mat.len() {
            for col in 0..mat[0].len() {
                if mat[row][col] == 0 {
                    queue.push_back((row, col, 0));
                } else {
                    mat[row][col] = -1;
                }
            }
        }

        while let Some((row, col, dist)) = queue.pop_front() {
            // eprintln!("VISITING [{}; {}]", row, col);
            for (n_row, n_col) in neighbours(row, col, row_count, col_count) {
                // eprintln!("  CONSIDERING [{}; {}]", n_row, n_col);
                if mat[n_row][n_col] == -1 || mat[n_row][n_col] > dist + 1 {
                    // eprintln!("  UPDATING [{}; {}]", n_row, n_col);
                    mat[n_row][n_col] = dist + 1;
                    queue.push_back((n_row, n_col, dist + 1));
                }
            }
        }

        mat
    }
}

fn neighbours(
    row: usize,
    col: usize,
    row_count: usize,
    col_count: usize,
) -> impl Iterator<Item = (usize, usize)> {
    let max_row = row_count - 1;
    let row_lo = row.checked_sub(1).unwrap_or(0);
    let row_hi = std::cmp::min(row + 1, max_row);

    (row_lo..=row_hi)
        .map(move |row_idx| {
            let max_col = col_count - 1;
            let col_lo = col.checked_sub(1).unwrap_or(0);
            let col_hi = std::cmp::min(max_col, col + 1);

            (col_lo..=col_hi).map(move |col_idx| (row_idx, col_idx))
        })
        .flatten()
        .filter(move |(r, c)| (*r, *c) != (row, col))
        .filter(move |(r, c)| *r == row || *c == col)
}
